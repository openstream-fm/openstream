use crate::Model;
use crate::{audio_chunk::AudioChunk, audio_file::AudioFile};
use mongodb::ClientSession;
use mongodb::{bson::doc, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

crate::register!(AudioUploadOperation);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[ts(rename = "AudioUploadOperationState")]
#[serde(rename_all = "snake_case")]
#[serde(tag = "state")]
pub enum State {
  Pending,
  Success {
    commited_at: DateTime,
  },
  Error {
    cancelled_at: DateTime,
    error_display: String,
    error_debug: String,
  },
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct AudioUploadOperation {
  #[serde(rename = "_id")]
  pub id: String,
  pub station_id: String,
  pub deployment_id: String,
  pub created_at: DateTime,

  #[serde(flatten)]
  pub state: State,
}

impl Model for AudioUploadOperation {
  const UID_LEN: usize = AudioFile::UID_LEN;
  const CL_NAME: &'static str = "audio_upload_operations";

  fn indexes() -> Vec<IndexModel> {
    let station_id = IndexModel::builder()
      .keys(doc! { Self::KEY_STATION_ID: 1 })
      .build();
    vec![station_id]
  }
}

impl AudioUploadOperation {
  pub async fn clean_up_chunks_after_error(
    operation_id: &str,
  ) -> Result<mongodb::results::DeleteResult, mongodb::error::Error> {
    let filter = doc! { AudioChunk::KEY_AUDIO_FILE_ID: operation_id };
    AudioChunk::cl().delete_many(filter, None).await
  }

  pub async fn clean_up_chunks_after_error_with_session(
    operation_id: &str,
    session: &mut ClientSession,
  ) -> Result<mongodb::results::DeleteResult, mongodb::error::Error> {
    let filter = doc! { AudioChunk::KEY_AUDIO_FILE_ID: operation_id };
    AudioChunk::cl()
      .delete_many_with_session(filter, None, session)
      .await
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn serialize_json_pending() {
    let operation = AudioUploadOperation {
      id: "id".into(),
      deployment_id: "dep_id".into(),
      station_id: "station_id".into(),
      created_at: DateTime::now(),
      state: State::Pending,
    };

    eprintln!("{}", serde_json::to_string_pretty(&operation).unwrap());
  }

  #[test]
  fn serialize_json_success() {
    let operation = AudioUploadOperation {
      id: "id".into(),
      station_id: "station_id".into(),
      deployment_id: "dep_id".into(),
      created_at: DateTime::now(),
      state: State::Success {
        commited_at: DateTime::now(),
      },
    };

    eprintln!("{}", serde_json::to_string_pretty(&operation).unwrap());
  }

  #[test]
  fn serialize_json_error() {
    let operation = AudioUploadOperation {
      id: "id".into(),
      station_id: "station_id".into(),
      deployment_id: "dep_id".into(),
      created_at: DateTime::now(),
      state: State::Error {
        cancelled_at: DateTime::now(),
        error_display: "error".into(),
        error_debug: "Error {}".into(),
      },
    };

    eprintln!("{}", serde_json::to_string_pretty(&operation).unwrap());
  }

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, AudioUploadOperation::KEY_ID);
  }
}
