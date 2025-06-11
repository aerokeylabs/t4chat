use std::sync::Arc;

use openai_api_rs::v1::api::OpenAIClient;
use snowflake::SnowflakeGenerator;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
  pub openrouter: Arc<Mutex<OpenAIClient>>,
  pub snowflakes: Arc<Mutex<SnowflakeGenerator>>,
}

impl AppState {
  pub fn new(openrouter_client: OpenAIClient, snowflakes: SnowflakeGenerator) -> Self {
    Self {
      openrouter: Arc::new(Mutex::new(openrouter_client)),
      snowflakes: Arc::new(Mutex::new(snowflakes)),
    }
  }
}
