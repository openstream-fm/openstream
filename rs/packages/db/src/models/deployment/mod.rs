use crate::Model;
use mongodb::{bson::doc, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use std::net::IpAddr;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct Deployment {
  #[serde(rename = "_id")]
  pub id: String,

  #[serde(with = "serde_util::ip")]
  pub local_ip: IpAddr,

  #[serde(with = "serde_util::u32_as_i64")]
  pub pid: u32,

  #[serde(with = "serde_util::port::vec")]
  #[serde(default)]
  pub source_ports: Vec<u16>,

  #[serde(with = "serde_util::port::vec")]
  #[serde(default)]
  pub stream_ports: Vec<u16>,

  #[serde(with = "serde_util::port::vec")]
  #[serde(default)]
  pub api_ports: Vec<u16>,

  pub state: DeploymentState,

  pub created_at: DateTime,
  pub updated_at: DateTime,

  pub dropped_at: Option<DateTime>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "lowercase")]
#[macros::keys]
pub enum DeploymentState {
  Active,
  Closing,
  Closed,
}

impl Model for Deployment {
  const UID_LEN: usize = 8;
  const CL_NAME: &'static str = "deployments";

  fn indexes() -> Vec<IndexModel> {
    let state = IndexModel::builder()
      .keys(doc! { Deployment::KEY_STATE: 1 })
      .build();

    let created_at = IndexModel::builder()
      .keys(doc! { Deployment::KEY_CREATED_AT: 1 })
      .build();

    let dropped_at = IndexModel::builder()
      .keys(doc! { Deployment::KEY_DROPPED_AT: 1 })
      .build();

    vec![state, created_at, dropped_at]
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, Deployment::KEY_ID);
  }
}
