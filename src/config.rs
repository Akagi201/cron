use std::{collections::HashMap, path::PathBuf};

use clap::Parser;
use config::{Config as FileConfig, ConfigError, Environment, File};
use serde_derive::Deserialize;

use crate::scheduler::Job;

#[derive(Clone, Parser)]
pub struct Cli {
  #[clap(short, long)]
  pub config: Option<PathBuf>,
  #[clap(short, long, default_value = "false")]
  pub version: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
  pub log_path: Option<String>,
  pub tg_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
  pub app: AppConfig,
  pub jobs: HashMap<String, Job>,
}

impl Config {
  pub fn new(config: Option<PathBuf>) -> Result<Self, ConfigError> {
    let config = config.unwrap_or_else(|| {
      let home_dir = homedir::my_home().expect("Failed to get home directory").unwrap();
      let path = home_dir.join(".config/cron/config.toml");
      path
    });
    let c = FileConfig::builder()
      .add_source(File::from(config))
      .add_source(Environment::with_prefix("CRON"))
      .build()?;
    c.try_deserialize()
  }
}
