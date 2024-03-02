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

#[allow(clippy::bool_comparison)]
#[inline(always)]
fn is_false(v: &bool) -> bool {
  *v == false
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct StreamConnectionLite {
  #[serde(rename = "_id")]
  pub id: String,

  // TODO: change this to String with a migration
  #[serde(rename = "dp")]
  pub deployment_id: Option<String>,

  #[serde(rename = "st")]
  pub station_id: String,

  #[serde(rename = "op")]
  pub is_open: bool,

  #[serde(rename = "ip")]
  #[serde(with = "serde_util::ip")]
  pub ip: IpAddr,

  #[serde(rename = "cc")]
  pub country_code: Option<CountryCode>,

  #[serde(rename = "du")]
  #[serde(with = "serde_util::as_f64::option")]
  pub duration_ms: Option<u64>,

  #[serde(rename = "by")]
  #[serde(with = "serde_util::as_f64::option")]
  pub transfer_bytes: Option<u64>,

  #[serde(rename = "br")]
  pub browser: Option<String>,

  #[serde(rename = "do")]
  pub domain: Option<String>,

  #[serde(rename = "os")]
  pub os: Option<String>,

  #[serde(rename = "ca")]
  pub created_at: DateTime,

  #[serde(rename = "re")]
  #[serde(default)]
  #[serde(skip_serializing_if = "is_false")]
  pub is_external_relay_redirect: bool,

  #[serde(rename = "_m")]
  #[serde(default)]
  #[serde(skip_serializing_if = "is_false")]
  pub manually_closed: bool,

  #[serde(rename = "cl")]
  pub closed_at: Option<DateTime>,
}

impl StreamConnectionLite {
  pub fn get_domain(full: &StreamConnection) -> Option<String> {
    match full.request.headers.get("referer") {
      None => None,
      Some(h) => match url::Url::parse(h) {
        Err(_) => None,
        Ok(url) => url.domain().map(String::from),
      },
    }
  }

  pub fn from_stream_connection_ref(full: &StreamConnection) -> Self {
    Self {
      id: full.id.clone(),
      deployment_id: Some(full.deployment_id.clone()),
      station_id: full.station_id.clone(),
      is_open: full.is_open,
      ip: full.ip,
      country_code: full.country_code,
      browser: full.request.user_agent.name.clone(),
      os: full.request.user_agent.os.clone(),
      domain: Self::get_domain(full),
      duration_ms: full.duration_ms,
      transfer_bytes: full.transfer_bytes,
      is_external_relay_redirect: full.is_external_relay_redirect,
      manually_closed: full.manually_closed,
      created_at: full.created_at,
      closed_at: full.closed_at,
    }
  }
}

impl From<StreamConnection> for StreamConnectionLite {
  fn from(full: StreamConnection) -> Self {
    Self {
      domain: Self::get_domain(&full),
      id: full.id,
      deployment_id: Some(full.deployment_id),
      station_id: full.station_id,
      is_open: full.is_open,
      ip: full.ip,
      browser: full.request.user_agent.name,
      os: full.request.user_agent.os,
      duration_ms: full.duration_ms,
      transfer_bytes: full.transfer_bytes,
      country_code: full.country_code,
      is_external_relay_redirect: full.is_external_relay_redirect,
      manually_closed: full.manually_closed,
      created_at: full.created_at,
      closed_at: full.closed_at,
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

    let station_id = IndexModel::builder()
      .keys(doc! { StreamConnectionLite::KEY_STATION_ID: 1 })
      .build();

    let open = IndexModel::builder()
      .keys(doc! { StreamConnectionLite::KEY_IS_OPEN: 1 })
      .build();

    let station_id_created_at = IndexModel::builder()
      .keys(
        doc! { StreamConnectionLite::KEY_STATION_ID: 1, StreamConnectionLite::KEY_CREATED_AT: 1 },
      )
      .build();

    vec![open, created_at, station_id, station_id_created_at]
  }
}
