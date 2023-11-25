use crate::{http::Request, Model};
use geoip::CountryCode;
use mongodb::bson::doc;
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use std::net::IpAddr;
use ts_rs::TS;

pub mod analytics;
pub mod index;
pub mod lite;
pub mod stats;

crate::register!(StreamConnection);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct StreamConnection {
  #[serde(rename = "_id")]
  pub id: String,
  pub station_id: String,
  pub deployment_id: String,
  #[serde(with = "serde_util::as_f64::option")]
  pub transfer_bytes: Option<u64>,
  #[serde(with = "serde_util::as_f64::option")]
  pub duration_ms: Option<u64>,
  pub is_open: bool,
  pub created_at: DateTime,
  pub country_code: Option<CountryCode>,
  #[serde(with = "serde_util::ip")]
  pub ip: IpAddr,

  pub request: Request,
  pub last_transfer_at: DateTime,
  pub closed_at: Option<DateTime>,
}

impl StreamConnection {
  pub const KEY_MANUALLY_CLOSED: &'static str = "_manually_closed";
}

impl Model for StreamConnection {
  const CL_NAME: &'static str = "stream_connections";
  const UID_LEN: usize = 12;

  fn indexes() -> Vec<IndexModel> {
    let station_id = IndexModel::builder()
      .keys(doc! { Self::KEY_STATION_ID: 1 })
      .build();

    let created_at = IndexModel::builder()
      .keys(doc! { Self::KEY_CREATED_AT: 1 })
      .build();

    let created_at_station_id = IndexModel::builder()
      .keys(doc! { Self::KEY_CREATED_AT: 1, Self::KEY_STATION_ID: 1 })
      .build();

    let is_open = IndexModel::builder()
      .keys(doc! { Self::KEY_IS_OPEN: 1 })
      .build();

    vec![station_id, created_at, created_at_station_id, is_open]
  }
}

// pub mod stats {
//   use std::{
//     collections::{HashMap, HashSet},
//     net::IpAddr,
//   };

//   use super::*;
//   use crate::http;
//   use const_str::concat as str;
//   use futures_util::TryStreamExt;
//   use mongodb::bson::Document;
//   const UNKNOWN: &str = "UNKNOWN";

//   #[derive(Debug, Clone, Serialize, Deserialize)]
//   #[macros::keys]
//   pub struct PartialConnection {
//     #[serde(with = "serde_util::ip")]
//     #[serde(rename = "i")]
//     ip: IpAddr,
//     #[serde(rename = "c")]
//     country_code: Option<String>,
//     #[serde(rename = "d")]
//     created_at: DateTime,
//     #[serde(rename = "s")]
//     state: State,
//   }

//   #[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
//   #[ts(export, export_to = "../../../defs/stream-connection-stats/")]
//   pub struct Stats {
//     pub total: u64,
//     pub sample: u64,
//     pub multiplier: f64,
//     pub sampled: bool,
//     pub time_now: StatsItem,
//     pub time_24h: StatsItem,
//     pub time_7d: StatsItem,
//     pub time_30d: StatsItem,
//   }

//   #[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
//   #[ts(export, export_to = "../../../defs/stream-connection-stats/")]
//   pub struct StatsItem {
//     pub sessions: f64,
//     pub ips: f64,
//     pub country_sessions: BTreeMap<String, f64>,
//     pub country_ips: BTreeMap<String, f64>,
//   }

//   impl Stats {
//     pub fn remove_country_zeros(&mut self) {
//       self.time_now.remove_country_zeros();
//       self.time_24h.remove_country_zeros();
//       self.time_7d.remove_country_zeros();
//       self.time_30d.remove_country_zeros();
//     }

//     pub async fn get_for_filter(filter: Document) -> Result<Stats, mongodb::error::Error> {
//       let job_start = std::time::Instant::now();
//       log::info!("stream connection stats job started");

