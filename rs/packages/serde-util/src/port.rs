use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S: Serializer>(port: u16, ser: S) -> Result<S::Ok, S::Error> {
  (port as i32).serialize(ser)
}

pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<u16, D::Error> {
  let port = i32::deserialize(de)?;
  port.try_into().map_err(D::Error::custom)
}

pub mod option {
  use super::*;

  pub fn serialize<S: Serializer>(port: &Option<u16>, ser: S) -> Result<S::Ok, S::Error> {
    port.as_ref().map(|v| *v as i32).serialize(ser)
  }

  pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<Option<u16>, D::Error> {
    let helper: Option<i32> = Deserialize::deserialize(de)?;
    match helper {
      None => Ok(None),
      Some(v) => match v.try_into() {
        Ok(v) => Ok(Some(v)),
        Err(e) => Err(D::Error::custom(e)),
      },
    }
  }
}

pub mod vec {
  use super::*;

  pub fn serialize<V: AsRef<[u16]>, S: Serializer>(vec: V, ser: S) -> Result<S::Ok, S::Error> {
    vec
      .as_ref()
      .iter()
      .map(|v| *v as i32)
      .collect::<Vec<i32>>()
      .serialize(ser)
  }

  pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<Vec<u16>, D::Error> {
    let helper: Vec<i32> = Deserialize::deserialize(de)?;
    let mut vec = Vec::with_capacity(helper.capacity());
    for item in helper {
      let v = item.try_into().map_err(D::Error::custom)?;
      vec.push(v);
    }
    Ok(vec)
  }
}
