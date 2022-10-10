use std::collections::HashMap;
use std::collections::hash_map::Entry; 
use std::sync::atomic::{Ordering, AtomicUsize};
use parking_lot::RwLock;
use static_init::dynamic;
use tokio::sync::broadcast::{self, channel};
use bytes::Bytes;
use heapless::Deque;

mod receiver;
mod transmitter;
pub use receiver::Receiver;
pub use transmitter::Transmitter;

/** 
 * Size of the burst in items
 * burst size in bytes will be <= (BURST_LEN * CHUNK_SIZE)
 */
pub const BURST_LEN: usize = 8;
pub const CHANNEL_CAPACITY: usize = 8;

#[dynamic]
pub(crate) static CHANNELS: RwLock<HashMap<String, Channel>> = RwLock::new(HashMap::new());

pub(crate) static SUBSCRIBER_COUNT: AtomicUsize = AtomicUsize::new(0);

pub struct Channel {
    #[allow(unused)]
    id: String,
    burst: Deque<Bytes, BURST_LEN>,
    sender: broadcast::Sender<Bytes>,
}

pub fn transmit(id: String) -> Option<Transmitter> {
    
    let (tx, count) = {
        
        let mut map = CHANNELS.write();
        
        match map.entry(id.clone()) {
            
            Entry::Occupied(_) => return None,
            
            Entry::Vacant(entry) => {
                let (sender, _) = channel(CHANNEL_CAPACITY);
                let channel = Channel {
                    id: id.clone(),
                    sender: sender.clone(),
                    burst: Deque::new()
                };

                entry.insert(channel);

                let tx = Transmitter {
                    id: id.clone(),
                    sender
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
            receiver: channel.sender.subscribe()
        };

        rx
    };

    let count = SUBSCRIBER_COUNT.fetch_add(1, Ordering::SeqCst) + 1;

    println!("[channels] subscriber created for channel {} => {} subscribers", rx.channel_id, count);

    Some(rx)
}