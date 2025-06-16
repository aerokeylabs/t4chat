mod router;
pub use router::{RouteInfo, Router, print_routes};

use crate::prelude::*;

mod message;
mod models;

#[tracing::instrument(name = "creating main router", skip(_state))]
pub fn router(_state: AppState) -> Router<AppState> {
  Router::new()
    .nest("/message", message::router())
    .nest("/models", models::router())
}
