use std::{
  collections::{hash_map::Entry, HashMap, HashSet},
  hash::Hash,
  net::IpAddr,
  time::Instant,
};

use futures_util::{StreamExt, TryStreamExt};
use geoip::CountryCode;
use mongodb::bson::doc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_util::{timezone_datetime::TimezoneDateTime, DateTime};
use time::{OffsetDateTime, UtcOffset};
use ts_rs::TS;

use crate::{station::Station, ws_stats_connection::WsStatsConnection, Model};

#[derive(Debug, Serialize, Deserialize)]
#[macros::keys]
pub struct Item {
  // #[serde(rename = "_id")]
  // pub id: String,
  #[serde(rename = "st")]
  pub station_id: String,

  // #[serde(rename = "dp")]
  // pub deployment_id: String,

  // #[serde(with = "serde_util::as_f64::option")]
  // pub transfer_bytes: Option<u64>,
  #[serde(rename = "du")]
  #[serde(with = "serde_util::as_f64::option")]
  pub duration_ms: Option<u64>,

  // #[serde(rename = "op")]
  // pub is_open: bool,
  #[serde(rename = "cc")]
  pub country_code: Option<CountryCode>,

  #[serde(rename = "ip")]
  #[serde(with = "serde_util::ip")]
  pub ip: IpAddr,

  #[serde(rename = "ap")]
  pub app_kind: Option<String>,

  #[serde(rename = "av")]
  #[serde(with = "serde_util::as_f64::option")]
  pub app_version: Option<u32>,

  #[serde(rename = "us")]
  pub user_id: Option<String>,

  // #[serde(rename = "re")]
  // #[serde(with = "serde_util::as_f64")]
  // pub reconnections: u16,
  #[serde(rename = "ca")]
  pub created_at: DateTime,
  // pub request: Request,
  // pub last_transfer_at: DateTime,

  // #[serde(rename = "cl")]
  // pub closed_at: Option<DateTime>,
}

impl Item {
  pub fn projection() -> mongodb::bson::Document {
    doc! {
      crate::KEY_ID: 0,
      WsStatsConnection::KEY_STATION_ID: 1,
      WsStatsConnection::KEY_DURATION_MS: 1,
      WsStatsConnection::KEY_COUNTRY_CODE: 1,
      WsStatsConnection::KEY_IP: 1,
      WsStatsConnection::KEY_APP_KIND: 1,
      WsStatsConnection::KEY_APP_VERSION: 1,
      WsStatsConnection::KEY_USER_ID: 1,
      WsStatsConnection::KEY_CREATED_AT: 1,
    }
  }
}

