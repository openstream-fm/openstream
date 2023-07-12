use crate::Model;
use mongodb::{bson::doc, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use std::net::IpAddr;
use ts_rs::TS;

crate::register!(Deployment);

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

  // TODO: this Option is for back compat only
  // create a migration and change this to DateTime
  pub health_checked_at: Option<DateTime>,

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

pub async fn start_health_check_job(deployment_id: String) {
  let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(
    constants::DEPLOYMENT_HEALTH_CHECK_INTERVAL_SECS as u64,
  ));
  loop {
    interval.tick().await;
    let now = serde_util::DateTime::now();
    let update = doc! { "$set": { Deployment::KEY_HEALTH_CHECKED_AT: now } };
    match Deployment::update_by_id(&deployment_id, update).await {
      Ok(_) => continue,
      Err(e) => {
        log::error!(
          target: "deployment-health",
          "error updating deployment {}: {} => {}",
          deployment_id,
          e,
          e,
        )
      }
    };
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
