use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S: Serializer>(password: &String, ser: S) -> Result<S::Ok, S::Error> {
  if ser.is_human_readable() {
    "".serialize(ser)
  } else {
    password.serialize(ser)
  }
}

pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<String, D::Error> {
  Deserialize::deserialize(de)
}

pub mod option {

  use serde::{Deserialize, Deserializer, Serialize, Serializer};

  pub fn serialize<S: Serializer>(option: &Option<String>, ser: S) -> Result<S::Ok, S::Error> {
    if ser.is_human_readable() {
      match option.as_ref() {
        None => Option::<String>::None.serialize(ser),
        Some(_) => Some("").serialize(ser),
      }
    } else {
      option.serialize(ser)
    }
  }

  pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<Option<String>, D::Error> {
    Deserialize::deserialize(de)
  }
}
