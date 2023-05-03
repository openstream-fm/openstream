use crate::{run_transaction, Model};
use macros::Singleton;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS, Singleton)]
#[singleton(collection = "db_writable_test")]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct DbWritableTest {
  #[serde(rename = "_id")]
  pub id: String,
  pub random: f64,
}

impl Default for DbWritableTest {
  fn default() -> Self {
    Self {
      id: DbWritableTest::uid(),
      random: 0.0,
    }
  }
}

pub async fn test() -> Result<(), mongodb::error::Error> {
  run_transaction!(session => {
    let random: f64 = rand::random();
    let query = doc! {};
    let update = doc! { "$set": { DbWritableTest::KEY_RANDOM: random } };
    tx_try!(DbWritableTest::cl().update_one(query, update, None).await);
  });

  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, DbWritableTest::KEY_ID);
  }
}
