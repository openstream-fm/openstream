use bytes::Bytes;
use heapless::Deque;
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

use cond_count::CondCount;

use constants::{STREAM_BURST_LENGTH, STREAM_CHANNNEL_CAPACITY};

pub type Burst = Deque<Bytes, STREAM_BURST_LENGTH>;

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
  pub fn new() -> Self {
    Self {
      inner: Arc::new(Inner {
        map: RwLock::new(HashMap::new()),
        rx_count: AtomicUsize::new(0),
        condcount: CondCount::new(),
      }),
    }
  }
}

#[derive(Debug)]
pub struct Inner {
  pub(crate) map: RwLock<HashMap<String, Channel>>,
  // we dont need tx_count as is the same as map.len()
  pub(crate) rx_count: AtomicUsize,
  pub(crate) condcount: CondCount,
}

impl ChannelMap {
  pub fn transmit(&self, id: String) -> Option<Transmitter> {
    let (tx, count) = {
      let mut map = self.inner.map.write();

      match map.entry(id.clone()) {
        Entry::Occupied(_) => return None,

        Entry::Vacant(entry) => {
          let (sender, _) = channel(STREAM_CHANNNEL_CAPACITY);

          let burst = Arc::new(RwLock::new(Burst::new()));

          let channel = Channel {
            id: id.clone(),
            sender: sender.clone(),
            burst: burst.clone(),
          };

          entry.insert(channel);

          let tx = Transmitter {
            id: id.clone(),
            sender,
            channels: self.clone(),
            burst,
            token: self.inner.condcount.token(),
          };

          (tx, map.len())
        }
      }
    };

    debug!("[channels] transitter created for channel {id} => {count} transmitters");

    Some(tx)
  }

  pub fn subscribe(&self, id: &str) -> Option<Receiver> {
    let rx = {
      let map = self.inner.map.read();

      let channel = map.get(id)?;

      let rx = Receiver {
        channel_id: id.to_string(),
        // this will make a snapshot of the burst at subscription time (not clone the Arc<RwLock<>>)
        burst: channel.burst.read().clone(),
        receiver: channel.sender.subscribe(),
        channels: self.clone(),
        token: self.inner.condcount.token(),
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
