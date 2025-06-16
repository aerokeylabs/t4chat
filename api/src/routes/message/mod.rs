use crate::prelude::*;
use crate::routes::Router;

pub mod cancel;
pub mod create;

pub fn router() -> Router<AppState> {
  Router::new()
    .post("/", create::create_message)
    .post("/cancel", cancel::cancel_message)
}