#[cfg(test)]
#[test]
fn ws_stat_item_keys() {
  assert_eq!(Item::KEY_STATION_ID, WsStatsConnection::KEY_STATION_ID);
  assert_eq!(Item::KEY_DURATION_MS, WsStatsConnection::KEY_DURATION_MS);
  assert_eq!(Item::KEY_COUNTRY_CODE, WsStatsConnection::KEY_COUNTRY_CODE);
  assert_eq!(Item::KEY_IP, WsStatsConnection::KEY_IP);
  assert_eq!(Item::KEY_APP_KIND, WsStatsConnection::KEY_APP_KIND);
  assert_eq!(Item::KEY_APP_VERSION, WsStatsConnection::KEY_APP_VERSION);
  assert_eq!(Item::KEY_USER_ID, WsStatsConnection::KEY_USER_ID);
  assert_eq!(Item::KEY_CREATED_AT, WsStatsConnection::KEY_CREATED_AT);
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
#[ts(export, export_to = "../../../defs/app-analytics/")]
pub struct Analytics {
  pub is_now: bool,

  pub kind: AnalyticsQueryKind,

  pub stations: Vec<AnalyticsStation>,

  pub since: TimezoneDateTime,

  pub until: TimezoneDateTime,

  pub utc_offset_minutes: i16,

  #[serde(serialize_with = "serde_util::as_f64::serialize")]
  #[serde(deserialize_with = "serde_util::as_f64::deserialize")]
  pub sessions: u64,

  #[serde(serialize_with = "serde_util::as_f64::serialize")]
  #[serde(deserialize_with = "serde_util::as_f64::deserialize")]
  pub ips: u64,

  #[serde(serialize_with = "serde_util::as_f64::serialize")]
  #[serde(deserialize_with = "serde_util::as_f64::deserialize")]
  pub users: u64,

  #[serde(serialize_with = "serde_util::as_f64::serialize")]
  #[serde(deserialize_with = "serde_util::as_f64::deserialize")]
  pub total_duration_ms: u64,

  // #[serde(with = "serde_util::as_f64")]
  // pub total_transfer_bytes: u64,
  #[cfg(feature = "analytics-max-concurrent")]
  #[serde(serialize_with = "serde_util::as_f64::serialize")]
  #[serde(deserialize_with = "serde_util::as_f64::deserialize")]
  #[ts(optional)]
  pub max_concurrent_listeners: u64,

  #[cfg(feature = "analytics-max-concurrent")]
  #[ts(optional)]
  pub max_concurrent_listeners_date: Option<serde_util::DateTime>,

  pub by_day: Vec<AnalyticsItem<YearMonthDay>>,
  pub by_hour: Option<Vec<AnalyticsItem<YearMonthDayHour>>>,
  pub by_country: Vec<AnalyticsItem<Option<CountryCode>>>,
  pub by_station: Vec<AnalyticsItem<String>>,
  // pub by_browser: Vec<AnalyticsItem<Option<String>>>,
  // pub by_domain: Vec<AnalyticsItem<Option<String>>>,
  // pub by_os: Vec<AnalyticsItem<Option<String>>>,
  pub by_app_kind: Vec<AnalyticsItem<Option<String>>>,
  pub by_app_version: Vec<AnalyticsItem<AppKindVersion>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
#[ts(export, export_to = "../../../defs/app-analytics/")]
pub struct AnalyticsItem<K> {
  pub key: K,

  #[serde(serialize_with = "serde_util::as_f64::serialize")]
  #[serde(deserialize_with = "serde_util::as_f64::deserialize")]
  pub sessions: u64,

  #[serde(serialize_with = "serde_util::as_f64::serialize")]
  #[serde(deserialize_with = "serde_util::as_f64::deserialize")]
  pub ips: u64,

  #[serde(serialize_with = "serde_util::as_f64::serialize")]
  #[serde(deserialize_with = "serde_util::as_f64::deserialize")]
  pub users: u64,

  #[serde(serialize_with = "serde_util::as_f64::serialize")]
  #[serde(deserialize_with = "serde_util::as_f64::deserialize")]
  pub total_duration_ms: u64,
  #[serde(serialize_with = "serde_util::as_f64::serialize")]
  #[serde(deserialize_with = "serde_util::as_f64::deserialize")]
  pub total_transfer_bytes: u64,

  #[cfg(feature = "analytics-max-concurrent")]
  #[serde(serialize_with = "serde_util::as_f64::serialize")]
  #[serde(deserialize_with = "serde_util::as_f64::deserialize")]
  #[ts(optional)]
  pub max_concurrent_listeners: u64,

  #[cfg(feature = "analytics-max-concurrent")]
  #[ts(optional)]
  pub max_concurrent_listeners_date: Option<serde_util::DateTime>,
}

// #[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq, Hash, Ord, PartialOrd, Deserialize, TS)]
// #[ts(export, export_to = "../../../defs/app-analytics/")]
// pub struct YearMonth {
//   pub year: u16,
//   pub month: u8,
// }

#[derive(
  Debug, Clone, Copy, Serialize, Eq, PartialEq, Hash, Ord, PartialOrd, Deserialize, TS, JsonSchema,
)]
#[ts(export, export_to = "../../../defs/app-analytics/")]
pub struct YearMonthDay {
  pub year: u16,
  pub month: u8,
  pub day: u8,
}

