#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
// this is needed because of static_init that is generating wrongly cased identifiers

use macros::const_register;

pub(crate) mod secs {
  pub const MIN: u32 = 60;
  pub const HOUR: u32 = MIN * 60;
  pub const DAY: u32 = HOUR * 24;
}

/// stream kbps
#[const_register]
pub const STREAM_KBITRATE: usize = 128;

/// stream chunk size
#[const_register]
pub const STREAM_CHUNK_SIZE: usize = STREAM_KBITRATE * 1000 / 8;

/// stream bust len in elements (use with STREAM_CHUNK_SIZE)
#[const_register]
pub const STREAM_BURST_LENGTH: usize = 12;

/// stream tokio broadcaster stream channel capacity
#[const_register]
pub const STREAM_CHANNEL_CAPACITY: usize = 16;

/// audio file chunk size in bytes
#[const_register]
pub const AUDIO_FILE_CHUNK_SIZE: usize = 256 * 1000;

/// audio file byte rate
#[const_register]
pub const AUDIO_FILE_BYTERATE: usize = 128_000 / 8; // 128 kbps

/// station's transfer save interval in milliseconds
#[const_register]
pub const TRANSFER_SAVE_INTERVAL_MILLIS: u64 = 5_000;

/// limit of concurrent stream connections from the same ip
#[const_register]
pub const STREAM_IP_CONNECTIONS_LIMIT: u64 = 8;

/// delay to shutdown a playlist media session when it run out of listeners
#[const_register]
pub const PLAYLIST_NO_LISTENERS_SHUTDOWN_DELAY_SECS: u64 = 10;

#[const_register]
pub const EXTERNAL_RELAY_NO_LISTENERS_SHUTDOWN_DELAY_SECS: u64 = 60;

#[const_register]
pub const EXTERNAL_RELAY_NO_LISTENERS_SHUTDOWN_INTERVAL_SECS: u64 = 20;

/// delay to shutdown a relay session when it run out of listeners
#[const_register]
pub const RELAY_NO_LISTENERS_SHUTDOWN_DELAY_SECS: u64 = 10;

/// delay of which if external relay produced no data, it will be cancelled
#[const_register]
pub const EXTERNAL_RELAY_NO_DATA_SHUTDOWN_SECS: u64 = 30;

/// delay of which if external doesn't produce first data chunk, it will be cancelled
#[const_register]
pub const EXTERNAL_RELAY_NO_DATA_START_SHUTDOWN_SECS: u64 = 30;

/// limit of authotization, or other sensible api endpoints requests from the same ip
#[const_register]
pub const API_IP_LIMIT: usize = 60;

/// limit restart interval of API_IP_LIMIT
#[const_register]
pub const API_IP_LIMIT_DURATION_SECS: u64 = 60;

/// time in seconds for which an account invitation can be accepted (or rejected)
#[const_register]
pub const ACCOUNT_INVITATION_VALIDITY_SECS: u32 = secs::DAY * 7; // 7 days

/// time in seconds for which an email verification code is valid
#[const_register]
pub const EMAIL_VERIFICATION_VALIDITY_SECS: u32 = secs::HOUR; // 1 hr

/// time in seconds for which an email verification code is valid
#[const_register]
pub const EMAIL_VERIFICATION_CODE_LEN: usize = 6; // Eg: 123456

/// time in seconds for which an user recovery token code is valid
#[const_register]
pub const TOKEN_USER_RECOVERY_VALIDITY_SECS: u32 = secs::HOUR; // 1 hr

/// access token autoremove validity
/// remove access tokens that are not used in the last X time
/// this is only for Login, Register or AdminAsUser access tokens
/// Cli or Api(not AdminAsUser) access tokens does not auto-expire
#[const_register]
pub const ACCESS_TOKEN_NOT_USED_AUTOREMOVE_SECS: u32 = secs::DAY * 7; // 7 days

/// Access token header used in API endpoints
#[const_register]
pub const ACCESS_TOKEN_HEADER: &str = "x-access-token";

/// Internal forwarded ip header used when openstream servers are connecting with each other
#[const_register]
pub const FORWARD_IP_HEADER: &str = "x-openstream-forwarded-ip";

/// External ip header used for connecting from a reverse proxy
#[const_register]
pub const REAL_IP_HEADER: &str = "x-real-ip";

/// Access token header used by payments servers implementations
#[const_register]
pub const PAYMENTS_ACCESS_TOKEN_HEADER: &str = "x-access-token";

/// we need to update this value after making code changes to the station pictures logic or sizes
/// changing to this value will make startup check and recreation of outdated images
/// and invalidation of service workers station pictures caches
#[const_register]
pub const STATION_PICTURES_VERSION: f64 = 5.0;

#[const_register]
pub const DEPLOYMENT_HEALTH_CHECK_INTERVAL_SECS: u16 = 1;

#[const_register]
pub const DEPLOYMENT_HEALTH_CHECK_SHUTDOWN_INTERVAL_SECS: u16 = 30;

