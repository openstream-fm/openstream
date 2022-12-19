#![allow(clippy::useless_format)]

use http_range::HttpRangeParseError;
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::http::HeaderValue;
use hyper::{Body, StatusCode};
use prex::request::ReadBodyJsonError;
use prex::*;
use serde_json;
use std::convert::Infallible;
use std::error::Error;
use std::fmt::Display;
use std::process::ExitStatus;

use self::public::{PublicErrorCode, PublicErrorPayload};
use db::error::ApplyPatchError;
use upload::UploadError;

pub mod public;

#[derive(Debug)]
pub enum Kind {
  TooManyRequests,
  ResourceNotFound,

  Db(mongodb::error::Error),
  Hyper(hyper::Error),
  QueryString(serde_querystring::Error),

  TokenMissing,
  TokenNotFound,
  TokenMalformed,
  TokenUserNotFound(String),
  TokenAccountNotFound(String),
  TokenAdminNotFound(String),
  TokenOutOfScope,

  AccountNotFound(String),
  AdminNotFound(String),
  UserNotFound(String),
  AudioFileNotFound(String),

  PayloadIo(hyper::Error),
  PayloadJson(serde_json::Error),
  PayloadTooLarge(usize),
  PayloadInvalid(String),

  AuthFailed,
  UserEmailExists,
  AdminEmailExists,

  UploadEmpty,
  UploadFfmpegExit {
    status: ExitStatus,
    stderr: Option<String>,
  },
  UploadFfmpegIo(std::io::Error),
  UploadSpawn(std::io::Error),
  UploadSizeExceeded,

  RangeInvalid,
  RangeNoOverlap,