#[derive(
  Debug, Clone, Copy, Serialize, Eq, PartialEq, Hash, Ord, PartialOrd, Deserialize, TS, JsonSchema,
)]
#[ts(export, export_to = "../../../defs/app-analytics/")]
pub struct YearMonthDayHour {
  pub year: u16,
  pub month: u8,
  pub day: u8,
  pub hour: u8,
}

#[derive(
  Debug, Clone, Serialize, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize, TS, JsonSchema,
)]
#[ts(export, export_to = "../../../defs/app-analytics/")]
pub struct AppKindVersion {
  pub kind: Option<String>,
  pub version: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
#[ts(export, export_to = "../../../defs/app-analytics/")]
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
  // pub browser: Option<Option<String>>,
  // pub os: Option<Option<String>>,
  // pub domain: Option<Option<String>>,
  pub app_kind: Option<Option<String>>,
  pub app_version: Option<Option<u32>>,
  pub min_duration_ms: Option<u64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, TS, JsonSchema)]
#[ts(export, export_to = "../../../defs/app-analytics/")]
pub enum AnalyticsQueryKind {
  #[serde(rename = "now")]
  Now { offset_date: TimezoneDateTime },

  #[serde(rename = "time_range")]
  TimeRange {
    since: TimezoneDateTime,
    until: TimezoneDateTime,
  },
}

type KeyedAccumulatorMap<K> = HashMap<K, AccumulatorItem>;

#[derive(Debug, Default)]
struct AccumulatorItem {
  sessions: u64,
  ips: HashSet<IpAddr>,
  users: HashSet<String>,
  total_duration_ms: u64,
  total_transfer_bytes: u64,
  #[cfg(feature = "analytics-max-concurrent")]
  start_stop_events: Vec<StartStopEvent>,
}

impl AccumulatorItem {
  #[inline(always)]
  #[allow(unused)]
  pub fn new() -> Self {
    Self::default()
  }

  #[inline(always)]
  fn merge(&mut self, dst: Self) {
    self.sessions += dst.sessions;
    self.ips.extend(dst.ips);
    self.users.extend(dst.users);
    self.total_duration_ms += dst.total_duration_ms;
    self.total_transfer_bytes += dst.total_transfer_bytes;

    #[cfg(feature = "analytics-max-concurrent")]
    self.start_stop_events.extend(dst.start_stop_events);
  }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct StartStopEvent(u32);

impl StartStopEvent {
  const START: u32 = 0b_1111_1111_1111_1111_1111_1111_1111_1110;
  const STOP: u32 = 0b_0000_0000_0000_0000_0000_0000_0000_0001;

  #[inline(always)]
  pub fn new(timestamp: u32, start_stop: bool) -> Self {
    let v = if start_stop {
      timestamp & Self::START
    } else {
      timestamp | Self::STOP
    };

    Self(v)
  }

  #[inline(always)]
  pub fn is_start(self) -> bool {
    self.0 & 1 == 0
  }
}

#[inline(always)]
fn merge_accumulator_maps<K: Eq + Hash + Clone>(
  src: &mut KeyedAccumulatorMap<K>,
  dst: KeyedAccumulatorMap<K>,
) {
  for (key, value) in dst.into_iter() {
    match src.entry(key) {
      Entry::Vacant(entry) => {
        entry.insert(value);
      }
      Entry::Occupied(mut entry) => {
        entry.get_mut().merge(value);
      }
    }
  }
}

#[derive(Debug)]
struct Batch {
  pub offset: UtcOffset,
  pub now_ms: u64,

  pub sessions: u64,
  pub ips: HashSet<IpAddr>,
  pub users: HashSet<String>,

  pub total_duration_ms: u64,
  pub total_transfer_bytes: u64,

  pub by_day: KeyedAccumulatorMap<YearMonthDay>,
  pub by_hour: KeyedAccumulatorMap<YearMonthDayHour>,
  pub by_app_kind: KeyedAccumulatorMap<Option<String>>,
  pub by_app_version: KeyedAccumulatorMap<AppKindVersion>,

