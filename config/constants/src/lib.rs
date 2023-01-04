pub const STREAM_KBITRATE: usize = 128;

pub const STREAM_CHUNK_SIZE: usize = STREAM_KBITRATE * 1000 / 8;

pub const STREAM_BURST_LENGTH: usize = 12;

pub const STREAM_CHANNEL_CAPACITY: usize = 8;

pub const AUDIO_FILE_CHUNK_SIZE: usize = 256 * 1000;

pub const AUDIO_FILE_BYTERATE: usize = 128_000 / 8; // 128 kbps
