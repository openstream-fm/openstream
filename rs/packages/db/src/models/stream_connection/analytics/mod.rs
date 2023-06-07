use std::{
  collections::{HashMap, HashSet},
  net::IpAddr,
};

use futures_util::TryStreamExt;
use geoip::CountryCode;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use ts_rs::TS;

use crate::{station::Station, stream_connection::lite::StreamConnectionLite, Model};

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/analytics/")]
pub struct Analytics {
  pub stations: Vec<AnalyticsStation>,

  #[ts(type = "/** time::DateTime */ string")]
  #[serde(with = "time::serde::iso8601")]
  pub since: time::OffsetDateTime,

  #[ts(type = "/** time::DateTime\n */ string")]
  #[serde(with = "time::serde::iso8601")]
  pub until: time::OffsetDateTime,

  pub utc_offset_minutes: i16,

  #[serde(with = "serde_util::as_f64")]
  pub sessions: u64,

  #[serde(with = "serde_util::as_f64")]
  pub ips: u64,

  #[serde(with = "serde_util::as_f64")]
  pub total_duration_ms: u64,

  #[serde(with = "serde_util::as_f64")]
  pub total_transfer_bytes: u64,

  pub by_month: Vec<AnalyticsItem<YearMonth>>,
  pub by_day: Vec<AnalyticsItem<YearMonthDay>>,
  pub by_hour: Vec<AnalyticsItem<u8>>,
  pub by_browser: Vec<AnalyticsItem<Option<String>>>,
  pub by_os: Vec<AnalyticsItem<Option<String>>>,
  pub by_country: Vec<AnalyticsItem<Option<CountryCode>>>,
  pub by_station: Vec<AnalyticsItem<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/analytics/")]
pub struct AnalyticsItem<K> {
  key: K,
  #[serde(with = "serde_util::as_f64")]
  sessions: u64,
  #[serde(with = "serde_util::as_f64")]
  total_duration_ms: u64,
  #[serde(with = "serde_util::as_f64")]
  total_transfer_bytes: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq, Hash, Ord, PartialOrd, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/analytics/")]
pub struct YearMonth {
  pub year: u16,
  pub month: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq, Hash, Ord, PartialOrd, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/analytics/")]
pub struct YearMonthDay {
  pub year: u16,
  pub month: u8,
  pub day: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/analytics/")]
#[macros::keys]
pub struct AnalyticsStation {
  #[serde(rename = "_id")]
  pub id: String,
  pub name: String,
  pub created_at: serde_util::DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsQuery {
  pub station_ids: Vec<String>,
  pub start_date: time::OffsetDateTime,
  pub end_date: time::OffsetDateTime,
}

pub async fn get_analytics(query: AnalyticsQuery) -> Result<Analytics, mongodb::error::Error> {
  let stations = {
    let filter = doc! {
      Station::KEY_ID: {
        "$in": &query.station_ids,
      }
    };

    let projection = doc! {
      Station::KEY_ID: 1,
      Station::KEY_NAME: 1,
      Station::KEY_CREATED_AT: 1,
    };

    let options = mongodb::options::FindOptions::builder()
      .projection(projection)
      .build();

    let stations: Vec<AnalyticsStation> = Station::cl_as::<AnalyticsStation>()
      .find(filter, options)
      .await?
      .try_collect()
      .await?;

    stations
  };

  let mut start_date = query.start_date;
  let mut end_date = query.end_date;

  if start_date.unix_timestamp() > end_date.unix_timestamp() {
    (start_date, end_date) = (end_date.to_offset(start_date.offset()), start_date);
  }

  let now = OffsetDateTime::now_utc().to_offset(start_date.offset());
  if now.unix_timestamp() < end_date.unix_timestamp() {
    end_date = now;
  }

  if now.unix_timestamp() < start_date.unix_timestamp() {
    start_date = now;
  }

  for station in stations.iter() {
    if start_date.unix_timestamp() < station.created_at.unix_timestamp() {
      start_date = station.created_at.to_offset(start_date.offset());
    }

    if end_date.unix_timestamp() < station.created_at.unix_timestamp() {
      end_date = station.created_at.to_offset(end_date.offset());
    }
  }

  let ser_start_date: serde_util::DateTime = start_date.into();
  let ser_end_date: serde_util::DateTime = end_date.into();

  let filter = doc! {
    StreamConnectionLite::KEY_STATION_ID: {
      "$in": &query.station_ids,
    },
    StreamConnectionLite::KEY_DURATION_MS: {
      "$ne": null
    },
    // StreamConnectionLite::KEY_IS_OPEN: false,
    StreamConnectionLite::KEY_CREATED_AT: {
      "$gte": ser_start_date,
      "$lt": ser_end_date,
    }
  };

  let sort = doc! {
    StreamConnectionLite::KEY_CREATED_AT: 1,
  };

  let options = mongodb::options::FindOptions::builder().sort(sort).build();

  let mut cursor = StreamConnectionLite::cl().find(filter, options).await?;

  let mut sessions: u64 = 0;
  let mut ips = HashSet::<IpAddr>::new();
  let mut total_duration_ms: u64 = 0;
  let mut total_transfer_bytes: u64 = 0;

  #[derive(Default)]
  struct AccumulatorItem {
    sessions: u64,
    total_duration_ms: u64,
    total_transfer_bytes: u64,
  }

  let mut months_accumulator = HashMap::<YearMonth, AccumulatorItem>::new();
  let mut days_accumulator = HashMap::<YearMonthDay, AccumulatorItem>::new();
  let mut hours_accumulator = HashMap::<u8, AccumulatorItem>::new();
  let mut browser_accumulator = HashMap::<Option<String>, AccumulatorItem>::new();
  let mut os_accumulator = HashMap::<Option<String>, AccumulatorItem>::new();
  let mut country_accumulator = HashMap::<Option<CountryCode>, AccumulatorItem>::new();
  let mut station_accumulator = HashMap::<String, AccumulatorItem>::new();

  // accumulate
  while let Some(conn) = cursor.try_next().await? {
    let created_at = conn.created_at.to_offset(query.start_date.offset());
    let conn_duration_ms = conn.duration_ms.unwrap_or(0);
    let conn_transfer_bytes = conn.transfer_bytes.unwrap_or(0);
    let conn_year = created_at.year() as u16;
    let conn_month = created_at.month() as u8;
    let conn_day = created_at.day();
    let conn_hour = conn.created_at.hour();
    let conn_browser = conn.browser;
    let conn_os = conn.os;

    sessions += 1;
    total_duration_ms += conn_duration_ms;
    total_transfer_bytes += conn_transfer_bytes;
    ips.insert(conn.ip);

    macro_rules! add {
      ($acc:ident, $key:expr) => {
        let mut item = $acc.entry($key).or_default();
        item.sessions += 1;
        item.total_duration_ms += conn_duration_ms;
        item.total_transfer_bytes += conn_transfer_bytes;
      };
    }

    add!(
      months_accumulator,
      YearMonth {
        year: conn_year,
        month: conn_month
      }
    );
    add!(
      days_accumulator,
      YearMonthDay {
        year: conn_year,
        month: conn_month,
        day: conn_day
      }
    );
    add!(hours_accumulator, conn_hour);
    add!(browser_accumulator, conn_browser);
    add!(os_accumulator, conn_os);
    add!(country_accumulator, conn.country_code);
    add!(station_accumulator, conn.station_id);
  }

  macro_rules! collect {
    ($acc:ident) => {
      $acc
        .into_iter()
        .map(|(key, value)| AnalyticsItem::<_> {
          key,
          sessions: value.sessions,
          total_duration_ms: value.total_duration_ms,
          total_transfer_bytes: value.total_transfer_bytes,
        })
        .collect::<Vec<_>>()
    };
  }

  // collect
  let mut by_month = collect!(months_accumulator);
  let mut by_day = collect!(days_accumulator);
  let mut by_hour = collect!(hours_accumulator);
  let mut by_browser = collect!(browser_accumulator);
  let mut by_os = collect!(os_accumulator);
  let mut by_country = collect!(country_accumulator);
  let mut by_station = collect!(station_accumulator);

  // sort
  macro_rules! sort_by_key {
    ($ident:ident) => {
      $ident.sort_by(|a, b| a.key.cmp(&b.key));
    };
  }

  macro_rules! sort_by_sessions {
    ($ident:ident) => {
      $ident.sort_by(|a, b| b.sessions.cmp(&a.sessions));
    };
  }

  sort_by_key!(by_month);
  sort_by_key!(by_day);
  sort_by_key!(by_hour);

  sort_by_sessions!(by_browser);
  sort_by_sessions!(by_os);
  sort_by_sessions!(by_country);
  sort_by_sessions!(by_station);

  // render
  let out = Analytics {
    since: start_date,
    until: end_date,
    utc_offset_minutes: query.start_date.offset().whole_minutes(),
    sessions,
    total_duration_ms,
    total_transfer_bytes,
    ips: ips.len() as u64,
    stations,
    by_month,
    by_day,
    by_hour,
    by_browser,
    by_country,
    by_os,
    by_station,
  };

  Ok(out)
}

#[cfg(test)]
pub mod test {

  use super::*;

  #[test]
  fn analytics_station_and_db_station_have_the_same_key_names() {
    assert_eq!(AnalyticsStation::KEY_ID, Station::KEY_ID);
    assert_eq!(AnalyticsStation::KEY_NAME, Station::KEY_NAME);
    assert_eq!(AnalyticsStation::KEY_CREATED_AT, Station::KEY_CREATED_AT);
  }
}
