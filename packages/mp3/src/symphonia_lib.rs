use bytes::{Bytes, BytesMut};
use futures_util::{Stream, StreamExt};
use log::*;
use std::io::Read;
use std::pin::Pin;
use symphonia::core::io::MediaSourceStreamOptions;
use tokio::runtime::Handle;

const MIN_CHUNK_SIZE: usize = constants::STREAM_CHUNK_SIZE;

/// caution! this reader must be used in a dedicated blocking thread
pub struct TryBytesStreamReader<S> {
  stream: Pin<Box<S>>,
  buffer: BytesMut,
}

impl<E: std::error::Error, S: Stream<Item = Result<Bytes, E>> + Send + Sync>
  TryBytesStreamReader<S>
{
  pub fn new(stream: S) -> Self {
    Self {
      stream: Box::pin(stream),
      buffer: BytesMut::new(),
    }
  }

  pub fn into_inner(self) -> (Pin<Box<S>>, BytesMut) {
    (self.stream, self.buffer)
  }
}

impl<
    E: std::error::Error + Send + Sync + 'static,
    S: Stream<Item = Result<Bytes, E>> + Send + Sync,
  > Read for TryBytesStreamReader<S>
{
  fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    if buf.is_empty() {
      return Ok(0);
    }

    loop {
      if !self.buffer.is_empty() {
        if buf.len() <= self.buffer.len() {
          let len = buf.len();
          let bytes = self.buffer.split_to(len);
          buf.copy_from_slice(bytes.as_ref());
          return Ok(len);
        } else {
          let len = self.buffer.len();
          let bytes = self.buffer.split_to(len);
          for i in 0..len {
            buf[i] = bytes[i]
          }
          return Ok(len);
        }
      } else {
        let bytes = tokio::task::block_in_place(|| Handle::current().block_on(self.stream.next()));
        match bytes {
          None => return Ok(0),
          Some(Err(e)) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
          Some(Ok(bytes)) => {
            self.buffer.extend_from_slice(bytes.as_ref());
            continue;
          }
        }
      }
    }
  }
}

#[derive(Debug, thiserror::Error)]
pub enum ProbeError {
  #[error("probe error: track not supported => {0}")]
  NotSupported(#[source] symphonia::core::errors::Error),
  #[error("probe error: track not mp3")]
  NotMP3,
  #[error("probe error: no default track")]
  NoDefaultTrack,
}

#[derive(Debug, thiserror::Error)]
pub enum PlayError {
  #[error("play error: reset without new track")]
  ResetNoDefaultTrack,
  #[error("play error: reset track not mp3")]
  ResetTrackNotMP3,
  #[error("play error: reset: {0}")]
  Reset(#[source] symphonia::core::errors::Error),
  // #[error("play error: reset track not supported => {0}")]
  // ResetNotSupported(#[source] symphonia::core::errors::Error),
  #[error("play error: packet: {0}")]
  Packet(#[source] symphonia::core::errors::Error),
  #[error("play error: missing time base")]
  MissingTimeBase,
}

// caution! this must run in a dedicated blocking thread
pub async fn readrate<R: Read + Send + Sync + 'static>(
  read: R,
) -> Result<spsc::Receiver<Result<Bytes, PlayError>>, ProbeError> {
  use symphonia::core::formats::FormatOptions;
  use symphonia::core::io::{MediaSource, MediaSourceStream, ReadOnlySource};
  use symphonia::core::meta::MetadataOptions;
  use symphonia::core::probe::Hint;

  tokio::task::spawn_blocking(move || {
    let format_options = FormatOptions {
      enable_gapless: true,
      ..Default::default()
    };

    let mut hint = Hint::new();
    hint.with_extension("mp3");

    let metadata_options = MetadataOptions::default();

    let media_source_stream_options = MediaSourceStreamOptions::default();

    let source = ReadOnlySource::new(read);

    let media_source_stream = MediaSourceStream::new(
      Box::new(source) as Box<dyn MediaSource>,
      media_source_stream_options,
    );

    let rx = match symphonia::default::get_probe().format(
      &hint,
      media_source_stream,
      &format_options,
      &metadata_options,
    ) {
       
      Err(e) => return Err(ProbeError::NotSupported(e)),

      Ok(mut probed) => {
      
        let start = tokio::time::Instant::now();
        let mut duration = tokio::time::Duration::ZERO;

        let track = match probed.format.default_track() {
          Some(track) => track.clone(),
          None => return Err(ProbeError::NoDefaultTrack),
        };

        if track.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_MP3 {
          return Err(ProbeError::NotMP3);
        }

        let (tx, rx) = spsc::channel::<Result<Bytes, PlayError>>();
        
        tokio::task::spawn_blocking(move || {
          Handle::current().block_on(async  {
            
            let mut buf = BytesMut::new();
            let mut track = track;
          
            let result = loop {
              
              let packet = match probed.format.next_packet() {
                
                Ok(packet) => packet,
                
                Err(symphonia::core::errors::Error::ResetRequired) => {
                  track = match probed.format.default_track() {
                    Some(track) => track.clone(),
                    None => break Err(PlayError::ResetNoDefaultTrack),
                  };

                  if track.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_MP3 {
                    break Err(PlayError::ResetTrackNotMP3);
                  }

                  continue;
                }

                Err(e) => break Err(PlayError::Reset(e)),
              };
              
              if packet.track_id() != track.id {
                continue;
              }

              let time_base = match track.codec_params.time_base {
                None => break Err(PlayError::MissingTimeBase),
                Some(time_base) => time_base,
              };

              let time_base_secs = time_base.numer as f64 / time_base.denom as f64;

              let packet_duration_secs = packet.dur as f64 * time_base_secs;

              trace!(
                "packet read: len={} time_base_secs={} packet.dur={} packet.ts={} packet_duration_secs={}",
                packet.data.len(), time_base_secs, packet.dur, packet.ts, packet_duration_secs,
              );

              duration += tokio::time::Duration::from_secs_f64(packet_duration_secs);

              buf.extend_from_slice(&packet.data);

              let len = buf.len();
              if len >= MIN_CHUNK_SIZE {
                let chunk = buf.split_to(len).freeze();
                match tx.send(Ok(chunk)).await {
                  Ok(_) => {
                    let until = start + duration;
                    if log_enabled!(Level::Trace) {
                      let ms = (until - tokio::time::Instant::now()).as_millis();
                      trace!("tx: {} KB sent sleeping {} ms ({})ms", len as f64 / 1000.0, ms, duration.as_millis())
                    }
                    tokio::time::sleep_until(until).await;
                    continue;
                  }
                  Err(_) => break Ok(()),
                }
              }
            };

            if let Err(e) = result {
              let _ = tx.send(Err(e)).await;
            }
          });
        });

        rx
      }
    };

    Ok(rx)
  }).await.unwrap()
}
