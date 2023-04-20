use std::collections::BTreeMap;

use crate::{http::Request, Model};
use mongodb::{
  bson::{self, doc, Bson},
  IndexModel,
};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct StreamConnection {
  #[serde(rename = "_id")]
  pub id: String,
  pub station_id: String,
  pub request: Request,

  #[serde(with = "serde_util::as_f64")]
  pub transfer_bytes: u64,

  #[serde(with = "serde_util::as_f64::option")]
  pub duration_ms: Option<u64>,

  pub state: State,
  pub created_at: DateTime,
  pub last_transfer_at: DateTime,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StreamConnectionMongoSet {
  #[serde(with = "serde_util::as_f64::option")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub transfer_bytes: Option<u64>,

  pub last_transfer_at: DateTime,

  #[serde(with = "serde_util::as_f64::option")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub duration_ms: Option<u64>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub state: Option<State>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, TS)]
#[ts(
  export,
  export_to = "../../../defs/db/",
  rename = "StreamConnectionState"
)]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub enum State {
  Open,
  Closed,
}

impl From<State> for Bson {
  fn from(state: State) -> Self {
    bson::to_bson(&state).unwrap()
  }
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

    let station_id_created_at = IndexModel::builder()
      .keys(doc! { Self::KEY_STATION_ID: 1, Self::KEY_CREATED_AT: 1 })
      .build();

    vec![station_id, created_at, station_id_created_at]
  }
}

impl StreamConnection {
  pub async fn set_transfer_bytes(
    id: &str,
    transfer_bytes: u64,
  ) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
    let set = StreamConnectionMongoSet {
      last_transfer_at: DateTime::now(),
      transfer_bytes: Some(transfer_bytes),
      duration_ms: None,
      state: None,
    };

    let update = doc! {
      "$set": bson::to_document(&set).unwrap()
    };

    Self::update_by_id(id, update).await
  }

  pub async fn set_closed(
    id: &str,
    duration_ms: u64,
    transfer_bytes: u64,
  ) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
    let set = StreamConnectionMongoSet {
      transfer_bytes: Some(transfer_bytes),
      duration_ms: Some(duration_ms),
      last_transfer_at: DateTime::now(),
      state: Some(State::Closed),
    };

    let update = doc! {
      "$set": bson::to_document(&set).unwrap(),
    };

    Self::update_by_id(id, update).await
  }

  pub async fn count_for_station_in_last(
    station_id: &str,
    in_last: time::Duration,
  ) -> Result<u64, mongodb::error::Error> {
    let since: DateTime = (time::OffsetDateTime::now_utc() - in_last).into();
    let filter = doc! {
      Self::KEY_STATION_ID: station_id,
      Self::KEY_CREATED_AT: { "$gte": since },
      Self::KEY_TRANSFER_BYTES: { "$ne": 0 },
    };
    let count = Self::cl().count_documents(filter, None).await?;
    Ok(count)
  }

  pub async fn count_unique_ips_for_station_in_last(
    station_id: &str,
    in_last: time::Duration,
  ) -> Result<u64, mongodb::error::Error> {
    use futures_util::TryStreamExt;

    let since: DateTime = (time::OffsetDateTime::now_utc() - in_last).into();
    let filter = doc! {
      Self::KEY_STATION_ID: station_id,
      Self::KEY_CREATED_AT: { "$gte": since },
      Self::KEY_TRANSFER_BYTES: { "$ne": 0 },
    };

    let pipeline = vec![
      doc! { "$match": filter },
      doc! { "$group": { "_id": format!("${}.{}", Self::KEY_REQUEST, Request::KEY_REAL_IP) } },
      doc! { "$group": { "_id": "_id", "count": { "$sum": 1 } } },
      doc! { "$project": { "_id": 0, "count": 1 } },
    ];

    #[derive(Debug, Serialize, Deserialize)]
    struct CountDocument {
      count: f64,
    }

    let count = match Self::cl()
      .aggregate(pipeline, None)
      .await?
      .try_next()
      .await?
    {
      None => 0.0,
      Some(doc) => {
        let CountDocument { count } =
          bson::from_document(doc).unwrap_or(CountDocument { count: 0.0 });
        count
      }
    };

    Ok(count as u64)
  }
}

pub mod stats {
  use mongodb::bson::Document;

  use super::*;

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/stream-connection-stats/")]
  pub struct Stats {
    pub time_now: TimedItem,
    pub time_24h: TimedItem,
    pub time_7d: TimedItem,
    pub time_30d: TimedItem,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/stream-connection-stats/")]
  pub struct TimedItem {
    pub sessions: f64,
    pub ips: f64,
    pub country_sessions: BTreeMap<String, f64>,
    pub country_ips: BTreeMap<String, f64>,
  }

