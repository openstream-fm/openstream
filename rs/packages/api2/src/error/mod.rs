use derive_more::Display;
use hyper::StatusCode;
use macros::GetStatus;
use serde::{Deserialize, Serialize, Serializer};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, thiserror::Error, GetStatus)]
#[serde(tag = "code", content = "meta")]
pub enum PublicError {
  // TODO: forward status
  #[error("{}", source)]
  #[status("source.status()")]
  FORWARDED {
    #[source]
    source: Box<PublicError>,
  },

  #[error("An internal error heppened when processing this request")]
  #[status("StatusCode::INTERNAL_SERVER_ERROR")]
  INTERNAL_DB,

  #[error("The request payload is too large, maximum allowed size is {max} bytes")]
  #[status("StatusCode::PAYLOAD_TOO_LARGE")]
  PAYLOAD_TOO_LARGE { max: usize },

  #[error("An error ocurred reading the request payload")]
  #[status("StatusCode::BAD_REQUEST")]
  PAYLOAD_READ,

  #[error("{message}")]
  #[status("StatusCode::BAD_REQUEST")]
  PAYLOAD_TYPE { message: String },

  #[error("{message}")]
  #[status("StatusCode::BAD_REQUEST")]
  PAYLOAD_VALIDATE { message: String },

  #[error("{message}")]
  #[status("StatusCode::BAD_REQUEST")]
  QUERY_STRING_TYPE { message: String },

  #[error("{message}")]
  #[status("StatusCode::BAD_REQUEST")]
  QUERY_STRING_VALIDATE { message: String },

  #[error("{message}")]
  #[status("StatusCode::BAD_REQUEST")]
  PATH_PARAM_INVALID { message: String },

  #[error("An error ocurred connecting to another server: {}", message)]
  #[status("StatusCode::BAD_GATEWAY")]
  BAD_GATEWAY { message: String },

  #[error("Auth token is missing from request")]
  #[status("StatusCode::UNAUTHORIZED")]
  AUTH_TOKEN_MISSING,

  #[error("Auth token is invalid")]
  #[status("StatusCode::UNAUTHORIZED")]
  AUTH_TOKEN_INVALID,

  #[error("Auth token not found or deleted")]
  #[status("StatusCode::UNAUTHORIZED")]
  AUTH_TOKEN_NOT_FOUND,

  #[error("Auth token admin with id {admin_id} not found or deleted")]
  #[status("StatusCode::UNAUTHORIZED")]
  AUTH_TOKEN_ADMIN_NOT_FOUND { admin_id: String },

  #[error("Auth token user with id {user_id} not found or deleted")]
  #[status("StatusCode::UNAUTHORIZED")]
  AUTH_TOKEN_USER_NOT_FOUND { user_id: String },

  #[error("{record_kind} with id {record_id} not found")]
  #[status("StatusCode::NOT_FOUND")]
  RECORD_NOT_FOUND {
    record_kind: RecordKind,
    record_id: String,
  },

  #[error("{record_kind} with id {record_id} not found")]
  #[status("StatusCode::BAD_REQUEST")]
  RECORD_NOT_FOUND_BAD_REQUEST {
    record_kind: RecordKind,
    record_id: String,
  },
}

/// internal type to add the message and status to the error payload
#[derive(Debug, Clone, Serialize)]
struct PublicErrorSerialize<'a> {
  status: u16,
  message: String,
  #[serde(flatten)]
  error: &'a PublicError,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PublicErrorPayload {
  pub error: PublicError,
}

impl Serialize for PublicError {
  fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
    PublicErrorSerialize {
      status: self.status().as_u16(),
      message: self.to_string(),
      error: self,
    }
    .serialize(ser)
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Display, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordKind {
  #[display(fmt = "Admin")]
  Admin,
  #[display(fmt = "User")]
  User,
  #[display(fmt = "Account")]
  Account,
  #[display(fmt = "Station")]
  Station,
}
