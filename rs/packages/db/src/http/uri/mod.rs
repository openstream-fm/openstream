use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/db/http/")]
pub struct Uri {
  pub uri: String,
  pub scheme: Option<String>,
  pub host: Option<String>,
  pub port: Option<u16>,
  pub path: String,
  pub query: Option<String>,
}

impl Uri {
  pub fn from_http(src: &hyper::Uri) -> Self {
    let uri = src.to_string();
    let host = src.host().map(|h| h.to_string());
    let path = src.path().to_string();
    let query = src.query().map(|q| q.to_string());
    let scheme = src.scheme_str().map(|s| s.to_string());
    let port = src.port_u16();

    Self {
      uri,
      scheme,
      host,
      port,
      path,
      query,
    }
  }
}
