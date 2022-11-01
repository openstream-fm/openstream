use bytes::Bytes;
use cond_count::Token;
use log::*;
use parking_lot::RwLock;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast::{self, error::SendError};

use crate::Burst;
use crate::ChannelMap;
#[derive(Debug)]
pub struct Transmitter {
  pub(crate) id: String,
  pub(crate) sender: broadcast::Sender<Bytes>,
  pub(crate) channels: ChannelMap,
  pub(crate) burst: Arc<RwLock<Burst>>,
  pub(crate) token: Token,
}

impl Transmitter {
  pub fn send(&self, bytes: Bytes) -> Result<usize, SendError<Bytes>> {
    {
      let mut burst = self.burst.write();
      if burst.is_full() {
        burst.pop_front();
      }

      // since we just removed one item, this will never fail
      burst.push_back(bytes.clone()).unwrap();
    }

    self.sender.send(bytes)
  }
}

impl Drop for Transmitter {
  fn drop(&mut self) {
    let (opt, len) = {
      let mut map = self.channels.inner.map.write();
      let opt = map.remove(&self.id).map(|_| ());
      (opt, map.len())
    };

    match opt {
      None => {
        warn!("[channels] transmitter dropped for channel {}, transmitter not found in channel map, {} open transmitters", self.id, len)
      }
      Some(()) => {
        debug!(
          "[channels] transmitter dropped for channel {} => {} transmitters",
          self.id, len
        )
      }
    }

    tokio::spawn({
      let token = self.token.clone();
      async move {
        tokio::time::sleep(Duration::from_millis(1_500)).await;
        drop(token);
      }
    });
  }
}
