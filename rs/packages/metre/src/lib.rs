//! The `#[derive(Config)]` macro and Configuration loader for Rust.
//! ```
//! use metre::Config;
//! use metre::ConfigLoader;
//!
//! #[derive(Config)]
//! #[config(rename_all = "snake_case")]
//! struct MyConfig {
//!   
//!   // a simple property
//!   port: u16,
//!   
//!   // a property with a default value
//!   #[config(default = std::net::SocketAddr::from(([0,0,0,0], 3000)))]
//!   addr: std::net::SocketAddr,
//!
//!   // some type that doesn't implement FromStr and with a custom merge function
//!   #[config(parse_env = parse_vec, merge = merge_vec)]
//!   custom_parse_env: Vec<String>,
//!
//!   // an optional value
//!   // you should only use Option for configurations that are really optional
//!   // under the hood metre creates a Config::Partial type that is a deep-partial version
//!   // of this struct, so it can be deserialized from partial configurations
//!   optional: Option<String>,   
//!   
//!   // a nested configuration
//!   // the nested type must also implement Config
//!   #[config(nested)]
//!   nested: NestedConfig,
//!
//!   // rename env variable
//!   // the default name for env variables is "{}{name}"
//!   // where name is the SCREAMING_SNAKE_CASE version of the
//!   // field name after applying rename and rename_all configurations
//!   // and the {} placeholder is filled with the auto calculated prefix
//!   #[config(env = "{}OTHER_NAME")]
//!   name: String,
//!
//!   // fixed env key (ignores prefixes)
//!   // this will ignore auto calculated prefixes and prefixes set by env_with_prefix loader calls
//!   #[config(env = "FIXED_ENV_KEY")]
//!   fixed_env_key: u64,
//!
//!   // skip env parsing for this variable
//!   #[config(skip_env)]
//!   skip_env: String
//! }
//!
//! #[derive(Config)]
//! // change the env prefix for this struct
//! #[config(env_prefix = "{}OTHER_")]
//! struct NestedConfig {
//!  #[config(rename = "other_deserialize_and_env_name")]
//!  deep_prop: String
//! }
//!
//! fn load_config() -> Result<MyConfig, metre::Error> {
//!   use metre::Format;
//!   
//!   // create an empty configuration object
//!   let mut loader = ConfigLoader::<MyConfig>::new();
//!
//!   // partial configurations can be added in stages to form the final configuration
//!   // each new stage will override the previous one for the present keys
//!   // you can control how the merge is done with the `#[config(merge = function_name)]` attribute
//!
//!   // add deep-partial defaults calculated from the `#[config(default = value)]` attributes
//!   loader.defaults()?;
//!   
//!   // add deep-partial values from config file
//!   loader.file("./config.json", Format::Json)?;
//!
//!   // the same as above but will do nothing if the file doesn't exist
//!   // Jsonc format is json with comments
//!   loader.file_optional("./config.jsonc", Format::Jsonc)?;
//!   
//!   // from memory
//!   loader.code("port=3000", Format::Toml)?;
//!
//!   // form a url
//!   loader.url("https://example.com/config.yaml", Format::Yaml)?;
//!
//!   // from a url but async
//!   async {
//!     loader.url_async("https://example.com/config.json", Format::Json).await.expect("error loading config from url");
//!   };
//!
//!   // from env variables
//!   loader.env()?;
//!
//!   // from env variables with a prefix
//!   loader.env_with_prefix("MY_APP_")?;
//!
//!   // from env variables with a custom provider
//!   // env provider must implement the metre::EnvProvider trait
//!   // that is already implemented for several types of Maps
//!   let mut env_provider = std::collections::HashMap::from([( "MY_APP_PORT", "3000" )]);
//!   loader.env_with_provider_and_prefix(&env_provider, "MY_APP_")?;
//!
//!   // from a pre generated partial configuration
//!   // PartialMyConfig type is auto generated from the `#[derive(Config)]` macro
//!   // and equals to <MyConfig as Config>::Partial: PartialConfig
//!   // see the PartialConfig trait too see methods asociated with partial config structs
//!   let partial = PartialMyConfig { port: Some(3000), ..Default::default()  };
//!   loader.partial(partial)?;
//!
//!   // compute the final values from the sum of partial configurations
//!   // if after all the stages, required properties are still missing
//!   // a pretty error indicating the missing bits will be returned
//!   let config = loader.finish()?;
//!
//!   // here config has the type `MyConfig`
//!   assert_eq!(config.port, 3000);
//!
//!   Ok(config)
//! }
//!
//!
//! // this is only needed to parse env for types that does not implement FromStr
//! // you can return any error here that implements Display
//! fn parse_vec(value: &str) -> Result<Option<Vec<String>>, std::convert::Infallible> {
//!   let vec = value.split(",").map(String::from).collect();
//!   Ok(Some(vec))
//! }
//!
//! // custom merge function that merges two lists
//! // the new stage will append entries to the previous stage
//! // instead of replace it entirely
//! // you can return any error here that implements Display
//! fn merge_vec(left: &mut Option<Vec<String>>, right: Option<Vec<String>>) -> Result<(), std::convert::Infallible> {
//!   if let Some(left) = left.as_mut() {
//!     if let Some(mut right) = right {
//!       left.append(&mut right);
//!     }
//!   } else {
//!     *left = right
//!   }
//!
//!   Ok(())
//! }

