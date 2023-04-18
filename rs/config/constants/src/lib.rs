use std::time::Duration;

/// stream kbps
pub const STREAM_KBITRATE: usize = 128;

/// stream chunk size
pub const STREAM_CHUNK_SIZE: usize = STREAM_KBITRATE * 1000 / 8;

/// stream bust len in elements (use with STREAM_CHUNK_SIZE)
pub const STREAM_BURST_LENGTH: usize = 8;

/// stream tokio broadcaster stream channel capacity
pub const STREAM_CHANNEL_CAPACITY: usize = 5;

/// audio file chunk size in bytes
pub const AUDIO_FILE_CHUNK_SIZE: usize = 256 * 1000;

/// audio file byte rate
pub const AUDIO_FILE_BYTERATE: usize = 128_000 / 8; // 128 kbps

/// station's transfer save interval in milliseconds
pub const TRANSFER_SAVE_INTERVAL_MILLIS: u64 = 5_000;

/// limit of concurrent stream connections from the same ip
pub const STREAM_IP_CONNECTIONS_LIMIT: u64 = 8;

/// delay to shutdown a playlist media session when it run out of listeners
pub const PLAYLIST_NO_LISTENERS_SHUTDOWN_DELAY: Duration = Duration::from_secs(10);

/// limit of authotization intents from the same ip
pub const API_IP_LIMIT: usize = 60;

/// limit restart interval of authorization intents from the same ip (use with API_IP_LIMIT)
pub const API_IP_LIMIT_DURATION: Duration = Duration::from_secs(60);
