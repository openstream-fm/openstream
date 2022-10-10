use super::CHANNELS;
use bytes::Bytes;
use tokio::sync::broadcast::{self, error::SendError};

#[derive(Debug)]
pub struct Transmitter {
    pub(crate) id: String,
    pub(crate) sender: broadcast::Sender<Bytes>
}

impl Transmitter {
    pub fn send(&self, bytes: Bytes) -> Result<usize, SendError<Bytes>> { 
        {
            if let Some(channel) = CHANNELS.write().get_mut(self.id.as_str()) {
                if channel.burst.is_full() {
                    channel.burst.pop_back();
                }
                // since we just removed one item, this will never fail
                channel.burst.push_front(bytes.clone()).expect("burst append");
            }
        }

        self.sender.send(bytes)
    }
}

impl Drop for Transmitter {
    fn drop(&mut self) {
        
        let (opt, len) = {
            let mut map = CHANNELS.write();
            let opt = map.remove(&self.id).map(|_| ());
            (opt, map.len())
        };

        match opt {
            None => {
                println!("[WARN] [CHANNELS] transmitter dropped for channel {}, transmitter not found in channel map, {} open transmitters", self.id, len)
            },
            Some(()) => {
                println!("[INFO] [CHANNELS] transmitter dropped for channel {} => {} transmitters", self.id, len)
            }
        }
    }
}