use bson;
use chrono::{self, Utc};
use log::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/**
 * Serialize as `chrono::DateTime` in human-readable formats and
 * as `bson::DateTime` in non human-readable formats  
 */
pub fn serialize<S: Serializer>(date: &chrono::DateTime<Utc>, ser: S) -> Result<S::Ok, S::Error> {
  if ser.is_human_readable() {
    trace!("serializing date as human readable");
    Serialize::serialize(&date, ser)
  } else {
    trace!("serializing date as NOT human readable");
    let target: bson::DateTime = (*date).into();
    Serialize::serialize(&target, ser)
  }
}

/**
 * Deserialize `chrono::DateTime` from `chrono::DateTime` in human-readable formats and
 * from `bson::DateTime` in non human-readable formats  
 */
pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<chrono::DateTime<Utc>, D::Error> {
  if de.is_human_readable() {
    trace!("deserializing date as human readable");
    Deserialize::deserialize(de)
  } else {
    trace!("deserializing date as NOT human readable");
    let helper: bson::DateTime = Deserialize::deserialize(de)?;
    Ok(helper.into())
  }
}

pub mod option {
  use super::*;

  /**
   * Serialize as `Option<chrono::DateTime>` in human-readable formats and
   * as `Option<bson::DateTime>` in non human-readable formats  
   */
  pub fn serialize<S: Serializer>(
    opt: &Option<chrono::DateTime<Utc>>,
    ser: S,
  ) -> Result<S::Ok, S::Error> {
    if ser.is_human_readable() {
      trace!("serializing optional date as human readable");
      opt.serialize(ser)
    } else {
      trace!("serializing optional date as NOT human readable");
      match opt {
        None => opt.serialize(ser),
        Some(date) => {
          let target: bson::DateTime = (*date).into();
          target.serialize(ser)
        }
      }
    }
  }

  /**
   * Deserialize `Option<chrono::DateTime>` from `Option<chrono::DateTime>` in human-readable formats and
   * from `Option<bson::DateTime>` in non human-readable formats  
   */
  pub fn deserialize<'de, D: Deserializer<'de>>(
    de: D,
  ) -> Result<Option<chrono::DateTime<Utc>>, D::Error> {
    if de.is_human_readable() {
      trace!("deserializing optional date as human readable");
      Deserialize::deserialize(de)
    } else {
      trace!("deserializing optional date as NOT human readable");
      let opt: Option<bson::DateTime> = Deserialize::deserialize(de)?;
      match opt {
        None => Ok(None),
        Some(date) => Ok(Some(date.into())),
      }
    }
  }
}
