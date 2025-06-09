mod router;
pub use router::{RouteInfo, Router, print_routes};

use crate::prelude::*;

#[tracing::instrument(name = "creating main router", skip(_state))]
pub fn router(_state: AppState) -> Router<AppState> {
  Router::new()
}
