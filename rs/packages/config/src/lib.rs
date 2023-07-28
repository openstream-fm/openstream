use schematic::{Config as SchemaConfig, ConfigError, ConfigLoader};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::Path;

fn parse_addrs(var: String) -> Result<Option<Vec<SocketAddr>>, ConfigError> {
  let mut addrs = vec![];
  for item in var.split(',') {
    let item = item.trim();
    if item.is_empty() {
      continue;
    }
    match item.parse::<SocketAddr>() {
      Err(e) => return Err(ConfigError::Message(e.to_string())),
      Ok(addr) => addrs.push(addr),
    }
  }
  Ok(Some(addrs))
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, SchemaConfig)]
#[serde(deny_unknown_fields)]
#[config(env_prefix = "OPENSTREAM_", rename_all = "snake_case")]
pub struct Config {
  #[setting(nested)]
  pub mongodb: Mongodb,

  #[setting(nested)]
  pub stream: Option<Stream>,

  #[setting(nested)]
  pub source: Option<Source>,

  #[setting(nested)]
  pub api: Option<Api>,

  #[setting(nested)]
  pub storage: Option<Storage>,

  #[serde(rename = "static")]
  #[setting(nested, rename = "static")]
  pub assets: Option<Static>,

  #[setting(nested)]
  pub smtp: Smtp,

  #[setting(nested)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, SchemaConfig)]
#[serde(deny_unknown_fields)]
#[config(env_prefix = "OPENSTREAM_SMTP_", rename_all = "snake_case")]
pub struct Smtp {
  pub hostname: String,
  pub port: u16,
  pub username: String,
  pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, SchemaConfig)]
#[serde(deny_unknown_fields)]
#[config(env_prefix = "OPENSTREAM_MONGODB_", rename_all = "snake_case")]
pub struct Mongodb {
  pub url: String,
  pub storage_db_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, SchemaConfig)]
#[serde(deny_unknown_fields)]
#[config(env_prefix = "OPENSTREAM_STREAM_", rename_all = "snake_case")]
pub struct Stream {
  #[setting(parse_env = parse_addrs)]
  pub addrs: Vec<SocketAddr>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, SchemaConfig)]
#[serde(deny_unknown_fields)]
#[config(env_prefix = "OPENSTREAM_STATIC_", rename_all = "snake_case")]
pub struct Static {
  #[setting(parse_env = parse_addrs)]
  pub addrs: Vec<SocketAddr>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, SchemaConfig)]
#[serde(deny_unknown_fields)]
#[config(env_prefix = "OPENSTREAM_SOURCE_", rename_all = "snake_case")]
pub struct Source {
  #[setting(parse_env = parse_addrs)]
  pub addrs: Vec<SocketAddr>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, SchemaConfig)]
#[serde(deny_unknown_fields)]
#[config(env_prefix = "OPENSTREAM_API_", rename_all = "snake_case")]
pub struct Api {
  #[setting(parse_env = parse_addrs)]
  pub addrs: Vec<SocketAddr>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, SchemaConfig)]
#[serde(deny_unknown_fields)]
#[config(env_prefix = "OPENSTREAM_STORAGE_", rename_all = "snake_case")]
pub struct Storage {
  #[setting(parse_env = parse_addrs)]
  pub addrs: Vec<SocketAddr>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, SchemaConfig)]
#[serde(deny_unknown_fields)]
#[config(env_prefix = "OPENSTREAM_PAYMENTS_", rename_all = "snake_case")]
pub struct Payments {
  pub base_url: String,
  pub access_token: String,
}

#[derive(Debug, thiserror::Error)]
pub enum LoadConfigError {
  #[error("schematic error loading config")]
  Schematic(#[from] schematic::ConfigError),

  #[error("io error loading config")]
  Io(#[from] std::io::Error),

  #[error("toml error loading config")]
  Toml(#[from] toml::de::Error),

  #[error("json error loading config")]
  Json(#[from] serde_json::Error),

  #[error(
    "invalid config: at least one of [stream], [source], [api] or [storage] must be defined"
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
  let config: Config;

  match code {
    Some((buf, format)) => match format {
      ConfigFileFormat::Json => {
        let reader = json_comments::StripComments::new(buf.as_bytes());
        let value: serde_json::Value = serde_json::from_reader(reader)?;
        let code = serde_json::to_string_pretty(&value)?;
        let result = ConfigLoader::<Config>::new()
          .code(code, schematic::Format::Json)?
          .load()?;

        config = result.config;
      }

      ConfigFileFormat::Toml => {
        let result = ConfigLoader::<Config>::new()
          .code(buf, schematic::Format::Toml)?
          .load()?;

        config = result.config
      }
    },

    None => {
      let result = ConfigLoader::<Config>::new().load()?;
      config = result.config;
    }
  }

  if !config.has_interfaces() {
    return Err(LoadConfigError::NoInterfaces);
  }

  Ok(config)
}

#[cfg(test)]
mod unit_tests;
