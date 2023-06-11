use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use time::Duration;
use ts_rs::TS;

use crate::Model;

crate::register!(EmailVerificationCode);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
#[macros::keys]
pub struct EmailVerificationCode {
  #[serde(rename = "_id")]
  pub id: String,
  pub email: String,
  pub hash: String,
  pub used_at: Option<DateTime>,
  pub created_at: DateTime,
}

impl Model for EmailVerificationCode {
  const UID_LEN: usize = 12;
  const CL_NAME: &'static str = "email_verification_codes";
}

impl EmailVerificationCode {
  pub fn random_code() -> String {
    static CHARSET: &str = "0123456789";
    random_string::generate(constants::EMAIL_VERIFICATION_CODE_LEN, CHARSET)
  }

  pub fn is_expired(&self) -> bool {
    let now = DateTime::now().inner();

    let until = self
      .created_at
      .saturating_add(Duration::SECOND * constants::EMAIL_VERIFICATION_VALIDITY_SECS);

    now > until
  }
}
