use std::sync::Arc;

use axum::response::Sse;
use axum::response::sse::Event;
use convex::ConvexClient;
use futures::{Stream, StreamExt, pin_mut};
use openai_api_rs::v1::chat_completion::{ChatCompletionMessage, ChatCompletionRequest, Content, MessageRole};
use reqwest::{Method, RequestBuilder};
use reqwest_eventsource::{Event as EventSourceEvent, EventSource};
use stream_cancel::{Trigger, Valved};
use tokio::sync::{Mutex, mpsc};

use crate::convex::messages::{CompleteMessageArgs, Message, MessageStatus, ModelParams};
use crate::convex::threads::Thread;
use crate::convex::{ConvexError, messages, threads};
use crate::openai::{ChatCompletion, ChatDelta};
use crate::prelude::*;
use crate::setup::OpenrouterClient;
use crate::{convex_serde, into_response};

#[derive(Debug, Deserialize, Type)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum MessagePart {
  #[serde(rename = "text")]
  Text { text: String },
}

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ModelParamsRequest {
  pub reasoning_effort: String,
  pub include_search: bool,
}

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CreateMessageRequest {
  pub message_parts: Vec<MessagePart>,
  pub thread_id: String,
  pub response_message_id: String,
  pub model: String,
  // pub convex_session_id: String,
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
}

into_response!(
  CreateMessageError {
    MessageNotFound => StatusCode::NOT_FOUND,
    ThreadNotFound => StatusCode::NOT_FOUND,
    ResponseMessageNotPending => StatusCode::BAD_REQUEST,
    Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
    Serialization(_) => StatusCode::INTERNAL_SERVER_ERROR,
    Convex(_) => StatusCode::INTERNAL_SERVER_ERROR,
  }
);

struct ChatContext {
  model: String,
  message: Message,
  thread: Thread,
  complete_args: CompleteMessageArgs,
  set_title: bool,
  user_message: String,
}

pub enum OpenrouterEvent {
  Completion(ChatCompletion),
  Error,
}

#[tracing::instrument("stream openrouter chat", skip_all, err)]
async fn stream_openrouter_chat(
  request: RequestBuilder,
) -> Result<(Trigger, impl Stream<Item = OpenrouterEvent>), anyhow::Error> {
  let source = EventSource::new(request)?;

  let stream = async_stream::stream! {
    for await value in source {
      match value {
        Ok(EventSourceEvent::Message(data)) => {
          let text = data.data;
          info!("Received text: {text}");
          match serde_json::from_str(&text) {
            Ok(parsed) => yield OpenrouterEvent::Completion(parsed),
            Err(err) => {
              error!("Failed to parse OpenRouter response: {err:?}");
              yield OpenrouterEvent::Error;
            },
          }
        },
        Err(err) => {
          error!("Stream error: {err:?}");
          yield OpenrouterEvent::Error;
          break;
        },
        _ => {},
      }
    }
  };

  let (trigger, stream) = Valved::new(stream);

  Ok((trigger, stream))
}

#[derive(Debug, thiserror::Error)]
pub enum StreamChatError {
  #[error("unexpected error: {0}")]
  Unexpected(#[from] anyhow::Error),
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
}

impl ChatEvent {
  fn into_string(self) -> String {
    match self {
      ChatEvent::Text(text) => format!("0:{text}"),
      ChatEvent::Error => "1:".into(),
      ChatEvent::Cancelled => "2:".into(),
      ChatEvent::Refusal(refusal) => format!("3:{refusal}"),
      ChatEvent::End => "4:".into(),
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
  let openrouter = openrouter.lock().await;

  let request = ChatCompletionRequest::new(
    context.model.clone(),
    vec![ChatCompletionMessage {
      role: MessageRole::user,
      content: Content::Text(context.user_message),
      name: None,
      tool_calls: None,
      tool_call_id: None,
    }],
  )
  .stream(true)
  .max_tokens(500)
  .temperature(0.7);

  let request = openrouter
    .request_builder(Method::POST, "chat/completions")?
    .json(&request);

  drop(openrouter);

  let (stream_trigger, stream) = stream_openrouter_chat(request)
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

    while let Some(event) = stream.next().await {
      let mut completion = match event {
        OpenrouterEvent::Completion(completion) => completion,
        OpenrouterEvent::Error => {
          let _ = chat_tx.send(ChatEvent::Error).await;
          break;
        }
      };

      if completion.choices.is_empty() {
        warn!("Received empty choices from OpenRouter response");
        continue;
      }

      let choice = completion.choices.swap_remove(0);

      let text = match choice.delta {
        ChatDelta::Text { content } => content,
        ChatDelta::Finished { finish_reason } => {
          info!("Chat completed with finish reason: {}", finish_reason);
          break;
        }
        ChatDelta::Refusal { refusal } => {
          let _ = chat_tx.send(ChatEvent::Refusal(refusal)).await;
          break;
        }
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
    }

    if !accumulator.is_empty() {
      let success = messages::append_text(&mut convex, context.message.id, accumulator).await?;

      if !success {
        return Err(StreamChatError::AppendText);
      }
    }

    Ok(ChatEvent::End)
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

  // todo: get title from model

  if context.set_title {
    let success = threads::set_title(&mut convex_client, context.thread.id, "Chat Conversation".to_string()).await?;

    if !success {
      return Err(StreamChatError::SetThreadTitle);
    }
  }

  Ok(())
}

#[tracing::instrument("create message", skip(state), err)]
#[axum::debug_handler]
pub async fn create_message(
  State(state): State<AppState>,
  Json(payload): Json<CreateMessageRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, CreateMessageError>>>, CreateMessageError> {
  info!("creating chat with payload: {:?}", payload);

  let mut convex = state.convex.clone();

  let Some(thread) = threads::get_by_id(&mut convex, payload.thread_id).await? else {
    return Err(CreateMessageError::ThreadNotFound);
  };

  let Some(message) = messages::get_by_id(&mut convex, payload.response_message_id).await? else {
    return Err(CreateMessageError::MessageNotFound);
  };

  if message.status != MessageStatus::Pending {
    return Err(CreateMessageError::ResponseMessageNotPending);
  }

  info!("response message: {:?}", message);

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

  let user_message = payload
    .message_parts
    .into_iter()
    .map(|part| match part {
      MessagePart::Text { text } => text,
    })
    .collect::<Vec<_>>()
    .join(" ");

  let openrouter = state.openrouter.clone();
  let convex_client = state.convex.clone();
  let active_threads = state.active_threads.clone();
  let thread_id = thread.id.clone();

  let context = ChatContext {
    model: payload.model,
    message,
    thread,
    complete_args,
    set_title,
    user_message,
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
