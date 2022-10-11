use bytes::Bytes;
use pin_project::pin_project;
use tokio::io::AsyncRead;
use tokio::time::Sleep;
use tokio::{
  self,
  time::{Duration, Instant},
};
use tokio_stream::Stream;

use std::future::Future;

use std::pin::Pin;
use std::task::{Context, Poll};

use crate::{IntoTryBytesStream, TryBytesStream};

#[pin_project]
pub struct BytesStreamRated<S, B> {
  // bytes per second
  rate: usize,
  start: Option<Instant>,
  bytes_readed: usize,

  next: Option<B>,

  #[pin]
  sleep: Sleep,

  #[pin]
  inner: S,
}

impl<S, B> BytesStreamRated<S, B> {
  pub fn from(stream: S, rate: usize) -> Self {
    Self {
      rate,
      start: None,
      bytes_readed: 0,
      next: None,
      inner: stream,
      sleep: tokio::time::sleep(Duration::from_micros(0)),
    }
  }

  pub fn into_inner(self) -> S {
    self.inner
  }
}

impl<S, B> Stream for BytesStreamRated<S, B>
where
  S: Stream<Item = B>,
  B: AsRef<[u8]>,
{
  type Item = B;

  fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    let mut this = self.project();

    let start = *this.start.get_or_insert_with(|| Instant::now());

    'outer: loop {
      match this.sleep.as_mut().poll(cx) {
        Poll::Pending => return Poll::Pending,
        Poll::Ready(()) => match this.next.take() {
          Some(buf) => return Poll::Ready(Some(buf)),
          None => match this.inner.as_mut().poll_next(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(None) => return Poll::Ready(None),
            Poll::Ready(Some(buf)) => {
              *this.bytes_readed += buf.as_ref().len();
              *this.next = Some(buf);
              let until =
                start + Duration::from_secs_f64(*this.bytes_readed as f64 / *this.rate as f64);
              this.sleep.as_mut().reset(until);
              continue 'outer;
            }
          },
        },
      }
    }
  }
}

#[pin_project]
pub struct TryBytesStreamRated<S, B> {
  /// bytes per second
  rate: usize,
  start: Option<Instant>,
  bytes_readed: usize,
  next: Option<B>,

  #[pin]
  sleep: Sleep,

  #[pin]
  inner: S,
}

impl<S, E, B> TryBytesStreamRated<S, B>
where
  S: Stream<Item = Result<B, E>>,
  B: AsRef<[u8]>,
{
  pub fn from(stream: S, rate: usize) -> Self {
    Self {
      rate,
      start: None,
      bytes_readed: 0,
      next: None,
      inner: stream,
      sleep: tokio::time::sleep(Duration::from_micros(0)),
    }
  }

  pub fn into_inner(self) -> S {
    self.inner
  }
}

impl<R: AsyncRead + Unpin> TryBytesStreamRated<TryBytesStream<R>, Bytes> {
  pub fn from_async_read(reader: R, chunk_size: usize, rate: usize) -> Self {
    Self::from(reader.into_bytes_stream(chunk_size), rate)
  }
}

impl<E, S, B> Stream for TryBytesStreamRated<S, B>
where
  S: Stream<Item = Result<B, E>>,
  B: AsRef<[u8]>,
{
  type Item = Result<B, E>;

  fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    let mut this = self.project();

    let start = *this.start.get_or_insert_with(|| Instant::now());

    'outer: loop {
      match this.sleep.as_mut().poll(cx) {
        Poll::Pending => return Poll::Pending,
        Poll::Ready(()) => match this.next.take() {
          Some(buf) => return Poll::Ready(Some(Ok(buf))),
          None => match this.inner.as_mut().poll_next(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(None) => return Poll::Ready(None),
            Poll::Ready(Some(Err(e))) => return Poll::Ready(Some(Err(e))),
            Poll::Ready(Some(Ok(buf))) => {
              *this.bytes_readed += buf.as_ref().len();
              *this.next = Some(buf);
              let until =
                start + Duration::from_secs_f64(*this.bytes_readed as f64 / *this.rate as f64);
              this.sleep.as_mut().reset(until);
              continue 'outer;
            }
          },
        },
      }
    }
  }
}

pub trait IntoTryBytesStreamRated<S, E, B> {
  fn rated(self, rate: usize) -> TryBytesStreamRated<S, B>;
}

impl<S, E, B> IntoTryBytesStreamRated<S, E, B> for S
where
  S: Stream<Item = Result<B, E>>,
  B: AsRef<[u8]>,
{
  fn rated(self, rate: usize) -> TryBytesStreamRated<S, B> {
    TryBytesStreamRated::from(self, rate)
  }
}

pub trait IntoBytesStreamRated<S, B> {
  fn rated(self, rate: usize) -> BytesStreamRated<S, B>;
}

impl<S, B> IntoBytesStreamRated<S, B> for S
where
  S: Stream<Item = B>,
  B: AsRef<[u8]>,
{
  fn rated(self, rate: usize) -> BytesStreamRated<S, B> {
    BytesStreamRated::from(self, rate)
  }
}

#[cfg(test)]
pub mod test {
  use super::*;
  use tokio_stream::StreamExt;

  #[tokio::test]
  async fn bytes_stream_rated() {
    let rate = 128 * 1000;
    let total: usize = 128 * 150;
    let stream = async_stream::stream! {
      for _ in 0..(total / 128) {
        yield Bytes::from(vec![0u8;128]);
      }
    };

    let start = Instant::now();
    let vec = stream.rated(rate).collect::<Vec<Bytes>>().await;
    let elapsed = start.elapsed().as_millis();
    let expected = (total * 1000 / rate) as u128;

    assert!(vec.len() == total / 128);

    assert!(elapsed <= expected + 5);
    assert!(elapsed >= expected - 5);
    eprintln!(
      "bytes_stream_rated, expected time elapsed {expected}ms, actual time elapsed: {elapsed}ms"
    )
  }
}
