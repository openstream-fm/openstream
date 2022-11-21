#![allow(clippy::useless_format)]

use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::http::HeaderValue;
use hyper::{Body, StatusCode};
use prex::*;
use serde_json;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum Kind {
  TooManyRequests,
  ResourceNotFound,
  Db(mongodb::error::Error),
  TokenMissing,
  TokenNotFound,
  TokenMalformed,
  TokenUserNotFound(String),
  TokenAdminNotFound,
  TokenOutOfScope,
  AccountNotFound(String),
  AudioFileNotFound(String),
  QueryString(serde_querystring::Error),
}

#[derive(Debug)]
pub struct ApiError {
  kind: Kind,
}

impl From<Kind> for ApiError {
  fn from(kind: Kind) -> Self {
    Self { kind }
  }
}

impl ApiError {
  fn status(&self) -> StatusCode {
    match self.kind {
      Kind::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
      Kind::ResourceNotFound => StatusCode::NOT_FOUND,
      Kind::Db(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Kind::TokenMissing => StatusCode::UNAUTHORIZED,
      Kind::TokenMalformed => StatusCode::UNAUTHORIZED,
      Kind::TokenNotFound => StatusCode::UNAUTHORIZED,
      Kind::TokenUserNotFound(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Kind::TokenAdminNotFound => StatusCode::INTERNAL_SERVER_ERROR,
      Kind::TokenOutOfScope => StatusCode::UNAUTHORIZED,
      Kind::AccountNotFound(_) => StatusCode::NOT_FOUND,
      Kind::AudioFileNotFound(_) => StatusCode::NOT_FOUND,
      Kind::QueryString(ref _e) => StatusCode::BAD_REQUEST,
    }
  }

  fn message(&self) -> String {
    match &self.kind {
      Kind::TooManyRequests => format!("Too many requests"),
      Kind::ResourceNotFound => format!("Resource not found"),
      Kind::Db(_) => format!("Internal server error"),
      Kind::TokenMissing => format!("Access token is required"),
      Kind::TokenMalformed => format!("Access token is malformed"),
      Kind::TokenNotFound => format!("Access token not found"),
      Kind::TokenUserNotFound(id) => format!("User with id {id} has been deleted"),
      Kind::TokenAdminNotFound => format!("Admin has been deleted"),
      Kind::TokenOutOfScope => format!("Not enought permissions"),
      Kind::AccountNotFound(id) => format!("Account with id {id} not found"),
      Kind::AudioFileNotFound(id) => format!("Audio file with id {id} not found"),
      Kind::QueryString(e) => format!("Invalid query string: {e}"),
    }
  }

  fn kind_str(&self) -> &'static str {
    match self.kind {
      Kind::TooManyRequests => "ERR_TOO_MANY_REQUESTS",
      Kind::ResourceNotFound => "ERR_RESOURCE_NOT_FOUND",
      Kind::Db(_) => "ERR_DB",
      Kind::TokenMissing => "ERR_TOKEN_MISSING",
      Kind::TokenMalformed => "ERR_TOKEN_MALFORMED",
      Kind::TokenNotFound => "ERR_TOKEN_NOT_FOUND",
      Kind::TokenUserNotFound(_) => "ERR_TOKEN_USER_NOT_FOUND",
      Kind::TokenAdminNotFound => "ERR_TOKEN_ADMIN_NOT_FOUND",
      Kind::TokenOutOfScope => "ERR_TOKEN_OUT_OF_SCOPE",
      Kind::AccountNotFound(_) => "ERR_ACCOUNT_NOT_FOUND",
      Kind::AudioFileNotFound(_) => "ERR_AUDIO_FILE_NOT_FOUND",
      Kind::QueryString(ref _e) => "ERR_INVALID_QUERY_STRING",
    }
  }

  pub fn as_json(&self) -> serde_json::Value {
    serde_json::json!({
      "error": {
        "status": self.status().as_u16(),
        "message": self.message(),
        "kind": self.kind_str()
      }
    })
  }

  pub fn into_json_response(self) -> Response {
    let mut res = Response::new(self.status());

    let body = serde_json::to_vec(&self.as_json()).unwrap();

    res.headers_mut().append(
      CONTENT_TYPE,
      HeaderValue::from_static("application/json;charset=utf-8"),
    );

    res.headers_mut().append(
      CONTENT_LENGTH,
      HeaderValue::from_str(body.len().to_string().as_str()).unwrap(),
    );

    *res.body_mut() = Body::from(body);

    res
  }
}

impl Display for ApiError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "ApiError: {}", self.kind_str())?;
    match &self.kind {
      Kind::Db(e) => write!(f, " mongo => {}", e)?,
      Kind::TokenUserNotFound(id) => write!(f, " user_id => {}", id)?,
      Kind::AccountNotFound(id) => write!(f, " account_id => {}", id)?,
      _ => {}
    };

    Ok(())
  }
}

impl Error for ApiError {}

impl From<mongodb::error::Error> for ApiError {
  fn from(e: mongodb::error::Error) -> Self {
    Self::from(Kind::Db(e))
  }
}

impl From<!> for ApiError {
  fn from(value: !) -> Self {
    match value {}
  }
}

impl From<serde_querystring::Error> for ApiError {
  fn from(e: serde_querystring::Error) -> Self {
    Self::from(Kind::QueryString(e))
  }
}
