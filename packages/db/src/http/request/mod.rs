use super::{Headers, Method, SocketAddr, Uri, Version};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use ts_rs::TS;
use user_agent::{UserAgent, UserAgentExt};

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/db/http/")]
#[serde(rename_all = "camelCase")]
pub struct Request {
  #[serde(with = "serde_util::ip")]
  pub remote_ip: IpAddr,
  pub local_addr: SocketAddr,
  pub version: Version,
  pub method: Method,
  pub uri: Uri,
  pub headers: Headers,
  pub user_agent: UserAgent,
}

impl Request {
  pub fn from_http(req: &prex::Request) -> Self {
    let remote_ip = req.isomorphic_ip();
    let version = Version::from_http(req.version());
    let method = Method::from_http(req.method());
    let local_addr = SocketAddr::from_http(req.local_addr());
    let headers = Headers::from_http(req.headers());
    let user_agent = req.parse_ua();
    let uri = Uri::from_http(req.uri());

    Self {
      remote_ip,
      local_addr,
      version,
      method,
      uri,
      headers,
      user_agent,
    }
  }
}
