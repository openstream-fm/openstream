use super::{Headers, Method, SocketAddr, Uri, Version};
use geoip::CountryCode;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use ts_rs::TS;
use user_agent::{UserAgent, UserAgentExt};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, TS)]
#[ts(export, export_to = "../../../defs/db/http/")]
#[macros::keys]
pub struct Request {
  #[serde(with = "serde_util::ip")]
  pub real_ip: IpAddr,
  pub country_code: Option<CountryCode>,
  pub local_addr: SocketAddr,
  pub remote_addr: SocketAddr,
  pub version: Version,
  pub method: Method,
  pub uri: Uri,
  pub headers: Headers,
  pub user_agent: UserAgent,
}

impl Request {
  pub fn from_http(req: &prex::Request) -> Self {
    let real_ip = req.isomorphic_ip();
    let country_code = geoip::ip_to_country_code(&real_ip);
    let remote_addr = SocketAddr::from_http(req.remote_addr());
    let local_addr = SocketAddr::from_http(req.local_addr());
    let version = Version::from_http(req.version());
    let method = Method::from_http(req.method());
    let headers = Headers::from_http(req.headers());
    let user_agent = req.parse_ua();
    let uri = Uri::from_http(req.uri());

    Self {
      real_ip,
      country_code,
      remote_addr,
      local_addr,
      version,
      method,
      uri,
      headers,
      user_agent,
    }
  }
}
