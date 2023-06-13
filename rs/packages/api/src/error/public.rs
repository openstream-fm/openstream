use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use serde_util;
use ts_rs::TS;

use super::ApiError;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/error/")]
pub struct PublicErrorPayload {
  pub error: PublicError,
}

impl From<PublicError> for PublicErrorPayload {
  fn from(error: PublicError) -> Self {
    Self { error }
  }
}

impl From<ApiError> for PublicErrorPayload {
  fn from(error: ApiError) -> Self {
    Self {
      error: error.into(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/error/")]
pub struct PublicError {
  #[ts(type = "number")]
  #[serde(with = "serde_util::status_code")]
  pub status: StatusCode,
  pub message: String,
  pub code: PublicErrorCode,
}

impl From<ApiError> for PublicError {
  fn from(e: ApiError) -> Self {
    Self {
      status: e.status(),
      message: e.message(),
      code: e.code(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../defs/error/")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PublicErrorCode {
  Internal,

  ResourceNotFound,
  TooManyRequests,

  BadRequest,

  InternalDb,
  InternalSerialize,

  IoRequest,

  TokenMissing,
  TokenMalformed,
  TokenNotFound,
  TokenUserNotFound,
  TokenStationNotFound,
  TokenAdminNotFound,
  TokenOutOfScope,
  TokenUserAccountNotOwner,

  StationNotFound,
  AdminNotFound,
  UserNotFound,
  AccountNotFound,
  AudioFileNotFound,
  DeviceNotFound,
  PlanNotFound,
  PaymentMethodNotFound,
  InvitationNotFound,

  QueryStringInvalid,
  PayloadIo,
  PayloadJson,
  PayloadTooLarge,
  PayloadInvalid,

  UserAuthFailed,
  AdminAuthFailed,

  UserEmailExists,
  AdminEmailExists,

  UploadEmpty,
  UploadQuotaExceeded,
  UploadInternalSpawn,
  UploadIntenralIo,
  UploadExit,

  RangeInvalid,
  RangeNoOverlap,

  PatchEmpty,
  PatchInvalid,
  PatchOutOfScope,

  ContentLengthRequired,

  UnresolvableAdminMe,
  UnresolvableUserMe,

  PlaylistStartIsLive,
  PlaylistStartNoFiles,

  RenderMail,
  SendMail,

  CreateStationAccountLimit,

  PaymentsPerform,
}