use owo_colors::*;
use serde::de::DeserializeOwned;
use std::collections::{BTreeMap, HashMap};
use std::convert::Infallible;
use std::env::VarError;
use std::fmt::Display;
use std::path::Path;
use std::sync::Arc;

pub mod error;
#[doc(hidden)]
pub mod util;

pub use error::Error;
pub use metre_macros::Config;

use error::{FromEnvError, FromPartialError, MergeError};
pub trait Config: Sized {
  type Partial: PartialConfig;
  fn from_partial(partial: Self::Partial) -> Result<Self, FromPartialError>;
}

pub trait PartialConfig: DeserializeOwned + Default {
  fn defaults() -> Self;

  fn merge(&mut self, other: Self) -> Result<(), MergeError>;

  fn list_missing_properties(&self) -> Vec<String>;

  fn is_empty(&self) -> bool;

  fn from_env_with_provider_and_optional_prefix<E: EnvProvider>(
    env: &E,
    prefix: Option<&str>,
  ) -> Result<Self, FromEnvError>;

  fn from_env_with_provider_and_prefix<E: EnvProvider, P: AsRef<str>>(
    env: &E,
    prefix: P,
  ) -> Result<Self, FromEnvError> {
    Self::from_env_with_provider_and_optional_prefix(env, Some(prefix.as_ref()))
  }

  fn from_env_with_provider<E: EnvProvider>(env: &E) -> Result<Self, FromEnvError> {
    Self::from_env_with_provider_and_optional_prefix(env, None)
  }

  fn from_env_with_prefix<P: AsRef<str>>(prefix: P) -> Result<Self, FromEnvError> {
    Self::from_env_with_provider_and_optional_prefix(&StdEnv, Some(prefix.as_ref()))
  }

  fn from_env() -> Result<Self, FromEnvError> {
    Self::from_env_with_provider_and_optional_prefix(&StdEnv, None)
  }
}

impl<T: Config> Config for Option<T> {
  type Partial = T::Partial;
  fn from_partial(partial: Self::Partial) -> Result<Self, FromPartialError> {
    if partial.is_empty() {
      Ok(None)
    } else {
      let v = T::from_partial(partial)?;
      Ok(Some(v))
    }
  }
}

impl<T: PartialConfig> PartialConfig for Option<T> {
  fn defaults() -> Self {
    Some(T::defaults())
  }

