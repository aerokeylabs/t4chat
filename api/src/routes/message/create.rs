use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Instant;

use axum::http::HeaderMap;
use axum::response::Sse;
use axum::response::sse::Event;
use futures::{Stream, StreamExt, pin_mut};
use tokio::sync::{Mutex, mpsc};

use crate::convex::messages::{
  Annotation as ConvexAnnotation, AnnotationArgs, CompleteMessageArgs, Message as ConvexMessage, MessagePart,
  MessageStatus, ModelParams, Role as ConvexRole,
};
use crate::convex::threads::Thread;
use crate::convex::{ConvexClient, ConvexError, messages, threads};
use crate::convex_serde;
use crate::openrouter::completions::{OpenrouterEvent, stream_completions, stream_openrouter_chat};
use crate::openrouter::title::generate_title_from_content;
use crate::openrouter::types::{Annotation, ChatDelta, Message, ReasoningEffort, Role};
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
pub enum ReasoningEffortRequest {
  Low,
  Medium,
  High,
}

impl From<ReasoningEffortRequest> for ReasoningEffort {
  fn from(val: ReasoningEffortRequest) -> Self {
    match val {
      ReasoningEffortRequest::Low => ReasoningEffort::Low,
      ReasoningEffortRequest::Medium => ReasoningEffort::Medium,
      ReasoningEffortRequest::High => ReasoningEffort::High,
    }
  }
}

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CreateMessageRequest {
  pub thread_id: String,
  pub response_message_id: String,
  pub model: String,
  pub model_params: Option<ModelParamsRequest>,
  pub reasoning_effort: Option<ReasoningEffortRequest>,
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
  reasoning_effort: Option<ReasoningEffort>,
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
    prompt_token_count: 0.0,
    token_count: 0.0,
    duration_ms: 0.0,
    tokens_per_second: 0.0,
    time_to_first_token_ms: 0.0,
  };

  let (text_tx, mut chat_rx) = mpsc::channel(128);
  let (kill_tx, kill_rx) = mpsc::channel(1);

  state.active_threads.lock().await.insert(thread.id.clone(), kill_tx);

  let set_title = thread.title.is_none();

  let openrouter = state.openrouter.clone();
  let convex_client = state.convex.clone();
  let active_threads = state.active_threads.clone();
  let thread_id = thread.id.clone();

  let reasoning_effort = payload.reasoning_effort.map(|effort| effort.into());

  let context = ChatContext {
    model: model.to_string(),
    message,
    thread,
    complete_args,
    custom_key,
    set_title,
    messages,
    reasoning_effort,
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
      yield Ok(Event::default().data(event.into_string()?).event("message"))
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
  #[error("failed to append annotations to message")]
  AppendAnnotations,
}

enum ChatEvent {
  Text(String),
  Reasoning(String),
  Cancelled,
  Error,
  Refusal(String),
  End,
  Unauthorized,
  Annotations(Vec<ConvexAnnotation>),
}

impl ChatEvent {
  fn into_string(self) -> anyhow::Result<String> {
    let string = match self {
      ChatEvent::Text(text) => format!("0:{text}"),
      ChatEvent::Reasoning(reasoning) => format!("1:{reasoning}"),
      ChatEvent::Annotations(annotations) => {
        let serialized = serde_json::to_string(&annotations).context("failed to serialize annotations")?;
        format!("2:{serialized}")
      }
      ChatEvent::Error => "3:".into(),
      ChatEvent::Cancelled => "4:".into(),
      ChatEvent::Refusal(refusal) => format!("5:{refusal}"),
      ChatEvent::End => "6:".into(),
      ChatEvent::Unauthorized => "7:".into(),
    };

    Ok(string)
  }
}

fn sanitize_text(text: &str) -> String {
  text.replace('\r', "").replace('\n', "\\n")
}

enum ReasoningOrText {
  Reasoning(String),
  Text(String),
}

