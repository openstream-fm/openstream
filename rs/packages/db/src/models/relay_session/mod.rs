use crate::Model;
use mongodb::{bson::doc, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

crate::register!(RelaySession);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct RelaySession {
  #[serde(rename = "_id")]
  pub id: String,
  pub station_id: String,
  pub deployment_id: String,
  pub target_deployment_id: String,

  pub state: RelaySessionState,

  #[serde(with = "serde_util::as_f64")]
  pub transfer_bytes: u64,

  pub closed_at: Option<DateTime>,

  #[serde(with = "serde_util::as_f64::option")]
  pub duration_ms: Option<u64>,

  pub created_at: DateTime,
  pub updated_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub enum RelaySessionState {
  Open,
  Closed,
}

impl Model for RelaySession {
  const CL_NAME: &'static str = "relay_sessions";
  const UID_LEN: usize = 16;

  fn indexes() -> Vec<IndexModel> {
    let station_id = IndexModel::builder()
      .keys(doc! { RelaySession::KEY_STATION_ID: 1 })
      .build();

    let state = IndexModel::builder()
      .keys(doc! { RelaySession::KEY_STATE: 1 })
      .build();

    let created_at = IndexModel::builder()
      .keys(doc! { RelaySession::KEY_CREATED_AT: 1 })
      .build();

    let closed_at = IndexModel::builder()
      .keys(doc! { RelaySession::KEY_CLOSED_AT: 1 })
      .build();

    vec![station_id, state, created_at, closed_at]
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::station::Station;

  #[test]
  fn serde() {
    let doc = RelaySession {
      id: RelaySession::uid(),
      station_id: Station::uid(),
      deployment_id: String::from("asd"),
      target_deployment_id: String::from("asd"),
      transfer_bytes: 0,
      state: RelaySessionState::Closed,
      closed_at: Some(DateTime::now()),
      duration_ms: Some(100),
      created_at: DateTime::now(),
      updated_at: DateTime::now(),
    };

    let buf = mongodb::bson::to_vec(&doc).unwrap();

    let target: RelaySession = mongodb::bson::from_slice(&buf).unwrap();

    assert_eq!(doc, target);
  }

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, RelaySession::KEY_ID);
  }
}
