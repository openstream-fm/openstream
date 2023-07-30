use garde::Validate;
use metre::{Config as MetreConfig, ConfigLoader};
use serde::{Deserialize, Serialize};
use std::net::{AddrParseError, SocketAddr};
use std::path::Path;

fn parse_addrs(var: &str) -> Result<Option<Vec<SocketAddr>>, AddrParseError> {
  let mut addrs = vec![];
  for item in var.split(',') {
    let item = item.trim();
    if item.is_empty() {
      continue;
    }
    let addr = item.parse::<SocketAddr>()?;
    addrs.push(addr);
  }

  Ok(Some(addrs))
}

fn default_addrs(ports: &[u16]) -> Vec<SocketAddr> {
  ports
    .iter()
    .copied()
    .map(|port| SocketAddr::from(([127, 0, 0, 1], port)))
    .collect()
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, MetreConfig, garde::Validate)]
#[serde(deny_unknown_fields)]
#[config(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub struct Config {
  #[config(nested)]
  #[garde(dive)]
  pub mongodb: Mongodb,

  #[config(nested)]
  #[garde(dive)]
  pub stream: Option<Stream>,

  #[config(nested)]
  #[garde(dive)]
  pub source: Option<Source>,

  #[config(nested)]
  #[garde(dive)]
  pub api: Option<Api>,

  #[config(nested)]
  #[garde(dive)]
  pub storage: Option<Storage>,

  #[serde(rename = "static")]
  #[config(nested, rename = "static")]
  #[garde(dive)]
  pub assets: Option<Static>,

  #[config(nested)]
  #[garde(dive)]
  pub smtp: Smtp,

  #[config(nested)]
  #[garde(dive)]
  pub payments: Payments,
}

impl Config {
  pub fn has_interfaces(&self) -> bool {
    self.stream.is_some()
      || self.source.is_some()
      || self.api.is_some()
      || self.storage.is_some()
      || self.assets.is_some()
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, MetreConfig, garde::Validate)]
#[serde(deny_unknown_fields)]
#[config(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub struct Smtp {
  #[garde(skip)]
  pub hostname: String,
  #[garde(skip)]
  pub port: u16,
  #[garde(skip)]
  pub username: String,
  #[garde(skip)]
  pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, MetreConfig, garde::Validate)]
#[serde(deny_unknown_fields)]
#[config(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub struct Mongodb {
  #[garde(skip)]
  pub url: String,
  #[garde(skip)]
  pub storage_db_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, MetreConfig, garde::Validate)]
#[serde(deny_unknown_fields)]
#[config(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub struct Stream {
  #[config(parse_env = parse_addrs)]
  #[garde(length(min = 1))]
  pub addrs: Vec<SocketAddr>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, MetreConfig, garde::Validate)]
#[serde(deny_unknown_fields)]
#[config(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub struct Static {
  #[config(parse_env = parse_addrs)]
  #[garde(length(min = 1))]
  pub addrs: Vec<SocketAddr>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, MetreConfig, garde::Validate)]
#[serde(deny_unknown_fields)]
#[config(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub struct Source {
  #[config(parse_env = parse_addrs)]
  #[garde(length(min = 1))]
  pub addrs: Vec<SocketAddr>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, MetreConfig, garde::Validate)]
#[serde(deny_unknown_fields)]
#[config(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub struct Api {
  #[config(parse_env = parse_addrs, default = default_addrs(&[10700]))]
  #[garde(length(min = 1))]
  pub addrs: Vec<SocketAddr>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, MetreConfig, garde::Validate)]
#[serde(deny_unknown_fields)]
#[config(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub struct Storage {
  #[config(parse_env = parse_addrs, default = default_addrs(&[10900]))]
  #[garde(length(min = 1))]
  pub addrs: Vec<SocketAddr>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, MetreConfig, garde::Validate)]
#[serde(deny_unknown_fields)]
#[config(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub struct Payments {
  #[garde(url)]
  pub base_url: String,
  #[garde(skip)]
  pub access_token: String,
}

#[derive(Debug, thiserror::Error)]
pub enum LoadConfigError {
  #[error(transparent)]
  Metre(#[from] metre::Error),

  #[error("config validation error")]
  Garde(#[from] garde::Errors),

  #[error("I/O error loading config")]
  Io(#[from] std::io::Error),

  #[error(
    "invalid config: at least one of [stream] [source] [api] [storage] or [static] must be defined"
  )]
  NoInterfaces,
}

#[derive(Debug, Clone, Copy)]
pub enum ConfigFileFormat {
  Toml,
  Json,
}

pub fn load(path: Option<impl AsRef<Path>>) -> Result<Config, LoadConfigError> {
  match path {
    Some(path) => {
      let path = path.as_ref();
      let format = if path.ends_with(".json") || path.ends_with(".jsonc") {
        ConfigFileFormat::Json
      } else {
        ConfigFileFormat::Toml
      };

      let buf = std::fs::read_to_string(path)?;
      load_from_memory(Some((&buf, format)))
    }

    None => load_from_memory(None),
  }
}

pub fn load_from_memory(code: Option<(&str, ConfigFileFormat)>) -> Result<Config, LoadConfigError> {
  let mut loader = ConfigLoader::<Config>::new();

  loader.defaults()?;

  if let Some((code, format)) = code {
    match format {
      ConfigFileFormat::Json => loader.code(code, metre::Format::Jsonc)?,
      ConfigFileFormat::Toml => loader.code(code, metre::Format::Toml)?,
    };
  };

  loader.env_with_prefix("OPENSTREAM_")?;

  let config = loader.finish()?;

  config.validate(&())?;

  if !config.has_interfaces() {
    return Err(LoadConfigError::NoInterfaces);
  }

  Ok(config)
}

#[cfg(test)]
mod unit_tests;
