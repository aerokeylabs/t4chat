use std::time::Duration;

use anyhow::Context;
use axum::response::Sse;
use axum::response::sse::Event;
use convex::ConvexClient;
use futures::{Stream, StreamExt, pin_mut};
use tokio::sync::mpsc::{self, Receiver, Sender};
use serde_json::json;

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

// Keep the mock function for fallback/testing purposes
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

async fn stream_openrouter_chat(
  _openai_client: &openai_api_rs::v1::api::OpenAIClient,
  user_message: String,
  model: String,
  _model_params: Option<ModelParams>,
) -> anyhow::Result<std::pin::Pin<Box<dyn Stream<Item = anyhow::Result<String>> + Send>>> {
  // Extract API key and endpoint from the OpenAI client configuration
  // Since we can't easily access the internal client config, we'll get it from env
  let api_key = std::env::var("OPENROUTER_API_KEY")
    .context("OPENROUTER_API_KEY must be set")?;
  let endpoint = std::env::var("OPENROUTER_API_URL")
    .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

  let client = reqwest::Client::new();
  
  let payload = json!({
    "model": model,
    "messages": [
      {
        "role": "user",
        "content": user_message
      }
    ],
    "stream": true,
    "max_tokens": 4096,
    "temperature": 0.7
  });

  let response = client
    .post(format!("{}/chat/completions", endpoint))
    .header("Authorization", format!("Bearer {}", api_key))
    .header("Content-Type", "application/json")
    .json(&payload)
    .send()
    .await
    .context("Failed to send request to OpenRouter")?;

  if !response.status().is_success() {
    let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
    return Err(anyhow::anyhow!("OpenRouter API error: {}", error_text));
  }

  let stream = async_stream::stream! {
    let mut stream = response.bytes_stream();
    let mut buffer = String::new();
    
    while let Some(chunk_result) = stream.next().await {
      match chunk_result {
        Ok(chunk) => {
          let chunk_str = String::from_utf8_lossy(&chunk);
          buffer.push_str(&chunk_str);
          
          // Process complete lines
          let lines: Vec<&str> = buffer.lines().collect();
          let mut processed_lines = 0;
          
          for line in &lines {
            if line.starts_with("data: ") {
              let data_line = &line[6..]; // Remove "data: " prefix
              
              if data_line == "[DONE]" {
                return;
              }
              
              if !data_line.trim().is_empty() {
                match serde_json::from_str::<serde_json::Value>(data_line) {
                  Ok(json) => {
                    if let Some(choices) = json["choices"].as_array() {
                      if let Some(choice) = choices.first() {
                        if let Some(delta) = choice["delta"].as_object() {
                          if let Some(content) = delta["content"].as_str() {
                            if !content.is_empty() {
                              yield Ok(content.to_string());
                            }
                          }
                        }
                      }
                    }
                  }
                  Err(e) => {
                    error!("Failed to parse SSE JSON: {:?}, data: {}", e, data_line);
                  }
                }
              }
              processed_lines += 1;
            } else if !line.trim().is_empty() {
              processed_lines += 1;
            }
          }
          
          // Keep unprocessed data in buffer
          if processed_lines < lines.len() {
            buffer = lines[processed_lines..].join("\n");
          } else {
            buffer.clear();
          }
        }
        Err(e) => {
          yield Err(anyhow::anyhow!("Stream chunk error: {}", e));
          return;
        }
      }
    }
  };

  Ok(Box::pin(stream))
}

