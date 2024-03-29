use bytes::Bytes;
use constants::STREAM_CHANNEL_CAPACITY;
use log::*;
use parking_lot::RwLock;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::broadcast::{self, channel};

mod receiver;
mod transmitter;
pub use receiver::Receiver;
pub use transmitter::Transmitter;

use drop_tracer::{DropTracer, Token};

use burst::Burst;

#[derive(Debug)]
pub struct Channel {
  #[allow(unused)]
  id: String,
  burst: Arc<RwLock<Burst>>,
  sender: broadcast::Sender<Bytes>,
}

#[derive(Debug, Clone)]
pub struct ChannelMap {
  pub(crate) inner: Arc<Inner>,
}

impl ChannelMap {
  pub fn new(drop_tracer: DropTracer) -> Self {
    Self {
      inner: Arc::new(Inner {
        map: RwLock::new(HashMap::new()),
        rx_count: AtomicUsize::new(0),
        drop_tracer,
      }),
    }
  }

  pub fn drop_token(&self) -> Token {
    self.inner.drop_tracer.token()
  }
}

#[derive(Debug)]
pub struct Inner {
  pub(crate) map: RwLock<HashMap<String, Channel>>,
  // we dont need tx_count as is the same as map.len()
  pub(crate) rx_count: AtomicUsize,
  pub(crate) drop_tracer: DropTracer,
}

impl ChannelMap {
  pub fn transmit(&self, id: &str) -> Option<Transmitter> {
    let mut map = self.inner.map.write();

    let (tx, count) = {
      match map.entry(id.to_string()) {
        Entry::Occupied(_) => return None,

        Entry::Vacant(entry) => {
          let (sender, _) = channel(STREAM_CHANNEL_CAPACITY);

          let burst = Arc::new(RwLock::new(Burst::new()));

          let channel = Channel {
            id: id.to_string(),
            sender: sender.clone(),
            burst: burst.clone(),
          };

          entry.insert(channel);

          let tx = Transmitter {
            id: id.to_string(),
            sender,
            channels: self.clone(),
            burst,
            // token: self.inner.drop_tracer.token(),
          };

          (tx, map.len())
        }
      }
    };

    debug!("[channels] transitter created for channel {id} => {count} transmitters");

    Some(tx)
  }

  pub fn subscribe(&self, id: &str) -> Option<Receiver> {
    let map = self.inner.map.read();
    let rx = {
      //let map = self.inner.map.read();
      let channel = map.get(id)?;

      let rx = Receiver {
        channel_id: id.to_string(),
        // this will make a snapshot of the burst at subscription time (not clone the Arc<RwLock<>>)
        burst: channel.burst.read().clone(),
        receiver: channel.sender.subscribe(),
        channels: self.clone(),
        token: self.inner.drop_tracer.token(),
      };

      rx
    };

    let count = self.inner.rx_count.fetch_add(1, Ordering::SeqCst) + 1;

    debug!(
      "[channels] subscriber created for channel {} => {} subscribers",
      rx.channel_id, count
    );

    Some(rx)
  }
}
