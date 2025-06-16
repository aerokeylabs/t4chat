use std::env;

use anyhow::{Context, bail};
use reqwest::Url;
use secrecy::SecretString;

fn get_var(key: &'static str) -> anyhow::Result<String> {
  let var = env::var(key).with_context(|| format!("{} must be set in the environment", key))?;

  if var.is_empty() {
    bail!("{} must not be empty", key);
  }

  Ok(var)
}

#[derive(Debug, Clone)]
pub struct Config {
  pub application: ApplicationConfig,
  pub snowflake: SnowflakeConfig,
  pub openrouter: OpenrouterConfig,
  pub convex: ConvexConfig,
}

impl Config {
  pub fn from_env() -> anyhow::Result<Self> {
    let application = ApplicationConfig::from_env()?;
    let snowflake = SnowflakeConfig::from_env()?;
    let openrouter = OpenrouterConfig::from_env()?;
    let convex = ConvexConfig::from_env()?;

    Ok(Self {
      application,
      snowflake,
      openrouter,
      convex,
    })
  }
}

#[derive(Debug, Clone)]
pub struct ApplicationConfig {
  pub port: u16,
  pub host: String,
}

impl ApplicationConfig {
  const HOST_KEY: &'static str = "APP_HOST";
  const PORT_KEY: &'static str = "APP_PORT";

  fn from_env() -> anyhow::Result<Self> {
    let port = get_var(Self::PORT_KEY)?
      .parse()
      .context(format!("{} must be a valid u16", Self::PORT_KEY))?;
    let host = get_var(Self::HOST_KEY)?;

    Ok(Self { port, host })
  }
}

pub const EPOCH_MS: u64 = 1698765529972;

#[derive(Debug, Clone)]
pub struct SnowflakeConfig {
  pub worker: u8,
  pub process: u8,
}

impl SnowflakeConfig {
  const WORKER_KEY: &'static str = "SNOWFLAKE_WORKER";

  fn from_env() -> anyhow::Result<Self> {
    let worker: u8 = get_var(Self::WORKER_KEY)?
      .parse()
      .context(format!("{} must be a valid u8", Self::WORKER_KEY))?;
    if worker > snowflake::WORKER_MAX {
      bail!("{} must be less than {}", Self::WORKER_KEY, snowflake::WORKER_MAX);
    }
    let process = (std::process::id() % snowflake::PROCESS_MAX as u32) as u8;

    Ok(Self { worker, process })
  }
}

#[derive(Debug, Clone)]
pub struct OpenrouterConfig {
  pub api_url: Url,
  pub api_key: SecretString,
  pub model_api_url: Url,
}

impl OpenrouterConfig {
  const API_KEY_KEY: &'static str = "OPENROUTER_API_KEY";
  const API_URL_KEY: &'static str = "OPENROUTER_API_URL";
  const MODEL_API_URL_KEY: &'static str = "OPENROUTER_MODEL_API_URL";

  fn from_env() -> anyhow::Result<Self> {
    let api_key = SecretString::from(get_var(Self::API_KEY_KEY)?);

    let api_url = get_var(Self::API_URL_KEY)?;
    let api_url = api_url.parse()?;

    let model_api_url = get_var(Self::MODEL_API_URL_KEY)?;
    let model_api_url = model_api_url.parse()?;

    Ok(Self { api_key, api_url, model_api_url })
  }
}

#[derive(Debug, Clone)]
pub struct ConvexConfig {
  pub url: String,
  pub id: String,
  pub api_key: SecretString,
}

impl ConvexConfig {
  const API_KEY_KEY: &'static str = "CONVEX_API_KEY";
  const DEPLOYMENT_KEY: &'static str = "CONVEX_DEPLOYMENT";
  const URL_KEY: &'static str = "CONVEX_URL";

  fn from_env() -> anyhow::Result<Self> {
    let url = get_var(Self::URL_KEY)?;
    let id = get_var(Self::DEPLOYMENT_KEY)?;
    let api_key = get_var(Self::API_KEY_KEY)?;
    let api_key = SecretString::from(api_key);

    Ok(Self { url, id, api_key })
  }
}
