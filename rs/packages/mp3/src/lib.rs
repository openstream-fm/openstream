use bytes::{Bytes, BytesMut};
use futures_util::{ready, Stream};
use tokio::io::AsyncRead;
use tokio::io::ReadBuf;

use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

// #[cfg(not(no_minimp3))]
use minimp3::Frame;

use log::*;

#[pin_project::pin_project]
pub struct TryStreamAsyncRead<S> {
  #[pin]
  stream: S,
  buffer: BytesMut,
}

impl<E: std::error::Error + Send + Sync + 'static, S: Stream<Item = Result<Bytes, E>>>
  TryStreamAsyncRead<S>
{
  pub fn new(stream: S) -> Self {
    Self {
      stream,
      buffer: BytesMut::new(),
    }
  }

  pub fn into_inner(self) -> (S, BytesMut) {
    (self.stream, self.buffer)
  }
}

impl<E: std::error::Error + Send + Sync + 'static, S: Stream<Item = Result<Bytes, E>>> AsyncRead
  for TryStreamAsyncRead<S>
{
  fn poll_read(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    buf: &mut ReadBuf<'_>,
  ) -> Poll<std::io::Result<()>> {
    let mut project = self.project();
    loop {
      if !buf.remaining() == 0 {
        return Poll::Ready(Err(std::io::ErrorKind::WriteZero.into()));
      }

      if !project.buffer.is_empty() {
        let len = usize::min(project.buffer.len(), buf.remaining());
        let bytes = project.buffer.split_to(len).freeze();
        buf.put_slice(bytes.as_ref());
        return Poll::Ready(Ok(()));
      } else {
        match ready!(project.stream.as_mut().poll_next(cx)) {
          None => return Poll::Ready(Ok(())),
          Some(Err(e)) => {
            return Poll::Ready(Err(std::io::Error::new(std::io::ErrorKind::Other, e)))
          }
          Some(Ok(bytes)) => {
            project.buffer.extend_from_slice(bytes.as_ref());
            continue;
          }
        }
      }
    }
  }
}

#[derive(Debug, thiserror::Error)]
pub enum ReadRateError {
  #[error("io: {0}")]
  Io(#[from] std::io::Error),
  #[error("no data timer elapsed")]
  NoData,
}

// #[cfg(not(no_minimp3))]
const NO_DATA_ERROR_DELAY: tokio::time::Duration = tokio::time::Duration::from_secs(20);

/// An adapter that lets you inspect the data that's being read.
///
/// This is useful for things like hashing data as it's read in.
#[pin_project::pin_project]
pub struct InspectBufferReader<R> {
  #[pin]
  reader: R,
  buffer: BytesMut,
}

impl<R> InspectBufferReader<R> {
  /// Create a new InspectReader, wrapping `reader` and calling `f` for the
  /// new data supplied by each read call.
  ///
  /// The closure will only be called with an empty slice if the inner reader
  /// returns without reading data into the buffer. This happens at EOF, or if
  /// `poll_read` is called with a zero-size buffer.
  pub fn new(reader: R) -> InspectBufferReader<R>
  where
    R: AsyncRead,
  {
    InspectBufferReader {
      reader,
      buffer: BytesMut::new(),
    }
  }

  /// Consumes the `InspectReader`, returning the wrapped reader
  pub fn into_inner(self) -> (R, BytesMut) {
    (self.reader, self.buffer)
  }

