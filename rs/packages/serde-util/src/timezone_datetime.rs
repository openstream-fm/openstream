use chrono::Utc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, ops::Deref};
use time::OffsetDateTime;
use ts_rs::TS;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/")]
pub struct TimezoneDateTime(
  #[ts(type = "/* TimezoneDateTime */ string")]
  #[serde(with = "time::serde::iso8601")]
  pub OffsetDateTime,
);

#[derive(JsonSchema)]
#[schemars(rename = "TimezoneDateTime")]
struct DateTimeSchemars(chrono::DateTime<Utc>);

impl JsonSchema for TimezoneDateTime {
  fn is_referenceable() -> bool {
    DateTimeSchemars::is_referenceable()
  }

  fn schema_id() -> std::borrow::Cow<'static, str> {
    DateTimeSchemars::schema_id()
  }

  fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
    DateTimeSchemars::json_schema(gen)
  }

  fn schema_name() -> String {
    DateTimeSchemars::schema_name()
  }
}

impl Display for TimezoneDateTime {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.fmt(f)
  }
}

impl Deref for TimezoneDateTime {
  type Target = OffsetDateTime;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl From<time::OffsetDateTime> for TimezoneDateTime {
  fn from(time: time::OffsetDateTime) -> Self {
    Self(time)
  }
}

impl From<TimezoneDateTime> for time::OffsetDateTime {
  fn from(date: TimezoneDateTime) -> Self {
    date.0
  }
}
