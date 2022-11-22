#![allow(clippy::bool_comparison)]

use std::fmt::Display;
use std::path::Path;

use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
  pub mongodb: Mongodb,
  pub stream: Option<Stream>,
  pub source: Option<Source>,
  pub api: Option<Api>,
}

impl Config {
  pub fn has_interfaces(&self) -> bool {
    matches!((&self.stream, &self.source, &self.api), (None, None, None)) == false
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mongodb {
  pub url: Url,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stream {
  pub addrs: Vec<SocketAddr>,
  pub public_base_url: Url,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
  pub receiver: SourceReceiver,
  pub broadcaster: SourceBroadcaster,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceReceiver {
  pub addrs: Vec<SocketAddr>,
  pub public_base_url: Url,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBroadcaster {
  pub addrs: Vec<SocketAddr>,
  /// if not set, this will default to http://PUBLIC_IP:PORT
  pub public_base_url: Option<Url>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Api {
  pub addrs: Vec<SocketAddr>,
  pub public_base_url: Option<Url>,
}

#[derive(Debug)]
pub enum LoadConfigError {
  Io(std::io::Error),
  Toml(toml::de::Error),
  NoInterfaces,
}

impl From<toml::de::Error> for LoadConfigError {
  fn from(e: toml::de::Error) -> Self {
    Self::Toml(e)
  }
}

impl From<std::io::Error> for LoadConfigError {
  fn from(e: std::io::Error) -> Self {
    Self::Io(e)
  }
}

impl Display for LoadConfigError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      LoadConfigError::Io(e) => write!(f, "io error: {e}"),
      LoadConfigError::Toml(e) => write!(f, "invalid config: {e}"),
      LoadConfigError::NoInterfaces => write!(
        f,
        "invalid config: at least one of [stream], [source] or [api] must be defined"
      ),
    }
  }
}

impl std::error::Error for LoadConfigError {
  fn cause(&self) -> Option<&dyn std::error::Error> {
    match self {
      LoadConfigError::Io(e) => Some(e),
      LoadConfigError::Toml(e) => Some(e),
      _ => None,
    }
  }
}

pub fn load(path: impl AsRef<Path>) -> Result<Config, LoadConfigError> {
  let buf = std::fs::read_to_string(path)?;
  let config: Config = toml::from_str(buf.as_str())?;

  if config.has_interfaces() == false {
    return Err(LoadConfigError::NoInterfaces);
  }

  Ok(config)
}
