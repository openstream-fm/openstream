#![allow(clippy::useless_format)]

use db::station_picture::CreateStationPictureError;
use http_range::HttpRangeParseError;
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::http::HeaderValue;
use hyper::{Body, StatusCode};
use prex::request::{ReadBodyBytesError, ReadBodyJsonError};
use prex::*;
use serde_json;
use std::convert::Infallible;
use std::process::ExitStatus;

use self::public::{PublicErrorCode, PublicErrorPayload};
use db::error::ApplyPatchError;
use upload::UploadError;

pub mod public;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
  #[error("too many requests")]
  TooManyRequests,

  #[error("resource not found")]
  ResourceNotFound,

  #[error("mongodb: {0}")]
  Db(#[from] mongodb::error::Error),

  #[error("hyper: {0}")]
  Hyper(#[from] hyper::Error),

  #[error("querystring: {0}")]
  QueryString(#[from] serde_querystring::de::Error),

  #[error("querystring2: {0}")]
  QueryString2(String),

  #[error("token missing")]
  TokenMissing,

  #[error("token not found")]
  TokenNotFound,

  #[error("token malformed")]
  TokenMalformed,

  #[error("token user not found: {0}")]
  TokenUserNotFound(String),

  #[error("token admin not found: {0}")]
  TokenAdminNotFound(String),

  #[error("token out of scope")]
  TokenOutOfScope,

  #[error("station not found: {0}")]
  StationNotFound(String),

  #[error("admin not found: {0}")]
  AdminNotFound(String),

  #[error("user not found: {0}")]
  UserNotFound(String),

  #[error("account not found: {0}")]
  AccountNotFound(String),

  #[error("audio file not found: {0}")]
  AudioFileNotFound(String),

  #[error("payload io: {0}")]
  PayloadIo(hyper::Error),

  #[error("payload json: {0}")]
  PayloadJson(#[from] serde_json::Error),

  #[error("payload too large: {0}")]
  PayloadTooLarge(usize),

  #[error("payload invalid: {0}")]
  PayloadInvalid(String),

  #[error("auth failed")]
  AuthFailed,

  #[error("user email exists")]
  UserEmailExists,

  #[error("admin email exists")]
  AdminEmailExists,

  #[error("upload empty")]
  UploadEmpty,

  #[error("upload ffmpeg exit: status: {}, stderr: {:?}", status, stderr)]
  UploadFfmpegExit {
    status: ExitStatus,
    stderr: Option<String>,
  },

  #[error("upload ffmpeg io: {0}")]
  UploadFfmpegIo(std::io::Error),

  #[error("upload ffmpeg spawn: {0}")]
  UploadSpawn(std::io::Error),

  #[error("upload quota exceeded")]
  UploadQuotaExceeded,

  #[error("range invalid")]
  RangeInvalid,

  #[error("range no overlap")]
  RangeNoOverlap,

  #[error("patch empty")]
  PatchEmpty,

  #[error("patch invalid: {0}")]
  PatchInvalid(String),

  #[error("patch out of scope: {0}")]
  PatchOutOfScope(String),

  #[error("content length required")]
  ContentLengthRequired,

  #[error("unresolvable admin me")]
  UnresolvableAdminMe,

  #[error("unresolvable user me")]
  UnresolvableUserMe,

  #[error("serialize json: {0}")]
  SerializeJSON(serde_json::Error),

  #[error("cannot start playlist (currently live streaming)")]
  PlaylistStartIsLive,

  #[error("cannot start playlist (no files for account")]
  PlaylistStartNoFiles,
}

impl ApiError {
  pub fn status(&self) -> StatusCode {
    use ApiError::*;

    match self {
      TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
      ResourceNotFound => StatusCode::NOT_FOUND,
      Db(_) => StatusCode::INTERNAL_SERVER_ERROR,
      // for Kind::Hyper(e) we assume that is a network error responsability of the client so we respond BAD_REQUEST
      Hyper(_) => StatusCode::BAD_REQUEST,
      TokenMissing => StatusCode::UNAUTHORIZED,
      TokenMalformed => StatusCode::UNAUTHORIZED,
      TokenNotFound => StatusCode::UNAUTHORIZED,
      TokenUserNotFound(_) => StatusCode::INTERNAL_SERVER_ERROR,
      TokenAdminNotFound(_) => StatusCode::INTERNAL_SERVER_ERROR,
      TokenOutOfScope => StatusCode::UNAUTHORIZED,
      StationNotFound(_) => StatusCode::NOT_FOUND,
      AdminNotFound(_) => StatusCode::NOT_FOUND,
      AccountNotFound(_) => StatusCode::NOT_FOUND,
      UserNotFound(_) => StatusCode::NOT_FOUND,
      AudioFileNotFound(_) => StatusCode::NOT_FOUND,
      QueryString(_) => StatusCode::BAD_REQUEST,
      QueryString2(_) => StatusCode::BAD_REQUEST,
      PayloadIo(_) => StatusCode::BAD_REQUEST,
      PayloadJson(_) => StatusCode::BAD_REQUEST,
      PayloadTooLarge(_) => StatusCode::BAD_REQUEST,
      PayloadInvalid(_) => StatusCode::BAD_REQUEST,
      AuthFailed => StatusCode::BAD_REQUEST,
      UserEmailExists => StatusCode::CONFLICT,
      AdminEmailExists => StatusCode::CONFLICT,

      UploadEmpty => StatusCode::BAD_REQUEST,
      UploadQuotaExceeded => StatusCode::BAD_REQUEST,
      UploadSpawn(_) => StatusCode::INTERNAL_SERVER_ERROR,
      UploadFfmpegIo(_) => StatusCode::INTERNAL_SERVER_ERROR,
      UploadFfmpegExit { .. } => StatusCode::BAD_REQUEST,

      RangeInvalid => StatusCode::RANGE_NOT_SATISFIABLE,
      RangeNoOverlap => StatusCode::RANGE_NOT_SATISFIABLE,

      PatchEmpty => StatusCode::BAD_REQUEST,
      PatchInvalid(_) => StatusCode::BAD_REQUEST,
      PatchOutOfScope(_) => StatusCode::BAD_REQUEST,

      ContentLengthRequired => StatusCode::LENGTH_REQUIRED,

      UnresolvableAdminMe => StatusCode::BAD_REQUEST,
      UnresolvableUserMe => StatusCode::BAD_REQUEST,

      SerializeJSON(_) => StatusCode::INTERNAL_SERVER_ERROR,

      PlaylistStartIsLive => StatusCode::BAD_REQUEST,
      PlaylistStartNoFiles => StatusCode::BAD_REQUEST,
    }
  }

  pub fn message(&self) -> String {
    use ApiError::*;
    match self {
      TooManyRequests => format!("Too many requests"),
      ResourceNotFound => format!("Resource not found"),
      Db(_) => format!("Internal server error"),
      Hyper(_) => format!("I/O request error"),
      TokenMissing => format!("Access token is required"),
      TokenMalformed => format!("Access token is malformed"),
      TokenNotFound => format!("Access token not found"),
      TokenUserNotFound(id) => format!("User with id {id} has been deleted"),
      TokenAdminNotFound(id) => format!("Admin with id {id} has been deleted"),
      TokenOutOfScope => format!("Not enough permissions"),
      StationNotFound(id) => format!("Station with id {id} not found"),
      AdminNotFound(id) => format!("Admin with id {id} not found"),
      UserNotFound(id) => format!("User with id {id} not found"),
      AccountNotFound(id) => format!("Account with id {id} not found"),
      AudioFileNotFound(id) => format!("Audio file with id {id} not found"),
      QueryString(e) => format!("Invalid query string: {e}"),
      QueryString2(message) => format!("Invalid query string: {message}"),
      PayloadIo(e) => format!("Error reading payload: {e}"),
      PayloadJson(e) => format!("Invalid JSON payload: {e}"),
      PayloadTooLarge(_) => format!("Payload size exceeded"),
      PayloadInvalid(e) => format!("{e}"),
      AuthFailed => format!("There's no user with that email and password"),
      UserEmailExists => format!("User email already exists"),
      AdminEmailExists => format!("Admin email already exists"),

      UploadEmpty => format!("Payload is empty"),
      UploadQuotaExceeded => format!("Audio quota exceeded"),
      UploadSpawn(_) => format!("Internal server error"),
      UploadFfmpegIo(_) => format!("Internal server error"),
      UploadFfmpegExit { .. } => {
        format!("Error procesing audio file, invalid, malformed or unsupported file or format")
      }

      RangeInvalid => format!("Range invalid"),
      RangeNoOverlap => format!("Range no satisfiable, no overlap"),

      PatchEmpty => format!("Update operation is empty"),
      PatchInvalid(message) => format!("{message}"),
      PatchOutOfScope(message) => format!("{message}"),
      ContentLengthRequired => format!("Content length is required"),

      UnresolvableAdminMe => format!("Cannot resolve 'me' admin with current access token scope"),
      UnresolvableUserMe => format!("Cannot resolve 'me' user with current access token scope"),

      SerializeJSON(_) => format!("Internal server error"),

      PlaylistStartIsLive => format!("Station is currenly live streaming"),
      PlaylistStartNoFiles => format!("Station playlist is empty"),
    }
  }

  pub fn code(&self) -> PublicErrorCode {
    use ApiError::*;
    match self {
      TooManyRequests => PublicErrorCode::TooManyRequests,
      ResourceNotFound => PublicErrorCode::ResourceNotFound,
      Db(_) => PublicErrorCode::InternalDb,
      Hyper(_) => PublicErrorCode::IoRequest,
      TokenMissing => PublicErrorCode::TokenMissing,
      TokenMalformed => PublicErrorCode::TokenMalformed,
      TokenNotFound => PublicErrorCode::TokenNotFound,
      TokenUserNotFound(_) => PublicErrorCode::TokenUserNotFound,
      TokenAdminNotFound(_) => PublicErrorCode::TokenAdminNotFound,
      TokenOutOfScope => PublicErrorCode::TokenOutOfScope,
      StationNotFound(_) => PublicErrorCode::StationNotFound,
      AdminNotFound(_) => PublicErrorCode::AdminNotFound,
      UserNotFound(_) => PublicErrorCode::UserNotFound,
      AccountNotFound(_) => PublicErrorCode::AccountNotFound,
      AudioFileNotFound(_) => PublicErrorCode::AudioFileNotFound,
      QueryString(_) => PublicErrorCode::QueryStringInvalid,
      QueryString2(_) => PublicErrorCode::QueryStringInvalid,
      PayloadIo(_) => PublicErrorCode::PayloadIo,
      PayloadJson(_) => PublicErrorCode::PayloadJson,
      PayloadTooLarge(_) => PublicErrorCode::PayloadTooLarge,
      PayloadInvalid(_) => PublicErrorCode::PayloadInvalid,
      AuthFailed => PublicErrorCode::AuthFailed,
      UserEmailExists => PublicErrorCode::UserEmailExists,
      AdminEmailExists => PublicErrorCode::AdminEmailExists,

      UploadEmpty => PublicErrorCode::UploadEmpty,
      UploadQuotaExceeded => PublicErrorCode::UploadQuotaExceeded,
      UploadSpawn(_) => PublicErrorCode::UploadInternalSpawn,
      UploadFfmpegIo(_) => PublicErrorCode::UploadIntenralIo,
      UploadFfmpegExit { .. } => PublicErrorCode::UploadExit,

      RangeInvalid => PublicErrorCode::RangeInvalid,
      RangeNoOverlap => PublicErrorCode::RangeNoOverlap,

      PatchEmpty => PublicErrorCode::PatchEmpty,
      PatchInvalid(_) => PublicErrorCode::PatchInvalid,
      PatchOutOfScope(_) => PublicErrorCode::PatchOutOfScope,

      ContentLengthRequired => PublicErrorCode::ContentLengthRequired,

      UnresolvableAdminMe => PublicErrorCode::UnresolvableAdminMe,
      UnresolvableUserMe => PublicErrorCode::UnresolvableUserMe,
      SerializeJSON(_) => PublicErrorCode::InternalSerialize,

      PlaylistStartIsLive => PublicErrorCode::PlaylistStartIsLive,
      PlaylistStartNoFiles => PublicErrorCode::PlaylistStartNoFiles,
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

impl From<Infallible> for ApiError {
  fn from(value: Infallible) -> Self {
    match value {}
  }
}

impl From<ReadBodyJsonError> for ApiError {
  fn from(e: ReadBodyJsonError) -> Self {
    match e {
      ReadBodyJsonError::Hyper(e) => Self::PayloadIo(e),
      ReadBodyJsonError::Json(e) => Self::PayloadJson(e),
      ReadBodyJsonError::TooLarge(maxlen) => Self::PayloadTooLarge(maxlen),
      ReadBodyJsonError::PayloadInvalid(s) => Self::PayloadInvalid(s),
    }
  }
}

impl From<ReadBodyBytesError> for ApiError {
  fn from(e: ReadBodyBytesError) -> Self {
    match e {
      ReadBodyBytesError::Hyper(e) => Self::PayloadIo(e),
      ReadBodyBytesError::TooLarge(maxlen) => Self::PayloadTooLarge(maxlen),
    }
  }
}

impl<E: Into<ApiError>> From<UploadError<E>> for ApiError {
  fn from(e: UploadError<E>) -> Self {
    match e {
      UploadError::Mongo(e) => e.into(),
      UploadError::Empty => ApiError::UploadEmpty,
      UploadError::FfmpegExit { status, stderr } => ApiError::UploadFfmpegExit { status, stderr },
      UploadError::StationNotFound(id) => ApiError::StationNotFound(id),
      UploadError::FfmpegIo(e) => ApiError::UploadFfmpegIo(e),
      UploadError::FfmpegSpawn(e) => ApiError::UploadSpawn(e),
      UploadError::QuotaExceeded => ApiError::UploadQuotaExceeded,
      UploadError::Stream(s) => s.into(),
    }
  }
}

impl From<HttpRangeParseError> for ApiError {
  fn from(e: HttpRangeParseError) -> Self {
    match e {
      HttpRangeParseError::InvalidRange => Self::RangeInvalid,
      HttpRangeParseError::NoOverlap => Self::RangeNoOverlap,
    }
  }
}

impl From<ApplyPatchError> for ApiError {
  fn from(e: ApplyPatchError) -> Self {
    match e {
      ApplyPatchError::PatchEmpty => ApiError::PatchEmpty,
      ApplyPatchError::PatchInvalid(message) => ApiError::PatchInvalid(message),
      ApplyPatchError::OutOfScope(message) => ApiError::PatchOutOfScope(message),
    }
  }
}

impl From<CreateStationPictureError> for ApiError {
  fn from(e: CreateStationPictureError) -> Self {
    use CreateStationPictureError::*;
    match e {
      Db(e) => e.into(),
      ImageTooLargeBytes | ImageNotSquare | ImageTooSmallSize | Ril(_) => {
        ApiError::PayloadInvalid(format!("{e}"))
      }
      AccountNotFound(_) => ApiError::QueryString2(format!("{e}")),
    }
  }
}
