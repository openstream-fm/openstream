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

openapi::impl_schema_from!(TimezoneDateTime, DateTimeSchema);

#[derive(JsonSchema)]
#[schemars(rename = "TimezoneDateTime")]
struct DateTimeSchema(chrono::DateTime<Utc>);

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