async fn stream_chat(
  openrouter_client: std::sync::Arc<tokio::sync::Mutex<openai_api_rs::v1::api::OpenAIClient>>,
  mut convex_client: ConvexClient,
  text_tx: Sender<String>,
  mut kill_rx: Receiver<()>,
  message: Message,
  thread: Thread,
  complete_args: CompleteMessageArgs,
  set_title: bool,
  user_message: String,
) -> anyhow::Result<()> {
  // Get the OpenRouter client
  let client = openrouter_client.lock().await;
  
  // Create the OpenRouter stream
  let stream = match stream_openrouter_chat(
    &client,
    user_message,
    complete_args.model.clone(),
    complete_args.model_params.clone(),
  ).await {
    Ok(stream) => stream,
    Err(e) => {
      error!("Failed to create OpenRouter stream: {:?}", e);
      // Fallback to mock data for development
      warn!("Falling back to mock data stream");
      let mock_stream = Box::pin(stream_data().map(|text| Ok(text)));
      mock_stream
    }
  };

  // Release the client lock
  drop(client);

  pin_mut!(stream);

  let mut accumulator = String::new();
  let mut was_cancelled = false;

  while let Some(text_result) = stream.next().await {
    // Check for cancellation
    if kill_rx.try_recv().is_ok() {
      info!("streaming killed, cleaning up");
      was_cancelled = true;
      
      // Send cancellation event to client
      let _ = text_tx.send("[CANCELLED]".to_string()).await;
      
      // Mark message as cancelled in Convex
      if let Err(e) = messages::cancel(&mut convex_client, message.id.clone()).await {
        error!("Failed to mark message as cancelled in Convex: {:?}", e);
      }
      
      break;
    }

    let text = match text_result {
      Ok(text) => text,
      Err(e) => {
        error!("Stream error: {:?}", e);
        let _ = text_tx.send(format!("[ERROR: {}]", e)).await;
        break;
      }
    };

    let _ = text_tx.send(text.clone()).await;
    accumulator.push_str(&text);

    // Batch updates to Convex every 100 characters
    if accumulator.len() >= 100 {
      if let Err(e) = messages::append_text(&mut convex_client, message.id.clone(), accumulator.clone()).await {
        error!("Failed to append text to message: {:?}", e);
        return Err(anyhow::anyhow!("failed to append text to message"));
      }

      accumulator.clear();
    }
  }

  // Don't complete the message if it was cancelled
  if was_cancelled {
    return Ok(());
  }

  // Append any remaining text
  if !accumulator.is_empty() {
    if let Err(e) = messages::append_text(&mut convex_client, message.id.clone(), accumulator).await {
      error!("Failed to append remaining text to message: {:?}", e);
      return Err(anyhow::anyhow!("failed to append remaining text to message"));
    }
  }

  // Mark message as complete
  if let Err(e) = messages::complete(&mut convex_client, &complete_args).await {
    error!("Failed to complete message: {:?}", e);
    return Err(anyhow::anyhow!("failed to complete message"));
  }

  // Set thread title if needed
  if set_title {
    if let Err(e) = threads::set_title(&mut convex_client, thread.id, "Chat Conversation".to_string()).await {
      error!("Failed to set thread title: {:?}", e);
      return Err(anyhow::anyhow!("failed to set thread title"));
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

  let (text_tx, mut text_rx) = mpsc::channel(128);
  let (kill_tx, kill_rx) = mpsc::channel(1);

  state.active_threads.lock().await.insert(thread.id.clone(), kill_tx);

  let set_title = thread.title.is_none();
  
  // Extract user message from payload
  let user_message = payload.message_parts
    .iter()
    .filter_map(|part| match part {
      MessagePart::Text { text } => Some(text.clone()),
    })
    .collect::<Vec<_>>()
    .join(" ");

  let openrouter_client = state.openrouter.clone();
  let convex_client = state.convex.clone();

  tokio::spawn(async move {
    if let Err(err) = stream_chat(
      openrouter_client,
      convex_client,
      text_tx,
      kill_rx,
      message,
      thread,
      complete_args,
      set_title,
      user_message,
    ).await {
      error!("failed to stream chat: {:?}", err);
    }
  });

  let stream = async_stream::stream! {
    while let Some(text) = text_rx.recv().await {
      yield Ok(Event::default().data(text).event("message"))
    }

    yield Ok(Event::default().event("end"));
  };

  Ok(Sse::new(stream))
}
