use std::time::Duration;

pub const STREAM_KBITRATE: usize = 128;

pub const STREAM_CHUNK_SIZE: usize = STREAM_KBITRATE * 1000 / 8;

pub const STREAM_BURST_LENGTH: usize = 10;

pub const STREAM_CHANNEL_CAPACITY: usize = 8;

pub const AUDIO_FILE_CHUNK_SIZE: usize = 256 * 1000;

pub const AUDIO_FILE_BYTERATE: usize = 128_000 / 8; // 128 kbps

pub const TRANSFER_SAVE_INTERVAL_MILLIS: u64 = 5_000;

pub const STREAM_IP_CONNECTIONS_LIMIT: u64 = 8;

pub const PLAYLIST_NO_LISTENERS_SHUTDOWN_DELAY: Duration = Duration::from_secs(10);
