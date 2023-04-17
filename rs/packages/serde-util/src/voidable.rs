use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S: Serializer, T: Serialize>(
  value: &Option<Option<T>>,
  ser: S,
) -> Result<S::Ok, S::Error> {
  Serialize::serialize(value, ser)
}

pub fn deserialize<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
  de: D,
) -> Result<Option<Option<T>>, D::Error> {
  Deserialize::deserialize(de).map(Some)
}

pub fn skip<T>(value: &Option<Option<T>>) -> bool {
  value.is_none()
}
