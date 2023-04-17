use std::collections::BTreeMap;
use std::ops::Index;

use std::string::ToString;

#[derive(Debug, Default)]
pub struct Params {
  pub(crate) map: BTreeMap<String, String>,
}

impl Params {
  #[inline]
  pub fn new() -> Self {
    Self::default()
  }

  #[inline]
  pub fn get(&self, key: &str) -> Option<&str> {
    self.map.get(key).map(AsRef::as_ref)
  }

  #[inline]
  pub fn set(&mut self, key: impl ToString, value: impl ToString) {
    self.map.insert(key.to_string(), value.to_string());
  }
}

impl Index<&str> for Params {
  type Output = str;

  #[inline]
  fn index(&self, index: &str) -> &Self::Output {
    self.get(index).unwrap()
  }
}
