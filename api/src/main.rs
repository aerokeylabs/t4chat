use anyhow::Context;
use api::config::Config;
use api::logger;
use api::prelude::*;
use api::setup::Application;
use api::types::export_types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let _ = dotenvy::dotenv();

  logger::init_from_env().context("failed to initialize logger")?;

  tracing::info!("hello awa");

  let config = Config::from_env().context("failed to load config from env")?;

  let app = Application::build(config).await?;

  #[cfg(debug_assertions)]
  {
    use std::env::args;
    let show_location = args().any(|arg| arg == "--debug-types");
    let routes = app.routes().clone();
    let thread = std::thread::spawn(move || export_types(show_location, routes));

    if args().any(|arg| arg == "--export-types") {
      thread.join().expect("failed to join thread")?;
      info!("Types exported");
      return Ok(());
    }
  }

  info!("Server starting on port {}", app.port());

  app.run_until_stopped().await?;

  Ok(())
}
