use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "code", content = "meta")]
pub enum PublicErrorCode {
  DB,
  AUTH_TOKEN_MISSING,
  AUTH_TOKEN_INVALID,
  AUTH_TOKEN_NOT_FOUND,
  AUTH_TOKEN_ADMIN_NOT_FOUND { admin_id: String },
  AUTH_TOKEN_USER_NOT_FOUND { user_id: String },
}
