use std::env;

use anyhow::{Context, bail};
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
  pub api_key: SecretString,
}

impl OpenrouterConfig {
  const API_KEY: &'static str = "OPENROUTER_API_KEY";

  fn from_env() -> anyhow::Result<Self> {
    let api_key = get_var(Self::API_KEY)?;

    let api_key = SecretString::from(api_key);

    Ok(Self { api_key })
  }
}

#[derive(Debug, Clone)]
pub struct ConvexConfig {
  pub url: String,
  pub deployment_url: String,
}

impl ConvexConfig {
  const DEPLOYMENT_KEY: &'static str = "CONVEX_DEPLOYMENT";
  const URL_KEY: &'static str = "CONVEX_URL";

  fn from_env() -> anyhow::Result<Self> {
    let url = get_var(Self::URL_KEY)?;
    let deployment = get_var(Self::DEPLOYMENT_KEY)?;

    Ok(Self { url, deployment_url: deployment })
  }
}
