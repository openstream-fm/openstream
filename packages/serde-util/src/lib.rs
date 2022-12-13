pub mod number;
pub use number::*;
pub mod bytes;
pub mod datetime;
pub mod ip;
pub mod password;
pub mod status_code;
pub mod voidable;

pub use datetime::DateTime;

#[cfg(test)]
mod test {
  use bson::Bson;
  use serde::Deserializer;
  use serde::Serializer;

  #[test]
  fn local_patch() {
    assert!(bson::is_local_patch());
  }

  #[test]
  fn serializer_is_not_human_readable() {
    let ser = bson::Serializer::new();
    assert!(!ser.is_human_readable())
  }

  #[test]
  fn deserializer_is_not_human_readable() {
    let de = bson::Deserializer::new(Bson::Null);
    assert!(!de.is_human_readable())
  }
}
