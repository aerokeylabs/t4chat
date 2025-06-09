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

pub fn init_tracing_human() -> anyhow::Result<()> {
  let subscriber = tracing_subscriber::fmt()
    .pretty()
    .with_timer(ChronoLocal::new("%H:%M:%S%.3f".to_owned()))
    .with_file(false)
    .with_line_number(true)
    .with_writer(std::io::stderr)
    .with_max_level(tracing::Level::TRACE)
    .finish();

  tracing::subscriber::set_global_default(subscriber)?;

  Ok(())
}

pub fn init_tracing_json() -> anyhow::Result<()> {
  let subscriber = tracing_subscriber::fmt().json().flatten_event(true).finish();
  tracing::subscriber::set_global_default(subscriber)?;
  Ok(())
}

pub fn init_human_readable() {
  if env::var("RUST_LOG").is_err() {
    unsafe { env::set_var("RUST_LOG", "info") };
  }

  let mut builder = Builder::new();
  builder.format(colog::formatter(LogStyle));
  builder.parse_default_env();
  builder.init();
}
