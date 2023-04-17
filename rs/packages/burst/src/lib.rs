use bytes::Bytes;
use constants::STREAM_BURST_LENGTH;
use heapless::Deque;

pub type Burst = Deque<Bytes, STREAM_BURST_LENGTH>;

#[cfg(test)]
mod test {
  use super::*;
  use constants::STREAM_CHUNK_SIZE;

  #[test]
  fn size_of_burst() {
    eprintln!(
      "size of burst (heap) per station (max): {} KB",
      std::mem::size_of::<[[u8; STREAM_CHUNK_SIZE]; STREAM_BURST_LENGTH]>() / 1000
    );

    eprintln!(
      "size of burst (stack) per listener: {} bytes",
      std::mem::size_of::<Burst>()
    );
  }
}
