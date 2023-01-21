use log::*;
use mongodb::IndexModel;
use mongodb::{bson::doc, options::IndexOptions};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

use crate::{
  run_transaction,
  station::{Limit, Limits, Station},
  Model,
};

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../defs/db/")]
#[macros::keys]
pub struct TransferCheckpoint {
  #[serde(rename = "_id")]
  id: String,
  year: f64,
  month: f64,
  matched_count: f64,
  modified_count: f64,
  created_at: DateTime,
}

impl Model for TransferCheckpoint {
  const CL_NAME: &'static str = "transfer_checkpoint";
  const UID_LEN: usize = 8;

  fn indexes() -> Vec<IndexModel> {
    let opts = IndexOptions::builder().unique(true).build();
    let year_month = IndexModel::builder()
      .keys(doc! {
        TransferCheckpoint::KEY_YEAR: 1,
        TransferCheckpoint::KEY_MONTH: 1,
      })
      .options(opts)
      .build();

    vec![year_month]
  }
}

pub async fn checkpoint_now() -> Result<Option<TransferCheckpoint>, mongodb::error::Error> {
  let now: time::OffsetDateTime = DateTime::now().into();

  let year = now.year() as f64;
  let month = now.month() as u8 as f64;

  run_transaction!(session => {
    let filter = doc! { TransferCheckpoint::KEY_YEAR: year , TransferCheckpoint::KEY_MONTH: month };
    let exists = tx_try!(TransferCheckpoint::exists_with_session(filter, &mut session).await);
    if exists {
      return Ok(None)
    }

    const KEY_LIMITS_TRANSFER_USED: &str = const_str::concat!(
      Station::KEY_LIMITS,
      ".",
      Limits::KEY_TRANSFER,
      ".",
      Limit::KEY_USED
    );

    let update = doc!{
      "$set": {
        KEY_LIMITS_TRANSFER_USED: 0_f64
      }
    };

    let update_result = tx_try!(Station::cl().update_many_with_session(doc!{}, update, None, &mut session).await);

    let doc = TransferCheckpoint {
      id: TransferCheckpoint::uid(),
      year,
      month,
      matched_count: update_result.matched_count as f64,
      modified_count: update_result.modified_count as f64,
      created_at: DateTime::now(),
    };

    tx_try!(TransferCheckpoint::insert_with_session(&doc, &mut session).await);

    Ok(Some(doc))
  })
}

/// This job will run every hour and check that a TransferCheckpoint was created for the current month
/// TransferCheckpoint will set used transfer to 0 for all stations at the start of every month
pub fn start_background_task() -> tokio::task::JoinHandle<()> {
  tokio::spawn(async move {
    info!("transfer checkpoint background job started");

    loop {
      match checkpoint_now().await {
        Err(e) => {
          warn!("error creating transfer checkpoint: {e}");
        }

        Ok(Some(doc)) => {
          info!(
            "transfer checkpoint created: {} {}-{} matched={}, modified={}",
            doc.id, doc.year, doc.month, doc.matched_count, doc.modified_count
          );
        }

        Ok(None) => {
          info!("transfer checkpoint => current month checkpoint already exists");
        }
      }

      let rand_offset_secs = rand::random::<f64>() * 10.0;

      let now: time::OffsetDateTime = DateTime::now().into();
      let sleep_until = now
        .replace_minute(0)
        .unwrap()
        .replace_second(0)
        .unwrap()
        .replace_microsecond(0)
        .unwrap()
        + time::Duration::HOUR
        + time::Duration::seconds_f64(rand_offset_secs);

      info!("transfer checkpoint: sleeping until {}", sleep_until);
      let sleep_secs = (sleep_until - now).as_seconds_f64();
      tokio::time::sleep(std::time::Duration::from_secs_f64(sleep_secs)).await;
    }
  })
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, TransferCheckpoint::KEY_ID);
  }
}
