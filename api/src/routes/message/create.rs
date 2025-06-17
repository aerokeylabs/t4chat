use std::sync::Arc;

use axum::http::HeaderMap;
use axum::response::Sse;
use axum::response::sse::Event;
use futures::{Stream, StreamExt, pin_mut};
use tokio::sync::{Mutex, mpsc};

use crate::convex::messages::{
  CompleteMessageArgs, Message as ConvexMessage, MessagePart, MessageStatus, ModelParams, Role as ConvexRole,
};
use crate::convex::threads::Thread;
use crate::convex::{ConvexClient, ConvexError, messages, threads};
use crate::convex_serde;
use crate::openrouter::completions::{OpenrouterEvent, stream_completions, stream_openrouter_chat};
use crate::openrouter::title::generate_title_from_content;
use crate::openrouter::types::{ChatDelta, Message, Role};
use crate::openrouter::{OpenrouterClient, OpenrouterError};
use crate::prelude::*;

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ModelParamsRequest {
  pub reasoning_effort: String,
  pub include_search: bool,
}

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CreateMessageRequest {
  pub thread_id: String,
  pub response_message_id: String,
  pub model: String,
  pub model_params: Option<ModelParamsRequest>,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateMessageError {
  #[error("message not found")]
  MessageNotFound,
  #[error("thread not found")]
  ThreadNotFound,
  #[error("response message not pending")]
  ResponseMessageNotPending,
  #[error("unexpected error: {0}")]
  Unexpected(#[from] anyhow::Error),
  #[error("serialization error: {0}")]
  Serialization(#[from] convex_serde::SerError),
  #[error("failed to get response message: {0}")]
  Convex(#[from] ConvexError),
  #[error("no model specified")]
  NoModelSpecified,
}

into_response!(
  CreateMessageError {
    MessageNotFound => StatusCode::NOT_FOUND,
    ThreadNotFound => StatusCode::NOT_FOUND,
    ResponseMessageNotPending => StatusCode::BAD_REQUEST,
    NoModelSpecified => StatusCode::BAD_REQUEST,
    Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
    Serialization(_) => StatusCode::INTERNAL_SERVER_ERROR,
    Convex(_) => StatusCode::INTERNAL_SERVER_ERROR,
  }
);

struct ChatContext {
  model: String,
  message: ConvexMessage,
  thread: Thread,
  complete_args: CompleteMessageArgs,
  custom_key: Option<String>,
  set_title: bool,
  messages: Vec<Message>,
}

fn message_to_part(message: ConvexMessage) -> Message {
  let role = match message.role {
    ConvexRole::User => Role::User,
    ConvexRole::Assistant => Role::Assistant,
    ConvexRole::System => Role::System,
  };

  let content = message
    .parts
    .into_iter()
    .map(|part| match part {
      MessagePart::Text { text } => text,
    })
    .collect();

  Message { role, content }
}

#[tracing::instrument("create message", skip(state), err)]
#[axum::debug_handler]
pub async fn create_message(
  State(state): State<AppState>,
  headers: HeaderMap,
  Json(payload): Json<CreateMessageRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, CreateMessageError>>>, CreateMessageError> {
  debug!("creating chat with payload: {:?}", payload);

  let message_id = payload.response_message_id.trim();
  if message_id.is_empty() {
    return Err(CreateMessageError::MessageNotFound);
  }

  let thread_id = payload.thread_id.trim();
  if thread_id.is_empty() {
    return Err(CreateMessageError::ThreadNotFound);
  }

  let response_message_id = payload.response_message_id.trim();
  if response_message_id.is_empty() {
    return Err(CreateMessageError::MessageNotFound);
  }

  let model = payload.model.trim();
  if model.is_empty() {
    return Err(CreateMessageError::NoModelSpecified);
  }

  let custom_key = headers
    .get("X-OpenRouter-Key")
    .and_then(|value| value.to_str().ok())
    .map(|s| s.to_string());

  let mut convex = state.convex.clone();

  let Some(thread) = threads::get_by_id(&mut convex, thread_id.to_string()).await? else {
    return Err(CreateMessageError::ThreadNotFound);
  };

  let Some(message) = messages::get_by_id(&mut convex, response_message_id.to_string()).await? else {
    return Err(CreateMessageError::MessageNotFound);
  };

  if message.status.as_ref() != Some(&MessageStatus::Pending) {
    return Err(CreateMessageError::ResponseMessageNotPending);
  }

  let Some(messages) = messages::get_until(&mut convex, thread.id.clone(), message.id.clone()).await? else {
    return Err(CreateMessageError::MessageNotFound);
  };

  let messages = messages.into_iter().map(message_to_part).collect();

  let complete_args = CompleteMessageArgs {
    message_id: message.id.clone(),
    model: payload.model.clone(),
    model_params: payload.model_params.as_ref().map(|params| ModelParams {
      reasoning_effort: params.reasoning_effort.clone(),
      include_search: params.include_search,
    }),
  };

  let (text_tx, mut chat_rx) = mpsc::channel(128);
  let (kill_tx, kill_rx) = mpsc::channel(1);

  state.active_threads.lock().await.insert(thread.id.clone(), kill_tx);

  let set_title = thread.title.is_none();

  let openrouter = state.openrouter.clone();
  let convex_client = state.convex.clone();
  let active_threads = state.active_threads.clone();
  let thread_id = thread.id.clone();

  let context = ChatContext {
    model: model.to_string(),
    message,
    thread,
    complete_args,
    custom_key,
    set_title,
    messages,
  };

  tokio::spawn(async move {
    let result = stream_chat(openrouter, convex_client, text_tx, kill_rx, context).await;

    active_threads.lock().await.remove(&thread_id);

    if let Err(err) = result {
      error!("Failed to stream chat: {:?}", err);
    }
  });

  let stream = async_stream::stream! {
    while let Some(event) = chat_rx.recv().await {
      yield Ok(Event::default().data(event.into_string()).event("message"))
    }

    yield Ok(Event::default().event("end"));
  };

  Ok(Sse::new(stream))
}

#[derive(Debug, thiserror::Error)]
pub enum StreamChatError {
  #[error("unexpected error: {0}")]
  Unexpected(#[from] anyhow::Error),
  #[error("openrouter client error: {0}")]
  OpenrouterError(#[from] OpenrouterError),
  #[error("convex error: {0}")]
  ConvexError(#[from] ConvexError),
  #[error("failed to create OpenRouter stream: {0}")]
  OpenRouter(anyhow::Error),
  #[error("failed to append text to message")]
  AppendText,
  #[error("failed to complete message")]
  CompleteMessage,
  #[error("failed to set thread title")]
  SetThreadTitle,
}

enum ChatEvent {
  Text(String),
  Cancelled,
  Error,
  Refusal(String),
  End,
  Unauthorized,
}

impl ChatEvent {
  fn into_string(self) -> String {
    match self {
      ChatEvent::Text(text) => format!("0:{text}"),
      ChatEvent::Error => "1:".into(),
      ChatEvent::Cancelled => "2:".into(),
      ChatEvent::Refusal(refusal) => format!("3:{refusal}"),
      ChatEvent::End => "4:".into(),
      ChatEvent::Unauthorized => "5:".into(),
    }
  }
}

#[tracing::instrument("stream_chat", skip_all, fields(model = context.model, message_id = context.message.id, thread_id = context.thread.id), err)]
async fn stream_chat(
  openrouter: Arc<Mutex<OpenrouterClient>>,
  mut convex_client: ConvexClient,
  chat_tx: mpsc::Sender<ChatEvent>,
  mut kill_rx: mpsc::Receiver<()>,
  context: ChatContext,
) -> Result<(), StreamChatError> {
  let using_custom_key = context.custom_key.is_some();

  let source = stream_completions(
    openrouter.clone(),
    &context.model,
    context.messages,
    context.custom_key.clone(),
    None,
  )
  .await?;

  let (stream_trigger, stream) = stream_openrouter_chat(source, using_custom_key)
    .await
    .map_err(StreamChatError::OpenRouter)?;

  pin_mut!(stream);

  let message_id = context.message.id.clone();
  let mut convex = convex_client.clone();

  let kill_listener = async move {
    info!("Listening for kill signal for message ID: {}", message_id);

    // if Some(_) is received, the kill signal was sent
    // if None is received the channel was closed so kill anyways
    let _ = kill_rx.recv().await;

    info!("streaming killed, cleaning up");
    stream_trigger.cancel();

    if !messages::cancel(&mut convex, message_id).await? {
      error!("failed to cancel message in Convex");
    }

    Ok(ChatEvent::Cancelled)
  };

  let mut convex = convex_client.clone();
  let event_listener_chat_tx = chat_tx.clone();

  let message_id = context.message.id.clone();

  let event_listener = async move {
    let chat_tx = event_listener_chat_tx;
    let mut accumulator = String::new();

    info!("Starting OpenRouter chat stream for message ID: {}", context.message.id);

    let final_event = loop {
      let Some(event) = stream.next().await else {
        info!("OpenRouter stream ended for message ID: {}", context.message.id);
        break ChatEvent::End;
      };

      let mut completion = match event {
        OpenrouterEvent::Completion(completion) => completion,
        OpenrouterEvent::Unauthorized => break ChatEvent::Unauthorized,
        OpenrouterEvent::Error => break ChatEvent::Error,
      };

      if completion.choices.is_empty() {
        warn!("Received empty choices from OpenRouter response");
        continue;
      }

      let choice = completion.choices.swap_remove(0);

      let text = match choice.delta {
        ChatDelta::Text { content } => content,
        ChatDelta::Finished { .. } => break ChatEvent::End,
        ChatDelta::Refusal { refusal } => break ChatEvent::Refusal(refusal),
      };

      accumulator.push_str(&text);
      let _ = chat_tx
        .send(ChatEvent::Text(text.replace('\r', "").replace('\n', "\\n")))
        .await;

      if accumulator.len() >= 100 {
        let success = messages::append_text(&mut convex, context.message.id.clone(), accumulator.clone()).await?;

        if !success {
          return Err(StreamChatError::AppendText);
        }

        accumulator.clear();
      }
    };

    if !accumulator.is_empty() {
      let success = messages::append_text(&mut convex, context.message.id, accumulator).await?;

      if !success {
        return Err(StreamChatError::AppendText);
      }
    }

    Ok(final_event)
  };

  info!("Starting event listeners for message ID: {}", message_id);

  let final_event = tokio::select! {
    res = event_listener => res,
    res = kill_listener => res,
  }?;

  let _ = chat_tx.send(final_event).await;

  let success = messages::complete(&mut convex_client, &context.complete_args).await?;

  if !success {
    return Err(StreamChatError::CompleteMessage);
  }

  if context.set_title {
    let messages = messages::get_by_thread_id(&mut convex_client, context.thread.id.clone()).await?;

    let messages = messages.into_iter().map(message_to_part).collect::<Vec<_>>();

    let title = generate_title_from_content(openrouter, context.thread.id.clone(), messages, context.custom_key)
      .await
      .unwrap_or_else(|_| "Chat Conversation".to_string());

    info!("Setting thread title to: {}", title);

    let success = threads::set_title(&mut convex_client, context.thread.id, title).await?;
    if !success {
      return Err(StreamChatError::SetThreadTitle);
    }
  }

  Ok(())
}
