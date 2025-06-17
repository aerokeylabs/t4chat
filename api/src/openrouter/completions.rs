use std::sync::Arc;

use anyhow::Context;
use futures::Stream;
use reqwest::Method;
use reqwest_eventsource::{Error as EventSourceError, Event as EventSourceEvent, EventSource};
use stream_cancel::{Trigger, Valved};
use tokio::sync::Mutex;

use crate::openrouter::types::{ChatCompletion, CompletionRequest, CompletionResponse, Message};
use crate::openrouter::{OpenrouterClient, OpenrouterError};
use crate::prelude::*;

const COMPLETIONS_PATH: &str = "chat/completions";

pub async fn get_completions(
  client: Arc<Mutex<OpenrouterClient>>,
  model: &str,
  messages: Vec<Message>,
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
  messages: Vec<Message>,
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

pub enum OpenrouterEvent {
  Completion(ChatCompletion),
  Error,
  Unauthorized,
}

#[tracing::instrument("stream openrouter chat", skip_all, err)]
pub async fn stream_openrouter_chat(
  source: EventSource,
  using_custom_key: bool,
) -> Result<(Trigger, impl Stream<Item = OpenrouterEvent>), anyhow::Error> {
  let stream = async_stream::stream! {
    for await value in source {
      match value {
        Ok(EventSourceEvent::Message(data)) => {
          if data.data == "[DONE]" || data.data.is_empty() {
            continue;
          }

          if !data.data.starts_with("{") {
            warn!("Received non-JSON data from OpenRouter: {}", data.data);
            continue;
          }

          match serde_json::from_str(&data.data) {
            Ok(parsed) => yield OpenrouterEvent::Completion(parsed),
            Err(err) => {
              error!("Failed to parse OpenRouter response: {err:?}");
              yield OpenrouterEvent::Error;
            },
          }
        },
        Err(EventSourceError::StreamEnded) => {
          break;
        },
        Err(EventSourceError::InvalidStatusCode(StatusCode::UNAUTHORIZED, _)) if using_custom_key => {
          yield OpenrouterEvent::Unauthorized;
        },
        Err(EventSourceError::InvalidStatusCode(_, response)) => {
          let body = response.text().await.unwrap_or_else(|_| "No body".to_string());
          error!("Invalid status code in OpenRouter response: {body:?}");
          yield OpenrouterEvent::Error;
        },
        Err(EventSourceError::InvalidContentType(_, response)) => {
          let body = response.text().await.unwrap_or_else(|_| "No body".to_string());
          error!("Invalid content type in OpenRouter response: {body:?}");
          yield OpenrouterEvent::Error;
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
