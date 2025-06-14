pub mod messages;
pub mod threads;

use std::collections::BTreeMap;

use convex::{ConvexClient, FunctionResult, Value};
use serde::de::DeserializeOwned;

use crate::convex_serde;

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

pub async fn convex_query<T: DeserializeOwned>(
  client: &mut ConvexClient,
  query: &'static str,
  args: BTreeMap<String, Value>,
) -> Result<T> {
  let value = parse_result(client.query(query, args).await?)?;
  Ok(convex_serde::from_value(value)?)
}

pub async fn convex_mutation<T: DeserializeOwned>(
  client: &mut ConvexClient,
  mutation: &'static str,
  args: BTreeMap<String, Value>,
) -> Result<T> {
  let value = parse_result(client.mutation(mutation, args).await?)?;
  Ok(convex_serde::from_value(value)?)
}
