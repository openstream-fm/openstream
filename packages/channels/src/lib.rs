use bytes::Bytes;
use heapless::Deque;
use parking_lot::RwLock;
use static_init::dynamic;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::broadcast::{self, channel};

mod receiver;
mod transmitter;
pub use receiver::Receiver;
pub use transmitter::Transmitter;

use constants::{STREAM_BURST_LENGTH, STREAM_CHANNNEL_CAPACITY};

#[dynamic]
pub(crate) static CHANNELS: RwLock<HashMap<String, Channel>> = RwLock::new(HashMap::new());

pub(crate) static SUBSCRIBER_COUNT: AtomicUsize = AtomicUsize::new(0);

pub struct Channel {
  #[allow(unused)]
  id: String,
  burst: Deque<Bytes, STREAM_BURST_LENGTH>,
  sender: broadcast::Sender<Bytes>,
}

pub fn transmit(id: String) -> Option<Transmitter> {
  let (tx, count) = {
    let mut map = CHANNELS.write();

    match map.entry(id.clone()) {
      Entry::Occupied(_) => return None,

      Entry::Vacant(entry) => {
        let (sender, _) = channel(STREAM_CHANNNEL_CAPACITY);
        let channel = Channel {
          id: id.clone(),
          sender: sender.clone(),
          burst: Deque::new(),
        };

        entry.insert(channel);

        let tx = Transmitter {
          id: id.clone(),
          sender,
        };

        (tx, map.len())
      }
    }
  };

  println!("[channels] transitter created for channel {id} => {count} transmitters");

  Some(tx)
}

pub fn subscribe(id: &str) -> Option<Receiver> {
  let rx = {
    let chans = CHANNELS.read();

    let channel = chans.get(id)?;

    let rx = Receiver {
      channel_id: id.to_string(),
      burst: channel.burst.clone(),
      receiver: channel.sender.subscribe(),
    };

    rx
  };

  let count = SUBSCRIBER_COUNT.fetch_add(1, Ordering::SeqCst) + 1;

  println!(
    "[channels] subscriber created for channel {} => {} subscribers",
    rx.channel_id, count
  );

  Some(rx)
}
