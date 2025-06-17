use std::sync::Arc;

use anyhow::Context;
use reqwest::Method;
use reqwest_eventsource::EventSource;
use tokio::sync::Mutex;

use crate::openrouter::types::{CompletionRequest, CompletionResponse, MessagePart};
use crate::openrouter::{OpenrouterClient, OpenrouterError};

const COMPLETIONS_PATH: &str = "chat/completions";

pub async fn get_completions(
  client: Arc<Mutex<OpenrouterClient>>,
  model: &str,
  messages: Vec<MessagePart>,
  custom_key: Option<String>,
  max_tokens: Option<u32>,
) -> Result<CompletionResponse, OpenrouterError> {
  let client = client.lock().await;
  let builder = client.request_builder(Method::POST, COMPLETIONS_PATH, custom_key)?;
  drop(client);

  let request = CompletionRequest {
    model: model.to_string(),
    messages,
    max_tokens,
    stream: false,
  };
  let response = builder.json(&request).send().await?;

  if !response.status().is_success() {
    return Err(OpenrouterError::NotOk(response.status(), response.text().await.ok()));
  }

  Ok(response.json().await?)
}

pub async fn stream_completions(
  client: Arc<Mutex<OpenrouterClient>>,
  model: &str,
  messages: Vec<MessagePart>,
  custom_key: Option<String>,
  max_tokens: Option<u32>,
) -> Result<EventSource, OpenrouterError> {
  let client = client.lock().await;
  let builder = client.request_builder(Method::POST, COMPLETIONS_PATH, custom_key)?;
  drop(client);

  let request = CompletionRequest {
    model: model.to_string(),
    messages,
    max_tokens,
    stream: true,
  };

  let request = builder.json(&request);

  Ok(EventSource::new(request).context("failed to create event source")?)
}
