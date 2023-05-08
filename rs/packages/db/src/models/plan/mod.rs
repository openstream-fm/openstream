use crate::Model;
use futures_util::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

use crate::current_filter_doc;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct Plan {
  #[serde(rename = "_id")]
  pub id: String,
  pub identifier: String,
  pub display_name: String,
  pub price: f64,
  pub limits: PlanLimits,
  pub order: f64,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,
}

impl Model for Plan {
  const UID_LEN: usize = 6;
  const CL_NAME: &'static str = "plans";

  fn indexes() -> Vec<IndexModel> {
    let order = IndexModel::builder()
      .keys(doc! { Self::KEY_ORDER: 1 })
      .build();
    let created_at = IndexModel::builder()
      .keys(doc! { Self::KEY_CREATED_AT: 1 })
      .build();
    let deleted_at = IndexModel::builder()
      .keys(doc! { Self::KEY_DELETED_AT: 1 })
      .build();

    vec![order, created_at, deleted_at]
  }
}

impl Plan {
  pub async fn list() -> Result<Vec<Plan>, mongodb::error::Error> {
    let filter = current_filter_doc! {};
    let sort = doc! { Self::KEY_ORDER: 1 };
    let options = FindOptions::builder().sort(sort).build();

    let plans: Vec<Plan> = Self::cl()
      .find(filter, options)
      .await?
      .try_collect()
      .await?;

    Ok(plans)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
pub struct PlanLimits {
  #[serde(with = "serde_util::as_f64")]
  pub stations: u64,
  #[serde(with = "serde_util::as_f64")]
  pub listeners: u64,
  #[serde(with = "serde_util::as_f64")]
  pub transfer: u64,
  #[serde(with = "serde_util::as_f64")]
  pub storage: u64,
}
