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

/// delay to shutdown a relay session when it run out of listeners
#[const_register]
pub const RELAY_NO_LISTENERS_SHUTDOWN_DELAY_SECS: u64 = 10;

/// delay of which if external relay produced no data, it will be cancelled
#[const_register]
pub const EXTERNAL_RELAY_NO_DATA_SHUTDOWN_SECS: u64 = 10;

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
