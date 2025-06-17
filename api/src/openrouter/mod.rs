use anyhow::Context;
use axum::http::header::AUTHORIZATION;
use axum::http::{HeaderMap, HeaderValue};
use reqwest::{Client, Method, RequestBuilder, StatusCode, Url};
use secrecy::ExposeSecret;

use crate::config::OpenrouterConfig;
use crate::openrouter::types::ListModelsResponse;
use crate::prelude::*;

pub mod completions;
pub mod types;

pub struct OpenrouterClient {
  pub base_url: Url,
  pub model_base_url: Url,
  pub client: Client,
}

#[derive(Debug, thiserror::Error)]
pub enum OpenrouterError {
  #[error("failed to build request: {0}")]
  BuildRequest(#[from] anyhow::Error),
  #[error("failed to send request: {0}")]
  Request(#[from] reqwest::Error),
  #[error("response not ok ({0}): {1:?}")]
  NotOk(StatusCode, Option<String>),
  #[error("failed to parse response: {0}")]
  Parse(#[from] serde_json::Error),
}

impl OpenrouterClient {
  pub fn request_builder(
    &self,
    method: Method,
    path: &str,
    custom_key: Option<String>,
  ) -> anyhow::Result<RequestBuilder> {
    let client = if let Some(custom_key) = custom_key {
      &create_http_client(&custom_key).context("failed to create HTTP client with custom API key")?
    } else {
      &self.client
    };

    let builder = client.request(method, self.base_url.join(path)?);

    Ok(builder)
  }

  fn model_request_builder(&self, method: Method, path: &str) -> anyhow::Result<RequestBuilder> {
    Ok(self.client.request(method, self.model_base_url.join(path)?))
  }

  pub async fn get_models(&self) -> Result<ListModelsResponse, OpenrouterError> {
    const PATH: &str = "models";
    // path is static so just unwrap
    let builder = self.model_request_builder(Method::GET, PATH).unwrap();

    let response = builder.send().await?;

    if !response.status().is_success() {
      let status = response.status();
      let text = response.text().await.ok();
      return Err(OpenrouterError::NotOk(status, text));
    }

    Ok(response.json().await?)
  }
}

pub fn create_http_client(api_key: &str) -> anyhow::Result<Client> {
  let mut headers = HeaderMap::new();
  #[cfg(debug_assertions)]
  headers.insert("api-key", HeaderValue::from_str(api_key)?);
  headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {api_key}"))?);

  Client::builder()
    .default_headers(headers)
    .build()
    .context("failed to create HTTP client")
}

pub fn create_openrouter_client(config: &OpenrouterConfig) -> anyhow::Result<OpenrouterClient> {
  let api_key = config.api_key.expose_secret();

  let client = create_http_client(api_key).context("failed to create HTTP client for Openrouter")?;

  Ok(OpenrouterClient {
    base_url: config.api_url.clone(),
    model_base_url: config.model_api_url.clone(),
    client,
  })
}
