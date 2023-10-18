use burst::Burst;
use bytes::Bytes;
use parking_lot::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::broadcast::{self, error::SendError as TokioSendError};

use crate::Info;

use super::Receiver;

#[derive(Debug, thiserror::Error)]
pub enum SendError {
  /// The channel is terminated terminate() was called on the channel Handle
  #[error("The channel is terminated")]
  Terminated(Bytes),
  /// This error doesn't mean that the channel is closed,
  /// the channel can have subscribers in the future by calling subscribe()
  #[error("The channel has no subscribers")]
  NoSubscribers(Bytes),
}

impl From<TokioSendError<Bytes>> for SendError {
  fn from(e: TokioSendError<Bytes>) -> Self {
    Self::NoSubscribers(e.0)
  }
}

#[derive(Debug, Clone)]
pub struct Sender {
  pub(crate) station_id: String,
  pub(crate) info: Info,
  pub(crate) terminated: Arc<AtomicBool>,
  pub(crate) sender: broadcast::Sender<Bytes>,
  pub(crate) burst: Arc<RwLock<Burst>>,
}

impl Sender {
  pub fn new(station_id: String, info: Info) -> Self {
    let terminated = Arc::new(AtomicBool::new(false));
    let (sender, _) = broadcast::channel(constants::STREAM_CHANNEL_CAPACITY);
    let burst = Arc::new(RwLock::new(Burst::new()));
    Self {
      station_id,
      info,
      terminated,
      sender,
      burst,
    }
  }

  pub fn send(&self, bytes: Bytes) -> Result<usize, SendError> {
    if self.terminated.load(Ordering::SeqCst) {
      return Err(SendError::Terminated(bytes));
    }

    {
      let mut burst = self.burst.write();
      if burst.is_full() {
        burst.pop_front();
      }

      // since we just removed one item, this will never fail
      burst.push_back(bytes.clone()).unwrap();
    }

    let n = self.sender.send(bytes)?;

    Ok(n)
  }

  pub fn receiver_count(&self) -> usize {
    self.sender.receiver_count()
  }

  pub fn subscribe(&self) -> Receiver {
    Receiver {
      station_id: self.station_id.clone(),
      kind: self.info.kind,
      content_type: self.info.content_type.clone(),
      receiver: self.sender.subscribe(),
      burst: self.burst.read().clone(),
    }
  }
}
