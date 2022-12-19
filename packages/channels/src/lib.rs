use bytes::Bytes;
use heapless::Deque;
use log::*;
use parking_lot::{RwLock, RwLockUpgradableReadGuard};
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
        drop_tracer: DropTracer::new(),
      }),
    }
  }

  pub fn drop_token(&self) -> Token {
    self.inner.drop_tracer.token()
  }
}

impl Default for ChannelMap {
  fn default() -> Self {
    Self::new()
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
  fn transmit_locked(&self, id: &str, map: &mut HashMap<String, Channel>) -> Option<Transmitter> {
    let (tx, count) = {
      match map.entry(id.to_string()) {
        Entry::Occupied(_) => return None,

        Entry::Vacant(entry) => {
          let (sender, _) = channel(STREAM_CHANNNEL_CAPACITY);

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
            token: self.inner.drop_tracer.token(),
          };

          (tx, map.len())
        }
      }
    };

    debug!("[channels] transitter created for channel {id} => {count} transmitters");

    Some(tx)
  }

  pub fn transmit(&self, id: &str) -> Option<Transmitter> {
    let mut map = self.inner.map.write();
    self.transmit_locked(id, &mut map)
  }

  fn subscribe_locked(&self, id: &str, map: &HashMap<String, Channel>) -> Option<Receiver> {
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

  pub fn subscribe(&self, id: &str) -> Option<Receiver> {
    let map = self.inner.map.read();
    self.subscribe_locked(id, &map)
  }

  pub fn subscribe_linked_or_transmit(&self, id: &str, other: &Self) -> RxTx {
    let map = self.inner.map.upgradable_read();
    match self.subscribe_locked(id, &map) {
      Some(rx) => RxTx::Rx(rx),

      None => {
        let other_map = other.inner.map.read();
        let mut map = RwLockUpgradableReadGuard::upgrade(map);
        // unwrap: this should never fail because we have a lock and subscribe_locked just returned None
        let tx = self.transmit_locked(id, &mut map).unwrap();
        let rx = Receiver {
          channel_id: id.to_string(),
          burst: Burst::new(),
          receiver: tx.sender.subscribe(),
          channels: self.clone(),
          token: self.inner.drop_tracer.token(),
        };

        match other.subscribe_locked(id, &other_map) {
          None => RxTx::Tx(rx, tx),

          Some(mut other_rx) => {
            let rx = Receiver {
              channel_id: id.to_string(),
              burst: Burst::new(),
              receiver: tx.sender.subscribe(),
              channels: self.clone(),
              token: self.inner.drop_tracer.token(),
            };

            tokio::spawn(async move {
              loop {
                match other_rx.recv().await {
                  Err(_e) => break,
                  Ok(bytes) => match tx.send(bytes) {
                    Err(_e) => break,
                    Ok(_) => continue,
                  },
                };
              }
            });

            RxTx::Rx(rx)
          }
        }
      }
    }
  }
}

pub enum RxTx {
  Rx(Receiver),
  Tx(Receiver, Transmitter),
}
