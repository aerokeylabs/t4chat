use serde::Serialize;
use specta::Type;

pub mod config;
pub mod convex;
pub mod convex_serde;
pub mod error;
pub mod logger;
pub mod routes;
pub mod setup;
pub mod state;
pub mod types;

pub mod prelude {
  pub use axum::Json;
  pub use axum::extract::{Extension, Path, Query, State};
  pub use axum::http::StatusCode;
  pub use axum::response::IntoResponse;
  pub use serde::{Deserialize, Serialize};
  pub use snowflake::Snowflake;
  pub use specta::Type;
  pub use tracing::{debug, error, info, warn};

  // pub use crate::auth::CurrentUser;
  pub use crate::state::AppState;
  // pub use crate::{db, into_response};

  pub type Timestamp = chrono::DateTime<chrono::Utc>;
}

#[derive(Debug, Serialize, Type)]
pub struct TestType {
  pub awa: String,
  pub wawa: i32,
}
