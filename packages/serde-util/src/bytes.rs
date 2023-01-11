use crate::bson::Binary;
use base64;
use bytes::Bytes;
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use std::sync::Arc;

/**
 * Trait for types that can be de/serialized to/from bytes
 */
pub trait SerdeBytes: Sized {
  fn from_bson_binary(src: Binary) -> Self;
  fn from_base64(src: &str) -> Result<Self, base64::DecodeError>;
  fn as_slice(&self) -> &[u8];
}

impl SerdeBytes for Bytes {
  fn from_bson_binary(src: Binary) -> Self {
    Self::from(src.bytes)
  }

  fn from_base64(src: &str) -> Result<Self, base64::DecodeError> {
    let vec = base64::decode(src)?;
    Ok(Self::from(vec))
  }

  fn as_slice(&self) -> &[u8] {
    self.as_ref()
  }
}

impl SerdeBytes for Vec<u8> {
  fn from_bson_binary(src: Binary) -> Self {
    src.bytes
  }

  fn from_base64(src: &str) -> Result<Self, base64::DecodeError> {
    base64::decode(src)
  }

  fn as_slice(&self) -> &[u8] {
    self.as_ref()
  }
}

impl SerdeBytes for Arc<Vec<u8>> {
  fn from_bson_binary(src: Binary) -> Self {
    Arc::new(SerdeBytes::from_bson_binary(src))
  }

  fn from_base64(src: &str) -> Result<Self, base64::DecodeError> {
    Ok(Arc::new(SerdeBytes::from_base64(src)?))
  }

  fn as_slice(&self) -> &[u8] {
    self.as_ref()
  }
}

/**
 * serialize as base64 string in human-readable formats and
 * as bytes in non human-readable formats
 */
pub fn serialize<S: Serializer, T: SerdeBytes>(bytes: &T, ser: S) -> Result<S::Ok, S::Error> {
  if ser.is_human_readable() {
    let hex = base64::encode(bytes.as_slice());
    hex.serialize(ser)
  } else {
    ser.serialize_bytes(bytes.as_slice())
  }
}

/**
 * deserialize from base64 string in human-readable formats and
 * from bson::Binary in non human-readable formats
 */
pub fn deserialize<'de, D: Deserializer<'de>, T: SerdeBytes>(de: D) -> Result<T, D::Error> {
  if de.is_human_readable() {
    let helper: &str = Deserialize::deserialize(de)?;
    match T::from_base64(helper) {
      Ok(v) => Ok(v),
      Err(e) => Err(D::Error::custom(format!(
        "cannot decode binary as base64: ({e})",
      ))),
    }
  } else {
    let bin: Binary = Deserialize::deserialize(de)?;
    Ok(T::from_bson_binary(bin))
  }
}
