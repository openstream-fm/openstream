use burst::Burst;
use bytes::Bytes;
use log::*;
use tokio::sync::broadcast::{self, error::RecvError as TokioRecvError};

use crate::Kind;

#[derive(Debug, thiserror::Error)]
pub enum RecvError {
  /// All senders of this channel were dropped and the burst is empty
  #[error("The channel is closed")]
  Closed,
  #[error("The channel is lagged")]
  Lagged(u64),
}

impl From<TokioRecvError> for RecvError {
  fn from(e: TokioRecvError) -> Self {
    match e {
      TokioRecvError::Closed => Self::Closed,
      TokioRecvError::Lagged(n) => Self::Lagged(n),
    }
  }
}

#[derive(Debug)]
pub struct Receiver {
  #[allow(unused)]
  pub(crate) station_id: String,
  pub(crate) content_type: String,
  pub(crate) kind: Kind,
  pub(crate) receiver: broadcast::Receiver<Bytes>,
  /// this is an owned copy of the burst at subscription time (Bytes instances are copied by reference)
  pub(crate) burst: Burst,
}

impl Receiver {
  pub fn content_type(&self) -> &str {
    &self.content_type
  }

  pub fn kind(&self) -> Kind {
    self.kind
  }
  /**
   * Receive the next Bytes value
   * first the internal burst of the channel will be drained
   * and then the broadcasting channel will be called to get new values
   */
  pub async fn recv(&mut self) -> Result<Bytes, RecvError> {
    match self.burst.pop_front() {
      Some(bytes) => Ok(bytes),
      None => {
        let bytes = self.receiver.recv().await?;
        Ok(bytes)
      }
    }
  }

  pub async fn resubscribe(&mut self) {
    self.receiver = self.receiver.resubscribe();
  }
}