  impl Stats {
    pub async fn get_for_filter(filter: Document) -> Result<Stats, mongodb::error::Error> {
      use crate::http;
      use const_str::concat as str;
      use futures_util::TryStreamExt;

      let now = time::OffsetDateTime::now_utc();
      let start_24h = now - time::Duration::HOUR * 24;
      let start_7d = now - time::Duration::DAY * 7;
      let start_30d = now - time::Duration::DAY * 30;

      const UNKNOWN: &str = "UNKNOWN";

      fn group_stage_timed_sessions(start: time::OffsetDateTime) -> Document {
        doc! {
          "$sum": {
            "$cond": {
              "if": { "$gte": [ str!("$", StreamConnection::KEY_CREATED_AT), DateTime::from(start) ] },
              "then": 1,
              "else": 0
            }
          }
        }
      }

      fn group_stage_timed_ips(start: time::OffsetDateTime) -> Document {
        doc! {
          "$addToSet": {
            "$cond": {
              "if": { "$gte": [ str!("$", StreamConnection::KEY_CREATED_AT), DateTime::from(start) ] },
              "then": str!("$", StreamConnection::KEY_REQUEST, ".", http::Request::KEY_REAL_IP),
              "else": "$NONE"
            }
          }
        }
      }

      let match_stage = doc! {
        "$match": {
          "$and": [
            { StreamConnection::KEY_CREATED_AT: { "$gte": DateTime::from(start_30d) } },
            filter,
          ]
        }
      };

      let country_group_stage = doc! {
        "$group": {
          "_id": str!("$", StreamConnection::KEY_REQUEST, ".", http::Request::KEY_COUNTRY_CODE),
          "sessions_now": {
            "$sum": {
              "$cond": {
                "if": { "$eq": [ str!("$", StreamConnection::KEY_STATE), State::KEY_ENUM_VARIANT_OPEN ] },
                "then": 1,
                "else": 0
              }
            }
          },
          "ips_now": {
            "$addToSet": {
              "$cond": {
                "if": { "$eq": [ str!("$", StreamConnection::KEY_STATE), State::KEY_ENUM_VARIANT_OPEN ] },
                "then": str!("$", StreamConnection::KEY_REQUEST, ".", http::Request::KEY_REAL_IP),
                "else": "$NONE"
              }
            }
          },
          "sessions_24h": group_stage_timed_sessions(start_24h),
          "ips_24h": group_stage_timed_ips(start_24h),
          "sessions_7d": group_stage_timed_sessions(start_7d),
          "ips_7d": group_stage_timed_ips(start_7d),
          "sessions_30d": group_stage_timed_sessions(start_30d),
          "ips_30d": group_stage_timed_ips(start_30d),
        }
      };

      let global_group_stage = doc! {
        "$group": {
          "_id": null,
          "sessions_now": { "$sum": "sessions_now" },
          "sessions_24h": { "$sum": "sessions_24d" },
          "sessions_7d": { "$sum": "sessions_7d" },
          "sessions_30d": { "$sum": "sessions_30d" },
          "ips_now": { "$sum": { "$size": "$ips_now" } },
          "ips_24h": { "$sum": { "$size": "$ips_20h" } },
          "ips_7d": { "$sum": { "$size": "$ips_7d" } },
          "ips_30d": { "$sum": { "$size": "$ips_30d" } },
          "country_sessions_now": {
            "$push": {
              "k": { "$ifNull": [ "$_id", UNKNOWN ] },
              "v": "$sessions_now",
            }
          },
          "country_sessions_24h": {
            "$push": {
              "k": { "$ifNull": [ "$_id", UNKNOWN ] },
              "v": "$sessions_24h",
            }
          },
          "country_sessions_7d": {
            "$push": {
              "k": { "$ifNull": [ "$_id", UNKNOWN ] },
              "v": "$sessions_7d",
            }
          },
          "country_sessions_30d": {
            "$push": {
              "k": { "$ifNull": [ "$_id", UNKNOWN ] },
              "v": "$sessions_30d",
            }
          },
          "country_ips_now": {
            "$push": {
              "k": { "$ifNull": [ "$_id", UNKNOWN ] },
              "v": { "$size": "$ips_now" },
            }
          },
          "country_ips_24h": {
            "$push": {
              "k": { "$ifNull": [ "$_id", UNKNOWN ] },
              "v": { "$size": "$ips_24h" },
            }
          },
          "country_ips_7d": {
            "$push": {
              "k": { "$ifNull": [ "$_id", UNKNOWN ] },
              "v": { "$size": "$ips_7d" },
            }
          },
          "country_ips_30d": {
            "$push": {
              "k": { "$ifNull": [ "$_id", UNKNOWN ] },
              "v": { "$size": "$ips_30d" },
            }
          },
        }
      };

      macro_rules! timed_item_projection {
        ($key:expr) => {
          doc! {
            "sessions": str!("$sessions_", $key),
            "ips": str!("$ips_", $key),
            "country_sessions": { "$arrayToObject": str!("$country_sessions_", $key) },
            "country_ips": { "$arrayToObject": str!("$country_ips_", $key) },
          }
        };
      }

      let projection_stage = doc! {
        "_id": 0,
        "time_now": timed_item_projection!("now"),
        "time_24h": timed_item_projection!("24h"),
        "time_7d": timed_item_projection!("7d"),
        "time_30d": timed_item_projection!("30d"),
      };

      let mut cursor = StreamConnection::cl()
        .aggregate(
          vec![
            match_stage,
            country_group_stage,
            global_group_stage,
            projection_stage,
          ],
          None,
        )
        .await?;

      let document = cursor.try_next().await?.unwrap();

      let stats: Stats = mongodb::bson::from_document(document).unwrap();

      Ok(stats)
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, StreamConnection::KEY_ID);
  }
}
