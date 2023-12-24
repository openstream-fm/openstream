use crate::Model;
use geoip::CountryCode;
use mongodb::bson::doc;
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use std::net::IpAddr;
use ts_rs::TS;

crate::register!(WsStatsConnection);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct WsStatsConnection {
  #[serde(rename = "_id")]
  pub id: String,

  #[serde(rename = "st")]
  pub station_id: String,

  #[serde(rename = "dp")]
  pub deployment_id: String,
  // #[serde(with = "serde_util::as_f64::option")]
  // pub transfer_bytes: Option<u64>,
  #[serde(rename = "du")]
  #[serde(with = "serde_util::as_f64::option")]
  pub duration_ms: Option<u64>,

  #[serde(rename = "op")]
  pub is_open: bool,

  #[serde(rename = "cc")]
  pub country_code: Option<CountryCode>,

  #[serde(rename = "ip")]
  #[serde(with = "serde_util::ip")]
  pub ip: IpAddr,

  #[serde(rename = "ap")]
  pub app_kind: Option<String>,

  #[serde(rename = "av")]
  pub app_version: Option<f64>,

  #[serde(rename = "ca")]
  pub created_at: DateTime,
  // pub request: Request,
  // pub last_transfer_at: DateTime,
  #[serde(rename = "cl")]
  pub closed_at: Option<DateTime>,
}

impl WsStatsConnection {
  pub const KEY_MANNUALLY_CLOSED: &'static str = "_m";
}

impl Model for WsStatsConnection {
  const CL_NAME: &'static str = "ws_stats_connection";
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

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, WsStatsConnection::KEY_ID);
  }
}
