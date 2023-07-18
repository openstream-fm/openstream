use macros::pick_from;
use serde::{Deserialize, Serialize};

use super::IntoPublic;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[pick_from(db::http::Request)]
pub struct PublicRequest {
  #[serde(with = "serde_util::ip")]
  pub real_ip: std::net::IpAddr,
  pub country_code: Option<geoip::CountryCode>,
  pub local_addr: db::http::SocketAddr,
  pub remote_addr: db::http::SocketAddr,
  pub version: db::http::Version,
  pub method: db::http::Method,
  pub uri: db::http::Uri,
  pub headers: db::http::Headers,
  pub user_agent: user_agent::UserAgent,
}

impl IntoPublic for db::http::Request {
  type Target = PublicRequest;
  fn into_public(self, _: &crate::auth::AccessScope) -> Self::Target {
    From::from(self)
  }
}
