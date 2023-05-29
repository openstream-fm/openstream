use super::StreamConnection;
use crate::Model;
use geoip::CountryCode;
use mongodb::bson::doc;
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use std::net::IpAddr;
use ts_rs::TS;

crate::register!(StreamConnectionLite);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct StreamConnectionLite {
  #[serde(rename = "_id")]
  pub id: String,
  #[serde(rename = "s")]
  pub station_id: String,
  #[serde(rename = "o")]
  pub is_open: bool,
  #[serde(rename = "i")]
  #[serde(with = "serde_util::ip")]
  pub ip: IpAddr,
  #[serde(rename = "c")]
  pub country_code: Option<CountryCode>,
  #[serde(rename = "d")]
  pub created_at: DateTime,
}

impl StreamConnectionLite {
  pub fn from_stream_connection_ref(full: &StreamConnection) -> Self {
    Self {
      id: full.id.clone(),
      station_id: full.station_id.clone(),
      is_open: full.is_open,
      ip: full.ip,
      country_code: full.country_code,
      created_at: full.created_at,
    }
  }
}

impl From<StreamConnection> for StreamConnectionLite {
  fn from(full: StreamConnection) -> Self {
    Self {
      id: full.id,
      station_id: full.station_id,
      is_open: full.is_open,
      ip: full.ip,
      country_code: full.country_code,
      created_at: full.created_at,
    }
  }
}

impl Model for StreamConnectionLite {
  const CL_NAME: &'static str = "stream_connections_lite";
  const UID_LEN: usize = StreamConnection::UID_LEN;

  fn indexes() -> Vec<IndexModel> {
    let created_at = IndexModel::builder()
      .keys(doc! { StreamConnectionLite::KEY_CREATED_AT: 1 })
      .build();

    vec![created_at]
  }
}
