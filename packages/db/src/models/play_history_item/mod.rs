use mongodb::bson::doc;
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

use crate::Model;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/db/", rename = "BasePlayHistoryItem")]
#[serde(rename_all = "camelCase")]
pub struct PlayHistoryItem {
  #[serde(rename = "_id")]
  pub id: String,
  pub account_id: String,

  // if we dont have name and artist in file metadata
  // we don't log the play history item
  // and we reject live log requests if they doesn't include both of them
  pub name: String,
  pub artist: String,

  pub start_at: DateTime,

  #[ts(skip)]
  #[serde(flatten)]
  pub kind: Kind,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/db/", rename = "PlayHistoryItemKind")]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum Kind {
  Live,
  File { file_id: String },
}

impl Model for PlayHistoryItem {
  const CL_NAME: &'static str = "play_history_items";
  const UID_LEN: usize = 20;

  fn indexes() -> Vec<IndexModel> {
    // TODO: should we use a compound index for this?
    // TODO: make some benchmarks with large number of items
    let account_id = IndexModel::builder().keys(doc! { "accountId": 1 }).build();
    let start_at = IndexModel::builder().keys(doc! { "startAt": 1 }).build();
    vec![account_id, start_at]
  }
}
