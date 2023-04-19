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

  pub created_at: DateTime,

  #[serde(with = "serde_util::as_f64")]
  pub transfer_bytes: u64,
  #[serde(with = "serde_util::as_f64::option")]
  pub duration_ms: Option<u64>,

  pub last_transfer_at: DateTime,
  pub state: State,
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

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, StreamConnection::KEY_ID);
  }
}
