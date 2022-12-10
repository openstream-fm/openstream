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
    for (key, value) in (other.0).0.into_iter() {
      let _ = self.insert(key, value);
    }
    self
  }
}

/*
impl Serialize for Metadata {
  #[inline]
  fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    (self.0).serialize(s)
  }
}
*/

/*
impl Serialize for Value {
  fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    match *self {
      Value::Null => ().serialize(s),
      Value::Bool(v) => v.serialize(s),
      Value::Number(v) => v.serialize(s),
      Value::String(ref v) => v.serialize(s),
      Value::Array(ref v) => v.serialize(s),
      Value::Document(ref v) => v.serialize(s),
    }
  }
}
 */

/*
impl Serialize for Document {
  fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    let mut doc = s.serialize_map(Some(self.len()))?;

    for (key, value) in self.iter() {
      match value {
        Value::Null => doc.serialize_entry(key, &())?,
        Value::Bool(v) => doc.serialize_entry(key, v)?,
        Value::Number(v) => doc.serialize_entry(key, v)?,
        Value::String(v) => doc.serialize_entry(key, v)?,
        Value::Array(v) => doc.serialize_entry(key, v)?,
        Value::Document(v) => doc.serialize_entry(key, v)?,
      }
    }

    doc.end()
  }
}
 */

/*
impl<'de> Deserialize<'de> for Document {
  #[inline]
  fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let map = BTreeMap::<String, Value>::deserialize(d)?;
    Ok(Self(map))
  }
}

impl<'de> Deserialize<'de> for Metadata {
  #[inline]
  fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let doc = Document::deserialize(d)?;
    Ok(Self(doc))
  }
}

impl<'de> Deserialize<'de> for Value {
  fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    d.deserialize_any(ValueVisitor {})
  }
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
  type Value = Value;
  fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "a valid JSON-like value")
  }

  #[inline]
  fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
    Ok(Value::Null)
  }

  #[inline]
  fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
    Ok(Value::Null)
  }

  #[inline]
  fn visit_some<D: serde::Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
    Deserialize::deserialize(d)
  }

  #[inline]
  fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<Self::Value, E> {
    Ok(Value::Bool(v))
  }

  #[inline]
  fn visit_u8<E: serde::de::Error>(self, v: u8) -> Result<self::Value, E> {
    Ok(Value::Number(v as f64))
  }

  #[inline]
  fn visit_i8<E: serde::de::Error>(self, v: i8) -> Result<self::Value, E> {
    Ok(Value::Number(v as f64))
  }

  #[inline]
  fn visit_u16<E: serde::de::Error>(self, v: u16) -> Result<self::Value, E> {
    Ok(Value::Number(v as f64))
  }

  #[inline]
  fn visit_i16<E: serde::de::Error>(self, v: i16) -> Result<self::Value, E> {
    Ok(Value::Number(v as f64))
  }

  #[inline]
  fn visit_u32<E: serde::de::Error>(self, v: u32) -> Result<self::Value, E> {
    Ok(Value::Number(v as f64))
  }

  #[inline]
  fn visit_i32<E: serde::de::Error>(self, v: i32) -> Result<self::Value, E> {
    Ok(Value::Number(v as f64))
  }

  #[inline]
  fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<self::Value, E> {
    Ok(Value::Number(v as f64))
  }

  #[inline]
  fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<self::Value, E> {
    Ok(Value::Number(v as f64))
  }

  #[inline]
  fn visit_u128<E: serde::de::Error>(self, v: u128) -> Result<self::Value, E> {
    Ok(Value::Number(v as f64))
  }

  #[inline]
  fn visit_i128<E: serde::de::Error>(self, v: i128) -> Result<self::Value, E> {
    Ok(Value::Number(v as f64))
  }

  #[inline]
  fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
    Ok(Value::String(String::from(v)))
  }

  #[inline]
  fn visit_borrowed_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
    Ok(Value::String(String::from(v)))
  }

  #[inline]
  fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
    Ok(Value::String(v))
  }

  #[inline]
  fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
    let mut vec = vec![];
    while let Some(v) = seq.next_element()? {
      vec.push(v)
    }

    Ok(Value::Array(vec))
  }

  #[inline]
  fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
    let mut bmap = BTreeMap::new();
    while let Some((k, v)) = map.next_entry()? {
      let _ = bmap.insert(k, v);
    }

    Ok(Value::Document(Document(bmap)))
  }
}
 */
