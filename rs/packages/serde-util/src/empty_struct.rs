use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use ts_rs::TS;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, TS)]
#[ts(export, export_to = "../../../defs/")]
pub struct EmptyStruct(#[ts(type = "Record<string, never>")] pub ());

impl JsonSchema for EmptyStruct {
  fn schema_id() -> std::borrow::Cow<'static, str> {
    EmptyStructSchema::schema_id()
  }

  fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
    EmptyStructSchema::json_schema(gen)
  }

  fn is_referenceable() -> bool {
    EmptyStructSchema::is_referenceable()
  }

  fn schema_name() -> String {
    EmptyStructSchema::schema_name()
  }
}

#[derive(JsonSchema)]
#[schemars(rename = "EmptyObjectw")]
struct EmptyStructSchema {
  #[schemars(skip)]
  _inner: (),
}

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
