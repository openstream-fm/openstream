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
pub mod merge;
pub mod parse;
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
  type Partial = Option<T::Partial>;
  fn from_partial(partial: Self::Partial) -> Result<Self, FromPartialError> {
    match partial {
      None => Ok(None),
      Some(inner) => {
        if inner.is_empty() {
          Ok(None)
        } else {
          let v = T::from_partial(inner)?;
          Ok(Some(v))
        }
      }
    }
  }
}

impl<T: PartialConfig> PartialConfig for Option<T> {
  fn defaults() -> Self {
    let inner = T::defaults();
    if inner.is_empty() {
      None
    } else {
      Some(inner)
    }
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
      None => vec![],
      Some(me) => {
        if !me.is_empty() {
          me.list_missing_properties()
        } else {
          vec![]
        }
      }
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

/// Implement this trait if you want to load a configuration from custom environment variables
/// that are not in [`std::env::var`]
///
/// This is speecially usefull for unit tests
///
/// This trait is already implemented for several kinds of [HashMap]'s and [BTreeMap]'s from the standard library
pub trait EnvProvider {
  type Error: Display;
  /// Read a variable from the enviroment
  ///
  /// This should fail if the variable is not UTF-8 encoded
  ///
  /// If the variable is not present, implementations should return `Ok(None)`
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

/// An implementation of [`EnvProvider`] that reads from the standard library's [`std::env::var`]
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

/// A location from where a configuration was loaded
///
/// can be from Memory, File, or URL
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

/// List of known configuration formats
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Format {
  Json,
  Jsonc,
  Toml,
  Yaml,
}

/// The configuration loader
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ConfigLoader<T: Config> {
  partial: T::Partial,
}

impl<T: Config> ConfigLoader<T> {
  /// Create a new configuration loader with all fields set as empty
  pub fn new() -> Self {
    Self {
      partial: T::Partial::default(),
    }
  }

  /// Add a partial configuration from a file
  pub fn file(&mut self, path: &str, format: Format) -> Result<&mut Self, Error> {
    let code = std::fs::read_to_string(path).map_err(|e| Error::Io {
      path: path.into(),
      source: Arc::new(e),
    })?;

    self.code_with_location(&code, format, LoadLocation::File(path.to_string()))
  }

  /// Add a partial configuration from a file, if it exists
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

  /// Add a partial configuration from enviroment varialbes
  #[inline(always)]
  pub fn env(&mut self) -> Result<&mut Self, Error> {
    self._env(&StdEnv, None)
  }

  /// Add a partial configuration from enviroment variables with a prefix
  #[inline(always)]
  pub fn env_with_prefix(&mut self, prefix: &str) -> Result<&mut Self, Error> {
    self._env(&StdEnv, Some(prefix))
  }

  /// Add a partial configuration from enviroment variables with a custom provider
  ///
  /// The provider must implement the [`EnvProvider`] trait
  ///
  /// The [`EnvProvider`] trait is already implemented for several kinds of Maps from the standard library
  #[inline(always)]
  pub fn env_with_provider<E: EnvProvider>(&mut self, env: &E) -> Result<&mut Self, Error> {
    self._env(env, None)
  }

  /// See [`Self::env_with_provider`] and [`Self::env_with_prefix`]
  #[inline(always)]
  pub fn env_with_provider_and_prefix<E: EnvProvider>(
    &mut self,
    env: &E,
    prefix: &str,
  ) -> Result<&mut Self, Error> {
    self._env(env, Some(prefix))
  }

  /// Add a partial configuration from in-memory code
  #[inline(always)]
  pub fn code<S: AsRef<str>>(&mut self, code: S, format: Format) -> Result<&mut Self, Error> {
    self._code(code.as_ref(), format, LoadLocation::Memory)
  }

  /// Add a partial configuration from in-memory code
  ///
  /// Specifying the [`LoadLocation`] of the in-memory code is useful for error reporting
  #[inline(always)]
  pub fn code_with_location<S: AsRef<str>>(
    &mut self,
    code: S,
    format: Format,
    location: LoadLocation,
  ) -> Result<&mut Self, Error> {
    self._code(code.as_ref(), format, location)
  }

  /// Add a partial configuration from a url
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

  /// Add a partial configuration from a url, async version
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

  #[inline(always)]
  fn _env<E: EnvProvider>(&mut self, env: &E, prefix: Option<&str>) -> Result<&mut Self, Error> {
    let partial = T::Partial::from_env_with_provider_and_optional_prefix(env, prefix)?;
    self._add(partial)
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

  /// Add a partial configuration from the `#[config(default = value)]` attributes
  #[inline(always)]
  pub fn defaults(&mut self) -> Result<&mut Self, Error> {
    self._add(T::Partial::defaults())
  }

  /// Add a pre generated partial configuration
  #[inline(always)]
  pub fn partial(&mut self, partial: T::Partial) -> Result<&mut Self, Error> {
    self._add(partial)
  }

  #[inline(always)]
  fn _add(&mut self, partial: T::Partial) -> Result<&mut Self, Error> {
    self.partial.merge(partial)?;
    Ok(self)
  }

  /// Get a reference to the partial configuration
  #[inline(always)]
  pub fn partial_state(&self) -> &T::Partial {
    &self.partial
  }

  /// Get a mutable reference to the partial configuration
  #[inline(always)]
  pub fn partial_state_mut(&mut self) -> &mut T::Partial {
    &mut self.partial
  }

  /// Get the final Config from the sum of all previously added stages
  ///
  /// this function will error if there are missing required properties
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
mod test {

  use super::*;

  #[test]
  fn test() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      #[config(default = "default".into())]
      default: String,

      #[config(nested)]
      nested: Nested,

      optional: Option<String>,

      #[config(parse_env = crate::parse::comma_separated::<String>)]
      list: Vec<String>,
    }

    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Nested {
      a: String,
      b: u8,
    }

    let mut loader = ConfigLoader::<Conf>::new();
    loader.defaults().unwrap();

    loader
      .code(
        r#"
        nested.a = "a"
        nested.b = 1
        list = ["item"]
        "#,
        Format::Toml,
      )
      .unwrap();

    let config = loader.finish().unwrap();

    assert_eq!(
      config,
      Conf {
        default: "default".into(),
        list: vec!["item".into()],
        nested: Nested {
          a: "a".into(),
          b: 1
        },
        optional: None,
      }
    );
  }

  #[test]
  fn from_fixed_env() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      #[config(env = "MY_APP_PORT")]
      port: u16,
    }

    let mut env = HashMap::new();
    env.insert("MY_APP_PORT", "3000");

    let mut loader = ConfigLoader::<Conf>::new();
    loader.env_with_provider(&env).unwrap();
    let config = loader.finish().unwrap();

    assert_eq!(config.port, 3000);
  }

  #[test]
  fn from_env_with_prefix() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate, env_prefix = "{}CONF_")]
    struct Conf {
      #[config(env = "PORT")]
      port: u16,
      #[config(env = "{}LIST_RENAMED", parse_env = crate::parse::comma_separated::<String>)]
      list: Vec<String>,
      #[config(rename = "opt")]
      optional: Option<String>,
    }

    let mut env = HashMap::new();
    env.insert("PORT", "3000");
    env.insert("MY_APP_CONF_LIST_RENAMED", "item1,item2");
    env.insert("MY_APP_CONF_OPT", "optional");

    let mut loader = ConfigLoader::<Conf>::new();
    loader
      .env_with_provider_and_prefix(&env, "MY_APP_")
      .unwrap();
    let config = loader.finish().unwrap();

    assert_eq!(config.port, 3000);
    assert_eq!(config.list, vec!["item1".to_string(), "item2".to_string()]);
    assert_eq!(config.optional, Some("optional".into()));
  }

  #[test]
  fn from_json_code() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      port: u16,
    }

    let mut loader = ConfigLoader::<Conf>::new();
    loader
      .code(
        r#"
        {
          "port": 3000
        }
        "#,
        Format::Json,
      )
      .unwrap();
    let config = loader.finish().unwrap();

    assert_eq!(config.port, 3000);
  }

  #[test]
  fn should_load_jsonc_code() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      port: u16,
    }

    let mut loader = ConfigLoader::<Conf>::new();
    loader
      .code(
        r#"
        {
          // this is a comment
          "port": 3000
        }
        "#,
        Format::Jsonc,
      )
      .unwrap();
    let config = loader.finish().unwrap();

    assert_eq!(config.port, 3000);
  }

  #[test]
  fn should_load_toml_code() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      port: u16,
    }

    let mut loader = ConfigLoader::<Conf>::new();
    loader
      .code(
        r#"
        port = 3000
        "#,
        Format::Toml,
      )
      .unwrap();

    let config = loader.finish().unwrap();

    assert_eq!(config.port, 3000);
  }

  #[test]
  fn should_load_yaml_code() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      port: u16,
    }

    let mut loader = ConfigLoader::<Conf>::new();
    loader
      .code(
        r#"
        port: 3000
        "#,
        Format::Yaml,
      )
      .unwrap();
    let config = loader.finish().unwrap();

    assert_eq!(config.port, 3000);
  }

  #[test]
  fn should_load_env() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      port: u16,
    }

    let mut env = HashMap::new();
    env.insert("PORT", "3000");

    let mut loader = ConfigLoader::<Conf>::new();
    loader.env_with_provider(&env).unwrap();
    let config = loader.finish().unwrap();

    assert_eq!(config.port, 3000);
  }

  #[test]
  fn should_accumulate_partial_states() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      port: u16,
    }

    let mut loader = ConfigLoader::<Conf>::new();
    loader
      .code(
        r#"
        port: 3000
        "#,
        Format::Yaml,
      )
      .unwrap();
    let partial_state = loader.partial_state();
    assert_eq!(partial_state.port, Some(3000));

    loader
      .code(
        r#"
        port: 3001
        "#,
        Format::Yaml,
      )
      .unwrap();
    let partial_state = loader.partial_state();
    assert_eq!(partial_state.port, Some(3001));
  }

  #[test]
  fn should_merge_partal_states() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      port: u16,
      addr: String,
    }

    let mut loader = ConfigLoader::<Conf>::new();
    loader
      .code(
        r#"
        port: 3000
        addr: "addr"
        "#,
        Format::Yaml,
      )
      .unwrap();

    loader
      .code(
        r#"
        port: 3001
        "#,
        Format::Yaml,
      )
      .unwrap();

    let partial_state = loader.partial_state();

    assert_eq!(partial_state.port, Some(3001));
    assert_eq!(partial_state.addr, Some("addr".into()));

    let config = loader.finish().unwrap();
    assert_eq!(config.port, 3001);
    assert_eq!(config.addr, "addr");
  }

  #[test]
  fn should_error_on_missing_properties() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      port: u16,
      addr: String,
    }

    let mut loader = ConfigLoader::<Conf>::new();
    loader
      .code(
        r#"
        port: 3000
        "#,
        Format::Yaml,
      )
      .unwrap();

    let err = loader.finish().unwrap_err();
    assert!(err.to_string().contains("missing"));
  }

  #[test]
  fn should_list_missing_properties_and_error() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      port: u16,
      addr: String,
    }

    let mut loader = ConfigLoader::<Conf>::new();
    loader
      .code(
        r#"
        port: 3000
        "#,
        Format::Yaml,
      )
      .unwrap();

    let missing = loader.partial_state().list_missing_properties();
    assert_eq!(missing, ["addr"]);

    assert!(loader.finish().is_err());
  }

  #[test]
  fn should_not_list_missing_properties_that_are_optional() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      port: u16,
      addr: Option<String>,
    }

    let mut loader = ConfigLoader::<Conf>::new();
    loader
      .code(
        r#"
        port: 3000
        "#,
        Format::Yaml,
      )
      .unwrap();

    let missing = loader.partial_state().list_missing_properties();
    assert_eq!(missing, Vec::<String>::new());
    assert!(loader.finish().is_ok());
  }

  #[test]
  fn should_skip_env() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      #[config(skip_env)]
      port: u16,
    }

    let mut env = HashMap::new();
    env.insert("PORT", "3000");

    let mut loader = ConfigLoader::<Conf>::new();
    loader.env_with_provider(&env).unwrap();
    let missing = loader.partial_state().list_missing_properties();
    assert_eq!(missing, vec!["port"]);

    loader.finish().unwrap_err();
  }

  #[test]
  fn should_skip_env_for_nested() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      #[config(nested)]
      nested: Nested,
    }

    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Nested {
      #[config(skip_env)]
      port: u16,
    }

    let mut env = HashMap::new();
    env.insert("NESTED_PORT", "3000");

    let mut loader = ConfigLoader::<Conf>::new();
    loader.env_with_provider(&env).unwrap();
    let missing = loader.partial_state().list_missing_properties();
    assert_eq!(missing, ["nested.port"]);

    loader.finish().unwrap_err();
  }

  #[test]
  fn should_skip_env_for_nested_with_prefix() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      #[config(nested)]
      nested: Nested,
    }

    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate, env_prefix = "{}_N_")]
    struct Nested {
      #[config(skip_env)]
      port: u16,
    }

    let mut env = HashMap::new();
    env.insert("MY_APP_N_PORT", "3000");

    let mut loader = ConfigLoader::<Conf>::new();
    loader
      .env_with_provider_and_prefix(&env, "MY_APP_")
      .unwrap();
    let missing = loader.partial_state().list_missing_properties();
    assert_eq!(missing, ["nested.port"]);

    loader.finish().unwrap_err();
  }

  #[test]
  fn should_override_with_env() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      port: u16,
    }

    let mut env = HashMap::new();
    env.insert("PORT", "3000");

    let mut loader = ConfigLoader::<Conf>::new();
    loader
      .code(
        r#"
        port: 3001
        "#,
        Format::Yaml,
      )
      .unwrap();
    loader.env_with_provider(&env).unwrap();
    let config = loader.finish().unwrap();

    assert_eq!(config.port, 3000);
  }

  #[test]
  fn should_override_with_env_with_prefix() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate, env_prefix = "{}CONF_")]
    struct Conf {
      port: u16,
    }

    let mut env = HashMap::new();
    env.insert("MY_APP_CONF_PORT", "3000");

    let mut loader = ConfigLoader::<Conf>::new();
    loader
      .code(
        r#"
        port: 3001
        "#,
        Format::Yaml,
      )
      .unwrap();
    loader
      .env_with_provider_and_prefix(&env, "MY_APP_")
      .unwrap();
    let config = loader.finish().unwrap();

    assert_eq!(config.port, 3000);
  }

  #[test]
  fn should_override_with_env_with_prefix_and_rename() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate, env_prefix = "{}CONF_")]
    struct Conf {
      #[config(rename = "port")]
      port_renamed: u16,
    }

    let mut env = HashMap::new();
    env.insert("MY_APP_CONF_PORT", "3000");

    let mut loader = ConfigLoader::<Conf>::new();
    loader
      .code(
        r#"
        port: 3001
        "#,
        Format::Yaml,
      )
      .unwrap();
    loader
      .env_with_provider_and_prefix(&env, "MY_APP_")
      .unwrap();
    let config = loader.finish().unwrap();

    assert_eq!(config.port_renamed, 3000);
  }

  #[test]
  fn should_override_with_env_with_prefix_and_rename_and_nested() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate, env_prefix = "{}CONF_")]
    struct Conf {
      #[config(nested)]
      nested: Nested,
    }

    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Nested {
      #[config(rename = "port")]
      port_renamed: u16,
    }

    let mut env = HashMap::new();
    env.insert("MY_APP_CONF_NESTED_PORT", "3000");

    let mut loader = ConfigLoader::<Conf>::new();
    loader
      .code(
        r#"
        nested:
          port: 3001
        "#,
        Format::Yaml,
      )
      .unwrap();
    loader
      .env_with_provider_and_prefix(&env, "MY_APP_")
      .unwrap();
    let config = loader.finish().unwrap();

    assert_eq!(config.nested.port_renamed, 3000);
  }

  #[test]
  fn should_error_on_invalid_type() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      port: u16,
    }

    let mut loader = ConfigLoader::<Conf>::new();
    loader
      .code(
        r#"
        {
          "port": "3001"
        }
        "#,
        Format::Json,
      )
      .unwrap_err();
  }

  #[test]
  fn should_not_list_as_missing_optional_types() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      port: Option<u16>,
    }

    let loader = ConfigLoader::<Conf>::new();
    let missing = loader.partial_state().list_missing_properties();
    assert_eq!(missing, Vec::<String>::new());
  }

  #[test]
  fn should_work_for_nested_optional_types() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      #[config(nested)]
      nested: Option<Nested>,
    }

    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Nested {
      port: u16,
    }

    let mut loader = ConfigLoader::<Conf>::new();
    loader
      .code(
        r#"
        nested:
          port: 3000
        "#,
        Format::Yaml,
      )
      .unwrap();
    let config = loader.finish().unwrap();

    assert_eq!(
      config,
      Conf {
        nested: Some(Nested { port: 3000 })
      }
    );
  }

  #[test]
  fn should_work_for_nested_optional_missing_values() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      #[config(nested)]
      nested: Option<Nested>,
    }

    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Nested {
      port: u16,
    }

    let loader = ConfigLoader::<Conf>::new();
    let config = loader.finish().unwrap();

    assert_eq!(config, Conf { nested: None });
  }

  #[test]
  fn should_respect_defaults_from_attrs() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      #[config(default = 3000)]
      port: u16,
    }

    let mut loader = ConfigLoader::<Conf>::new();
    loader.defaults().unwrap();
    let config = loader.finish().unwrap();

    assert_eq!(config.port, 3000);
  }

  #[test]
  fn should_respect_defaults_for_nested_configs() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      #[config(nested)]
      nested: Nested,
    }

    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Nested {
      #[config(default = 3000)]
      port: u16,
    }

    let mut loader = ConfigLoader::<Conf>::new();
    loader.defaults().unwrap();
    let config = loader.finish().unwrap();

    assert_eq!(
      config,
      Conf {
        nested: Nested { port: 3000 }
      }
    );
  }

  #[test]
  fn should_work_with_custom_merge_functions() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      #[config(merge = crate::merge::append_vec, skip_env)]
      list: Vec<String>,
    }

    let mut loader = ConfigLoader::<Conf>::new();
    loader
      .code(
        r#"
        list = ["item1"]
        "#,
        Format::Toml,
      )
      .unwrap();
    loader
      .code(
        r#"
        list = ["item2"]
        "#,
        Format::Toml,
      )
      .unwrap();
    let config = loader.finish().unwrap();

    assert_eq!(config.list, ["item1", "item2"]);
  }

  #[test]
  fn should_error_on_unkown_extra_properties() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      port: u16,
    }

    let mut loader = ConfigLoader::<Conf>::new();
    loader
      .code(
        r#"
        port: 3000
        addr: "addr"
        "#,
        Format::Yaml,
      )
      .unwrap_err();
  }

  #[test]
  fn should_not_error_on_unkown_extra_properties_with_allow_unkown_fields_attr() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate, allow_unknown_fields)]
    struct Conf {
      port: u16,
    }

    let mut loader = ConfigLoader::<Conf>::new();
    loader
      .code(
        r#"
        port: 3000
        addr: "addr"
        "#,
        Format::Yaml,
      )
      .unwrap();
    let config = loader.finish().unwrap();

    assert_eq!(config.port, 3000);
  }

  #[test]
  fn partial_config_should_not_serialize_missing_properties() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      port: u16,
      addr: String,
    }

    let mut loader = ConfigLoader::<Conf>::new();
    loader
      .code(
        r#"
        port: 3000
        "#,
        Format::Yaml,
      )
      .unwrap();
    let partial = loader.partial_state();

    let serialized = serde_json::to_string(&partial).unwrap();
    assert_eq!(serialized, "{\"port\":3000}");
  }

  #[test]
  fn partial_config_should_not_serialize_empty_nested_configs() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      port: u16,
      #[config(nested)]
      nested: Nested,
    }

    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Nested {
      prop: String,
    }

    let mut loader = ConfigLoader::<Conf>::new();
    loader
      .code(
        r#"
        port: 3000
        "#,
        Format::Yaml,
      )
      .unwrap();
    let partial = loader.partial_state();

    let serialized = serde_json::to_string(&partial).unwrap();
    assert_eq!(serialized, "{\"port\":3000}");
  }

  #[test]
  fn should_load_json_file() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      port: u16,
    }

    let path = std::env::temp_dir()
      .as_path()
      .join("metre-test-config.json");
    std::fs::write(&path, "{\"port\": 3000}").unwrap();

    let mut loader = ConfigLoader::<Conf>::new();
    loader.file(path.to_str().unwrap(), Format::Json).unwrap();
    let config = loader.finish().unwrap();

    assert_eq!(config.port, 3000);
  }

  #[test]
  fn should_load_jsonc_file() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      port: u16,
      addr: String,
    }

    let path = std::env::temp_dir()
      .as_path()
      .join("metre-test-config.jsonc");
    std::fs::write(
      &path,
      r#"
      {
        // this is a comment
        "port": 3000,
        "addr": "addr"
      }
      "#,
    )
    .unwrap();

    let mut loader = ConfigLoader::<Conf>::new();
    loader.file(path.to_str().unwrap(), Format::Jsonc).unwrap();
    let config = loader.finish().unwrap();

    assert_eq!(config.port, 3000);
    assert_eq!(config.addr, "addr");
  }

  #[test]
  fn should_load_toml_file() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      port: u16,
      addr: String,
    }

    let path = std::env::temp_dir()
      .as_path()
      .join("metre-test-config.toml");
    std::fs::write(
      &path,
      r#"
      port = 3000
      addr = "addr"
      "#,
    )
    .unwrap();

    let mut loader = ConfigLoader::<Conf>::new();
    loader.file(path.to_str().unwrap(), Format::Toml).unwrap();
    let config = loader.finish().unwrap();

    assert_eq!(config.port, 3000);
    assert_eq!(config.addr, "addr");
  }

  #[test]
  fn should_load_yaml_file() {
    #[derive(crate::Config, Debug, Eq, PartialEq)]
    #[config(crate = crate)]
    struct Conf {
      port: u16,
      addr: String,
    }

    let path = std::env::temp_dir()
      .as_path()
      .join("metre-test-config.yaml");
    std::fs::write(
      &path,
      r#"
      port: 3000
      addr: "addr"
      "#,
    )
    .unwrap();

    let mut loader = ConfigLoader::<Conf>::new();
    loader.file(path.to_str().unwrap(), Format::Yaml).unwrap();
    let config = loader.finish().unwrap();

    assert_eq!(config.port, 3000);
    assert_eq!(config.addr, "addr");
  }
}
