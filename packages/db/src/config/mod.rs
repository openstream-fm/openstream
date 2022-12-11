use crate::{singleton_uid, Model, Singleton, SINGLETON_UID_LEN};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/db/")]
#[serde(rename_all = "camelCase")]
pub struct Config {
  #[serde(rename = "_id")]
  pub id: String,
  pub limits: Limits,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      id: Config::uid(),
      limits: Limits::default(),
    }
  }
}

#[async_trait]
impl Model for Config {
  const CL_NAME: &'static str = "config";
  const UID_LEN: usize = SINGLETON_UID_LEN;

  fn uid() -> String {
    singleton_uid()
  }

  async fn ensure_collection() -> Result<(), mongodb::error::Error> {
    Self::ensure_indexes().await?;
    Self::ensure_instance().await?;
    Ok(())
  }
}

impl Singleton for Config {}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/db/")]
#[ts(rename = "ConfigLimits")]
#[serde(rename_all = "camelCase")]
pub struct Limits {
  /// default max concurrent listeners for new accounts
  #[serde(with = "serde_util::as_f64")]
  pub listeners: u64,

  /// default max transfer / month in bytes for new accounts
  #[serde(with = "serde_util::as_f64")]
  pub transfer: u64,

  /// default max storage in bytes for new accounts
  #[serde(with = "serde_util::as_f64")]
  pub storage: u64,
}

impl Default for Limits {
  fn default() -> Self {
    Self {
      listeners: 1000,             // 1000 concurrent listeners
      transfer: 5_000_000_000_000, // 5 TB / month
      storage: 2_000_000_000_000,  // 2 TB
    }
  }
}
