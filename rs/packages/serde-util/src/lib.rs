pub mod number;
pub use number::*;
pub mod bytes;
pub mod datetime;
pub mod empty_struct;
pub mod ip;
pub mod password;
pub mod port;
pub mod status_code;
pub mod voidable;
pub use datetime::DateTime;
pub use mongodb::bson;

use serde::{Deserialize, Deserializer};

pub fn map_some<'de, T: Deserialize<'de>, D: Deserializer<'de>>(
  de: D,
) -> Result<Option<T>, D::Error> {
  let v = T::deserialize(de)?;
  Ok(Some(v))
}

#[cfg(test)]
mod test {
  use crate::bson::{self, Bson};
  use serde::Deserializer;
  use serde::Serializer;

  // #[ignore]
  // #[test]
  // fn local_patch() {
  //   assert!(bson::is_local_patch());
  // }

  #[ignore]
  #[test]
  fn serializer_is_not_human_readable() {
    let ser = bson::Serializer::new();
    assert!(!ser.is_human_readable())
  }

  #[ignore]
  #[test]
  fn deserializer_is_not_human_readable() {
    let de = bson::Deserializer::new(Bson::Null);
    assert!(!de.is_human_readable())
  }
}
