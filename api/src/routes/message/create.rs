use std::time::Duration;

use anyhow::Context;
use axum::response::Sse;
use axum::response::sse::Event;
use convex::ConvexClient;
use futures::{Stream, StreamExt, pin_mut};
use tokio::sync::mpsc::{self, Sender};

use crate::convex::messages::{CompleteMessageArgs, Message, MessageStatus, ModelParams};
use crate::convex::threads::Thread;
use crate::convex::{ConvexError, messages, threads};
use crate::prelude::*;
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

const DATA: &str = include_str!("../../../test/message.md");

pub fn str_chunks(s: &str, chunk_size: usize) -> Vec<String> {
  s.chars()
    .collect::<Vec<_>>()
    .chunks(chunk_size)
    .map(|chunk| chunk.iter().collect())
    .collect()
}

fn stream_data() -> impl Stream<Item = String> {
  let chunks = str_chunks(DATA, 8);

  async_stream::stream! {
    let mut interval = tokio::time::interval(Duration::from_millis(25));

    for text in chunks {
      interval.tick().await;

      yield text;
    }
  }
}

async fn stream_chat(
  mut client: ConvexClient,
  tx: Sender<String>,
  message: Message,
  thread: Thread,
  complete_args: CompleteMessageArgs,
  set_title: bool,
) -> anyhow::Result<()> {
  let stream = stream_data();

  pin_mut!(stream);

  let mut accumulator = String::new();

  while let Some(text) = stream.next().await {
    let _ = tx.send(text.clone()).await;
    accumulator.push_str(&text);

    if accumulator.len() >= 100 {
      if !messages::append_text(&mut client, message.id.clone(), accumulator.clone()).await? {
        return Err(anyhow::anyhow!("failed to append text to message"));
      }

      accumulator.clear();
    }
  }

  if !accumulator.is_empty() && !messages::append_text(&mut client, message.id.clone(), accumulator).await? {
    return Err(anyhow::anyhow!("failed to append remaining text to message"));
  }

  if !messages::complete(&mut client, &complete_args)
    .await
    .context("failed to complete message")?
  {
    return Err(anyhow::anyhow!("failed to complete message"));
  }

  if set_title
    && !threads::set_title(&mut client, thread.id, "Invert Binary Search Tree Code".to_string())
      .await
      .context("failed to set thread title")?
  {
    return Err(anyhow::anyhow!("failed to set thread title"));
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

  let (tx, mut rx) = mpsc::channel(128);

  let set_title = thread.title.is_none();

  tokio::spawn(async move {
    if let Err(err) = stream_chat(convex, tx, message, thread, complete_args, set_title).await {
      error!("failed to stream chat: {:?}", err);
    }
  });

  let stream = async_stream::stream! {
    while let Some(text) = rx.recv().await {
      yield Ok(Event::default().data(text).event("message"))
    }

    yield Ok(Event::default().event("end"));
  };

  Ok(Sse::new(stream))
}
