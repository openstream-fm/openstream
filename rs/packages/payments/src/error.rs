use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error, ts_rs::TS)]
#[ts(rename = "PaymentsErrorBase")]
#[error("{:?}: {}", kind, message)]
pub struct PaymentsError {
  pub message: String,
  #[ts(skip)]
  #[serde(flatten)]
  pub kind: PaymentsErrorKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "kebab-case", tag = "kind", content = "meta")]
pub enum PaymentsErrorKind {
  Provider,
  Payload,
  ResourceNotFound,
  Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
pub struct PaymentsErrorPayload {
  error: PaymentsError,
}
