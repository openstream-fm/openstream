use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use schemars::JsonSchema;
//use serde::de::Visitor;
//use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize, TS, JsonSchema)]
#[ts(export)]
#[ts(export_to = "../../../defs/db/")]
pub struct Metadata(Document);

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
#[ts(export_to = "../../../defs/db/")]
pub struct Document(BTreeMap<String, Value>);

openapi::impl_schema_from!(Document, DocumentSchema);

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(untagged)]
pub enum Value {
  Null,
  Bool(bool),
  Number(f64),
  String(String),
  Array(Vec<Value>),
  Document(Document),
}

impl PartialEq for Value {
  fn eq(&self, other: &Self) -> bool {
    use Value::*;
    match (self, other) {
      (Null, Null) => true,
      (Bool(a), Bool(b)) => a == b,
      (String(a), String(b)) => a == b,
      (Array(a), Array(b)) => a == b,
      (Document(a), Document(b)) => a == b,
      (Number(a), Number(b)) => match (f64::is_nan(*a), f64::is_nan(*b)) {
        (true, true) => true,
        (false, false) => a == b,
        _ => false,
      },
      _ => false,
    }
  }
}

impl Eq for Value {}

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

#[derive(JsonSchema)]
#[schemars(rename = "JsonDocument")]
pub struct DocumentSchema(#[allow(unused)] BTreeMap<String, serde_json::Value>);
