use crate::model;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub const CL_NAME: &str = "audio_upload_operation";
pub const UID_LEN: usize = super::audio_file::UID_LEN;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "state")]
pub enum State {
  Pending,
  Success {
    #[serde(with = "serde_util::datetime")]
    commited_at: DateTime<Utc>,
  },
  Error {
    #[serde(with = "serde_util::datetime")]
    cancelled_at: DateTime<Utc>,
    error: String,
    error_debug: String,
  },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioUploadOperation {
  #[serde(rename = "_id")]
  pub id: String,
  pub account_id: String,

  #[serde(with = "serde_util::datetime")]
  pub created_at: DateTime<Utc>,

  #[serde(flatten)]
  pub state: State,
}

model!(AudioUploadOperation);

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn serialize_json_pending() {
    let operation = AudioUploadOperation {
      id: "id".into(),
      account_id: "account_id".into(),
      created_at: Utc::now(),
      state: State::Pending,
    };

    eprintln!("{}", serde_json::to_string_pretty(&operation).unwrap());
  }

  #[test]
  fn serialize_json_success() {
    let operation = AudioUploadOperation {
      id: "id".into(),
      account_id: "account_id".into(),
      created_at: Utc::now(),
      state: State::Success {
        commited_at: Utc::now(),
      },
    };

    eprintln!("{}", serde_json::to_string_pretty(&operation).unwrap());
  }

  #[test]
  fn serialize_json_error() {
    let operation = AudioUploadOperation {
      id: "id".into(),
      account_id: "account_id".into(),
      created_at: Utc::now(),
      state: State::Error {
        cancelled_at: Utc::now(),
        error: "error".into(),
        error_debug: "Error {}".into(),
      },
    };

    eprintln!("{}", serde_json::to_string_pretty(&operation).unwrap());
  }
}