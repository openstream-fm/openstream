use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error, ts_rs::TS)]
// #[ts(rename = "PaymentsErrorBase")]
#[error("{:?}: {}", kind, message)]
pub struct PaymentsError {
  pub message: String,
  // #[ts(skip)]
  #[serde(flatten)]
  pub kind: PaymentsErrorKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "kebab-case", tag = "kind")]
pub enum PaymentsErrorKind {
  Provider { provider_error_type: Option<String> },
  Payload,
  AccessTokenNotPresent,
  AccessTokenMismatch,
  ResourceNotFound,
  Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
pub struct PaymentsErrorPayload {
  pub error: PaymentsError,
}

#[derive(Debug, thiserror::Error)]
pub enum PerformError {
  #[error("fetch: {0}")]
  Fetch(#[source] reqwest::Error),

  #[error("get_error_payload: status={status} {source}")]
  GetErrorPayload {
    status: u16,
    #[source]
    source: reqwest::Error,
  },

  #[error("get_payload: {0}")]
  GetPayload(#[source] reqwest::Error),

  #[error("endpoint: {0}")]
  Endpoint(#[from] PaymentsError),
}
