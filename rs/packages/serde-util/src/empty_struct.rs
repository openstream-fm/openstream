use serde::{Deserialize, Deserializer, Serialize, Serializer};
use ts_rs::TS;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, TS)]
#[ts(export, export_to = "../../../defs/")]
pub struct EmptyStruct(#[ts(type = "Record<string, never>")] pub ());

impl Serialize for EmptyStruct {
  fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    SerdeEmptyStruct { _inner: () }.serialize(s)
  }
}

impl<'de> Deserialize<'de> for EmptyStruct {
  fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let _ = SerdeEmptyStruct::deserialize(d)?;
    Ok(EmptyStruct(()))
  }
}

#[derive(Serialize, Deserialize)]
struct SerdeEmptyStruct {
  #[serde(skip, default)]
  _inner: (),
}
