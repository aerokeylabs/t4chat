pub mod config;
pub mod convex;
pub mod convex_serde;
pub mod error;
pub mod logger;
pub mod openrouter;
pub mod routes;
pub mod setup;
pub mod state;
pub mod types;

pub mod prelude {
  pub use anyhow::{Context as _, anyhow, bail};
  pub use axum::Json;
  pub use axum::extract::{Extension, Path, Query, State};
  pub use axum::http::StatusCode;
  pub use axum::response::IntoResponse;
  pub use serde::{Deserialize, Serialize};
  pub use snowflake::Snowflake;
  pub use specta::Type;
  pub use tracing::{debug, error, info, warn};

  pub use crate::into_response;
  pub use crate::state::AppState;

  pub type Timestamp = chrono::DateTime<chrono::Utc>;
}