  fn merge(&mut self, other: Self) -> Result<(), MergeError> {
    match (self.as_mut(), other) {
      (None, Some(other)) => *self = Some(other),
      (Some(me), Some(other)) => me.merge(other)?,
      (Some(_), None) => {}
      (None, None) => {}
    };

    Ok(())
  }

  fn list_missing_properties(&self) -> Vec<String> {
    match self {
      None => T::default().list_missing_properties(),
      Some(me) => me.list_missing_properties(),
    }
  }

  fn is_empty(&self) -> bool {
    match self {
      None => true,
      Some(me) => me.is_empty(),
    }
  }

  fn from_env_with_provider_and_optional_prefix<E: EnvProvider>(
    env: &E,
    prefix: Option<&str>,
  ) -> Result<Self, FromEnvError> {
    let v = T::from_env_with_provider_and_optional_prefix(env, prefix)?;
    if v.is_empty() {
      Ok(None)
    } else {
      Ok(Some(v))
    }
  }
}

pub trait EnvProvider {
  type Error: Display;
  fn get(&self, key: &str) -> Result<Option<String>, Self::Error>;
}

macro_rules! impl_env_provider_for_map {
  ($ty:ty) => {
    impl EnvProvider for $ty {
      type Error = Infallible;
      fn get(&self, key: &str) -> Result<Option<String>, Self::Error> {
        Ok(self.get(key).map(ToString::to_string))
      }
    }
  };
}

impl_env_provider_for_map!(HashMap<String, String>);
impl_env_provider_for_map!(HashMap<&str, String>);
impl_env_provider_for_map!(HashMap<String, &str>);
impl_env_provider_for_map!(HashMap<&str, &str>);
impl_env_provider_for_map!(BTreeMap<String, String>);
impl_env_provider_for_map!(BTreeMap<&str, String>);
impl_env_provider_for_map!(BTreeMap<String, &str>);
impl_env_provider_for_map!(BTreeMap<&str, &str>);

#[derive(Debug, Clone, Copy)]
pub struct StdEnv;

