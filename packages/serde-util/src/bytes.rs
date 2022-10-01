use serde::{de::Error, Serializer, Deserializer, Serialize, Deserialize};
use bytes::Bytes;
use bson::Binary;
use base64;

/**
 * Trait for types that can be de/serialized to/from bytes
 */
pub trait SerdeBytes: Sized {
  fn from_bson_binary(src: Binary) -> Self;
  fn from_base64(src: &str) -> Result<Self, base64::DecodeError>;
}

impl SerdeBytes for Bytes {
  fn from_bson_binary(src: Binary) -> Self {
    Self::from(src.bytes)
  }

  fn from_base64(src: &str) -> Result<Self, base64::DecodeError> {
    let vec = base64::decode(src)?;
    Ok(Self::from(vec))
  }
}

impl SerdeBytes for Vec<u8> {
  fn from_bson_binary(src: Binary) -> Self {
    src.bytes   
  }

  fn from_base64(src: &str) -> Result<Self, base64::DecodeError> {
    base64::decode(src)
  }
}

/**
 * serialize as base64 string in human-readable formats and
 * as bytes in non human-readable formats
 */
pub fn serialize<'b, S, T>(ser: S, bytes: T) -> Result<S::Ok, S::Error>
where S: Serializer, T: AsRef<&'b [u8]> {
  if ser.is_human_readable() {
    let hex = base64::encode(bytes.as_ref());
    hex.serialize(ser)
  } else {
    ser.serialize_bytes(bytes.as_ref())
  }
}

/**
 * deserialize from base64 string in human-readable formats and
 * from bson::Binary in non human-readable formats
 */

pub fn deserialize<'de, D, T>(de: D) -> Result<T, D::Error>
  where D: Deserializer<'de>, T: SerdeBytes {
    if de.is_human_readable() {
      let helper: &str = Deserialize::deserialize(de)?;
      match T::from_base64(helper) {
        Ok(v) => Ok(v),
        Err(e) => Err(D::Error::custom(format!("Cannot decode binary as base64: ({})", e)))
      }
    } else {
      let bin: Binary = Deserialize::deserialize(de)?;
      Ok(T::from_bson_binary(bin))
    }
}