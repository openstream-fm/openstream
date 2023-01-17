use mongodb::IndexModel;
use mongodb::{bson::doc, options::IndexOptions};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

use crate::Model;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/db/")]
#[macros::keys]
pub struct UserStationRelation {
  #[serde(rename = "_id")]
  pub id: String,
  pub user_id: String,
  pub station_id: String,
  pub kind: UserStationRelationKind,
  pub created_at: DateTime,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/db/")]
pub enum UserStationRelationKind {
  #[serde(rename = "owner")]
  Owner,
}

impl UserStationRelationKind {
  pub const TAG_OWNER: &str = "owner";
}

impl Model for UserStationRelation {
  const CL_NAME: &'static str = "user_station_relations";
  const UID_LEN: usize = 8;

  fn indexes() -> Vec<IndexModel> {
    let user_id = IndexModel::builder()
      .keys(doc! {
        UserStationRelation::KEY_USER_ID: 1,
      })
      .build();

    let station_id = IndexModel::builder()
      .keys(doc! {
        UserStationRelation::KEY_STATION_ID: 1,
      })
      .build();

    let opts = IndexOptions::builder().unique(true).build();
    let user_station = IndexModel::builder()
      .keys(doc! {
        UserStationRelation::KEY_USER_ID: 1,
        UserStationRelation::KEY_STATION_ID: 1,
      })
      .options(opts)
      .build();

    vec![user_id, station_id, user_station]
  }
}
