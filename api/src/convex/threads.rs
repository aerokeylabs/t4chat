use std::collections::BTreeMap;

use convex::{ConvexClient, Value};
use serde::Deserialize;

use crate::convex::{Result, convex_mutation, convex_query};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thread {
  #[serde(rename = "_id")]
  pub id: String,
  pub title: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ThreadIdOnly {
  #[serde(rename = "_id")]
  pub _id: String,
}

pub async fn get_by_id(client: &mut ConvexClient, id: String) -> Result<Option<Thread>> {
  const GET_BY_ID: &str = "threads:getById";

  convex_query(
    client,
    GET_BY_ID,
    BTreeMap::from([("id".to_string(), Value::String(id))]),
  )
  .await
}

pub async fn set_title(client: &mut ConvexClient, thread_id: String, title: String) -> Result<bool> {
  const SET_TITLE: &str = "threads:apiSetTitle";

  let args = BTreeMap::from([
    ("threadId".to_string(), Value::String(thread_id)),
    ("title".to_string(), Value::String(title)),
  ]);

  let result = convex_mutation::<Option<ThreadIdOnly>>(client, SET_TITLE, args).await?;

  Ok(result.is_some())
}
