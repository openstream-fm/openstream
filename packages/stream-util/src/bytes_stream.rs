use tokio::io::{AsyncRead, ReadBuf};
use tokio_stream::Stream;
use bytes::{Bytes, BytesMut};
use std::task::Poll;
use pin_project::pin_project;

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
  /// chunk for each Bytes item in the stream
  /// the last one may have less size than chunk_size
  state: State,
  buf: BytesMut,
  chunk_size: usize,
  
  #[pin]
  inner: R,
}

impl<R: AsyncRead> TryBytesStream<R> {
  pub fn from(inner: R, chunk_size: usize) -> Self {
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

  fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {

    let mut this = self.project();

    'outer: loop {
    
      if this.buf.len() >= *this.chunk_size {
        let mut buf = this.buf.split_off(*this.chunk_size);
        std::mem::swap(&mut buf, &mut this.buf);
        let bytes = buf.freeze();
        return Poll::Ready(Some(Ok(bytes)));
      }

      if matches!(*this.state, State::Closed) {
        if this.buf.len() == 0 {
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
        },

        Poll::Ready(Ok(())) => {
          let filled = read_buf.filled();
          if filled.len() == 0 {
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