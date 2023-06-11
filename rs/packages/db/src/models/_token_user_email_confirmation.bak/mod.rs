use crate::Model;
use mongodb::bson::doc;
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use time::Duration;
use ts_rs::TS;

crate::register!(TokenUserEmailConfirmation);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[serde(rename_all = "snake_case")]
#[macros::keys]
pub struct TokenUserEmailConfirmation {
  #[serde(rename = "_id")]
  pub id: String,
  /// sha256 of a Self::random_key()
  pub hash: String,
  pub user_id: String,
  pub email: String,
  pub accepted_at: Option<DateTime>,
  pub created_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub enum TokenUserEmailConfirmationState {}

impl TokenUserEmailConfirmation {
  pub const RANDOM_KEY_LEN: usize = 24;
  pub const VALIDITY_SECONDS: u32 = 60 * 60 * 3; // 3 hr

  pub fn random_key() -> String {
    uid::uid(Self::RANDOM_KEY_LEN)
  }

  pub fn is_valid_now(&self) -> bool {
    let now = DateTime::now().inner();

    let until = self
      .created_at
      .saturating_add(Duration::SECOND * Self::VALIDITY_SECONDS);

    now < until
  }
}

impl Model for TokenUserEmailConfirmation {
  const UID_LEN: usize = 12;
  const CL_NAME: &'static str = "token_user_email_confirmation";

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
    assert_eq!(crate::KEY_ID, TokenUserEmailConfirmation::KEY_ID);
  }
}
