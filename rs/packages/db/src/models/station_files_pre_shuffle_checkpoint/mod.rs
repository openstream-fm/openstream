use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

use crate::models::station::Station;
use crate::Model;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
  export,
  export_to = "../../../defs/db/",
  rename = "StationFilesPreShuffleCheckpoint"
)]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct StationFilesPreShuffleCheckpoint {
  #[serde(rename = "_id")]
  pub id: String,
  pub file_ids: Vec<String>,
  pub created_at: DateTime,
}

impl Model for StationFilesPreShuffleCheckpoint {
  const CL_NAME: &'static str = "station_files_pre_shuffle_checkpoint";
  const UID_LEN: usize = Station::UID_LEN;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, StationFilesPreShuffleCheckpoint::KEY_ID);
  }
}
