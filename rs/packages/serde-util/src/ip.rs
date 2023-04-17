use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{net::IpAddr, str::FromStr};

pub fn serialize<S: Serializer>(ip: &IpAddr, ser: S) -> Result<S::Ok, S::Error> {
  ip.to_string().serialize(ser)
}

pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<IpAddr, D::Error> {
  let helper = String::deserialize(de)?;
  match IpAddr::from_str(helper.as_str()) {
    Ok(v) => Ok(v),
    Err(e) => Err(D::Error::custom(format!("invalid ip address: {e}"))),
  }
}

pub mod option {
  use super::*;

  pub fn serialize<S: Serializer>(ip: &Option<IpAddr>, ser: S) -> Result<S::Ok, S::Error> {
    ip.as_ref().map(ToString::to_string).serialize(ser)
  }

  pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<Option<IpAddr>, D::Error> {
    let helper: Option<String> = Deserialize::deserialize(de)?;
    match helper {
      None => Ok(None),
      Some(v) => match IpAddr::from_str(v.as_str()) {
        Ok(v) => Ok(Some(v)),
        Err(e) => Err(D::Error::custom(format!("invalid ip address: {e}"))),
      },
    }
  }
}
