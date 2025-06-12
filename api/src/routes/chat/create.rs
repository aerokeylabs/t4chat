use std::collections::BTreeMap;

use axum::response::Sse;
use axum::response::sse::Event;
use convex::Value;
use futures::{Stream, StreamExt, TryStreamExt};

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
  #[error("failed to get response message: {0}")]
  GetResponseMessageError(anyhow::Error),
}

into_response!(
  CreateChatError {
    UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
    GetResponseMessageError(_) => StatusCode::BAD_REQUEST,
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
  let chunks = str_chunks(DATA, 4);

  async_stream::stream! {
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(50));

    for text in chunks {
      interval.tick().await;

      yield text;
    }
  }
}

fn args<const N: usize>(args: [(&'static str, String); N]) -> BTreeMap<String, Value> {
  args
    .into_iter()
    .map(|(k, v)| (k.to_string(), Value::String(v)))
    .collect()
}

#[tracing::instrument("create chat", skip(state))]
#[axum::debug_handler]
pub async fn create_chat(
  State(state): State<AppState>,
  Json(payload): Json<CreateChatRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, CreateChatError>>>, CreateChatError> {
  info!("creating chat with payload: {:?}", payload);

  let mut convex = state.convex.clone();

  let response_message = match convex
    .query("message:get_by_id", args([("id", payload.response_message_id)]))
    .await
  {
    Ok(response_message) => response_message,
    Err(err) => {
      return Err(CreateChatError::UnexpectedError(err));
    }
  };

  let stream = stream_data();

  let stream = async_stream::stream! {
    for await text in stream {
      let args = BTreeMap::from([(String::from("text"), Value::String(text.clone()))]);

      convex.mutation("append_text", args).await?;

      yield Ok(Event::default().data(text).event("message"))
    }

    yield Ok(Event::default().event("end"));
  };

  Ok(Sse::new(stream))
}
