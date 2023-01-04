use crate::Model;
use crate::{audio_chunk::AudioChunk, audio_file::AudioFile};
use mongodb::ClientSession;
use mongodb::{bson::doc, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/db/")]
#[ts(rename = "AudioUploadOperationState")]
#[serde(rename_all = "camelCase")]
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
#[ts(export)]
#[ts(export_to = "../../defs/db/")]
#[ts(rename = "BaseAudioUploadOperation")]
#[serde(rename_all = "camelCase")]
pub struct AudioUploadOperation {
  #[serde(rename = "_id")]
  pub id: String,
  pub account_id: String,
  pub created_at: DateTime,
  /// working in adding support for flattened enums in ts-rs
  #[serde(flatten)]
  #[ts(skip)]
  pub state: State,
}

impl Model for AudioUploadOperation {
  const UID_LEN: usize = AudioFile::UID_LEN;
  const CL_NAME: &'static str = "audio_upload_operations";

  fn indexes() -> Vec<IndexModel> {
    let account_id = IndexModel::builder().keys(doc! { "accountId": 1 }).build();
    vec![account_id]
  }
}

impl AudioUploadOperation {
  pub async fn clean_up_chunks_after_error(
    operation_id: &str,
  ) -> Result<mongodb::results::DeleteResult, mongodb::error::Error> {
    let filter = doc! { "audioFileId": operation_id };
    AudioChunk::cl().delete_many(filter, None).await
  }

  pub async fn clean_up_chunks_after_error_with_session(
    operation_id: &str,
    session: &mut ClientSession,
  ) -> Result<mongodb::results::DeleteResult, mongodb::error::Error> {
    let filter = doc! { "audioFileId": operation_id };
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
      account_id: "account_id".into(),
      created_at: DateTime::now(),
      state: State::Pending,
    };

    eprintln!("{}", serde_json::to_string_pretty(&operation).unwrap());
  }

  #[test]
  fn serialize_json_success() {
    let operation = AudioUploadOperation {
      id: "id".into(),
      account_id: "account_id".into(),
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
      account_id: "account_id".into(),
      created_at: DateTime::now(),
      state: State::Error {
        cancelled_at: DateTime::now(),
        error_display: "error".into(),
        error_debug: "Error {}".into(),
      },
    };

    eprintln!("{}", serde_json::to_string_pretty(&operation).unwrap());
  }
}
