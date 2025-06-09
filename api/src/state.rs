use std::sync::Arc;

use snowflake::SnowflakeGenerator;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
  pub snowflakes: Arc<Mutex<SnowflakeGenerator>>,
}

impl AppState {
  pub fn new(snowflakes: SnowflakeGenerator) -> Self {
    Self {
      snowflakes: Arc::new(Mutex::new(snowflakes)),
    }
  }
}