  PatchEmpty,
  PatchInvalid(String),
  PatchOutOfScope(String),
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
      // for Kind::Hyper(e) we assume that is a network error responsability of the client so we respond BAD_REQUEST
      Kind::Hyper(_) => StatusCode::BAD_REQUEST,
      Kind::TokenMissing => StatusCode::UNAUTHORIZED,
      Kind::TokenMalformed => StatusCode::UNAUTHORIZED,
      Kind::TokenNotFound => StatusCode::UNAUTHORIZED,
      Kind::TokenUserNotFound(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Kind::TokenAccountNotFound(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Kind::TokenAdminNotFound(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Kind::TokenOutOfScope => StatusCode::UNAUTHORIZED,
      Kind::AccountNotFound(_) => StatusCode::NOT_FOUND,
      Kind::AdminNotFound(_) => StatusCode::NOT_FOUND,
      Kind::UserNotFound(_) => StatusCode::NOT_FOUND,
      Kind::AudioFileNotFound(_) => StatusCode::NOT_FOUND,
      Kind::QueryString(_) => StatusCode::BAD_REQUEST,
      Kind::PayloadIo(_) => StatusCode::BAD_REQUEST,
      Kind::PayloadJson(_) => StatusCode::BAD_REQUEST,
      Kind::PayloadTooLarge(_) => StatusCode::BAD_REQUEST,
      Kind::PayloadInvalid(_) => StatusCode::BAD_REQUEST,
      Kind::AuthFailed => StatusCode::BAD_REQUEST,
      Kind::UserEmailExists => StatusCode::CONFLICT,
      Kind::AdminEmailExists => StatusCode::CONFLICT,

      Kind::UploadEmpty => StatusCode::BAD_REQUEST,
      Kind::UploadSizeExceeded => StatusCode::BAD_REQUEST,
      Kind::UploadSpawn(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Kind::UploadFfmpegIo(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Kind::UploadFfmpegExit { .. } => StatusCode::BAD_REQUEST,

      Kind::RangeInvalid => StatusCode::RANGE_NOT_SATISFIABLE,
      Kind::RangeNoOverlap => StatusCode::RANGE_NOT_SATISFIABLE,

      Kind::PatchEmpty => StatusCode::BAD_REQUEST,
      Kind::PatchInvalid(_) => StatusCode::BAD_REQUEST,
      Kind::PatchOutOfScope(_) => StatusCode::BAD_REQUEST,
    }
  }

  fn message(&self) -> String {
    match &self.kind {
      Kind::TooManyRequests => format!("Too many requests"),
      Kind::ResourceNotFound => format!("Resource not found"),
      Kind::Db(_) => format!("Internal server error"),
      Kind::Hyper(_) => format!("I/O request error"),
      Kind::TokenMissing => format!("Access token is required"),
      Kind::TokenMalformed => format!("Access token is malformed"),
      Kind::TokenNotFound => format!("Access token not found"),
      Kind::TokenUserNotFound(id) => format!("User with id {id} has been deleted"),
      Kind::TokenAccountNotFound(id) => format!("Account with id {id} has been deleted"),
      Kind::TokenAdminNotFound(id) => format!("Admin with id {id} has been deleted"),
      Kind::TokenOutOfScope => format!("Not enough permissions"),
      Kind::AccountNotFound(id) => format!("Account with id {id} not found"),
      Kind::AdminNotFound(id) => format!("Admin with id {id} not found"),
      Kind::UserNotFound(id) => format!("User with id {id} not found"),
      Kind::AudioFileNotFound(id) => format!("Audio file with id {id} not found"),
      Kind::QueryString(e) => format!("Invalid query string: {e}"),
      Kind::PayloadIo(e) => format!("Error reading payload: {e}"),
      Kind::PayloadJson(e) => format!("Invalid JSON payload: {e}"),
      Kind::PayloadTooLarge(_) => format!("Payload size exceeded"),
      Kind::PayloadInvalid(e) => format!("{e}"),
      Kind::AuthFailed => format!("There's no user with that email and password"),
      Kind::UserEmailExists => format!("User email already exists"),
      Kind::AdminEmailExists => format!("Admin email already exists"),

      Kind::UploadEmpty => format!("Payload is empty"),
      Kind::UploadSizeExceeded => format!("Audio quota exceeded"),
      Kind::UploadSpawn(_) => format!("Internal server error"),
      Kind::UploadFfmpegIo(_) => format!("Internal server error"),
      Kind::UploadFfmpegExit { .. } => {
        format!("Error procesing audio file, invalid, malformed or unsupported file or format")
      }

      Kind::RangeInvalid => format!("Range invalid"),
      Kind::RangeNoOverlap => format!("Range no satisfiable, no overlap"),

      Kind::PatchEmpty => format!("Update operation is empty"),
      Kind::PatchInvalid(message) => format!("{message}"),
      Kind::PatchOutOfScope(message) => format!("{message}"),
    }
  }

  fn code(&self) -> PublicErrorCode {
    match self.kind {
      Kind::TooManyRequests => PublicErrorCode::TooManyRequests,
      Kind::ResourceNotFound => PublicErrorCode::ResourceNotFound,
      Kind::Db(_) => PublicErrorCode::InternalDb,
      Kind::Hyper(_) => PublicErrorCode::IoRequest,
      Kind::TokenMissing => PublicErrorCode::TokenMissing,
      Kind::TokenMalformed => PublicErrorCode::TokenMalformed,
      Kind::TokenNotFound => PublicErrorCode::TokenNotFound,
      Kind::TokenUserNotFound(_) => PublicErrorCode::TokenUserNotFound,
      Kind::TokenAccountNotFound(_) => PublicErrorCode::TokenAccountNotFound,
      Kind::TokenAdminNotFound(_) => PublicErrorCode::TokenAdminNotFound,
      Kind::TokenOutOfScope => PublicErrorCode::TokenOutOfScope,
      Kind::AccountNotFound(_) => PublicErrorCode::AccountNotFound,
      Kind::AdminNotFound(_) => PublicErrorCode::AdminNotFound,
      Kind::UserNotFound(_) => PublicErrorCode::UserNotFound,
      Kind::AudioFileNotFound(_) => PublicErrorCode::AudioFileNotFound,
      Kind::QueryString(_) => PublicErrorCode::QueryStringInvalid,
      Kind::PayloadIo(_) => PublicErrorCode::PayloadIo,
      Kind::PayloadJson(_) => PublicErrorCode::PayloadJson,
      Kind::PayloadTooLarge(_) => PublicErrorCode::PayloadTooLarge,
      Kind::PayloadInvalid(_) => PublicErrorCode::PayloadInvalid,
      Kind::AuthFailed => PublicErrorCode::AuthFailed,
      Kind::UserEmailExists => PublicErrorCode::UserEmailExists,
      Kind::AdminEmailExists => PublicErrorCode::AdminEmailExists,

      Kind::UploadEmpty => PublicErrorCode::UploadEmpty,
      Kind::UploadSizeExceeded => PublicErrorCode::UploadSizeExceeded,
      Kind::UploadSpawn(_) => PublicErrorCode::UploadInternalSpawn,
      Kind::UploadFfmpegIo(_) => PublicErrorCode::UploadIntenralIo,
      Kind::UploadFfmpegExit { .. } => PublicErrorCode::UploadExit,

      Kind::RangeInvalid => PublicErrorCode::RangeInvalid,
      Kind::RangeNoOverlap => PublicErrorCode::RangeNoOverlap,

      Kind::PatchEmpty => PublicErrorCode::PatchEmpty,
      Kind::PatchInvalid(_) => PublicErrorCode::PatchInvalid,
      Kind::PatchOutOfScope(_) => PublicErrorCode::PatchOutOfScope,
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
      Kind::Hyper(e) => write!(f, " hyper => {}", e)?,

      Kind::TokenUserNotFound(id) => write!(f, " id => {id}")?,
      Kind::TokenAccountNotFound(id) => write!(f, " id: {id}")?,
      Kind::TokenAdminNotFound(id) => write!(f, " id: {id}")?,

      Kind::UserNotFound(id) => write!(f, " id: {id}")?,
      Kind::AccountNotFound(id) => write!(f, " id: {id}")?,
      Kind::AdminNotFound(id) => write!(f, " id: {id}")?,
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
      Kind::AdminEmailExists => {}

      Kind::UploadEmpty => {}
      Kind::UploadSizeExceeded => {}
      Kind::UploadSpawn(e) => write!(f, " inner: {e}")?,
      Kind::UploadFfmpegIo(e) => write!(f, "inner: {e}")?,
      Kind::UploadFfmpegExit { status, stderr } => {
        write!(f, " status: {status}, stderr: {:?}", stderr)?
      }

      Kind::RangeInvalid => {}
      Kind::RangeNoOverlap => {}

      Kind::PatchEmpty => {}
      Kind::PatchInvalid(message) => write!(f, " message: {message}")?,
      Kind::PatchOutOfScope(message) => write!(f, " message: {message}")?,
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

impl From<hyper::Error> for ApiError {
  fn from(e: hyper::Error) -> Self {
    Self::from(Kind::Hyper(e))
  }
}

impl From<Infallible> for ApiError {
  fn from(value: Infallible) -> Self {
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

impl<E: Into<ApiError>> From<UploadError<E>> for ApiError {
  fn from(e: UploadError<E>) -> Self {
    match e {
      UploadError::Mongo(e) => e.into(),
      UploadError::Empty => ApiError::from(Kind::UploadEmpty),
      UploadError::FfmpegExit { status, stderr } => {
        ApiError::from(Kind::UploadFfmpegExit { status, stderr })
      }
      UploadError::FfmpegIo(e) => ApiError::from(Kind::UploadFfmpegIo(e)),
      UploadError::FfmpegSpawn(e) => ApiError::from(Kind::UploadSpawn(e)),
      UploadError::SizeExceeded => ApiError::from(Kind::UploadSizeExceeded),
      UploadError::Stream(s) => s.into(),
    }
  }
}

impl From<HttpRangeParseError> for ApiError {
  fn from(e: HttpRangeParseError) -> Self {
    match e {
      HttpRangeParseError::InvalidRange => Self::from(Kind::RangeInvalid),
      HttpRangeParseError::NoOverlap => Self::from(Kind::RangeNoOverlap),
    }
  }
}

impl From<ApplyPatchError> for ApiError {
  fn from(e: ApplyPatchError) -> Self {
    match e {
      ApplyPatchError::PatchEmpty => ApiError::from(Kind::PatchEmpty),
      ApplyPatchError::PatchInvalid(message) => ApiError::from(Kind::PatchInvalid(message)),
      ApplyPatchError::OutOfScope(message) => ApiError::from(Kind::PatchOutOfScope(message)),
    }
  }
}
