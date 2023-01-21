use mongodb::bson::doc;
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

use crate::Model;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/db/", rename = "BasePlayHistoryItem")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct PlayHistoryItem {
  #[serde(rename = "_id")]
  pub id: String,
  pub station_id: String,

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
#[serde(rename_all = "snake_case")]
#[serde(tag = "kind")]
#[macros::keys]
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
    let station_id = IndexModel::builder()
      .keys(doc! { Self::KEY_STATION_ID: 1 })
      .build();
    let start_at = IndexModel::builder()
      .keys(doc! { Self::KEY_START_AT: 1 })
      .build();
    let kind = IndexModel::builder()
      .keys(doc! { Kind::KEY_ENUM_TAG: 1 })
      .build();

    vec![station_id, start_at, kind]
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, PlayHistoryItem::KEY_ID);
  }
}
