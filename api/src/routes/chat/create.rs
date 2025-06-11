use axum::response::Sse;
use axum::response::sse::Event;
use futures::Stream;

use crate::into_response;
use crate::prelude::*;

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
pub struct CreateChatRequest {
  pub message_parts: Vec<MessagePart>,
  pub thread_id: String,
  pub response_message_id: String,
  pub model: String,
  // pub convex_session_id: String,
  pub model_params: Option<ModelParamsRequest>,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateChatError {
  #[error("unexpected error: {0}")]
  UnexpectedError(#[from] anyhow::Error),
}

into_response!(
  CreateChatError {
    UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR
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

#[tracing::instrument("create chat", skip(_state))]
#[axum::debug_handler]
pub async fn create_chat(
  State(_state): State<AppState>,
  Json(payload): Json<CreateChatRequest>,
) -> Sse<impl Stream<Item = Result<Event, CreateChatError>>> {
  // send stream of 10 character chunks of DATA every 100ms

  info!("creating chat with payload: {:?}", payload);

  let chunks = str_chunks(DATA, 4);

  let stream = async_stream::stream! {
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(10));
    for text in chunks {
      interval.tick().await;

      yield Ok(Event::default().data(text));
    }
  };

  Sse::new(stream)
}
