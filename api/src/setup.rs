use anyhow::Context;
use convex::ConvexClient;
use openai_api_rs::v1::api::OpenAIClient;
use secrecy::ExposeSecret;
use snowflake::SnowflakeGenerator;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use crate::config::{Config, EPOCH_MS, OpenrouterConfig};
use crate::prelude::*;
use crate::routes::{RouteInfo, Router, print_routes, router};

fn create_openrouter_client(config: &OpenrouterConfig) -> OpenAIClient {
  // unwrap because build function just cant fail yet returns
  // result<client, box<dyn Error>> for some reason

  OpenAIClient::builder()
    .with_endpoint(config.api_url.clone())
    .with_api_key(config.api_key.expose_secret())
    .with_header("api-key", config.api_key.expose_secret())
    .build()
    .unwrap()
}

async fn create_convex_client(config: &Config) -> anyhow::Result<ConvexClient> {
  ConvexClient::new(&config.convex.url)
    .await
    .context("failed to create Convex client")
}

pub struct Application {
  port: u16,
  listener: TcpListener,
  state: AppState,
  router: Router<AppState>,
}

impl Application {
  pub async fn build(config: Config) -> anyhow::Result<Self> {
    let openrouter = create_openrouter_client(&config.openrouter);
    let convex = create_convex_client(&config).await?;

    let snowflakes = SnowflakeGenerator::new(config.snowflake.worker, config.snowflake.process, EPOCH_MS);

    let address = format!("{}:{}", config.application.host, config.application.port);

    let listener = TcpListener::bind(address)
      .await
      .context("application failed to bind port")?;

    let port = listener.local_addr()?.port();

    let (state, router) = create_router(openrouter, config.openrouter, convex, snowflakes);

    Ok(Self {
      port,
      listener,
      state,
      router,
    })
  }

  pub fn routes(&self) -> Vec<RouteInfo> {
    self.router.routes.clone()
  }

  pub fn port(&self) -> u16 {
    self.port
  }

  pub async fn run_until_stopped(self) -> std::io::Result<()> {
    let Self {
      listener,
      state,
      router,
      ..
    } = self;

    let app = axum::Router::new()
      .merge(router)
      .layer(CorsLayer::permissive())
      .with_state(state);

    axum::serve(listener, app).await
  }
}

#[tracing::instrument(name = "root handler")]
async fn root() -> &'static str {
  "hello awa"
}

fn create_router(
  openrouter: OpenAIClient,
  openrouter_config: OpenrouterConfig,
  convex: ConvexClient,
  snowflakes: SnowflakeGenerator,
) -> (AppState, Router<AppState>) {
  let state = AppState::new(openrouter, openrouter_config, convex, snowflakes);

  let router = router(state.clone()).get("/", root);

  print_routes(&router.routes);

  (state, router)
}
