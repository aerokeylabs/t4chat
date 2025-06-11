use std::convert::Infallible;
use std::env;
use std::num::ParseIntError;
use std::time::Duration;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::sse::{Event, Sse};
use openai_api_rs::v1::chat_completion::ChatCompletionRequest;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
  pub id: String,
  pub content: String,
  pub role: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp: Option<String>,
}

// Placeholder for chat session structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatSession {
  pub id: String,
  pub title: String,
  #[serde(rename = "lastUpdatedAt")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub last_updated_at: Option<String>,
}

// Placeholder for full chat session with messages
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatSessionWithMessages {
  pub id: String,
  pub title: String,
  pub messages: Vec<Message>,
}

#[derive(Deserialize, Debug, Type)]
pub struct SendMessagePayload {
  #[serde(rename = "sessionId")]
  pub session_id: Option<String>,
  pub message: String,
  // pub history: Option<Vec<Message>>, // For later context passing
}

// SendMessageResponse is no longer needed as we stream directly
// We might re-introduce a similar concept if we need to send initial metadata
// before the stream.

#[derive(Serialize, Debug)]
struct SessionDetailsEventData {
  #[serde(rename = "sessionId")]
  session_id: String,
}

// Helper struct for OpenRouter request
#[derive(Serialize, Debug)]
struct OpenRouterMessage {
  role: String,
  content: String,
}

#[derive(Serialize, Debug)]
struct OpenRouterRequestPayload {
  model: String,
  messages: Vec<OpenRouterMessage>,
  stream: bool,
  // Add other parameters like temperature, max_tokens if needed
}

#[derive(thiserror::Error, Debug)]
pub enum ChatSendError {
  #[error("Validation error: {0}")]
  ValidationError(#[from] ParseIntError),
}

// POST /api/chat
#[tracing::instrument(name = "handle_chat_post", skip(state, payload), fields(session_id = %payload.session_id.as_deref().unwrap_or("none")))]
pub async fn chat_send(
  State(state): State<AppState>,
  Json(payload): Json<SendMessagePayload>,
) -> Result<(), ChatSendError> {
  let session_id = if let Some(id) = payload.session_id {
    id.parse()?
  } else {
    state.snowflakes.lock().await.generate()
  };

  let completions = {
    let mut openrouter = state.openrouter.lock().await;

    openrouter
      .chat_completion(ChatCompletionRequest {
        model: todo!(),
        messages: todo!(),
        temperature: todo!(),
        top_p: todo!(),
        n: todo!(),
        response_format: todo!(),
        stream: todo!(),
        stop: todo!(),
        max_tokens: todo!(),
        presence_penalty: todo!(),
        frequency_penalty: todo!(),
        logit_bias: todo!(),
        user: todo!(),
        seed: todo!(),
        tools: todo!(),
        parallel_tool_calls: todo!(),
        tool_choice: todo!(),
      })
      .await
  };

  todo!();
}

// GET /api/chats
#[tracing::instrument(name = "handle_get_chats")]
// State(state): State<AppState>, Uncomment and use if AppState is needed
pub async fn handle_get_chats() -> (StatusCode, Json<Vec<ChatSession>>) {
  tracing::info!("Fetching all chat sessions");
  // Simulate fetching from a database
  let chats = vec![
    ChatSession {
      id: "chat-session-1".to_string(),
      title: "Conversation about Axum".to_string(),
      last_updated_at: Some(chrono::Utc::now().to_rfc3339()),
    },
    ChatSession {
      id: "chat-session-2".to_string(),
      title: "Rust project ideas".to_string(),
      last_updated_at: Some(
        chrono::Utc::now()
          .checked_sub_signed(chrono::Duration::days(1))
          .unwrap()
          .to_rfc3339(),
      ),
    },
  ];
  (StatusCode::OK, Json(chats))
}

// GET /api/chats/{sessionId}
#[tracing::instrument(name = "handle_get_chat_session", skip(session_id))]
pub async fn handle_get_chat_session(
  Path(session_id): Path<String>,
  // State(state): State<AppState>, // Uncomment and use if AppState is needed
) -> (StatusCode, Json<ChatSessionWithMessages>) {
  tracing::info!("Fetching chat session: {}", session_id);
  // Simulate fetching from a database
  let messages = vec![
    Message {
      id: "msg-1".to_string(),
      role: "user".to_string(),
      content: "Tell me about Axum.".to_string(),
      timestamp: Some(
        chrono::Utc::now()
          .checked_sub_signed(chrono::Duration::minutes(5))
          .unwrap()
          .to_rfc3339(),
      ),
    },
    Message {
      id: "msg-2".to_string(),
      role: "assistant".to_string(),
      content: "Axum is a web application framework that focuses on ergonomics and modularity.".to_string(),
      timestamp: Some(
        chrono::Utc::now()
          .checked_sub_signed(chrono::Duration::minutes(4))
          .unwrap()
          .to_rfc3339(),
      ),
    },
  ];
  let chat_session = ChatSessionWithMessages {
    id: session_id,
    title: "Conversation about Axum".to_string(), // Placeholder title
    messages,
  };
  (StatusCode::OK, Json(chat_session))
}