#[const_register]
pub const DEPLOYMENT_HEALTH_CHECK_SHUTDOWN_DELAY_SECS: u16 = 60 * 4; // 4 mins

/// interval in which
/// $stations.owner_deployment_info.health_checked_at
/// and $media_session.health_checked_at
/// will be set to $NOW
#[const_register]
pub const MEDIA_SESSION_HEALTH_CHECK_INTERVAL_SECS: u16 = 6;

/// time to check if a $media_session (and $station.owner_deployment_info) is healthy
/// otherwise kill it in database
#[const_register]
pub const MEDIA_SESSION_HEALTH_SHUTDOWN_TIMEOUT_SECS: u16 = 33;

/// interval to check if $stations.owner_deployment_info and $media_sessions are healthy
#[const_register]
pub const MEDIA_SESSION_HEALTH_CHECK_KILL_INTERVAL_SECS: u16 = 5;

/// internal interval to start new probe task if needed
#[const_register]
pub const PROBE_BACKGROUND_JOB_CHECK_INTERVAL_SECS: u32 = 10; // 10 secs

/// interval to run a probe request (multiplied by the number of stations)
#[const_register]
pub const PROBE_STATION_INTERVAL_SECS: u32 = 5 * 60; // 5 min

#[const_register]
pub const HEADER_RELAY_SOURCE_DEPLOYMENT: &str = "x-source-deployment";

#[const_register]
pub const MEDIA_RELAY_TIMEOUT_SECS: u64 = 35;

/// timeout to wait to obtain a lock on a media session items
/// if not released in this timeout, probably the item is poisoned
/// and the process is aborted with a panic (and restarted by the process manager)
#[const_register]
pub const MEDIA_LOCK_TIMEOUT_SECS: u64 = 30;

/// validation constants
pub mod validate {
  use super::*;

  // name
  #[const_register]
  pub const VALIDATE_STATION_NAME_MIN_LEN: usize = 1;

  #[const_register]
  pub const VALIDATE_STATION_NAME_MAX_LEN: usize = 60;

  // slogan
  #[const_register]
  pub const VALIDATE_STATION_SLOGAN_MIN_LEN: usize = 1;

  #[const_register]
  pub const VALIDATE_STATION_SLOGAN_MAX_LEN: usize = 100;

  // description
  #[const_register]
  pub const VALIDATE_STATION_DESC_MIN_LEN: usize = 1;

  #[const_register]
  pub const VALIDATE_STATION_DESC_MAX_LEN: usize = 4000;

  // email
  #[const_register]
  pub const VALIDATE_STATION_EMAIL_MAX_LEN: usize = 100;

  // phone
  #[const_register]
  pub const VALIDATE_STATION_PHONE_MAX_LEN: usize = 60;

  // whatsapp
  #[const_register]
  pub const VALIDATE_STATION_WHATSAPP_MAX_LEN: usize = 60;

  // urls
  #[const_register]
  pub const VALIDATE_STATION_URLS_MAX_LEN: usize = 150;

  // external relay url
  #[const_register]
  pub const VALIDATE_STATION_EXTERNAL_RELAY_URL_MAX_LEN: usize = 200;

  #[const_register]
  pub const VALIDATE_STATION_FREQUENCY_MAX: f64 = 100_000.0;

  #[const_register]
  pub const VALIDATE_STATION_FREQUENCY_MIN: f64 = 0.0;

  #[const_register]
  pub const VALIDATE_ACCOUNT_NAME_MIN_LEN: usize = 1;

  #[const_register]
  pub const VALIDATE_ACCOUNT_NAME_MAX_LEN: usize = 60;

  #[const_register]
  pub const VALIDATE_USER_EMAIL_MAX_LEN: usize = 80;

  #[const_register]
  pub const VALIDATE_USER_FIRST_NAME_MAX_LEN: usize = 100;

  #[const_register]
  pub const VALIDATE_USER_LAST_NAME_MAX_LEN: usize = 100;

  #[const_register]
  pub const VALIDATE_USER_PASSWORD_MIN_LEN: usize = 8;

  #[const_register]
  pub const VALIDATE_USER_PASSWORD_MAX_LEN: usize = 60;

  #[const_register]
  pub const VALIDATE_USER_PHONE_MAX_LEN: usize = 40;

  #[const_register]
  pub const VALIDATE_ADMIN_FIRST_NAME_MAX_LEN: usize = 100;

  #[const_register]
  pub const VALIDATE_ADMIN_LAST_NAME_MAX_LEN: usize = 100;

  #[const_register]
  pub const VALIDATE_ADMIN_PASSWORD_MIN_LEN: usize = 8;

  #[const_register]
  pub const VALIDATE_ADMIN_PASSWORD_MAX_LEN: usize = 60;
}

#[cfg(test)]
pub mod test {
  use std::path::Path;

  #[test]
  fn export_constants() {
    let target = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../defs/constants.ts");
    macros::ConstRegistry::global()
      .export_to_file(target)
      .expect("Error exporting constants");
  }
}
