mod router;
pub use router::{RouteInfo, Router, print_routes};

use crate::prelude::*;

mod chat;

#[tracing::instrument(name = "creating main router", skip(_state))]
pub fn router(_state: AppState) -> Router<AppState> {
  Router::new().nest("/chat", Router::new().post("/", chat::create::create_chat))
}
// .post("/chat", chat_handlers::chat_send)
