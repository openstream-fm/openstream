use bytes::{Bytes, BytesMut};
use pin_project::pin_project;
use std::task::Poll;
use tokio::io::{AsyncRead, ReadBuf};
use tokio_stream::Stream;

pub trait IntoTryBytesStream<R> {
  fn into_bytes_stream(self, chunk_size: usize) -> TryBytesStream<R>;
}

impl<R: AsyncRead> IntoTryBytesStream<R> for R {
  fn into_bytes_stream(self, chunk_size: usize) -> TryBytesStream<R> {
    TryBytesStream::from(self, chunk_size)
  }
}

#[derive(Debug, Clone, Copy)]
enum State {
  Open,
  Closed,
}

#[pin_project]
pub struct TryBytesStream<R> {
  state: State,
  buf: BytesMut,
  /// size for each Bytes item in the stream
  /// expect for the last one that may have less
  chunk_size: usize,

  #[pin]
  inner: R,
}

impl<R: AsyncRead> TryBytesStream<R> {
  pub fn from(inner: R, chunk_size: usize) -> Self {
    assert!(chunk_size != 0, "chunk_size cannot be 0");

    Self {
      state: State::Open,
      buf: BytesMut::new(),
      chunk_size,
      inner,
    }
  }

  pub fn chunk_size(&self) -> usize {
    self.chunk_size
  }

  pub fn into_inner(self) -> R {
    self.inner
  }
}

impl<R: AsyncRead> Stream for TryBytesStream<R> {
  type Item = Result<Bytes, std::io::Error>;

  fn poll_next(
    self: std::pin::Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Option<Self::Item>> {
    let mut this = self.project();

    'outer: loop {
      if this.buf.len() >= *this.chunk_size {
        let buf = this.buf.split_to(*this.chunk_size);
        let bytes = buf.freeze();
        return Poll::Ready(Some(Ok(bytes)));
      }

      if matches!(*this.state, State::Closed) {
        if this.buf.is_empty() {
          return Poll::Ready(None);
        } else {
          let buf = this.buf.split();
          let bytes = buf.freeze();
          return Poll::Ready(Some(Ok(bytes)));
        }
      }

      let mut slice = [0; 8 * 1024];
      let mut read_buf = ReadBuf::new(&mut slice);

      match this.inner.as_mut().poll_read(cx, &mut read_buf) {
        Poll::Pending => return Poll::Pending,

        Poll::Ready(Err(e)) => {
          *this.state = State::Closed;
          return Poll::Ready(Some(Err(e)));
        }

        Poll::Ready(Ok(())) => {
          let filled = read_buf.filled();
          if filled.is_empty() {
            *this.state = State::Closed;
            continue 'outer;
          } else {
            this.buf.extend_from_slice(filled);
            continue 'outer;
          }
        }
      }
    }
  }
}

#[cfg(test)]
mod test {

  use std::io::Cursor;
  use tokio_stream::StreamExt;

  use super::*;

  #[test_util::async_test]
  async fn into_bytes_stream() {
    // (chunks_size, full_chunks, remaining)
    let values = [
      (1, 1, 0),
      (200, 2, 0),
      (256, 5, 0),
      (2000, 11, 5),
      (392, 28, 25),
      (1052, 105, 104),
    ];

    for (chunk_size, full_chunks, remaining) in values.into_iter() {
      let buf = vec![0u8; chunk_size * full_chunks + remaining];
      let reader = Cursor::new(buf);
      let stream = reader.into_bytes_stream(chunk_size).map(Result::unwrap);
      let result: Vec<Bytes> = stream.collect().await;

      let expected = {
        let mut vec = vec![Bytes::from(vec![0u8; chunk_size]); full_chunks];
        if remaining != 0 {
          vec.push(Bytes::from(vec![0u8; remaining]));
        }
        vec
      };

      assert_eq!(result, expected);
    }
  }
}
