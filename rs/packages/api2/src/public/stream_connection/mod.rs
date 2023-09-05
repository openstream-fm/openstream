use db::{http::Request, stream_connection::StreamConnection};
use geoip::CountryCode;
use macros::pick_from;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use std::net::IpAddr;

use super::IntoPublic;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[pick_from(db::stream_connection::StreamConnection)]
pub struct PublicStreamConnection {
  pub id: String,
  pub station_id: String,
  pub is_open: bool,
  pub deployment_id: String,
  pub transfer_bytes: Option<u64>,
  pub duration_ms: Option<u64>,
  pub country_code: Option<CountryCode>,
  #[serde(with = "serde_util::ip")]
  pub ip: IpAddr,
  pub request: Request,
  pub created_at: DateTime,
  pub last_transfer_at: DateTime,
  pub closed_at: Option<DateTime>,
}

impl IntoPublic for StreamConnection {
  type Target = PublicStreamConnection;
  fn into_public(self, _: &crate::auth::AccessScope) -> Self::Target {
    self.into()
  }
}