fn openrouter_annotation_to_convex(annotation: Annotation) -> ConvexAnnotation {
  match annotation {
    Annotation::UrlCitation { url_citation } => ConvexAnnotation {
      title: url_citation.title,
      url: url_citation.url,
      content: url_citation.content,
    },
  }
}

#[tracing::instrument("stream_chat", skip_all, fields(model = context.model, message_id = context.message.id, thread_id = context.thread.id), err)]
async fn stream_chat(
  openrouter: Arc<Mutex<OpenrouterClient>>,
  mut convex_client: ConvexClient,
  chat_tx: mpsc::Sender<ChatEvent>,
  mut kill_rx: mpsc::Receiver<()>,
  mut context: ChatContext,
) -> Result<(), StreamChatError> {
  let using_custom_key = context.custom_key.is_some();

  let source = stream_completions(
    openrouter.clone(),
    &context.model,
    context.messages,
    context.custom_key.clone(),
    None,
    context.reasoning_effort,
  )
  .await?;

  let (stream_trigger, stream) = stream_openrouter_chat(source, using_custom_key)
    .await
    .map_err(StreamChatError::OpenRouter)?;

  pin_mut!(stream);

  // region: kill listener

  let kl_message_id = context.message.id.clone();
  let mut kl_convex_client = convex_client.clone();

  let kill_listener = async move {
    info!("Listening for kill signal for message ID: {}", kl_message_id);

    // if Some(_) is received, the kill signal was sent
    // if None is received the channel was closed so kill anyways
    let _ = kill_rx.recv().await;

    info!("streaming killed, cleaning up");
    stream_trigger.cancel();

    if !messages::cancel(&mut kl_convex_client, kl_message_id).await? {
      error!("failed to cancel message in Convex");
    }

    Ok(ChatEvent::Cancelled)
  };

  // endregion

  let start = Instant::now();
  let prompt_token_count = Arc::new(AtomicU32::new(0));
  let completion_token_count = Arc::new(AtomicU32::new(0));
  let time_to_first_token_ms = Arc::new(AtomicU32::new(0));

  // region: event listener

  let mut el_convex_client = convex_client.clone();
  let el_chat_tx = chat_tx.clone();

  let el_message_id = context.message.id.clone();

  let el_prompt_token_count = prompt_token_count.clone();
  let el_completion_token_count = completion_token_count.clone();
  let el_time_to_first_token_ms = time_to_first_token_ms.clone();

  let event_listener = async move {
    let mut text_accumulator = String::new();
    let mut reasoning_accumulator = String::new();

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

      if let Some(usage) = completion.usage {
        el_prompt_token_count.store(usage.prompt_tokens, Ordering::Relaxed);
        el_completion_token_count.store(usage.completion_tokens, Ordering::Relaxed);
      }

      if completion.choices.is_empty() {
        warn!("Received empty choices from OpenRouter response");
        continue;
      }

      let choice = completion.choices.swap_remove(0);

      let (text, annotations) = match choice.delta {
        ChatDelta::Text {
          content,
          reasoning,
          annotations,
        } => {
          if let Some(reasoning) = reasoning {
            info!("Received reasoning: {}", reasoning);
            (ReasoningOrText::Reasoning(reasoning), annotations)
          } else {
            (ReasoningOrText::Text(content), annotations)
          }
        }
        ChatDelta::Finished { .. } => break ChatEvent::End,
        ChatDelta::Refusal { refusal } => break ChatEvent::Refusal(refusal),
      };

      if el_time_to_first_token_ms.load(Ordering::Relaxed) == 0 {
        let elapsed = start.elapsed();
        el_time_to_first_token_ms.store(elapsed.as_millis() as u32, Ordering::Relaxed);
      }

      let event = match text {
        ReasoningOrText::Reasoning(text) => {
          reasoning_accumulator.push_str(&text);
          ChatEvent::Reasoning(sanitize_text(&text))
        }
        ReasoningOrText::Text(text) => {
          text_accumulator.push_str(&text);
          ChatEvent::Text(sanitize_text(&text))
        }
      };

      if let Some(annotations) = annotations {
        let annotations = annotations
          .into_iter()
          .map(openrouter_annotation_to_convex)
          .collect::<Vec<_>>();

        if !annotations.is_empty() {
          let _ = el_chat_tx.send(ChatEvent::Annotations(annotations.clone())).await;

          let args = AnnotationArgs {
            message_id: context.message.id.clone(),
            annotations,
          };

          let success = messages::append_annotations(&mut el_convex_client, args).await?;

          if !success {
            return Err(StreamChatError::AppendAnnotations);
          }
        }
      }

      let _ = el_chat_tx.send(event).await;

      if text_accumulator.len() >= 100 {
        let success = messages::append_text(
          &mut el_convex_client,
          context.message.id.clone(),
          text_accumulator.clone(),
        )
        .await?;

        if !success {
          return Err(StreamChatError::AppendText);
        }

        text_accumulator.clear();
      }

      if reasoning_accumulator.len() >= 100 {
        let success = messages::append_reasoning(
          &mut el_convex_client,
          context.message.id.clone(),
          reasoning_accumulator.clone(),
        )
        .await?;

        if !success {
          return Err(StreamChatError::AppendText);
        }

        reasoning_accumulator.clear();
      }
    };

    if !text_accumulator.is_empty() {
      let success = messages::append_text(&mut el_convex_client, context.message.id.clone(), text_accumulator).await?;

      if !success {
        return Err(StreamChatError::AppendText);
      }
    }

    if !reasoning_accumulator.is_empty() {
      let success =
        messages::append_reasoning(&mut el_convex_client, context.message.id, reasoning_accumulator).await?;

      if !success {
        return Err(StreamChatError::AppendText);
      }
    }

    Ok(final_event)
  };

  // endregion

  info!("Starting event listeners for message ID: {}", el_message_id);

  let final_event = tokio::select! {
    res = event_listener => res,
    res = kill_listener => res,
  }?;

  let token_count = completion_token_count.load(Ordering::Relaxed);
  let time_to_first_token_ms = time_to_first_token_ms.load(Ordering::Relaxed);

  let duration = start.elapsed();
  let tokens_per_second = if duration.as_secs() > 0 {
    token_count as f64 / duration.as_secs_f64()
  } else {
    token_count as f64
  };
  let duration_ms = duration.as_millis() as u32;

  let prompt_token_count = prompt_token_count.load(Ordering::Relaxed);

  let _ = chat_tx.send(final_event).await;

  context.complete_args.prompt_token_count = prompt_token_count as f64;
  context.complete_args.token_count = token_count as f64;
  context.complete_args.duration_ms = duration_ms as f64;
  context.complete_args.tokens_per_second = tokens_per_second;
  context.complete_args.time_to_first_token_ms = time_to_first_token_ms as f64;

  let success = messages::complete(&mut convex_client, &context.complete_args).await?;

  if !success {
    return Err(StreamChatError::CompleteMessage);
  }

  if context.set_title {
    let messages = messages::get_by_thread_id(&mut convex_client, context.thread.id.clone()).await?;

    let messages = messages.into_iter().map(message_to_part).collect::<Vec<_>>();

    match generate_title_from_content(openrouter, context.thread.id.clone(), messages, context.custom_key).await {
      Ok(title) => {
        info!("Setting thread title to: {}", title);
        let success = threads::set_title(&mut convex_client, context.thread.id, title).await?;
        if !success {
          return Err(StreamChatError::SetThreadTitle);
        }
      }
      Err(err) => {
        error!("Failed to generate title: {:?}", err);
        return Err(StreamChatError::Unexpected(err));
      }
    }
  }

  Ok(())
}