//       let now = time::OffsetDateTime::now_utc();
//       let start_24h = now - time::Duration::HOUR * 24;
//       let start_7d = now - time::Duration::DAY * 7;
//       let start_30d = now - time::Duration::DAY * 30;

//       let mut sessions_now = 0u64;
//       let mut sessions_7d = 0u64;
//       let mut sessions_24h = 0u64;
//       let mut sessions_30d = 0u64;

//       let mut ips_now = HashSet::<IpAddr>::new();
//       let mut ips_24h = HashSet::<IpAddr>::new();
//       let mut ips_7d = HashSet::<IpAddr>::new();
//       let mut ips_30d = HashSet::<IpAddr>::new();

//       let mut country_sessions_now = HashMap::<String, u64>::new();
//       let mut country_sessions_24h = HashMap::<String, u64>::new();
//       let mut country_sessions_7d = HashMap::<String, u64>::new();
//       let mut country_sessions_30d = HashMap::<String, u64>::new();

//       let mut country_ips_now = HashMap::<String, HashSet<IpAddr>>::new();
//       let mut country_ips_24h = HashMap::<String, HashSet<IpAddr>>::new();
//       let mut country_ips_7d = HashMap::<String, HashSet<IpAddr>>::new();
//       let mut country_ips_30d = HashMap::<String, HashSet<IpAddr>>::new();

//       const KEY_IP: &str = const_str::concat!(
//         StreamConnection::KEY_REQUEST,
//         ".",
//         http::Request::KEY_REAL_IP
//       );

//       const KEY_COUNTRY_CODE: &str = const_str::concat!(
//         StreamConnection::KEY_REQUEST,
//         ".",
//         http::Request::KEY_COUNTRY_CODE
//       );

//       let filter = doc! {
//         "$and": [
//           {
//             StreamConnection::KEY_CREATED_AT: { "$gte": DateTime::from(start_30d) },
//             KEY_IP: { "$ne": null },
//             KEY_COUNTRY_CODE: { "$ne": "" },
//           },
//           filter,
//         ]
//       };

//       let projection = doc! {
//         PartialConnection::KEY_CREATED_AT: str!("$", StreamConnection::KEY_CREATED_AT),
//         PartialConnection::KEY_IP: str!("$", StreamConnection::KEY_REQUEST, ".", http::Request::KEY_REAL_IP),
//         PartialConnection::KEY_COUNTRY_CODE: str!("$", StreamConnection::KEY_REQUEST, ".", http::Request::KEY_COUNTRY_CODE),
//         PartialConnection::KEY_STATE: str!("$", StreamConnection::KEY_STATE),
//       };

//       let fetch_start = std::time::Instant::now();

//       let (total, sample, sampled, multiplier, docs) = {
//         log::info!("stream connection stats fetch started");
//         let total = StreamConnection::cl()
//           .count_documents(filter.clone(), None)
//           .await?;
//         #[warn(clippy::overly_complex_bool_expr)]
//         if total < 20_000 {
//           let options = mongodb::options::FindOptions::builder()
//             .projection(projection)
//             .build();
//           let cursor = StreamConnection::cl_as::<PartialConnection>()
//             .find(filter, options)
//             .await?;
//           let docs: Vec<_> = cursor.try_collect().await?;
//           (total, total, false, 1.0, docs)
//         } else {
//           let rand_limit = 10_000_f64 / total as f64;
//           let pipeline = [
//             doc! { "$match": filter },
//             doc! { "$match": {} },
//             doc! { "$set": { "_r": { "$rand": {} } } },
//             doc! { "$match": { "_r": { "$lte": rand_limit } } },
//             doc! { "$project": projection, },
//           ];
//           let cursor = StreamConnection::cl().aggregate(pipeline, None).await?;
//           let cursor = cursor.with_type::<PartialConnection>();
//           let docs: Vec<_> = cursor.try_collect().await?;
//           let sample = docs.len() as u64;
//           let multiplier = total as f64 / sample as f64;
//           (total, sample, total != sample, multiplier, docs)
//         }
//       };

