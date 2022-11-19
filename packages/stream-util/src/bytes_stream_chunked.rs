use bytes::{Bytes, BytesMut};
use pin_project::pin_project;
use std::task::Poll;
use tokio_stream::Stream;

#[pin_project]
pub struct BytesStreamChunked<S> {
  chunk_size: usize,
  buf: BytesMut,
  done: bool,
  #[pin]
  inner: S,
}

impl<S, B> Stream for BytesStreamChunked<S>
where
  S: Stream<Item = B>,
  B: AsRef<[u8]>,
{
  type Item = Bytes;

  fn poll_next(
    self: std::pin::Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Option<Self::Item>> {
    let mut this = self.project();

    'outer: loop {
      if this.buf.len() >= *this.chunk_size {
        let buf = this.buf.split_to(*this.chunk_size);
        let bytes = buf.freeze();
        return Poll::Ready(Some(bytes));
      }

      if *this.done && !this.buf.is_empty() {
        let buf = this.buf.split();
        let bytes = buf.freeze();
        return Poll::Ready(Some(bytes));
      }

      match this.inner.as_mut().poll_next(cx) {
        Poll::Pending => return Poll::Pending,
        Poll::Ready(None) => {
          *this.done = true;
          continue 'outer;
        }
        Poll::Ready(Some(bytes)) => {
          this.buf.extend_from_slice(bytes.as_ref());
          continue 'outer;
        }
      }
    }
  }
}

impl<B: AsRef<[u8]>, E, S: Stream<Item = Result<B, E>>> TryBytesStreamChunked<S> {
  pub fn from(stream: S, chunk_size: usize) -> Self {
    Self {
      inner: stream,
      done: false,
      chunk_size,
      buf: BytesMut::new(),
    }
  }

  pub fn chunk_size(&self) -> usize {
    self.chunk_size
  }

  pub fn into_inner(self) -> S {
    self.inner
  }
}

pub trait IntoBytesStreamChunked<S> {
  fn chunked(self, chunk_size: usize) -> BytesStreamChunked<S>;
}

impl<B, S> IntoBytesStreamChunked<S> for S
where
  S: Stream<Item = B>,
  B: AsRef<[u8]>,
{
  fn chunked(self, chunk_size: usize) -> BytesStreamChunked<S> {
    BytesStreamChunked::from(self, chunk_size)
  }
}

#[pin_project]
pub struct TryBytesStreamChunked<S> {
  chunk_size: usize,
  buf: BytesMut,
  done: bool,
  #[pin]
  inner: S,
}

impl<S, B, E> Stream for TryBytesStreamChunked<S>
where
  S: Stream<Item = Result<B, E>>,
  B: AsRef<[u8]>,
{
  type Item = Result<Bytes, E>;

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

      if *this.done && !this.buf.is_empty() {
        let buf = this.buf.split();
        let bytes = buf.freeze();
        return Poll::Ready(Some(Ok(bytes)));
      }

      match this.inner.as_mut().poll_next(cx) {
        Poll::Pending => return Poll::Pending,

        Poll::Ready(None) => {
          *this.done = true;
          continue 'outer;
        }

        Poll::Ready(Some(Err(e))) => {
          return Poll::Ready(Some(Err(e)));
        }

        Poll::Ready(Some(Ok(bytes))) => {
          this.buf.extend_from_slice(bytes.as_ref());
          continue 'outer;
        }
      }
    }
  }
}

impl<B: AsRef<[u8]>, S: Stream<Item = B>> BytesStreamChunked<S> {
  pub fn from(stream: S, chunk_size: usize) -> Self {
    Self {
      inner: stream,
      done: false,
      chunk_size,
      buf: BytesMut::new(),
    }
  }

  pub fn chunk_size(&self) -> usize {
    self.chunk_size
  }

  pub fn into_inner(self) -> S {
    self.inner
  }
}

pub trait IntoTryBytesStreamChunked<S> {
  fn chunked(self, chunk_size: usize) -> TryBytesStreamChunked<S>;
}

impl<B, S, E> IntoTryBytesStreamChunked<S> for S
where
  S: Stream<Item = Result<B, E>>,
  B: AsRef<[u8]>,
{
  fn chunked(self, chunk_size: usize) -> TryBytesStreamChunked<S> {
    TryBytesStreamChunked::from(self, chunk_size)
  }
}
