use std::sync::Arc;

use convex::ConvexClient;
use openai_api_rs::v1::api::OpenAIClient;
use snowflake::SnowflakeGenerator;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
  pub openrouter: Arc<Mutex<OpenAIClient>>,
  pub convex: ConvexClient,
  pub snowflakes: Arc<Mutex<SnowflakeGenerator>>,
}

fn am<T>(value: T) -> Arc<Mutex<T>> {
  Arc::new(Mutex::new(value))
}

impl AppState {
  pub fn new(openrouter: OpenAIClient, convex: ConvexClient, snowflakes: SnowflakeGenerator) -> Self {
    Self {
      openrouter: am(openrouter),
      convex,
      snowflakes: am(snowflakes),
    }
  }
}