impl EnvProvider for StdEnv {
  type Error = VarError;
  fn get(&self, key: &str) -> Result<Option<String>, Self::Error> {
    match std::env::var(key) {
      Err(e) => match &e {
        VarError::NotPresent => Ok(None),
        VarError::NotUnicode(_) => Err(e),
      },
      Ok(v) => Ok(Some(v)),
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum LoadLocation {
  Memory,
  File(String),
  Url(String),
}

impl Display for LoadLocation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use LoadLocation::*;
    match self {
      Memory => write!(f, "{}", "memory".yellow()),
      File(location) => write!(f, "file: {}", location.yellow()),
      Url(location) => write!(f, "url: {}", location.yellow()),
    }
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Format {
  Json,
  Jsonc,
  Toml,
  Yaml,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ConfigLoader<T: Config> {
  partial: T::Partial,
}

impl<T: Config> ConfigLoader<T> {
  pub fn new() -> Self {
    Self {
      partial: T::Partial::default(),
    }
  }

  pub fn file(&mut self, path: &str, format: Format) -> Result<&mut Self, Error> {
    let code = std::fs::read_to_string(path).map_err(|e| Error::Io {
      path: path.into(),
      source: Arc::new(e),
    })?;

    self.code_with_location(&code, format, LoadLocation::File(path.to_string()))
  }

  pub fn file_optional(&mut self, path: &str, format: Format) -> Result<&mut Self, Error> {
    let exists = Path::new(path).try_exists().map_err(|e| Error::Io {
      path: path.into(),
      source: Arc::new(e),
    })?;

    if exists {
      self.file(path, format)
    } else {
      Ok(self)
    }
  }

  #[inline(always)]
  pub fn env(&mut self) -> Result<&mut Self, Error> {
    self._env(&StdEnv, None)
  }

  #[inline(always)]
  pub fn env_with_prefix(&mut self, prefix: &str) -> Result<&mut Self, Error> {
    self._env(&StdEnv, Some(prefix))
  }

  #[inline(always)]
  pub fn env_with_provider<E: EnvProvider>(&mut self, env: &E) -> Result<&mut Self, Error> {
    self._env(env, None)
  }

  #[inline(always)]
  pub fn env_with_provider_and_prefix<E: EnvProvider>(
    &mut self,
    env: &E,
    prefix: &str,
  ) -> Result<&mut Self, Error> {
    self._env(env, Some(prefix))
  }

  #[inline(always)]
  fn _env<E: EnvProvider>(&mut self, env: &E, prefix: Option<&str>) -> Result<&mut Self, Error> {
    let partial = T::Partial::from_env_with_provider_and_optional_prefix(env, prefix)?;
    self._add(partial)
  }

  #[inline(always)]
  pub fn code<S: AsRef<str>>(&mut self, code: S, format: Format) -> Result<&mut Self, Error> {
    self._code(code.as_ref(), format, LoadLocation::Memory)
  }

  #[inline(always)]
  pub fn code_with_location<S: AsRef<str>>(
    &mut self,
    code: S,
    format: Format,
    location: LoadLocation,
  ) -> Result<&mut Self, Error> {
    self._code(code.as_ref(), format, location)
  }

  pub fn url(&mut self, url: &str, format: Format) -> Result<&mut Self, Error> {
    let map_err = |e| Error::Network {
      url: url.to_string(),
      source: Arc::new(e),
    };

    let code = reqwest::blocking::get(url)
      .map_err(map_err)?
      .text()
      .map_err(map_err)?;

    self._code(&code, format, LoadLocation::Url(url.to_string()))
  }

  pub async fn url_async(&mut self, url: &str, format: Format) -> Result<&mut Self, Error> {
    let map_err = |e| Error::Network {
      url: url.to_string(),
      source: Arc::new(e),
    };

    let code = reqwest::get(url)
      .await
      .map_err(map_err)?
      .text()
      .await
      .map_err(map_err)?;

    self._code(&code, format, LoadLocation::Url(url.to_string()))
  }

  fn _code(
    &mut self,
    code: &str,
    format: Format,
    location: LoadLocation,
  ) -> Result<&mut Self, Error> {
    let partial = match format {
      Format::Json => serde_json::from_str(code).map_err(|e| Error::Json {
        location,
        source: Arc::new(e),
      })?,

      Format::Jsonc => {
        let reader = json_comments::StripComments::new(code.as_bytes());
        serde_json::from_reader(reader).map_err(|e| Error::Json {
          location,
          source: Arc::new(e),
        })?
      }

      Format::Toml => toml::from_str(code).map_err(|e| Error::Toml {
        location,
        source: e,
      })?,

      Format::Yaml => serde_yaml::from_str(code).map_err(|e| Error::Yaml {
        location,
        source: Arc::new(e),
      })?,
    };

    self._add(partial)
  }

  #[inline(always)]
  pub fn defaults(&mut self) -> Result<&mut Self, Error> {
    self._add(T::Partial::defaults())
  }

  #[inline(always)]
  pub fn partial(&mut self, partial: T::Partial) -> Result<&mut Self, Error> {
    self._add(partial)
  }

  #[inline(always)]
  fn _add(&mut self, partial: T::Partial) -> Result<&mut Self, Error> {
    self.partial.merge(partial)?;
    Ok(self)
  }

  #[inline(always)]
  pub fn partial_state(&self) -> &T::Partial {
    &self.partial
  }

  #[inline(always)]
  pub fn partial_state_mut(&mut self) -> &mut T::Partial {
    &mut self.partial
  }

  #[inline(always)]
  pub fn finish(self) -> Result<T, Error> {
    let v = T::from_partial(self.partial)?;
    Ok(v)
  }
}

impl<T: Config> Default for ConfigLoader<T> {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod test {}
