use crate::Model;
use mongodb::{bson::doc, options::FindOneOptions, IndexModel};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

mod job;
pub use job::start_probe_background_job;

crate::register!(Probe);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct Probe {
  #[serde(rename = "_id")]
  pub id: String,

  pub station_id: String,
  pub url: String,

  pub duration_ms: f64,

  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,

  #[serde(flatten)]
  pub result: ProbeResult,
}

impl Probe {
  pub async fn last_for_url(url: &str) -> Result<Option<Self>, mongodb::error::Error> {
    let filter = doc! { Self::KEY_URL: url };
    let sort = doc! { Self::KEY_CREATED_AT: -1 };
    let options = FindOneOptions::builder().sort(sort).build();
    Self::cl().find_one(filter, options).await
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(tag = "r", rename_all = "snake_case")]
#[ts(export, export_to = "../../../defs/db/")]
pub enum ProbeResult {
  Ok {
    #[ts(type = "Record<string, any>")]
    document: ffmpeg::probe::Object,
  },
  Error {
    error_exit_code: Option<i32>,
    error_stdout: Option<String>,
    error_stderr: Option<String>,
    error_display: String,
    error_debug: String,
  },
}

impl Model for Probe {
  const UID_LEN: usize = 12;
  const CL_NAME: &'static str = "probes";

  fn indexes() -> Vec<IndexModel> {
    let station_id = IndexModel::builder()
      .keys(doc! { Self::KEY_STATION_ID: 1 })
      .build();

    let url = IndexModel::builder()
      .keys(doc! { Self::KEY_URL: 1 })
      .build();

    let created_at = IndexModel::builder()
      .keys(doc! { Self::KEY_CREATED_AT: 1 })
      .build();

    vec![station_id, url, created_at]
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, Probe::KEY_ID);
    assert_eq!(crate::KEY_DELETED_AT, Probe::KEY_DELETED_AT);
  }
}
