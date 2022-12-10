use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use serde_util;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[ts(export_to = "../../defs/error/")]
pub struct PublicError {
  #[ts(type = "number")]
  #[serde(with = "serde_util::status_code")]
  status: StatusCode,
  message: String,
  code: PublicErrorCode,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PublicErrorCode {}
