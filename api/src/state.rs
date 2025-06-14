use std::collections::HashMap;
use std::sync::Arc;

use convex::ConvexClient;
use openai_api_rs::v1::api::OpenAIClient;
use snowflake::SnowflakeGenerator;
use tokio::sync::{Mutex, mpsc};

use crate::config::OpenrouterConfig;

#[derive(Clone)]
pub struct AppState {
  pub openrouter: Arc<Mutex<OpenAIClient>>,
  pub openrouter_config: OpenrouterConfig,
  pub convex: ConvexClient,
  pub snowflakes: Arc<Mutex<SnowflakeGenerator>>,

  /// map of thread ids to kill signal senders
  pub active_threads: Arc<Mutex<HashMap<String, mpsc::Sender<()>>>>,
}

fn am<T>(value: T) -> Arc<Mutex<T>> {
  Arc::new(Mutex::new(value))
}

impl AppState {
  pub fn new(openrouter: OpenAIClient, openrouter_config: OpenrouterConfig, convex: ConvexClient, snowflakes: SnowflakeGenerator) -> Self {
    Self {
      openrouter: am(openrouter),
      openrouter_config,
      convex,
      snowflakes: am(snowflakes),

      active_threads: am(HashMap::new()),
    }
  }
}
