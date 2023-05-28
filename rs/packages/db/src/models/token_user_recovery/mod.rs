use crate::Model;
use mongodb::bson::doc;
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use time::Duration;
use ts_rs::TS;

crate::register!(TokenUserRecovery);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct TokenUserRecovery {
  #[serde(rename = "_id")]
  pub id: String,
  /// sha256 of a Self::random_key()
  pub hash: String,
  pub user_id: String,
  pub used_at: Option<DateTime>,
  pub created_at: DateTime,
}

impl TokenUserRecovery {
  pub const RANDOM_KEY_LEN: usize = 24;
  pub const VALIDITY_SECONDS: u32 = 60 * 60; // 1 hr

  pub fn random_key() -> String {
    uid::uid(Self::RANDOM_KEY_LEN)
  }

  pub fn is_expired(&self) -> bool {
    let now = DateTime::now().inner();

    let until = self
      .created_at
      .saturating_add(Duration::SECOND * Self::VALIDITY_SECONDS);

    now > until
  }
}

impl Model for TokenUserRecovery {
  const UID_LEN: usize = 12;
  const CL_NAME: &'static str = "token_user_recovery";

  fn indexes() -> Vec<IndexModel> {
    let user_id = IndexModel::builder()
      .keys(doc! { Self::KEY_USER_ID: 1 })
      .build();

    let created_at = IndexModel::builder()
      .keys(doc! { Self::KEY_CREATED_AT: 1 })
      .build();

    vec![user_id, created_at]
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn keys_match() {
    assert_eq!(crate::KEY_ID, TokenUserRecovery::KEY_ID);
  }
}
