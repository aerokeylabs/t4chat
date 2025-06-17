use std::collections::BTreeMap;

use convex::Value;
use serde::{Deserialize, Serialize};

use crate::convex::{ConvexClient, Result, convex_mutation, convex_query};
use crate::convex_serde::to_map;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum MessageStatus {
  Pending,
  Complete,
  Cancelled,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum MessagePart {
  Text { text: String },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Role {
  User,
  Assistant,
  System,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
  #[serde(rename = "_id")]
  pub id: String,
  pub thread_id: String,
  pub status: Option<MessageStatus>,
  pub role: Role,
  pub parts: Vec<MessagePart>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessageIdOnly {
  #[serde(rename = "_id")]
  pub _id: String,
}

pub async fn get_by_id(client: &mut ConvexClient, id: String) -> Result<Option<Message>> {
  const GET_BY_ID: &str = "messages:apiGetById";

  convex_query(
    client,
    GET_BY_ID,
    BTreeMap::from([("id".to_string(), Value::String(id))]),
  )
  .await
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessagesResponse {
  messages: Vec<Message>,
}

pub async fn get_by_thread_id(client: &mut ConvexClient, thread_id: String) -> Result<Vec<Message>> {
  const GET_BY_THREAD_ID: &str = "messages:apiGetByThreadId";

  // Use the correct parameter name that Convex expects (likely camelCase)
  let response: MessagesResponse = convex_query(
    client,
    GET_BY_THREAD_ID,
    BTreeMap::from([("threadId".to_string(), Value::String(thread_id))]),
  )
  .await?;

  Ok(response.messages)
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModelParams {
  pub reasoning_effort: String,
  pub include_search: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteMessageArgs {
  pub message_id: String,
  pub model: String,
  pub model_params: Option<ModelParams>,
}

pub async fn complete(client: &mut ConvexClient, args: &CompleteMessageArgs) -> Result<bool> {
  const COMPLETE_MESSAGE: &str = "messages:apiComplete";

  let result = convex_mutation::<Option<MessageIdOnly>>(client, COMPLETE_MESSAGE, to_map(args)?).await?;

  Ok(result.is_some())
}

pub async fn append_text(client: &mut ConvexClient, message_id: String, text: String) -> Result<bool> {
  const APPEND_TEXT: &str = "messages:apiAppendText";

  let args = BTreeMap::from([
    ("messageId".to_string(), Value::String(message_id)),
    ("text".to_string(), Value::String(text)),
  ]);

  let result = convex_mutation::<Option<MessageIdOnly>>(client, APPEND_TEXT, args).await?;

  Ok(result.is_some())
}

pub async fn cancel(client: &mut ConvexClient, message_id: String) -> Result<bool> {
  const CANCEL_MESSAGE: &str = "messages:apiCancel";

  let args = BTreeMap::from([("messageId".to_string(), Value::String(message_id))]);

  let result = convex_mutation::<Option<MessageIdOnly>>(client, CANCEL_MESSAGE, args).await?;

  Ok(result.is_some())
}

pub async fn get_until(client: &mut ConvexClient, thread_id: String, until_id: String) -> Result<Option<Vec<Message>>> {
  const GET_UNTIL: &str = "threads:apiGetMessagesUntil";

  let args = BTreeMap::from([
    ("threadId".to_string(), Value::String(thread_id)),
    ("untilId".to_string(), Value::String(until_id)),
  ]);

  convex_query(client, GET_UNTIL, args).await
}
