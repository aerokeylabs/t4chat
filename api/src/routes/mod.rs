mod router;
pub use router::{RouteInfo, Router, print_routes};

use crate::prelude::*;

mod message;

#[tracing::instrument(name = "creating main router", skip(_state))]
pub fn router(_state: AppState) -> Router<AppState> {
  Router::new().nest("/message", Router::new().post("/", message::create::create_message))
}
