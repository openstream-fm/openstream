use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use ts_rs::TS;

use crate::Model;

crate::register!(SentEmail);

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/", rename = "SentEmailBase")]
// #[ts(rename = "SentEmailBase")]
pub struct SentEmail {
  #[serde(rename = "_id")]
  pub id: String,
  pub to: SentEmailAddress,
  pub from: SentEmailAddress,
  pub subject: String,
  pub text: String,
  pub html: String,
  pub reply_to: Option<SentEmailAddress>,

  // #[ts(skip)]
  #[serde(flatten)]
  pub kind: SentEmailKind,

  pub created_at: DateTime,
}

impl Model for SentEmail {
  const UID_LEN: usize = 24;
  const CL_NAME: &'static str = "sent_emails";
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/db/")]
pub struct SentEmailAddress {
  pub name: Option<String>,
  pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(tag = "kind", content = "data")]
#[ts(export, export_to = "../../../defs/db/")]
pub enum SentEmailKind {
  #[serde(rename = "user-recovery")]
  UserRecovery { user_id: String, token_id: String },

  #[serde(rename = "email-verification-code")]
  EmailVerificationCode { email: String, code_id: String },
}
