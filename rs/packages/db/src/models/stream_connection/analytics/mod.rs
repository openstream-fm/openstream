use std::{
  collections::{HashMap, HashSet},
  hash::Hash,
  net::IpAddr,
  time::Instant,
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
  pub is_now: bool,

  pub kind: AnalyticsQueryKind,

  pub stations: Vec<AnalyticsStation>,

  #[ts(type = "/** time::DateTime */ string")]
  #[serde(with = "time::serde::iso8601")]
  pub since: time::OffsetDateTime,

  #[ts(type = "/** time::DateTime */ string")]
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

  #[cfg(feature = "analytics-max-concurrent")]
  #[serde(with = "serde_util::as_f64")]
  #[ts(optional)]
  pub max_concurrent_listeners: u64,

  #[cfg(feature = "analytics-max-concurrent")]
  #[ts(optional)]
  pub max_concurrent_listeners_date: Option<serde_util::DateTime>,

  pub by_month: Vec<AnalyticsItem<YearMonth>>,
  pub by_day: Vec<AnalyticsItem<YearMonthDay>>,
  pub by_hour: Vec<AnalyticsItem<u8>>,
  pub by_browser: Vec<AnalyticsItem<Option<String>>>,
  pub by_os: Vec<AnalyticsItem<Option<String>>>,
  pub by_country: Vec<AnalyticsItem<Option<CountryCode>>>,
  pub by_station: Vec<AnalyticsItem<String>>,
  pub by_domain: Vec<AnalyticsItem<Option<String>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/analytics/")]
pub struct AnalyticsItem<K> {
  pub key: K,
  #[serde(with = "serde_util::as_f64")]
  pub sessions: u64,
  #[serde(with = "serde_util::as_f64")]
  pub ips: u64,
  #[serde(with = "serde_util::as_f64")]
  pub total_duration_ms: u64,
  #[serde(with = "serde_util::as_f64")]
  pub total_transfer_bytes: u64,

  #[cfg(feature = "analytics-max-concurrent")]
  #[serde(with = "serde_util::as_f64")]
  #[ts(optional)]
  pub max_concurrent_listeners: u64,

  #[cfg(feature = "analytics-max-concurrent")]
  #[ts(optional)]
  pub max_concurrent_listeners_date: Option<serde_util::DateTime>,
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
  pub kind: AnalyticsQueryKind,
  pub station_ids: Vec<String>,
  pub country_code: Option<Option<CountryCode>>,
  pub browser: Option<Option<String>>,
  pub os: Option<Option<String>>,
  pub domain: Option<Option<String>>,
  pub min_duration_ms: Option<u64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ts_rs::TS)]
#[ts(export, export_to = "../../../defs/analytics/")]
pub enum AnalyticsQueryKind {
  #[serde(rename = "now")]
  Now {
    #[ts(type = "/** time::DateTime */ string")]
    #[serde(with = "time::serde::iso8601")]
    offset_date: time::OffsetDateTime,
  },

  #[serde(rename = "time_range")]
  TimeRange {
    #[ts(type = "/** time::DateTime */ string")]
    #[serde(with = "time::serde::iso8601")]
    since: time::OffsetDateTime,

    #[ts(type = "/** time::DateTime */ string")]
    #[serde(with = "time::serde::iso8601")]
    until: time::OffsetDateTime,
  },
}

