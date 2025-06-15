use std::env;

use colog::format::CologStyle;
use env_logger::Builder;
use log::Level;
use tracing_subscriber::fmt::time::ChronoLocal;

pub struct LogStyle;

impl CologStyle for LogStyle {
  fn level_token(&self, level: &Level) -> &str {
    match level {
      Level::Trace => "TRC",
      Level::Debug => "DBG",
      Level::Info => "INF",
      Level::Warn => "WRN",
      Level::Error => "ERR",
    }
  }

  fn prefix_token(&self, level: &Level) -> String {
    self.level_color(level, self.level_token(level))
  }

  fn first_line_separator(&self) -> String {
    "╭ ".to_string()
  }

  fn line_separator(&self) -> String {
    "\n│     ".to_string()
  }

  fn final_line_separator(&self) -> String {
    "\n╰     ".to_string()
  }
}

fn set_env_log_level() {
  if let Ok(level) = env::var("RUST_LOG") {
    eprintln!("using log level: {:?}", level);
  } else {
    eprintln!("using log level: info (fallback)");
    unsafe { env::set_var("RUST_LOG", "info") };
  }
}

fn tracing_level_from_env() -> tracing::Level {
  match env::var("RUST_LOG").as_deref() {
    Ok("trace") => tracing::Level::TRACE,
    Ok("debug") => tracing::Level::DEBUG,
    Ok("info") => tracing::Level::INFO,
    Ok("warn") => tracing::Level::WARN,
    Ok("error") => tracing::Level::ERROR,
    _ => tracing::Level::INFO,
  }
}

pub fn init_tracing_human() -> anyhow::Result<()> {
  set_env_log_level();

  let subscriber = tracing_subscriber::fmt()
    .pretty()
    .with_timer(ChronoLocal::new("%H:%M:%S%.3f".to_owned()))
    .with_file(false)
    .with_line_number(true)
    .with_writer(std::io::stderr)
    .with_max_level(tracing_level_from_env())
    .finish();

  tracing::subscriber::set_global_default(subscriber)?;

  Ok(())
}

pub fn init_tracing_json() -> anyhow::Result<()> {
  set_env_log_level();

  let subscriber = tracing_subscriber::fmt()
    .json()
    .flatten_event(true)
    .with_max_level(tracing_level_from_env())
    .finish();

  tracing::subscriber::set_global_default(subscriber)?;

  Ok(())
}

pub fn init_human_readable() -> anyhow::Result<()> {
  set_env_log_level();

  let mut builder = Builder::new();
  builder.format(colog::formatter(LogStyle));
  builder.parse_default_env();
  builder.init();

  Ok(())
}

pub fn init_from_env() -> anyhow::Result<()> {
  match env::var("RUST_LOG_DETAIL").unwrap_or("0".to_string()).as_ref() {
    "0" | "human" => init_human_readable(),
    "1" | "detailed" => init_tracing_human(),
    "2" | "json" => init_tracing_json(),
    _ => {
      eprintln!("invalid RUST_LOG_DETAIL value, defaulting to 0");
      init_human_readable()
    }
  }
}
