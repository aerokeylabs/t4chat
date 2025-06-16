pub mod messages;
pub mod threads;

use std::collections::BTreeMap;

use anyhow::Context;
use convex::{FunctionResult, Value};
use secrecy::{ExposeSecret, SecretString};
use serde::de::DeserializeOwned;

use crate::config::ConvexConfig;
use crate::convex_serde;
use crate::prelude::*;

#[derive(Clone)]
pub struct ConvexClient {
  pub client: convex::ConvexClient,
  pub api_key: SecretString,
}

pub async fn create_convex_client(config: &ConvexConfig) -> anyhow::Result<ConvexClient> {
  let client = convex::ConvexClient::new(config.url.as_ref())
    .await
    .context("failed to create Convex client")?;

  Ok(ConvexClient {
    client,
    api_key: config.api_key.clone(),
  })
}

#[derive(Debug, thiserror::Error)]
pub enum ConvexError {
  /// convex internal error probably on oneshot receiver
  #[error("unexpected error: {0}")]
  Unexpected(#[from] anyhow::Error),
  #[error("query error: {0}")]
  Query(convex::ConvexError),
  #[error("query error: {0}")]
  Message(String),
  #[error("deserialization error: {0}")]
  Deserialization(#[from] convex_serde::DeError),
  #[error("serialization error: {0}")]
  Serialization(#[from] convex_serde::SerError),
}

type Result<T> = std::result::Result<T, ConvexError>;

fn parse_result(result: FunctionResult) -> Result<Value> {
  match result {
    FunctionResult::Value(value) => Ok(value),
    FunctionResult::ErrorMessage(message) => Err(ConvexError::Message(message)),
    FunctionResult::ConvexError(convex_error) => Err(ConvexError::Query(convex_error)),
  }
}

const API_KEY: &str = "apiKey";

fn auth_args(api_key: &SecretString, args: &mut BTreeMap<String, Value>) {
  args.insert(API_KEY.into(), Value::String(api_key.expose_secret().to_string()));
}

pub async fn convex_query<T: DeserializeOwned>(
  ConvexClient { client, api_key }: &mut ConvexClient,
  query: &'static str,
  mut args: BTreeMap<String, Value>,
) -> Result<T> {
  auth_args(api_key, &mut args);
  let value = parse_result(client.query(query, args).await?)?;
  Ok(convex_serde::from_value(value)?)
}

pub async fn convex_mutation<T: DeserializeOwned>(
  ConvexClient { client, api_key }: &mut ConvexClient,
  mutation: &'static str,
  mut args: BTreeMap<String, Value>,
) -> Result<T> {
  auth_args(api_key, &mut args);
  let value = parse_result(client.mutation(mutation, args).await?)?;
  Ok(convex_serde::from_value(value)?)
}