//       log::info!(
//         "stream connection stats fetch end: {} documents in {}ms",
//         docs.len(),
//         fetch_start.elapsed().as_millis()
//       );

//       let sort_start = std::time::Instant::now();
//       log::info!("stream connection stats sort started");

//       for doc in docs {
//         if matches!(doc.state, State::Open) {
//           sessions_now += 1;
//           ips_now.insert(doc.ip);
//           if let Some(code) = &doc.country_code {
//             *country_sessions_now.entry(code.clone()).or_default() += 1;
//             country_ips_now
//               .entry(code.clone())
//               .or_default()
//               .insert(doc.ip);
//           }
//         };

//         sessions_30d += 1;
//         ips_30d.insert(doc.ip);
//         if let Some(code) = &doc.country_code {
//           *country_sessions_30d.entry(code.clone()).or_default() += 1;
//           country_ips_30d
//             .entry(code.clone())
//             .or_default()
//             .insert(doc.ip);
//         };

//         if doc.created_at.inner() > start_7d {
//           sessions_7d += 1;
//           ips_7d.insert(doc.ip);
//           if let Some(code) = &doc.country_code {
//             *country_sessions_7d.entry(code.clone()).or_default() += 1;
//             country_ips_7d
//               .entry(code.clone())
//               .or_default()
//               .insert(doc.ip);
//           }

//           if doc.created_at.inner() > start_24h {
//             sessions_24h += 1;
//             ips_24h.insert(doc.ip);
//             if let Some(code) = &doc.country_code {
//               *country_sessions_24h.entry(code.clone()).or_default() += 1;
//               country_ips_24h
//                 .entry(code.clone())
//                 .or_default()
//                 .insert(doc.ip);
//             }
//           }
//         };
//       }

//       #[inline]
//       fn item(
//         multiplier: f64,
//         sessions: u64,
//         ips: HashSet<IpAddr>,
//         country_sessions: HashMap<String, u64>,
//         country_ips: HashMap<String, HashSet<IpAddr>>,
//       ) -> StatsItem {
//         let sessions = sessions as f64 * multiplier;
//         let ips = ips.len() as f64 * multiplier;
//         let country_sessions = BTreeMap::from_iter(
//           country_sessions
//             .into_iter()
//             .map(|(k, v)| (k, v as f64 * multiplier)),
//         );
//         let country_ips = BTreeMap::from_iter(
//           country_ips
//             .into_iter()
//             .map(|(k, set)| (k, set.len() as f64 * multiplier)),
//         );

//         StatsItem {
//           sessions,
//           ips,
//           country_sessions,
//           country_ips,
//         }
//       }

//       let stats = Stats {
//         total,
//         sample,
//         multiplier,
//         sampled,
//         time_now: item(
//           multiplier,
//           sessions_now,
//           ips_now,
//           country_sessions_now,
//           country_ips_now,
//         ),
//         time_24h: item(
//           multiplier,
//           sessions_24h,
//           ips_24h,
//           country_sessions_24h,
//           country_ips_24h,
//         ),
//         time_7d: item(
//           multiplier,
//           sessions_7d,
//           ips_7d,
//           country_sessions_7d,
//           country_ips_7d,
//         ),
//         time_30d: item(
//           multiplier,
//           sessions_30d,
//           ips_30d,
//           country_sessions_30d,
//           country_ips_30d,
//         ),
//       };

//       log::info!(
//         "stream connection stats sort end: {}ms",
//         sort_start.elapsed().as_millis()
//       );
//       log::info!(
//         "stream connection stats job end: {}ms",
//         job_start.elapsed().as_millis()
//       );

//       Ok(stats)
//     }

