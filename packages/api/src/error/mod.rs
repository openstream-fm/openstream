#![allow(clippy::useless_format)]

use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::http::HeaderValue;
use hyper::{Body, StatusCode};
use prex::request::ReadBodyJsonError;
use prex::*;
use serde_json;
use std::error::Error;
use std::fmt::Display;

use self::public::{PublicErrorCode, PublicErrorPayload};

pub mod public;

#[derive(Debug)]
pub enum Kind {
  TooManyRequests,
  ResourceNotFound,
  Db(mongodb::error::Error),
  TokenMissing,
  TokenNotFound,
  TokenMalformed,
  TokenUserNotFound(String),
  TokenAccountNotFound(String),
  TokenAdminNotFound(String),
  TokenOutOfScope,
  AccountNotFound(String),
  UserNotFound(String),
  AudioFileNotFound(String),
  QueryString(serde_querystring::Error),
  PayloadIo(hyper::Error),
  PayloadJson(serde_json::Error),
  PayloadTooLarge(usize),
  PayloadInvalid(String),
  AuthFailed,
  UserEmailExists,
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
      Kind::TokenAccountNotFound(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Kind::TokenAdminNotFound(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Kind::TokenOutOfScope => StatusCode::UNAUTHORIZED,
      Kind::AccountNotFound(_) => StatusCode::NOT_FOUND,
      Kind::UserNotFound(_) => StatusCode::NOT_FOUND,
      Kind::AudioFileNotFound(_) => StatusCode::NOT_FOUND,
      Kind::QueryString(_) => StatusCode::BAD_REQUEST,
      Kind::PayloadIo(_) => StatusCode::BAD_REQUEST,
      Kind::PayloadJson(_) => StatusCode::BAD_REQUEST,
      Kind::PayloadTooLarge(_) => StatusCode::BAD_REQUEST,
      Kind::PayloadInvalid(_) => StatusCode::BAD_REQUEST,
      Kind::AuthFailed => StatusCode::BAD_REQUEST,
      Kind::UserEmailExists => StatusCode::CONFLICT,
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
      Kind::TokenAccountNotFound(id) => format!("Account with id {id} has been deleted"),
      Kind::TokenAdminNotFound(id) => format!("Admin with id {id} has been deleted"),
      Kind::TokenOutOfScope => format!("Not enough permissions"),
      Kind::AccountNotFound(id) => format!("Account with id {id} not found"),
      Kind::UserNotFound(id) => format!("User with id {id} not found"),
      Kind::AudioFileNotFound(id) => format!("Audio file with id {id} not found"),
      Kind::QueryString(e) => format!("Invalid query string: {e}"),
      Kind::PayloadIo(e) => format!("Error reading payload: {e}"),
      Kind::PayloadJson(e) => format!("Invalid JSON payload: {e}"),
      Kind::PayloadTooLarge(_) => format!("Payload size exceeded"),
      Kind::PayloadInvalid(e) => format!("{e}"),
      Kind::AuthFailed => format!("There's no user with that email and password"),
      Kind::UserEmailExists => format!("User email already exists"),
    }
  }

  fn code(&self) -> PublicErrorCode {
    match self.kind {
      Kind::TooManyRequests => PublicErrorCode::TooManyRequests,
      Kind::ResourceNotFound => PublicErrorCode::ResourceNotFound,
      Kind::Db(_) => PublicErrorCode::InternalDb,
      Kind::TokenMissing => PublicErrorCode::TokenMissing,
      Kind::TokenMalformed => PublicErrorCode::TokenMalformed,
      Kind::TokenNotFound => PublicErrorCode::TokenNotFound,
      Kind::TokenUserNotFound(_) => PublicErrorCode::TokenUserNotFound,
      Kind::TokenAccountNotFound(_) => PublicErrorCode::TokenAccountNotFound,
      Kind::TokenAdminNotFound(_) => PublicErrorCode::TokenAdminNotFound,
      Kind::TokenOutOfScope => PublicErrorCode::TokenOutOfScope,
      Kind::AccountNotFound(_) => PublicErrorCode::AccountNotFound,
      Kind::UserNotFound(_) => PublicErrorCode::UserNotFound,
      Kind::AudioFileNotFound(_) => PublicErrorCode::AudioFileNotFound,
      Kind::QueryString(_) => PublicErrorCode::QueryStringInvalid,
      Kind::PayloadIo(_) => PublicErrorCode::PayloadIo,
      Kind::PayloadJson(_) => PublicErrorCode::PayloadJson,
      Kind::PayloadTooLarge(_) => PublicErrorCode::PayloadTooLarge,
      Kind::PayloadInvalid(_) => PublicErrorCode::PayloadInvalid,
      Kind::AuthFailed => PublicErrorCode::AuthFailed,
      Kind::UserEmailExists => PublicErrorCode::UserEmailExists,
    }
  }

  pub fn into_json_response(self) -> Response {
    let mut res = Response::new(self.status());

    let payload: PublicErrorPayload = self.into();

    let body = serde_json::to_vec(&payload).unwrap();

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
    write!(f, "ApiError: {:?}", self.code())?;
    match &self.kind {
      Kind::Db(e) => write!(f, " mongo => {}", e)?,

      Kind::TokenUserNotFound(id) => write!(f, " id => {id}")?,
      Kind::TokenAccountNotFound(id) => write!(f, " id: {id}")?,
      Kind::TokenAdminNotFound(id) => write!(f, " id: {id}")?,

      Kind::UserNotFound(id) => write!(f, " id: {id}")?,
      Kind::AccountNotFound(id) => write!(f, " id: {id}")?,
      Kind::AudioFileNotFound(id) => write!(f, " id: {id}")?,

      Kind::PayloadIo(e) => write!(f, " inner: {e}")?,
      Kind::PayloadInvalid(e) => write!(f, " message: {e}")?,
      Kind::PayloadJson(e) => write!(f, " inner: {e}")?,
      Kind::PayloadTooLarge(n) => write!(f, " max: {n}")?,
      Kind::QueryString(e) => write!(f, " inner: {e}")?,

      Kind::AuthFailed => {}
      Kind::ResourceNotFound => {}

      Kind::TokenNotFound => {}
      Kind::TokenMalformed => {}
      Kind::TokenMissing => {}
      Kind::TokenOutOfScope => {}
      Kind::TooManyRequests => {}

      Kind::UserEmailExists => {}
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

impl From<ReadBodyJsonError> for ApiError {
  fn from(e: ReadBodyJsonError) -> Self {
    match e {
      ReadBodyJsonError::Hyper(e) => Self::from(Kind::PayloadIo(e)),
      ReadBodyJsonError::Json(e) => Self::from(Kind::PayloadJson(e)),
      ReadBodyJsonError::TooLarge(maxlen) => Self::from(Kind::PayloadTooLarge(maxlen)),
      ReadBodyJsonError::PayloadInvalid(s) => Self::from(Kind::PayloadInvalid(s)),
    }
  }
}
