use hyper::header::{HeaderMap, HeaderName, HeaderValue};
use indexmap::{
  map::{IntoIter, Iter, Keys, Values},
  IndexMap,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Default, Clone, Serialize, Deserialize, TS, Eq, PartialEq, JsonSchema)]
#[ts(export, export_to = "../../../defs/db/http/")]
pub struct Headers(IndexMap<String, String>);

impl Headers {
  pub const REDACTED_VALUE: &'static str = "REDACTED";

  #[inline]
  pub fn is_sensible_key(key: &str) -> bool {
    key.eq_ignore_ascii_case(constants::ACCESS_TOKEN_HEADER) || key.eq_ignore_ascii_case("cookie")
  }

  #[inline]
  pub fn new() -> Self {
    Self(IndexMap::new())
  }

  #[inline]
  pub fn with_capacity(cap: usize) -> Self {
    Self(IndexMap::with_capacity(cap))
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.0.len()
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  #[inline]
  pub fn capacity(&self) -> usize {
    self.0.capacity()
  }

  #[inline]
  pub fn reserve(&mut self, additional: usize) {
    self.0.reserve(additional);
  }

  #[inline]
  pub fn keys(&self) -> Keys<String, String> {
    self.0.keys()
  }

  #[inline]
  pub fn values(&self) -> Values<String, String> {
    self.0.values()
  }

  #[inline]
  pub fn iter(&self) -> Iter<String, String> {
    self.0.iter()
  }

  #[inline]
  pub fn get(&self, key: &str) -> Option<&str> {
    self.0.get(&key.to_lowercase()).map(|v| v.as_str())
  }

  #[inline]
  pub fn insert(&mut self, key: &str, value: &str) -> Option<String> {
    self.0.insert(key.to_lowercase(), value.to_lowercase())
  }

  #[inline]
  pub fn remove(&mut self, key: &str) -> Option<String> {
    self.0.remove(&key.to_lowercase())
  }

  pub fn from_http(headers: &HeaderMap) -> Self {
    let mut map = IndexMap::with_capacity(headers.keys_len());
    for (key, value) in headers.iter() {
      let key = key.as_str().to_lowercase();
      let value = if Self::is_sensible_key(&key) {
        String::from(Self::REDACTED_VALUE)
      } else {
        String::from_utf8_lossy(value.as_bytes()).to_string()
      };

      map.insert(key, value);
    }

    Self(map)
  }

  pub fn to_http(&self) -> HeaderMap {
    let mut map = HeaderMap::with_capacity(self.len());
    for (key, value) in self.iter() {
      if let Ok(key) = HeaderName::from_bytes(key.as_bytes()) {
        if let Ok(value) = HeaderValue::from_str(value) {
          map.append(key, value);
        }
      }
    }

    map
  }
}

impl From<HeaderMap> for Headers {
  #[inline]
  fn from(headers: HeaderMap) -> Self {
    Self::from_http(&headers)
  }
}

impl From<Headers> for HeaderMap {
  #[inline]
  fn from(me: Headers) -> Self {
    me.to_http()
  }
}

impl IntoIterator for Headers {
  type Item = (String, String);
  type IntoIter = IntoIter<String, String>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}
