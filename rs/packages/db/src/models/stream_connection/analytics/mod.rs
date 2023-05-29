use std::{
  collections::{HashMap, HashSet},
  net::IpAddr,
};

use futures_util::TryStreamExt;
use geoip::CountryCode;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{station::Station, stream_connection::lite::StreamConnectionLite, Model};

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/analytics/")]
pub struct Analytics {
  pub stations: Vec<AnalyticsStation>,

  pub start_date: AnalyticsDate,
  pub end_date: AnalyticsDate,

  #[serde(with = "serde_util::as_f64")]
  pub sessions: u64,

  #[serde(with = "serde_util::as_f64")]
  pub ips: u64,

  pub total_duration_ms: f64,

  pub by_day: Vec<AnalyticsByDay>,
  pub by_month: Vec<AnalyticsByMonth>,
  pub by_browser: Vec<AnalyticsByBrowser>,
  pub by_os: Vec<AnalyticsByOs>,
  pub by_country: Vec<AnalyticsByCountry>,
  pub by_station: Vec<AnalyticsByStation>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/analytics/")]
pub struct AnalyticsDate {
  pub year: u16,
  pub month: u8,
  pub day: u8,
}

impl From<time::OffsetDateTime> for AnalyticsDate {
  fn from(value: time::OffsetDateTime) -> Self {
    Self {
      year: value.year() as u16,
      month: value.month() as u8,
      day: value.day(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/analytics/")]
pub struct AnalyticsByCountry {
  pub country_code: Option<CountryCode>,

  #[serde(with = "serde_util::as_f64")]
  pub sessions: u64,
  pub total_duration_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/analytics/")]
pub struct AnalyticsByBrowser {
  pub browser: Option<String>,

  #[serde(with = "serde_util::as_f64")]
  pub sessions: u64,
  pub total_duration_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/analytics/")]
pub struct AnalyticsByOs {
  pub os: Option<String>,

  #[serde(with = "serde_util::as_f64")]
  pub sessions: u64,
  pub total_duration_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/analytics/")]
pub struct AnalyticsByDay {
  #[serde(with = "serde_util::as_f64")]
  pub year: u16,

  #[serde(with = "serde_util::as_f64")]
  pub month: u8,

  #[serde(with = "serde_util::as_f64")]
  pub day: u8,

  #[serde(with = "serde_util::as_f64")]
  pub sessions: u64,

  pub total_duration_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/analytics/")]
pub struct AnalyticsByStation {
  pub station_id: String,

  #[serde(with = "serde_util::as_f64")]
  pub sessions: u64,

  pub total_duration_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/analytics/")]
pub struct AnalyticsByMonth {
  #[serde(with = "serde_util::as_f64")]
  pub year: u16,

  #[serde(with = "serde_util::as_f64")]
  pub month: u8,

  #[serde(with = "serde_util::as_f64")]
  pub sessions: u64,

  pub total_duration_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/analytics/")]
#[macros::keys]
pub struct AnalyticsStation {
  #[serde(rename = "_id")]
  pub id: String,
  pub name: String,
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

  let start_date: serde_util::DateTime = query.start_date.into();
  let end_date: serde_util::DateTime = query.end_date.into();

  let filter = doc! {
    StreamConnectionLite::KEY_STATION_ID: {
      "$in": &query.station_ids,
    },
    // TODO: change this to StreamConnectionLite::KEY_DURATION: { "$ne": null }
    StreamConnectionLite::KEY_IS_OPEN: false,
    StreamConnectionLite::KEY_CREATED_AT: {
      "$gte": start_date,
      "$lt": end_date,
    }
  };

  let sort = doc! {
    StreamConnectionLite::KEY_CREATED_AT: 1,
  };

  let options = mongodb::options::FindOptions::builder().sort(sort).build();

  let mut cursor = StreamConnectionLite::cl().find(filter, options).await?;

  let mut sessions: u64 = 0;
  let mut ips = HashSet::<IpAddr>::new();
  let mut total_duration_ms: f64 = 0.0;

  #[derive(Default)]
  struct AccumulatorItem {
    sessions: u64,
    total_duration_ms: f64,
  }

  let mut days_accumulator = HashMap::<(u16, u8, u8), AccumulatorItem>::new();
  let mut months_accumulator = HashMap::<(u16, u8), AccumulatorItem>::new();
  let mut browser_accumulator = HashMap::<Option<String>, AccumulatorItem>::new();
  let mut os_accumulator = HashMap::<Option<String>, AccumulatorItem>::new();
  let mut country_accumulator = HashMap::<Option<CountryCode>, AccumulatorItem>::new();
  let mut station_accumulator = HashMap::<String, AccumulatorItem>::new();

  while let Some(conn) = cursor.try_next().await? {
    let conn_duration_ms = 0.0; // conn.duration_ms.unwrap_or(0.0);
    let conn_year = conn.created_at.year() as u16;
    let conn_month = conn.created_at.month() as u8;
    let conn_day = conn.created_at.day();
    let conn_browser: Option<String> = None; // conn.browser
    let conn_os: Option<String> = None; // conn.os

    sessions += 1;
    total_duration_ms += conn_duration_ms;
    ips.insert(conn.ip);

    macro_rules! add {
      ($acc:ident, $key:expr) => {
        let mut item = $acc.entry($key).or_default();
        item.sessions += 1;
        item.total_duration_ms = conn_duration_ms;
      };
    }

    add!(months_accumulator, (conn_year, conn_month));
    add!(days_accumulator, (conn_year, conn_month, conn_day));
    add!(browser_accumulator, conn_browser);
    add!(os_accumulator, conn_os);
    add!(country_accumulator, conn.country_code);
    add!(station_accumulator, conn.station_id);
  }

  let by_month = months_accumulator
    .into_iter()
    .map(|(key, value)| AnalyticsByMonth {
      year: key.0,
      month: key.1,
      sessions: value.sessions,
      total_duration_ms: value.total_duration_ms,
    })
    .collect::<Vec<_>>();

  let by_day = days_accumulator
    .into_iter()
    .map(|(key, value)| AnalyticsByDay {
      year: key.0,
      month: key.1,
      day: key.2,
      sessions: value.sessions,
      total_duration_ms: value.total_duration_ms,
    })
    .collect::<Vec<_>>();

  let by_browser = browser_accumulator
    .into_iter()
    .map(|(key, value)| AnalyticsByBrowser {
      browser: key,
      sessions: value.sessions,
      total_duration_ms: value.total_duration_ms,
    })
    .collect::<Vec<_>>();

  let by_os = os_accumulator
    .into_iter()
    .map(|(key, value)| AnalyticsByOs {
      os: key,
      sessions: value.sessions,
      total_duration_ms: value.total_duration_ms,
    })
    .collect::<Vec<_>>();

  let by_country = country_accumulator
    .into_iter()
    .map(|(key, value)| AnalyticsByCountry {
      country_code: key,
      sessions: value.sessions,
      total_duration_ms: value.total_duration_ms,
    })
    .collect::<Vec<_>>();

  let by_station = station_accumulator
    .into_iter()
    .map(|(key, value)| AnalyticsByStation {
      station_id: key,
      sessions: value.sessions,
      total_duration_ms: value.total_duration_ms,
    })
    .collect::<Vec<_>>();

  let out = Analytics {
    start_date: query.start_date.into(),
    end_date: query.end_date.into(),
    sessions,
    total_duration_ms,
    ips: ips.len() as u64,
    stations,
    by_browser,
    by_country,
    by_day,
    by_month,
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
  }
}
