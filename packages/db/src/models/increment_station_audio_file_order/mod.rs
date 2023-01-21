use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{station::Station, Incrementer, Model};

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct IncrementStationAudioFileOrder {
  #[serde(rename = "_id")]
  pub id: String,
  pub next: f64,
}

impl Model for IncrementStationAudioFileOrder {
  const CL_NAME: &'static str = "increment_station_audio_file_order";
  const UID_LEN: usize = Station::UID_LEN;
}

impl Incrementer for IncrementStationAudioFileOrder {
  fn item_next(&self) -> f64 {
    self.next
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, IncrementStationAudioFileOrder::KEY_ID);
    assert_eq!(
      crate::KEY_INCREMENT_NEXT,
      IncrementStationAudioFileOrder::KEY_NEXT
    );
  }
}
