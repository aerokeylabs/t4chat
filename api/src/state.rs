use std::collections::HashMap;
use std::sync::Arc;

use snowflake::SnowflakeGenerator;
use tokio::sync::{Mutex, mpsc};

use crate::convex::ConvexClient;
use crate::openrouter::OpenrouterClient;

#[derive(Clone)]
pub struct AppState {
  pub openrouter: Arc<Mutex<OpenrouterClient>>,
  pub convex: ConvexClient,
  pub snowflakes: Arc<Mutex<SnowflakeGenerator>>,

  /// map of thread ids to kill signal senders
  pub active_threads: Arc<Mutex<HashMap<String, mpsc::Sender<()>>>>,
}

fn am<T>(value: T) -> Arc<Mutex<T>> {
  Arc::new(Mutex::new(value))
}

impl AppState {
  pub fn new(openrouter: OpenrouterClient, convex: ConvexClient, snowflakes: SnowflakeGenerator) -> Self {
    Self {
      openrouter: am(openrouter),
      convex,
      snowflakes: am(snowflakes),

      active_threads: am(HashMap::new()),
    }
  }
}