// pub async fn get_for_filter_aggregate(
//   filter: Document,
// ) -> Result<Stats, mongodb::error::Error> {
//   let now = time::OffsetDateTime::now_utc();
//   let start_24h = now - time::Duration::HOUR * 24;
//   let start_7d = now - time::Duration::DAY * 7;
//   let start_30d = now - time::Duration::DAY * 30;

//   let match_stage = doc! {
//     "$match": {
//       "$and": [
//         { StreamConnection::KEY_CREATED_AT: { "$gte": DateTime::from(start_30d) } },
//         filter,
//       ]
//     }
//   };

//   #[inline]
//   fn group_stage_timed_sessions(start: time::OffsetDateTime) -> Document {
//     doc! {
//       "$sum": {
//         "$cond": {
//           "if": { "$gte": [ str!("$", StreamConnection::KEY_CREATED_AT), DateTime::from(start) ] },
//           "then": 1,
//           "else": 0
//         }
//       }
//     }
//   }

//   #[inline]
//   fn group_stage_timed_ips(start: time::OffsetDateTime) -> Document {
//     doc! {
//       "$addToSet": {
//         "$cond": {
//           "if": { "$gte": [ str!("$", StreamConnection::KEY_CREATED_AT), DateTime::from(start) ] },
//           "then": str!("$", StreamConnection::KEY_REQUEST, ".", http::Request::KEY_REAL_IP),
//           "else": "$NONE"
//         }
//       }
//     }
//   }

//   let country_group_stage = doc! {
//     "$group": {
//       "_id": { "$ifNull": [ str!("$", StreamConnection::KEY_REQUEST, ".", http::Request::KEY_COUNTRY_CODE), UNKNOWN ] },
//       "sessions_now": {
//         "$sum": {
//           "$cond": {
//             "if": { "$eq": [ str!("$", StreamConnection::KEY_STATE), State::KEY_ENUM_VARIANT_OPEN ] },
//             "then": 1,
//             "else": 0
//           }
//         }
//       },
//       "ips_now": {
//         "$addToSet": {
//           "$cond": {
//             "if": { "$eq": [ str!("$", StreamConnection::KEY_STATE), State::KEY_ENUM_VARIANT_OPEN ] },
//             "then": str!("$", StreamConnection::KEY_REQUEST, ".", http::Request::KEY_REAL_IP),
//             "else": "$NONE"
//           }
//         }
//       },
//       "sessions_24h": group_stage_timed_sessions(start_24h),
//       "sessions_7d": group_stage_timed_sessions(start_7d),
//       "sessions_30d": group_stage_timed_sessions(start_30d),
//       "ips_24h": group_stage_timed_ips(start_24h),
//       "ips_7d": group_stage_timed_ips(start_7d),
//       "ips_30d": group_stage_timed_ips(start_30d),
//     }
//   };

//   macro_rules! country_sessions_sum {
//     ($key:expr) => {
//       doc! {
//         "$push": {
//           "k": "$_id",
//           "v": str!("$sessions_", $key)
//         }
//       }
//     };
//   }

//   macro_rules! country_ips_sum {
//     ($key:expr) => {
//       doc! {
//         "$push": {
//           "k": "$_id",
//           "v": { "$size": str!("$ips_", $key) }
//         }
//       }
//     };
//   }

//   let global_group_stage = doc! {
//     "$group": {
//       "_id": null,
//       "sessions_now": { "$sum": "$sessions_now" },
//       "sessions_24h": { "$sum": "$sessions_24d" },
//       "sessions_7d": { "$sum": "$sessions_7d" },
//       "sessions_30d": { "$sum": "$sessions_30d" },
//       "ips_now": { "$sum": { "$size": "$ips_now" } },
//       "ips_24h": { "$sum": { "$size": "$ips_24h" } },
//       "ips_7d": { "$sum": { "$size": "$ips_7d" } },
//       "ips_30d": { "$sum": { "$size": "$ips_30d" } },
//       "country_sessions_now": country_sessions_sum!("now"),
//       "country_sessions_24h": country_sessions_sum!("24h"),
//       "country_sessions_7d": country_sessions_sum!("7d"),
//       "country_sessions_30d": country_sessions_sum!("30d"),
//       "country_ips_now": country_ips_sum!("now"),
//       "country_ips_24h": country_ips_sum!("24h"),
//       "country_ips_7d": country_ips_sum!("7d"),
//       "country_ips_30d": country_ips_sum!("30d"),
//     }
//   };