  pub by_country: KeyedAccumulatorMap<Option<CountryCode>>,
  pub by_station: KeyedAccumulatorMap<String>,
  #[cfg(feature = "analytics-max-concurrent")]
  pub start_stop_events: Vec<StartStopEvent>,
}

impl Batch {
  #[inline(always)]
  pub fn new(offset: UtcOffset) -> Self {
    Self {
      offset,
      now_ms: OffsetDateTime::now_utc().unix_timestamp() as u64 * 1000,

      sessions: 0,
      ips: Default::default(),
      users: Default::default(),

      total_duration_ms: 0,
      total_transfer_bytes: 0,

      by_day: Default::default(),
      by_hour: Default::default(),
      by_country: Default::default(),
      by_station: Default::default(),
      // by_browser: Default::default(),
      // by_os: Default::default(),
      // by_domain: Default::default(),
      by_app_kind: Default::default(),
      by_app_version: Default::default(),
      #[cfg(feature = "analytics-max-concurrent")]
      start_stop_events: Default::default(),
    }
  }

  #[inline(always)]
  pub fn add(&mut self, conn: Item) {
    let created_at = conn.created_at.to_offset(self.offset);

    let conn_duration_ms = conn
      .duration_ms
      .unwrap_or_else(|| (self.now_ms - created_at.unix_timestamp() as u64 * 1000));

    let conn_year = created_at.year() as u16;
    let conn_month = created_at.month() as u8;
    let conn_day = created_at.day();
    let conn_hour = created_at.hour();

    let conn_kind_version = AppKindVersion {
      kind: conn.app_kind.clone(),
      version: conn.app_version,
    };

    self.sessions += 1;
    self.total_duration_ms += conn_duration_ms;
    self.ips.insert(conn.ip);

    if let Some(id) = conn.user_id {
      self.users.insert(id);
    }

    #[cfg(feature = "analytics-max-concurrent")]
    let start_s = conn.created_at.unix_timestamp() as u32;

    #[cfg(feature = "analytics-max-concurrent")]
    let start = StartStopEvent::new(start_s, true);
    #[cfg(feature = "analytics-max-concurrent")]
    self.start_stop_events.push(start);
    #[cfg(feature = "analytics-max-concurrent")]
    let stop = StartStopEvent::new(start_s + (conn_duration_ms / 1000) as u32, false);

    if conn.duration_ms.is_some() {
      #[cfg(feature = "analytics-max-concurrent")]
      self.start_stop_events.push(stop);
    }

    macro_rules! add {
      ($acc:expr, $key:expr) => {
        let item = $acc.entry($key).or_default();
        item.sessions += 1;
        item.ips.insert(conn.ip);
        item.total_duration_ms += conn_duration_ms;
        // item.total_transfer_bytes += conn_transfer_bytes;

        #[cfg(feature = "analytics-max-concurrent")]
        item.start_stop_events.push(start);

        #[cfg(feature = "analytics-max-concurrent")]
        if conn.duration_ms.is_some() {
          item.start_stop_events.push(stop);
        }
      };
    }

    add!(
      self.by_day,
      YearMonthDay {
        year: conn_year,
        month: conn_month,
        day: conn_day
      }
    );

    add!(
      self.by_hour,
      YearMonthDayHour {
        year: conn_year,
        month: conn_month,
        day: conn_day,
        hour: conn_hour,
      }
    );

    add!(self.by_country, conn.country_code);
    add!(self.by_station, conn.station_id);
    // add!(self.by_browser, conn_browser);
    // add!(self.by_os, conn_os);
    // add!(self.by_domain, conn.domain);
    add!(self.by_app_kind, conn.app_kind);
    add!(self.by_app_version, conn_kind_version);
  }

