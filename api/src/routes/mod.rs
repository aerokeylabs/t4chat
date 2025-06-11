mod chat_handlers;
mod router;
pub use router::{RouteInfo, Router, print_routes};

use crate::prelude::*;

#[tracing::instrument(name = "creating main router", skip(_state))]
pub fn router(_state: AppState) -> Router<AppState> {
  Router::new().nest(
    "/api",
    Router::new()
      .get("/chats", chat_handlers::handle_get_chats)
      .get("/chats/:sessionId", chat_handlers::handle_get_chat_session),
  )
}
// .post("/chat", chat_handlers::chat_send)