//   macro_rules! timed_item_projection {
//     ($key:expr) => {
//       doc! {
//         "sessions": str!("$sessions_", $key),
//         "ips": str!("$ips_", $key),
//         "country_sessions": { "$arrayToObject": str!("$country_sessions_", $key) },
//         "country_ips": { "$arrayToObject": str!("$country_ips_", $key) },
//       }
//     };
//   }

//   let projection_stage = doc! {
//     "$project": {
//       "_id": 0,
//       "time_now": timed_item_projection!("now"),
//       "time_24h": timed_item_projection!("24h"),
//       "time_7d": timed_item_projection!("7d"),
//       "time_30d": timed_item_projection!("30d"),
//     }
//   };

//   let cursor = StreamConnection::cl()
//     .aggregate(
//       [
//         match_stage,
//         country_group_stage,
//         global_group_stage,
//         projection_stage,
//       ],
//       None,
//     )
//     .await?;

//   let mut cursor = cursor.with_type::<Stats>();

//   let mut stats = cursor.try_next().await?.unwrap_or_default();

//   stats.remove_country_zeros();

//   Ok(stats)
// }
// }

//   impl StatsItem {
//     pub fn remove_country_zeros(&mut self) {
//       self.country_sessions.retain(|_, v| *v != 0.0);
//       self.country_ips.retain(|_, v| *v != 0.0);
//     }

//     pub async fn get_for_filter(filter: Document) -> Result<StatsItem, mongodb::error::Error> {
//       let match_stage = doc! {
//         "$match": filter,
//       };

//       let country_group_stage = doc! {
//         "$group": {
//           "_id": {
//             "$ifNull": [ str!("$", StreamConnection::KEY_REQUEST, ".", http::Request::KEY_COUNTRY_CODE), UNKNOWN ]
//           },
//           "sessions": {
//             "$sum": 1
//           },
//           "ips": {
//             "$addToSet": str!("$", StreamConnection::KEY_REQUEST, ".", http::Request::KEY_REAL_IP),
//           }
//         }
//       };

//       let global_group_stage = doc! {
//         "$group": {
//           "_id": null,
//           "sessions": { "$sum": "$sessions" },
//           "ips": { "$sum": { "$size": "$ips" } },
//           "country_sessions": {
//             "$push": {
//               "k": "$_id",
//               "v": "$sessions",
//             }
//           },
//           "country_ips": {
//             "$push": {
//               "k": "$_id",
//               "v": { "$size": "$ips" },
//             }
//           }
//         }
//       };

//       let projection_stage = doc! {
//         "$project": {
//           "_id": null,
//           "sessions": 1,
//           "ips": 1,
//           "country_sessions": { "$arrayToObject": "$country_sessions" },
//           "country_ips": { "$arrayToObject": "$country_ips" },
//         }
//       };

//       let cursor = StreamConnection::cl()
//         .aggregate(
//           [
//             match_stage,
//             country_group_stage,
//             global_group_stage,
//             projection_stage,
//           ],
//           None,
//         )
//         .await?;

//       let mut cursor = cursor.with_type::<StatsItem>();

//       let mut item = cursor.try_next().await?.unwrap_or_default();

//       item.remove_country_zeros();

//       Ok(item)
//     }
//   }
// }

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, StreamConnection::KEY_ID);
  }
}