  #[inline(always)]
  pub fn merge(&mut self, dst: Self) {
    self.sessions += dst.sessions;
    self.total_duration_ms += dst.total_duration_ms;
    self.total_transfer_bytes += dst.total_transfer_bytes;
    self.ips.extend(dst.ips);
    merge_accumulator_maps(&mut self.by_day, dst.by_day);
    merge_accumulator_maps(&mut self.by_hour, dst.by_hour);
    merge_accumulator_maps(&mut self.by_country, dst.by_country);
    merge_accumulator_maps(&mut self.by_station, dst.by_station);
    // merge_accumulator_maps(&mut self.by_browser, dst.by_browser);
    // merge_accumulator_maps(&mut self.by_os, dst.by_os);
    // merge_accumulator_maps(&mut self.by_domain, dst.by_domain);
    merge_accumulator_maps(&mut self.by_app_kind, dst.by_app_kind);
    merge_accumulator_maps(&mut self.by_app_version, dst.by_app_version);

    #[cfg(feature = "analytics-max-concurrent")]
    self.start_stop_events.extend(dst.start_stop_events);
  }
}

pub async fn get_analytics(query: AnalyticsQuery) -> Result<Analytics, mongodb::error::Error> {
  tokio::task::spawn_blocking(move || {
    tokio::runtime::Handle::current().block_on(async move {
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

      let mut and = vec![doc! {
        WsStatsConnection::KEY_STATION_ID: {
          "$in": &query.station_ids,
        }
      }];

      let offset_date: OffsetDateTime;
      let kind = query.kind;
      let is_now: bool;
      let with_hours: bool;

      let mut start_end_date: Option<(OffsetDateTime, OffsetDateTime)> = None;

      match kind {
        AnalyticsQueryKind::Now { offset_date: d } => {
          is_now = true;
          with_hours = false;
          offset_date = OffsetDateTime::now_utc().to_offset(d.offset());

          and.push(doc! { WsStatsConnection::KEY_IS_OPEN: true });
        }

        AnalyticsQueryKind::TimeRange { since, until } => {
          is_now = false;
          let mut start_date = *since;
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

          with_hours =
            (end_date.unix_timestamp() - start_date.unix_timestamp()) < (60 * 60 * 24 * 32);

          // let ser_start_date: serde_util::DateTime = start_date.into();
          // let ser_end_date: serde_util::DateTime = end_date.into();

          // and.push(doc! {
          //   WsStatsConnection::KEY_CREATED_AT: {
          //     "$gte": ser_start_date,
          //     "$lt": ser_end_date,
          //   }
          // });

          start_end_date = Some((start_date, end_date));

          offset_date = start_date;
        }
      }

      if let Some(cc) = query.country_code {
        and.push(doc! {
          // this convertion should never fail
          WsStatsConnection::KEY_COUNTRY_CODE: mongodb::bson::to_bson(&cc).unwrap(),
        });
      }

      // if let Some(os) = query.os {
      //   and.push(doc! { WsStatsConnection::KEY_OS: os });
      // }

      // if let Some(browser) = query.browser {
      //   and.push(doc! { WsStatsConnection::KEY_BROWSER: browser });
      // }

      // if let Some(domain) = query.domain {
      //   and.push(doc! { WsStatsConnection::KEY_DOMAIN: domain });
      // }

      if let Some(app_kind) = query.app_kind {
        and.push(doc! { WsStatsConnection::KEY_APP_KIND: app_kind });
      }

      if let Some(app_version) = query.app_version {
        and.push(doc! { WsStatsConnection::KEY_APP_VERSION: app_version });
      }

      if let Some(d) = query.min_duration_ms {
        and.push(doc! {
          "$or": [
            { WsStatsConnection::KEY_DURATION_MS: null },
            { WsStatsConnection::KEY_DURATION_MS: { "$gte": d as f64 } },
          ]
        });
      }

      // let (count, count_ms) = {
      //   let start = Instant::now();
      //   let filter = doc! { "$and": &and };
      //   let count = WsStatsConnection::cl()
      //     .count_documents(filter, None)
      //     .await?;

      //   let ms = start.elapsed().as_millis();
      //   (count, ms)
      // };

      // let filter = doc!{ "$and": and };
      // let mut cursor = WsStatsConnection::cl().find(filter, options).await?;

      let mut first_last_and = and.clone();
      if let Some((start_date, end_date)) = start_end_date {
        let start_date: DateTime = start_date.into();
        let end_date: DateTime = end_date.into();
        first_last_and.push(
          doc! { WsStatsConnection::KEY_CREATED_AT: { "$gte": start_date, "$lt": end_date } },
        );
      }

      let get_bound = |direction: i32| {
        let sort = doc! { WsStatsConnection::KEY_CREATED_AT: direction };
        async {
          let filter = doc! { "$and": &first_last_and };
          let options = mongodb::options::FindOneOptions::builder()
            .sort(sort)
            .build();
          let item_option = WsStatsConnection::cl().find_one(filter, options).await?;
          Ok::<_, mongodb::error::Error>(item_option)
        }
      };

      let (first, last) = match tokio::try_join!(get_bound(1), get_bound(-1))? {
        (Some(first), Some(last)) => (first, last),
        _ => {
          return Ok(Analytics {
            is_now,
            kind,
            stations,
            since: offset_date.into(),
            until: offset_date.into(),
            utc_offset_minutes: offset_date.offset().whole_minutes(),
            sessions: 0,
            ips: 0,
            users: 0,
            total_duration_ms: 0,
            // total_transfer_bytes: 0,
            #[cfg(feature = "analytics-max-concurrent")]
            max_concurrent_listeners: 0,
            #[cfg(feature = "analytics-max-concurrent")]
            max_concurrent_listeners_date: None,
            by_day: vec![],
            by_hour: None,
            by_country: vec![],
            by_station: vec![],
            // by_browser: vec![],
            // by_os: vec![],
            // by_domain: vec![],
            by_app_kind: vec![],
            by_app_version: vec![],
          });
        }
      };

      let first_ms = first.created_at.unix_timestamp_nanos() as u64 / 1_000_000;
      let last_ms = last.created_at.unix_timestamp_nanos() as u64 / 1_000_000;

      let accumulate_start = Instant::now();

      let batches_n = if is_now { 1 } else { 16 };
      let step = (last_ms - first_ms) / batches_n as u64;
      let batch = {
        futures_util::stream::repeat(())
          .take(batches_n)
          .enumerate()
          .map(|(i, ())| {
            let mut and = and.clone();
            async move {
              tokio::task::spawn_blocking(move || {
                tokio::runtime::Handle::current().block_on(async move {
                  if !is_now {
                    // let step = time::Duration::milliseconds(step as i64);
                    // let start: DateTime = (*first_ts + step * i as u16).into();
                    // let end: DateTime = (*start + step).into();
                    let start_ms = first_ms + step * i as u64;
                    let end_ms = first_ms + step * (i + 1) as u64;

                    let start = mongodb::bson::DateTime::from_millis(start_ms as i64);
                    let mut end = mongodb::bson::DateTime::from_millis(end_ms as i64);

                    let is_last = (i + 1) == batches_n;

                    let mut lt = "$lt";
                    if is_last {
                      lt = "$lte";
                      end = mongodb::bson::DateTime::from_millis(last_ms as i64);
                    }

                    and
                      .push(doc! { WsStatsConnection::KEY_CREATED_AT: { "$gte": start, lt: end } });
                  }

                  let sort = doc! { WsStatsConnection::KEY_CREATED_AT: 1 };

                  // let options = mongodb::options::AggregateOptions::builder().build();

                  // let pipeline = vec![
                  //   doc! { "$match": { "$and": and } },
                  //   //doc! { "$sample": { "size": 100_000 } },
                  //   doc! { "$sort": sort },
                  // ];

                  // let mut cursor = WsStatsConnection::cl()
                  // .find(pipeline, options)
                  // .await?
                  // .with_type::<StreamConnectionLite>();

                  let options = mongodb::options::FindOptions::builder()
                    .sort(sort)
                    .projection(Item::projection())
                    .build();

                  let filter = doc! { "$and": and };
                  let mut cursor = WsStatsConnection::cl_as::<Item>()
                    .find(filter, options)
                    .await?;

                  let mut batch = Batch::new(offset_date.offset());

                  // accumulate
                  #[cfg(feature = "test-analytics-base-measure")]
                  while let Some(_conn) = cursor.try_next().await? {
                    batch.sessions += 1;
                  }

                  #[cfg(not(feature = "test-analytics-base-measure"))]
                  while let Some(conn) = cursor.try_next().await? {
                    batch.add(conn);
                  }

                  Ok::<_, mongodb::error::Error>(batch)
                })
              })
              .await
              .unwrap()
            }
          })
          .buffer_unordered(batches_n)
          .try_fold(Batch::new(offset_date.offset()), |src, mut dst| async {
            dst.merge(src);
            Ok(dst)
          })
          .await?
      };

      let accumulate_ms = accumulate_start.elapsed().as_millis();
      let sort_start = Instant::now();

      #[cfg(feature = "analytics-max-concurrent")]
      macro_rules! max_concurrent {
        ($vec:expr) => {{
          use rayon::prelude::*;

          let mut vec = $vec;
          //vec.sort_unstable_by(|a, b| a.0.cmp(&b.0));
          //vec.par_sort_unstable_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));
          vec.par_sort_unstable();

          let mut max: u32 = 0;
          let mut max_timestamp: u32 = 0;
          let mut current: u32 = 0;
          for event in vec.into_iter() {
            if event.is_start() {
              current = current.saturating_add(1);
              if current > max {
                max = current;
                max_timestamp = event.0
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
        ($acc:expr) => {
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
                users: value.users.len() as u64,
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
      // let mut by_month = collect!(months_accumulator);
      let mut by_day = collect!(batch.by_day);
      let mut by_hour = collect!(batch.by_hour);
      let mut by_country = collect!(batch.by_country);
      let mut by_station = collect!(batch.by_station);
      // let mut by_browser = collect!(batch.by_browser);
      // let mut by_os = collect!(batch.by_os);
      // let mut by_domain = collect!(batch.by_domain);
      let mut by_app_kind = collect!(batch.by_app_kind);
      let mut by_app_version = collect!(batch.by_app_version);

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

      // sort_by_key!(by_month);
      sort_by_key!(by_day);
      sort_by_key!(by_hour);

      sort_by_sessions!(by_country);
      sort_by_sessions!(by_station);
      // sort_by_sessions!(by_browser);
      // sort_by_sessions!(by_os);
      // sort_by_sessions!(by_domain);
      sort_by_sessions!(by_app_kind);
      sort_by_sessions!(by_app_version);

      #[cfg(feature = "analytics-max-concurrent")]
      let (max_concurrent_listeners, max_concurrent_listeners_date) =
        max_concurrent!(batch.start_stop_events);

      let sort_ms = sort_start.elapsed().as_millis();

      let since = first.created_at.to_offset(offset_date.offset());
      let until = last.created_at.to_offset(offset_date.offset());

      log::info!(
        target: "analytics",
        "got analytics, processed {} connections in {}ms => {}ms acculumate, {}ms sort",
        batch.sessions,
        start.elapsed().as_millis(),
        accumulate_ms,
        sort_ms,
      );

      let by_hour = if with_hours { Some(by_hour) } else { None };

      // render
      let out = Analytics {
        is_now,
        kind,
        since: since.into(),
        until: until.into(),
        utc_offset_minutes: offset_date.offset().whole_minutes(),
        sessions: batch.sessions,
        total_duration_ms: batch.total_duration_ms,
        // total_transfer_bytes: batch.total_transfer_bytes,
        ips: batch.ips.len() as u64,
        users: batch.users.len() as u64,
        #[cfg(feature = "analytics-max-concurrent")]
        max_concurrent_listeners,
        #[cfg(feature = "analytics-max-concurrent")]
        max_concurrent_listeners_date,
        stations,
        by_day,
        by_hour,
        by_country,
        by_station,
        // by_browser,
        // by_os,
        // by_domain,
        by_app_kind,
        by_app_version,
      };

      Ok(out)
    })
  })
  .await
  .unwrap()
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
