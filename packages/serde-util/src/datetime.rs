use serde::{Serialize, Deserialize, Serializer, Deserializer};
use chrono::{self, Utc};
use bson;

/**
 * Serialize as chrono::DateTime in human-readable formats and 
 * as bson::DateTime in non human-readable formats  
 */
pub fn serialize<S>(ser: S, date: chrono::DateTime<Utc> ) -> Result<S::Ok, S::Error> 
where S: Serializer {
  if ser.is_human_readable() {
    Serialize::serialize(&date, ser)
  } else {
    let target = bson::DateTime::from(date);
    Serialize::serialize(&target, ser)
  }
} 


/**
 * Deserialize chrono::DateTime from chrono::DateTime in human-readable formats and 
 * from bson::DateTime in non human-readable formats  
 */
pub fn deserialize<'de, D>(de: D) -> Result<chrono::DateTime<Utc>, D::Error>
where D: Deserializer<'de> {
    if de.is_human_readable() {
      Deserialize::deserialize(de)
    } else {
      let temp: bson::DateTime = Deserialize::deserialize(de)?;
      let target: chrono::DateTime<Utc> = temp.into();
      Ok(target)
    }
}