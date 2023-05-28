use mongodb::bson::doc;
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

use crate::Model;

crate::register!(PlayHistoryItem);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
  export,
  export_to = "../../../defs/db/",
  rename = "BasePlayHistoryItem"
)]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct PlayHistoryItem {
  #[serde(rename = "_id")]
  pub id: String,
  pub station_id: String,

  pub deployment_id: String,

  // if we dont have at least name in file metadata
  // we don't log the play history item
  // and we reject live metadata requests if they doesn't include song at least
  pub title: String,
  pub artist: Option<String>,

  #[ts(skip)]
  #[serde(flatten)]
  pub kind: Kind,

  pub created_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
  export,
  export_to = "../../../defs/db/",
  rename = "PlayHistoryItemKind"
)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "kind")]
#[macros::keys]
pub enum Kind {
  Live,
  Playlist { file_id: String },
}

impl Model for PlayHistoryItem {
  const CL_NAME: &'static str = "play_history_items";
  const UID_LEN: usize = 20;

  fn indexes() -> Vec<IndexModel> {
    // TODO: should we add more indexes ?

    let created_at = IndexModel::builder()
      .keys(doc! { Self::KEY_CREATED_AT: 1 })
      .build();

    let station_id_created_at = IndexModel::builder()
      .keys(doc! { Self::KEY_STATION_ID: 1, Self::KEY_CREATED_AT: 1 })
      .build();

    let kind = IndexModel::builder()
      .keys(doc! { Kind::KEY_ENUM_TAG: 1 })
      .build();

    vec![created_at, station_id_created_at, kind]
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
