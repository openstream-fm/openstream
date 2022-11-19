use bson;
use chrono::{self, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/**
 * Serialize as `chrono::DateTime` in human-readable formats and
 * as `bson::DateTime` in non human-readable formats  
 */
pub fn serialize<S: Serializer>(date: &chrono::DateTime<Utc>, ser: S) -> Result<S::Ok, S::Error> {
  if ser.is_human_readable() {
    Serialize::serialize(&date, ser)
  } else {
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
    Deserialize::deserialize(de)
  } else {
    let temp: bson::DateTime = Deserialize::deserialize(de)?;
    let target: chrono::DateTime<Utc> = temp.into();
    Ok(target)
  }
}

pub mod option {

  use chrono::Utc;
  use serde::{Deserialize, Deserializer, Serialize, Serializer};

  /**
   * Serialize as `Option<chrono::DateTime>` in human-readable formats and
   * as `Option<bson::DateTime>` in non human-readable formats  
   */
  pub fn serialize<S: Serializer>(
    opt: &Option<chrono::DateTime<Utc>>,
    ser: S,
  ) -> Result<S::Ok, S::Error> {
    if ser.is_human_readable() {
      opt.serialize(ser)
    } else {
      match opt {
        None => ().serialize(ser),
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
      Deserialize::deserialize(de)
    } else {
      let opt: Option<bson::DateTime> = Deserialize::deserialize(de)?;
      match opt {
        None => Ok(None),
        Some(date) => Ok(Some(date.into())),
      }
    }
  }
}