pub async fn get_analytics(query: AnalyticsQuery) -> Result<Analytics, mongodb::error::Error> {
  let start = Instant::now();

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

    let sort = doc! {
      Station::KEY_CREATED_AT: 1,
    };

    let options = mongodb::options::FindOptions::builder()
      .projection(projection)
      .sort(sort)
      .build();

    let stations: Vec<AnalyticsStation> = Station::cl_as::<AnalyticsStation>()
      .find(filter, options)
      .await?
      .try_collect()
      .await?;

    stations
  };

  let mut filter = doc! {
    StreamConnectionLite::KEY_STATION_ID: {
      "$in": &query.station_ids,
    }
  };

  let offset_date: OffsetDateTime;
  let kind = query.kind;
  let is_now: bool;

  match kind {
    AnalyticsQueryKind::Now { offset_date: d } => {
      is_now = true;
      offset_date = OffsetDateTime::now_utc().to_offset(d.offset());

      filter = doc! {
        "$and": [
          filter,
          { StreamConnectionLite::KEY_IS_OPEN: true }
        ]
      };
    }

    AnalyticsQueryKind::TimeRange { since, until } => {
      is_now = false;
      let mut start_date = since;
      let mut end_date = (until).to_offset(since.offset());

      let now = OffsetDateTime::now_utc();
      let first_station_created_at = stations
        .first()
        .map(|station| *station.created_at)
        .unwrap_or_else(OffsetDateTime::now_utc);

      if start_date < first_station_created_at {
        start_date = first_station_created_at.to_offset(start_date.offset());
      }

      if end_date < first_station_created_at {
        end_date = first_station_created_at.to_offset(start_date.offset());
      }

      if end_date > now {
        end_date = now.to_offset(end_date.offset());
      }

      if start_date > now {
        start_date = now.to_offset(start_date.offset());
      }

      if start_date > end_date {
        (start_date, end_date) = (end_date, start_date);
      }

      let ser_start_date: serde_util::DateTime = start_date.into();
      let ser_end_date: serde_util::DateTime = end_date.into();

      filter = doc! {
        "$and": [
          filter,
          {
            StreamConnectionLite::KEY_CREATED_AT: {
              "$gte": ser_start_date,
              "$lt": ser_end_date,
            }
          }
        ]
      };

      offset_date = start_date;
    }
  }

  if let Some(os) = query.os {
    filter = doc! {
      "$and": [
        filter,
        {
          StreamConnectionLite::KEY_OS: os,
        }
      ]
    }
  }

  if let Some(browser) = query.browser {
    filter = doc! {
      "$and": [
        filter,
        {
          StreamConnectionLite::KEY_BROWSER: browser,
        }
      ]
    }
  }

  if let Some(cc) = query.country_code {
    filter = doc! {
      "$and": [
        filter,
        {
          // this convertion should never fail
          StreamConnectionLite::KEY_COUNTRY_CODE: mongodb::bson::to_bson(&cc).unwrap()
        }
      ]
    }
  }

  if let Some(domain) = query.domain {
    filter = doc! {
      "$and": [
        filter,
        {
          StreamConnectionLite::KEY_DOMAIN: domain
        }
      ]
    }
  }

  if let Some(d) = query.min_duration_ms {
    filter = doc! {
      "$and": [
        filter,
        {
          "$or": [
            { StreamConnectionLite::KEY_DURATION_MS: null },
            { StreamConnectionLite::KEY_DURATION_MS: { "$gte": d as f64 } },
          ]
        }
      ]
    }
  }

  let sort = doc! {
    StreamConnectionLite::KEY_CREATED_AT: 1,
    // force index usage { ca: 1, st: 1 }
    StreamConnectionLite::KEY_STATION_ID: 1,
  };

  let options = mongodb::options::FindOptions::builder().sort(sort).build();

  let mut cursor = StreamConnectionLite::cl().find(filter, options).await?;

  let mut sessions: u64 = 0;
  let mut ips = HashSet::<IpAddr>::new();
  let mut total_duration_ms: u64 = 0;
  let mut total_transfer_bytes: u64 = 0;

  // u32 is the timestamp and bool is true => start, false => stop
  #[cfg(feature = "analytics-max-concurrent")]
  let mut start_stop_events: Vec<(u32, bool)> = vec![];

  #[derive(Default)]
  struct AccumulatorItem {
    sessions: u64,
    ips: HashSet<IpAddr>,
    total_duration_ms: u64,
    total_transfer_bytes: u64,
    #[cfg(feature = "analytics-max-concurrent")]
    start_stop_events: Vec<(u32, bool)>,
  }

  let mut months_accumulator = HashMap::<YearMonth, AccumulatorItem>::new();
  let mut days_accumulator = HashMap::<YearMonthDay, AccumulatorItem>::new();
  let mut hours_accumulator = HashMap::<u8, AccumulatorItem>::new();
  let mut browser_accumulator = HashMap::<Option<String>, AccumulatorItem>::new();
  let mut os_accumulator = HashMap::<Option<String>, AccumulatorItem>::new();
  let mut country_accumulator = HashMap::<Option<CountryCode>, AccumulatorItem>::new();
  let mut station_accumulator = HashMap::<String, AccumulatorItem>::new();
  let mut domain_accumulator = HashMap::<Option<String>, AccumulatorItem>::new();

  // accumulate
  #[cfg(feature = "test-analytics-base-measure")]
  while let Some(_conn) = cursor.try_next().await? {
    sessions += 1;
  }

  let now = time::OffsetDateTime::now_utc();

  let mut since: Option<OffsetDateTime> = None;
  let mut until: Option<OffsetDateTime> = None;

  #[cfg(not(feature = "test-analytics-base-measure"))]
  while let Some(conn) = cursor.try_next().await? {
    let created_at = conn.created_at.to_offset(offset_date.offset());

    // first (not override)
    if since.is_none() {
      since = Some(created_at);
    }

    // last (override)
    until = Some(created_at);

    let conn_duration_ms = conn
      .duration_ms
      .unwrap_or_else(|| ((created_at - now).as_seconds_f64() * 1000.0) as u64);
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

    #[cfg(feature = "analytics-max-concurrent")]
    let start = created_at.unix_timestamp() as u32;
    #[cfg(feature = "analytics-max-concurrent")]
    start_stop_events.push((start, true));
    #[cfg(feature = "analytics-max-concurrent")]
    let stop = start + (conn_duration_ms / 1000) as u32;

    if !conn.is_open {
      #[cfg(feature = "analytics-max-concurrent")]
      start_stop_events.push((stop, false));
    }

    macro_rules! add {
      ($acc:ident, $key:expr) => {
        let item = $acc.entry($key).or_default();
        item.sessions += 1;
        item.ips.insert(conn.ip);
        item.total_duration_ms += conn_duration_ms;
        item.total_transfer_bytes += conn_transfer_bytes;

        #[cfg(feature = "analytics-max-concurrent")]
        item.start_stop_events.push((start, true));

        #[cfg(feature = "analytics-max-concurrent")]
        if !conn.is_open {
          item.start_stop_events.push((stop, false));
        }
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
    add!(domain_accumulator, conn.domain);
  }

  let accumulate_ms = start.elapsed().as_millis();
  let sort_start = Instant::now();

  #[cfg(feature = "analytics-max-concurrent")]
  macro_rules! max_concurrent {
    ($vec:expr) => {{
      let mut vec = $vec;
      vec.sort_by(|a, b| a.0.cmp(&b.0));

      let mut max: u32 = 0;
      let mut max_timestamp: u32 = 0;
      let mut current: u32 = 0;
      for (timestamp, start) in vec.into_iter() {
        if start {
          current = current.saturating_add(1);
          if current > max {
            max = current;
            max_timestamp = timestamp
          }
        } else {
          current = current.saturating_sub(1);
        }
      }

      let max_concurrent_listeners_date = if max_timestamp == 0 {
        None
      } else {
        time::OffsetDateTime::from_unix_timestamp(max_timestamp as i64)
          .ok()
          .map(serde_util::DateTime::from)
      };

      (max as u64, max_concurrent_listeners_date)
    }};
  }

  macro_rules! collect {
    ($acc:ident) => {
      $acc
        .into_iter()
        .map(|(key, value)| {
          #[cfg(feature = "analytics-max-concurrent")]
          let (max_concurrent_listeners, max_concurrent_listeners_date) =
            max_concurrent!(value.start_stop_events);

          AnalyticsItem::<_> {
            key,
            sessions: value.sessions,
            ips: value.ips.len() as u64,
            total_duration_ms: value.total_duration_ms,
            total_transfer_bytes: value.total_transfer_bytes,
            #[cfg(feature = "analytics-max-concurrent")]
            max_concurrent_listeners,
            #[cfg(feature = "analytics-max-concurrent")]
            max_concurrent_listeners_date,
          }
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
  let mut by_domain = collect!(domain_accumulator);

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
  sort_by_sessions!(by_domain);

  #[cfg(feature = "analytics-max-concurrent")]
  let (max_concurrent_listeners, max_concurrent_listeners_date) =
    max_concurrent!(start_stop_events);

  let sort_ms = sort_start.elapsed().as_millis();

  let since = since.unwrap_or(offset_date);
  let until = until.unwrap_or(offset_date);

  log::info!(
    target: "analytics",
    "got analytics, processed {} connections in {}ms => {}ms acculumate | {}ms sort",
    sessions,
    start.elapsed().as_millis(),
    accumulate_ms,
    sort_ms,
  );

  // render
  let out = Analytics {
    is_now,
    kind,
    since,
    until,
    utc_offset_minutes: offset_date.offset().whole_minutes(),
    sessions,
    total_duration_ms,
    total_transfer_bytes,
    ips: ips.len() as u64,
    #[cfg(feature = "analytics-max-concurrent")]
    max_concurrent_listeners,
    #[cfg(feature = "analytics-max-concurrent")]
    max_concurrent_listeners_date,
    stations,
    by_month,
    by_day,
    by_hour,
    by_browser,
    by_country,
    by_os,
    by_station,
    by_domain,
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
