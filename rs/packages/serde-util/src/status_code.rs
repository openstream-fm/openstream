use hyper::StatusCode;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S: Serializer>(status: &StatusCode, ser: S) -> Result<S::Ok, S::Error> {
  status.as_u16().serialize(ser)
}

pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<StatusCode, D::Error> {
  let helper = u16::deserialize(de)?;
  match StatusCode::from_u16(helper) {
    Ok(v) => Ok(v),
    Err(e) => Err(D::Error::custom(format!("invalid status code: {e}"))),
  }
}

pub mod option {
  use super::*;

  pub fn serialize<S: Serializer>(status: &Option<StatusCode>, ser: S) -> Result<S::Ok, S::Error> {
    status.as_ref().map(StatusCode::as_u16).serialize(ser)
  }

  pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<Option<StatusCode>, D::Error> {
    let helper: Option<u16> = Deserialize::deserialize(de)?;
    match helper {
      None => Ok(None),
      Some(v) => match StatusCode::from_u16(v) {
        Ok(v) => Ok(Some(v)),
        Err(e) => Err(D::Error::custom(format!("invalid status code: {e}"))),
      },
    }
  }
}
