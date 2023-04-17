use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use ts_rs::TS;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, TS)]
#[ts(export, export_to = "../../../defs/db/http/")]
// #[serde(rename_all = "camelCase")]
pub struct SocketAddr {
  #[serde(with = "serde_util::ip")]
  pub ip: IpAddr,
  pub port: u16,
}

impl SocketAddr {
  pub fn from_http(addr: std::net::SocketAddr) -> Self {
    Self {
      ip: addr.ip(),
      port: addr.port(),
    }
  }

  pub fn to_http(self) -> std::net::SocketAddr {
    std::net::SocketAddr::from((self.ip, self.port))
  }
}
