use std::env;

use anyhow::{Context, bail};

#[derive(Debug, Clone)]
pub struct Config {
  pub application: ApplicationConfig,
  pub snowflake: SnowflakeConfig,
}

impl Config {
  pub fn from_env() -> anyhow::Result<Self> {
    let application = ApplicationConfig::from_env()?;
    let snowflake = SnowflakeConfig::from_env()?;

    Ok(Self { application, snowflake })
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

fn get_var(key: &'static str) -> anyhow::Result<String> {
  env::var(key).with_context(|| format!("{} must be set in the environment", key))
}
