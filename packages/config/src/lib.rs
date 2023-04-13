use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
  pub mongodb: Mongodb,
  pub stream: Option<Stream>,
  pub source: Option<Source>,
  // pub router: Option<Router>,
  pub api: Option<Api>,
  pub storage: Option<Storage>,
}

impl Config {
  pub fn has_interfaces(&self) -> bool {
    self.stream.is_some() || self.source.is_some() || self.api.is_some() || self.storage.is_some()
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mongodb {
  pub url: String,
  pub storage_db_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stream {
  pub addrs: Vec<SocketAddr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
  pub addrs: Vec<SocketAddr>,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(deny_unknown_fields)]
// pub struct Router {
//   pub addrs: Vec<SocketAddr>,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceBroadcaster {
  pub addrs: Vec<SocketAddr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Api {
  pub addrs: Vec<SocketAddr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Storage {
  pub addrs: Vec<SocketAddr>,
}

#[derive(Debug, thiserror::Error)]
pub enum LoadConfigError {
  #[error("io error: {0}")]
  Io(#[from] std::io::Error),

  #[error("invalid config: {0}")]
  Toml(#[from] toml::de::Error),

  #[error(
    "invalid config: at least one of [stream], [source], [api] or [storage] must be defined"
  )]
  NoInterfaces,
}

pub fn load(path: impl AsRef<Path>) -> Result<Config, LoadConfigError> {
  let buf = std::fs::read_to_string(path)?;
  parse(buf)
}

pub fn parse(contents: impl AsRef<str>) -> Result<Config, LoadConfigError> {
  let config: Config = toml::from_str(contents.as_ref())?;

  if !config.has_interfaces() {
    return Err(LoadConfigError::NoInterfaces);
  }

  Ok(config)
}

#[cfg(test)]
mod unit_tests;
