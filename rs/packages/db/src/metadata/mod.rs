use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

//use serde::de::Visitor;
//use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/db/")]
pub struct Metadata(Document);

#[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/db/")]
// Record<string, Value> cannot reference itself in typescript
pub struct Document(BTreeMap<String, Value>);

#[derive(Debug, Clone, TS, Serialize, Deserialize)]
#[ts(export)]
#[ts(export_to = "../../defs/db/")]
#[serde(untagged)]
pub enum Value {
  Null,
  Bool(bool),
  Number(f64),
  String(String),
  Array(Vec<Value>),
  Document(Document),
}

impl Deref for Document {
  type Target = BTreeMap<String, Value>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Document {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Deref for Metadata {
  type Target = BTreeMap<String, Value>;
  fn deref(&self) -> &Self::Target {
    &(self.0).0
  }
}

impl DerefMut for Metadata {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut (self.0).0
  }
}

impl Metadata {
  pub fn merge(&mut self, other: Metadata) -> &mut Self {
    for (key, value) in other.0 .0 {
      if matches!(value, Value::Null) {
        let _ = self.remove(&key);
      } else {
        let _ = self.insert(key, value);
      }
    }
    self
  }
}
