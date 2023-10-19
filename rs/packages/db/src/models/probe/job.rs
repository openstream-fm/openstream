use std::time::Instant;

use mongodb::bson::doc;
use serde_util::DateTime;

use super::{Probe, ProbeResult};
use crate::station::Station;
use crate::Model;

async fn get_probe_next_station() -> Result<Option<Station>, mongodb::error::Error> {
  let limit: DateTime = (time::OffsetDateTime::now_utc()
    - time::Duration::seconds(constants::PROBE_STATION_INTERVAL_SECS as i64))
  .into();
  // this filters includes null values
  let filter = doc! {
    "$and": [
      { Station::KEY_EXTERNAL_RELAY_URL: { "$ne": null } },
      {
        "$or": [
          { Station::KEY_LAST_EXTERNAL_RELAY_PROBE_STARTED_AT: { "$lt": limit } },
          { Station::KEY_LAST_EXTERNAL_RELAY_PROBE_STARTED_AT: null }
        ]
      }
    ]
  };

  let sort = doc! {
    Station::KEY_LAST_EXTERNAL_RELAY_PROBE_STARTED_AT: 1
  };

  let update = doc! {
    "$set": {
      Station::KEY_LAST_EXTERNAL_RELAY_PROBE_STARTED_AT: DateTime::now(),
    }
  };

  let options = mongodb::options::FindOneAndUpdateOptions::builder()
    .sort(sort)
    .return_document(mongodb::options::ReturnDocument::After)
    .build();

  Station::cl()
    .find_one_and_update(filter, update, options)
    .await
}

async fn run_one() -> Result<Option<Probe>, mongodb::error::Error> {
  let station = match get_probe_next_station().await? {
    None => return Ok(None),
    Some(station) => station,
  };

  // this unwrap is enforced with the mongodb filter
  let url = station.external_relay_url.clone().unwrap();

  let start = Instant::now();

  let result = match ffmpeg::probe::get(&url).await {
    Ok(document) => ProbeResult::Ok { document },
    Err(e) => {
      let error_display = format!("{}", e);
      let error_debug = format!("{:?}", e);
      match e {
        ffmpeg::probe::FfprobeError::Status {
          code,
          stdout,
          stderr,
        } => ProbeResult::Error {
          error_exit_code: Some(code),
          error_stdout: Some(stdout),
          error_stderr: Some(stderr),
          error_display,
          error_debug,
        },

        _ => ProbeResult::Error {
          error_exit_code: None,
          error_stdout: None,
          error_stderr: None,
          error_display,
          error_debug,
        },
      }
    }
  };

  let duration_ms = start.elapsed().as_millis() as f64;
  let now = DateTime::now();

  let document = Probe {
    id: Probe::uid(),
    station_id: station.id,
    url,
    duration_ms,
    result,
    created_at: now,
    updated_at: now,
    deleted_at: None,
  };

  Probe::insert(&document).await?;

  Ok(Some(document))
}

pub async fn start_probe_background_job() {
  async fn sleep() {
    tokio::time::sleep(tokio::time::Duration::from_secs(
      constants::PROBE_BACKGROUND_JOB_CHECK_INTERVAL_SECS as u64,
    ))
    .await;
  }

  loop {
    match run_one().await {
      Err(e) => {
        log::warn!("probe task error: {} => {:?}", e, e);
        sleep().await;
        continue;
      }

      Ok(None) => {
        sleep().await;
        continue;
      }

      Ok(Some(_)) => {
        continue;
      }
    }
  }
}
