use std::{fmt::Display, ops::Deref};

use crate::bson;
use log::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use static_init::dynamic;
use time::{macros::offset, OffsetDateTime, UtcOffset};
use ts_rs::TS;

#[dynamic]
static LOCAL_OFFSET: UtcOffset = {
  use chrono::{Local, TimeZone};

  let offset_secs = Local
    .timestamp_opt(0, 0)
    .unwrap()
    .offset()
    .local_minus_utc();

  UtcOffset::from_whole_seconds(offset_secs).unwrap_or(offset!(UTC))
};

pub fn local_offset() -> UtcOffset {
  *LOCAL_OFFSET
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, TS)]
#[ts(export)]
#[ts(export_to = "../../../defs/")]
pub struct DateTime(#[ts(type = "string")] OffsetDateTime);

impl Display for DateTime {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let (y, m, d, h, min, sec, milli) = (
      self.year(),
      self.month() as u8,
      self.day(),
      self.hour(),
      self.minute(),
      self.second(),
      self.millisecond(),
    );

    if y.is_negative() {
      let abs_y = y.abs();
      write!(
        f,
        "-{abs_y:06}-{m:02}-{d:02}T{h:02}:{min:02}:{sec:02}.{milli:03}Z",
      )
    } else {
      write!(
        f,
        "{y:04}-{m:02}-{d:02}T{h:02}:{min:02}:{sec:02}.{milli:03}Z"
      )
    }
  }
}

impl Deref for DateTime {
  type Target = OffsetDateTime;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

fn with_ms_precision(date: OffsetDateTime) -> OffsetDateTime {
  date.replace_millisecond(date.millisecond()).unwrap()
}

impl DateTime {
  pub fn new(datetime: OffsetDateTime) -> Self {
    Self(with_ms_precision(datetime.to_offset(offset!(UTC))))
  }

  pub fn now() -> Self {
    Self::new(OffsetDateTime::now_utc())
  }

  pub fn inner(self) -> OffsetDateTime {
    self.0
  }

  pub fn into_bson(self) -> bson::DateTime {
    bson::DateTime::from_millis(self.timestamp_millis() as i64)
  }

  pub fn from_bson(date: bson::DateTime) -> Self {
    Self::new(
      OffsetDateTime::from_unix_timestamp_nanos((date.timestamp_millis() as i128) * 1_000_000)
        .unwrap(),
    )
  }

  pub fn from_ts(ts: iso8601_timestamp::Timestamp) -> Self {
    let millis = ts
      .duration_since(iso8601_timestamp::Timestamp::UNIX_EPOCH)
      .whole_milliseconds() as i64;

    Self::new(OffsetDateTime::from_unix_timestamp_nanos(millis as i128 * 1_000_000).unwrap())
  }

  pub fn timestamp_millis(self) -> i128 {
    self.0.unix_timestamp_nanos() / 1_000_000
  }
}

impl Serialize for DateTime {
  fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
    if ser.is_human_readable() {
      trace!("serializing date as human readable");
      format!("{self}").serialize(ser)
    } else {
      trace!("serializing date as NOT human readable");
      let target: bson::DateTime = self.into_bson();
      target.serialize(ser)
    }
  }
}

// impl.
struct DateTimeVisitor;

impl<'de> serde::de::Visitor<'de> for DateTimeVisitor {
  type Value = DateTime;

  fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    formatter.write_str("datetime string")
  }

  fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<DateTime, E> {
    match iso8601_timestamp::Timestamp::parse(value) {
      Some(ts) => Ok(DateTime::from_ts(ts)),
      None => Err(E::custom("Invalid iso8601 datetime")),
    }
  }

  fn visit_map<M: serde::de::MapAccess<'de>>(self, map: M) -> Result<DateTime, M::Error> {
    // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
    // into a `Deserializer`, allowing it to be used as the input to T's
    // `Deserialize` implementation. T then deserializes itself using
    // the entries from the map visitor.
    let bson = bson::DateTime::deserialize(serde::de::value::MapAccessDeserializer::new(map))?;
    Ok(DateTime::from_bson(bson))
  }
}

impl<'de> Deserialize<'de> for DateTime {
  fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
    if de.is_human_readable() {
      trace!("deserializing date as human readable");
      de.deserialize_any(DateTimeVisitor)
      // time::serde::iso8601::deserialize(de).map(Self::new)
    } else {
      trace!("deserializing date as NOT human readable");
      let bson = bson::DateTime::deserialize(de)?;
      Ok(Self::from_bson(bson))
    }
  }
}

