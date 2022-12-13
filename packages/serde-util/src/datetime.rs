use std::{fmt::Display, ops::Deref};

use bson;
use log::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use time::{macros::offset, OffsetDateTime};
use ts_rs::TS;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/")]
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
      write!(
        f,
        "-{:06}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
        y.abs(),
        m,
        d,
        h,
        min,
        sec,
        milli
      )
    } else {
      write!(
        f,
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
        y, m, d, h, min, sec, milli
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
    bson::DateTime::from_millis(self.timestamp_millis())
  }

  pub fn from_bson(date: bson::DateTime) -> Self {
    Self::new(
      OffsetDateTime::from_unix_timestamp_nanos((date.timestamp_millis() as i128) * 1_000_000)
        .unwrap(),
    )
  }

  pub fn timestamp_millis(self) -> i64 {
    (self.0.unix_timestamp_nanos() / 1_000_000) as i64
  }
}

impl Serialize for DateTime {
  fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
    if ser.is_human_readable() {
      trace!("serializing date as human readable");
      format!("{}", self).serialize(ser)
    } else {
      trace!("serializing date as NOT human readable");
      let target: bson::DateTime = self.into_bson();
      target.serialize(ser)
    }
  }
}

impl<'de> Deserialize<'de> for DateTime {
  fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
    if de.is_human_readable() {
      trace!("deserializing date as human readable");
      time::serde::iso8601::deserialize(de).map(Self::new)
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
      let serialized = serde_json::to_string(&date).unwrap();
      eprintln!("{}", serialized);
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
