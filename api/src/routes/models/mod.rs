use crate::prelude::*;
use crate::routes::Router;

pub mod get;

pub fn router() -> Router<AppState> {
  Router::new().get("/", get::get_models)
}