  pub fn take_buffer(&mut self) -> BytesMut {
    self.buffer.split_to(self.buffer.len())
  }
}

impl<R: AsyncRead> AsyncRead for InspectBufferReader<R> {
  fn poll_read(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    buf: &mut ReadBuf<'_>,
  ) -> Poll<std::io::Result<()>> {
    let me = self.project();
    let filled_length = buf.filled().len();
    ready!(me.reader.poll_read(cx, buf))?;
    me.buffer.extend_from_slice(&buf.filled()[filled_length..]);
    Poll::Ready(Ok(()))
  }
}

// #[cfg(not(no_minimp3))]
pub fn readrate<R: AsyncRead + Send + Sync + Unpin + 'static>(
  reader: R,
) -> impl Stream<Item = Result<Bytes, ReadRateError>> {
  let reader = InspectBufferReader::new(reader);

  async_stream::stream! {
    let start = tokio::time::Instant::now();
    let duration = tokio::time::Duration::ZERO;
    let mut error_instant = None;

    let mut decoder = minimp3::Decoder::new(reader);

    loop {

      let Frame { bitrate, sample_rate, channels, layer: _, data } = match decoder.next_frame_future().await {

        Ok(frame) => frame,

        Err(e) => {

          warn!("decoder frame error: {e} => {e:?}");

          use minimp3::Error::*;

          match error_instant {
            None => {
              let _ = error_instant.insert(tokio::time::Instant::now());
            }

            Some(instant) => {
              if instant.elapsed() > NO_DATA_ERROR_DELAY {
                yield Err(ReadRateError::NoData);
                break;
              }
            }
          };

          match e {
            InsufficientData => continue,
            SkippedData => continue,
            Eof => break,
            Io(e) => {
              yield Err(e.into());
              break;
            },
          }
        }
      };

      let _ = error_instant.take();

      // Vec<i16>

      let bytes = decoder.reader_mut().take_buffer().freeze();

      let samples = data.len() / channels;
      let duration_secs = samples as f64 / sample_rate as f64;

      let decoded_len = data.len() * 2;
      let transfer_len = bytes.len();

      let ms = (tokio::time::Instant::now() - start + duration).as_millis();

      info!("frame: decoded-len={decoded_len}, transfer_len={transfer_len}, samples={samples}, sample_rate={sample_rate}, kbitrate={bitrate} duration={duration_secs}s ms_until={ms}");
      //duration += tokio::time::Duration::from_secs_f64(duration_secs);

      let duration = tokio::time::Duration::from_secs_f64(duration_secs);

      tokio::time::sleep(duration).await;

      yield Ok(bytes);
    }
  }
}

// #[cfg(no_minimp3)]
// pub fn readrate<R: AsyncRead + Send + Sync + Unpin + 'static>(
//   reader: R,
// ) -> impl Stream<Item = Result<Bytes, ReadRateError>> {
//   let mut reader = InspectBufferReader::new(reader);

//   async_stream::stream! {
//     // let start = tokio::time::Instant::now();
//     // let mut duration = tokio::time::Duration::ZERO;
//     // let mut error_instant = None;

//     // let mut decoder = minimp3::Decoder::new(reader);
//     // let mut decoder = tokio_puremp3::ReadRate(reader);

//     loop {

//       let _header = match tokio_puremp3::next_frame(&mut reader).await {
//         Ok(header) => header,
//         Err(e) => {
//           match e {
//             tokio_puremp3::Error::IoError(e) => match e.kind() {
//               std::io::ErrorKind::UnexpectedEof => break,
//               _ => {
//                 yield Err(e.into());
//                 break;
//               }
//             }

//             _ => {
//               yield Err(ReadRateError::NoData);
//               break;
//             }
//           }
//         }
//       };

//       // Vec<i16>

//       let bytes = reader.take_buffer().freeze();

//       // let samples = data.len() / channels;
//       // let duration_secs = samples as f64 / sample_rate as f64;

//       // let decoded_len = data.len() * 2;
//       // let transfer_len = bytes.len();

//       // let sample_rate = header.sample_rate;
//       // let bitrate = header.bitrate;
//       // let duration = header.

//       // trace!("frame: transfer_len={transfer_len}, samples={samples}, sample_rate={sample_rate}, kbitrate={bitrate} duration={duration_secs}s");
//       // duration += tokio::time::Duration::from_secs_f64(duration_secs);

//       yield Ok(bytes);

//       // tokio::time::sleep_until(start + duration).await;
//     }
//   }
// }
