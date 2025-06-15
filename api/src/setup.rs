use anyhow::Context;
use axum::http::header::AUTHORIZATION;
use axum::http::{HeaderMap, HeaderValue};
use convex::ConvexClient;
use openai_api_rs::v1::api::OpenAIClient;
use reqwest::{Client, Method, RequestBuilder, Url};
use secrecy::ExposeSecret;
use snowflake::SnowflakeGenerator;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use crate::config::{Config, EPOCH_MS, OpenrouterConfig};
use crate::prelude::*;
use crate::routes::{RouteInfo, Router, print_routes, router};

pub struct OpenrouterClient {
  pub base_url: Url,
  pub openai: OpenAIClient,
  pub http: Client,
}

impl OpenrouterClient {
  pub fn request_builder(&self, method: Method, path: &str) -> anyhow::Result<RequestBuilder> {
    Ok(self.http.request(method, self.base_url.join(path)?))
  }
}

fn create_openrouter_client(config: &OpenrouterConfig) -> anyhow::Result<OpenrouterClient> {
  let api_key = config.api_key.expose_secret();

  // unwrap because build function just cant fail yet returns
  // result<client, box<dyn Error>> for some reason
  let openai = OpenAIClient::builder()
    .with_endpoint(config.api_url.clone())
    .with_api_key(api_key)
    .with_header("api-key", api_key)
    .build()
    .unwrap();

  let mut headers = HeaderMap::new();
  headers.insert("api-key", HeaderValue::from_str(api_key)?);
  headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {api_key}"))?);

  let http = Client::builder()
    .default_headers(headers)
    .build()
    .context("failed to create HTTP client")?;

  Ok(OpenrouterClient {
    base_url: config.api_url.parse()?,
    openai,
    http,
  })
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
    let openrouter = create_openrouter_client(&config.openrouter)?;
    let convex = create_convex_client(&config).await?;

    let snowflakes = SnowflakeGenerator::new(config.snowflake.worker, config.snowflake.process, EPOCH_MS);

    let address = format!("{}:{}", config.application.host, config.application.port);

    let listener = TcpListener::bind(address)
      .await
      .context("application failed to bind port")?;

    let port = listener.local_addr()?.port();

    let (state, router) = create_router(openrouter, convex, snowflakes);

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
  openrouter: OpenrouterClient,
  convex: ConvexClient,
  snowflakes: SnowflakeGenerator,
) -> (AppState, Router<AppState>) {
  let state = AppState::new(openrouter, convex, snowflakes);

  let router = router(state.clone()).get("/", root);

  print_routes(&router.routes);

  (state, router)
}
