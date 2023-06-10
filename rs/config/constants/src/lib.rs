use macros::const_register;

#[const_register]
/// stream kbps
pub const STREAM_KBITRATE: usize = 128;

#[const_register]
/// stream chunk size
pub const STREAM_CHUNK_SIZE: usize = STREAM_KBITRATE * 1000 / 8;

#[const_register]
/// stream bust len in elements (use with STREAM_CHUNK_SIZE)
pub const STREAM_BURST_LENGTH: usize = 8;

#[const_register]
/// stream tokio broadcaster stream channel capacity
pub const STREAM_CHANNEL_CAPACITY: usize = 5;

#[const_register]
/// audio file chunk size in bytes
pub const AUDIO_FILE_CHUNK_SIZE: usize = 256 * 1000;

#[const_register]
/// audio file byte rate
pub const AUDIO_FILE_BYTERATE: usize = 128_000 / 8; // 128 kbps

#[const_register]
/// station's transfer save interval in milliseconds
pub const TRANSFER_SAVE_INTERVAL_MILLIS: u64 = 5_000;

#[const_register]
/// limit of concurrent stream connections from the same ip
pub const STREAM_IP_CONNECTIONS_LIMIT: u64 = 8;

#[const_register]
/// delay to shutdown a playlist media session when it run out of listeners
pub const PLAYLIST_NO_LISTENERS_SHUTDOWN_DELAY_SECS: u64 = 10;

#[const_register]
/// delay to shutdown a relay session when it run out of listeners
pub const RELAY_NO_LISTENERS_SHUTDOWN_DELAY_SECS: u64 = 10;

#[const_register]
/// limit of authotization, or other sensible api endpoints requests from the same ip
pub const API_IP_LIMIT: usize = 60;

#[const_register]
/// limit restart interval of API_IP_LIMIT
pub const API_IP_LIMIT_DURATION_SECS: u64 = 60;

/// access token autoremove validity
/// remove access tokens that are not used in the last X time
/// this is only for Login, Register or AdminAsUser access tokens
/// Cli or Api(not AdminAsUser) access tokens does not auto-expire
#[const_register]
pub const ACCESS_TOKEN_NOT_USED_AUTOREMOVE_SECS: u64 = 60 * 60 * 24 * 7; // 7 days

#[const_register]
pub const ACCESS_TOKEN_HEADER: &str = "x-access-token";

#[const_register]
pub const FORWARD_IP_HEADER: &str = "x-openstream-forwarded-ip";

#[const_register]
pub const REAL_IP_HEADER: &str = "x-real-ip";

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
