use std::sync::atomic::Ordering;

use bytes::Bytes;
use heapless::Deque;
use tokio::sync::broadcast::{self, error::RecvError};

use crate::SUBSCRIBER_COUNT;

use super::BURST_LEN;

#[derive(Debug)]
/**
 * Receiver used to subscribe to one stream channel
 * type returned from `channels::subscribe` if the `channel_id` is actively streaming
 * ```
 * let rx: channels::Receiver = channels::subscribe("channel-id")?;
 * ```
 */
pub struct Receiver {
  #[allow(unused)]
  pub(crate) channel_id: String,  
  pub(crate) receiver: broadcast::Receiver<Bytes>,
  // this is an owned copy of the burst at subscription time (Bytes instances are copied by reference)
  pub(crate) burst: Deque<Bytes, BURST_LEN>
}

impl Receiver {
    /**
     * Receive the next Bytes value
     * first the internal burst of the channel will be drained
     * and then the broadcasting channel will be called to get new values
     */
    pub async fn recv(&mut self) -> Result<Bytes, RecvError> {
        match self.burst.pop_back() {
            Some(bytes) => Ok(bytes),
            None => self.receiver.recv().await
        }
    }
}

impl Drop for Receiver {
  fn drop(&mut self) {
    let count = SUBSCRIBER_COUNT.fetch_sub(1, Ordering::SeqCst) - 1; 
    println!("[channels] subscriber dropped for channel {} => {} subscribers", self.channel_id, count);
  }
}