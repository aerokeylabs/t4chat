use std::collections::BTreeMap;

use convex::Value;
use serde::Deserialize;

use crate::convex::{ConvexClient, Result, convex_query};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
  #[serde(rename = "_id")]
  pub id: String,
  pub name: String,
  pub url: String,
  pub mime_type: String,
}

pub async fn get_by_id(client: &mut ConvexClient, id: String) -> Result<Option<Attachment>> {
  const GET_BY_ID: &str = "attachments:apiGetById";

  convex_query(
    client,
    GET_BY_ID,
    BTreeMap::from([("id".to_string(), Value::String(id))]),
  )
  .await
}
