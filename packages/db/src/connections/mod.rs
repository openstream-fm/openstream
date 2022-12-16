use std::net::IpAddr;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;
use user_agent::UserAgent;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../defs/db/", rename = "BaseConnection")]
pub struct Connection {
  pub account_id: String,
  pub ip: IpAddr,
  pub user_agent: UserAgent,
  pub request_headers: IndexMap<String, String>,

  pub connected_at: DateTime,

  #[serde(with = "serde_util::as_f64")]
  pub transfer_bytes: u64,

  #[serde(flatten)]
  #[ts(skip)]
  pub state: State,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/db/", rename = "ConnectionState")]
#[serde(rename_all = "camelCase", tag = "state")]
pub enum State {
  Open,
  Closed {
    closed_at: DateTime,

    /// denormalization of the entire duration of the session
    /// to provide easier query access (eg: sort by, etc)
    #[serde(with = "serde_util::as_f64")]
    duration_ms: u64,
  },
}
