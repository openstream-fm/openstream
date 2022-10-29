use std::fmt::Display;
use std::path::Path;

pub mod raw {

  use serde::{Deserialize, Serialize};
  use url::Url;

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct Config {
    pub mongodb: Mongodb,
    pub stream: Option<Stream>,
    pub source: Option<Source>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct Mongodb {
    pub url: Url,
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct Stream {
    pub port: u16,
    pub public_base_url: Url,
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct Source {
    pub receiver: SourceReceiver,
    pub broadcaster: SourceBroadcaster,
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct SourceReceiver {
    pub port: u16,
    pub public_base_url: Url,
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct SourceBroadcaster {
    pub port: u16,
    /// if not set, this will default to http://PUBLIC_IP:PORT
    pub public_base_url: Option<Url>,
  }
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
        "invalid config: at least one of [stream] or [source] should be defined"
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

#[derive(Debug)]
pub struct Config {
  pub mongodb: raw::Mongodb,
  pub interfaces: Interfaces,
}

#[derive(Debug)]
pub enum Interfaces {
  Stream(raw::Stream),
  Source(raw::Source),
  Both(Both),
}

#[derive(Debug)]
pub struct Both {
  pub source: raw::Source,
  pub stream: raw::Stream,
}

pub fn load(path: impl AsRef<Path>) -> Result<Config, LoadConfigError> {
  let buf = std::fs::read_to_string(path)?;
  let raw::Config {
    mongodb,
    stream,
    source,
  } = toml::from_str(buf.as_str())?;

  let config = match (source, stream) {
    (None, None) => return Err(LoadConfigError::NoInterfaces),
    (Some(source), None) => Config {
      mongodb,
      interfaces: Interfaces::Source(source),
    },
    (None, Some(stream)) => Config {
      mongodb,
      interfaces: Interfaces::Stream(stream),
    },
    (Some(source), Some(stream)) => Config {
      mongodb,
      interfaces: Interfaces::Both(Both { source, stream }),
    },
  };

  Ok(config)
}
