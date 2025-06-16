use axum::Json;
use axum::extract::State;

use crate::convex_serde;
use crate::prelude::*;

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CancelMessageRequest {
  pub thread_id: String,
}

#[derive(Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CancelMessageResponse {
  pub success: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum CancelMessageError {
  #[error("failed to send cancel signal")]
  FailedToCancel,
  #[error("unexpected error: {0}")]
  Unexpected(#[from] anyhow::Error),
  #[error("serialization error: {0}")]
  Serialization(#[from] convex_serde::SerError),
}

into_response!(
  CancelMessageError {
    FailedToCancel => StatusCode::INTERNAL_SERVER_ERROR,
    Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
    Serialization(_) => StatusCode::INTERNAL_SERVER_ERROR,
  }
);

#[tracing::instrument(name = "cancel_message", skip(state))]
pub async fn cancel_message(
  State(state): State<AppState>,
  Json(payload): Json<CancelMessageRequest>,
) -> Result<Json<CancelMessageResponse>, CancelMessageError> {
  let mut active_threads = state.active_threads.lock().await;

  if let Some(kill_tx) = active_threads.remove(&payload.thread_id) {
    match kill_tx.send(()).await {
      Ok(_) => Ok(Json(CancelMessageResponse { success: true })),
      Err(e) => {
        error!(
          "Failed to send cancel signal for thread_id {}: {:?}",
          payload.thread_id, e
        );
        Err(CancelMessageError::FailedToCancel)
      }
    }
  } else {
    warn!(
      "Thread {} not found in active threads or already completed",
      payload.thread_id
    );
    Ok(Json(CancelMessageResponse { success: false }))
  }
}
