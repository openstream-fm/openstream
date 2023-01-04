use bytes::Bytes;
use constants::STREAM_BURST_LENGTH;
use heapless::Deque;

pub type Burst = Deque<Bytes, STREAM_BURST_LENGTH>;