impl From<DateTime> for bson::Bson {
  fn from(date: DateTime) -> bson::Bson {
    date.into_bson().into()
  }
}

impl From<DateTime> for bson::DateTime {
  fn from(date: DateTime) -> bson::DateTime {
    date.into_bson()
  }
}

impl From<time::OffsetDateTime> for DateTime {
  fn from(time: time::OffsetDateTime) -> Self {
    Self::new(time)
  }
}

impl From<DateTime> for time::OffsetDateTime {
  fn from(date: DateTime) -> Self {
    date.0
  }
}

#[cfg(test)]
pub mod test {
  use super::*;
  use time::macros::datetime;
  #[test]
  fn serde_json() {
    let dates = [
      // TODO allow negative and more wide date ranges (ISO8601)
      // negative years are not allowed in RFC3339
      // 4 digit years only are allowed in RFC3339

      //datetime!(-1-01-01 00:00:00.000 UTC),
      datetime!(0001-01-01 00:01:32.1000 UTC),
      datetime!(0002-01-01 00:01:32.1000 UTC),
      //datetime!(-150-08-01 01:02:32.1000 UTC),
      datetime!(2002-06-01 09:03:32.10001 UTC),
      datetime!(9309-04-01 02:07:32.10002 UTC),
      //datetime!(-9300-02-01 00:03:32.10003 UTC),
      //datetime!(-20-01-01 00:01:32.10000004 -1:30),
      //datetime!(-1700-04-01 1:01:32.1000 -1),
      datetime!(1700-05-01 5:02:32.1000 +3:30),
      datetime!(2001-07-01 10:04:52.1000 +3),
    ];

    for date in dates {
      let date = DateTime::new(date);
      let serialized = serde_json::to_string(&date).unwrap();
      eprintln!("{serialized}");
      let deserialized: DateTime = serde_json::from_str(&serialized).unwrap();
      assert_eq!(date, deserialized)
    }
  }

  #[test]
  fn serde_bson() {
    let dates = [
      datetime!(-1-01-01 00:00:00.000 UTC),
      datetime!(0001-01-01 00:01:32.1000 UTC),
      datetime!(0002-01-01 00:01:32.1000 UTC),
      datetime!(-150-08-01 01:02:32.1000 UTC),
      datetime!(2002-06-01 09:03:32.10001 UTC),
      datetime!(9309-04-01 02:07:32.10002 UTC),
      datetime!(-9300-02-01 00:03:32.10003 UTC),
      datetime!(-20-01-01 00:01:32.10000004 -1:30),
      datetime!(-1700-04-01 1:01:32.1000 -1),
      datetime!(1700-05-01 5:02:32.1000 +3:30),
      datetime!(2001-07-01 10:04:52.1000 +3),
    ];

    for date in dates {
      let date = DateTime::new(date);
      let serialized = bson::to_bson(&date).unwrap();
      let deserialized: DateTime = bson::from_bson(serialized).unwrap();
      assert_eq!(date, deserialized)
    }
  }

  #[test]
  fn serde_bson_binary() {
    #[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
    struct Doc {
      date: DateTime,
    }

    let dates = [
      datetime!(-1-01-01 00:00:00.000 UTC),
      datetime!(0001-01-01 00:01:32.1000 UTC),
      datetime!(0002-01-01 00:01:32.1000 UTC),
      datetime!(-150-08-01 01:02:32.1000 UTC),
      datetime!(2002-06-01 09:03:32.10001 UTC),
      datetime!(9309-04-01 02:07:32.10002 UTC),
      datetime!(-9300-02-01 00:03:32.10003 UTC),
      datetime!(-20-01-01 00:01:32.10000004 -1:30),
      datetime!(-1700-04-01 1:01:32.1000 -1),
      datetime!(1700-05-01 5:02:32.1000 +3:30),
      datetime!(2001-07-01 10:04:52.1000 +3),
    ];

    for date in dates {
      let doc = Doc {
        date: DateTime::new(date),
      };
      let serialized = bson::to_vec(&doc).unwrap();
      let deserialized: Doc = bson::from_slice(serialized.as_ref()).unwrap();
      assert_eq!(doc, deserialized);
    }
  }
}
