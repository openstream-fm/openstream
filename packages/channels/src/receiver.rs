use bytes::Bytes;
use cond_count::Ref;
use heapless::Deque;
use log::*;
use std::{sync::atomic::Ordering, time::Duration};
use tokio::sync::broadcast::{self, error::RecvError};

use constants::STREAM_BURST_LENGTH;

use crate::ChannelMap;

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
  pub(crate) burst: Deque<Bytes, STREAM_BURST_LENGTH>,
  pub(crate) channels: ChannelMap,
  pub(crate) counter_ref: Ref,
}

impl Receiver {
  /**
   * Receive the next Bytes value
   * first the internal burst of the channel will be drained
   * and then the broadcasting channel will be called to get new values
   */
  pub async fn recv(&mut self) -> Result<Bytes, RecvError> {
    match self.burst.pop_front() {
      Some(bytes) => Ok(bytes),
      None => self.receiver.recv().await,
    }
  }
}

impl Drop for Receiver {
  fn drop(&mut self) {
    let count = self.channels.inner.rx_count.fetch_sub(1, Ordering::SeqCst) - 1;
    debug!(
      "[channels] subscriber dropped for channel {} => {} subscribers",
      self.channel_id, count
    );

    let counter_ref = self.counter_ref.clone();
    tokio::spawn(async move {
      tokio::time::sleep(Duration::from_millis(1_000)).await;
      drop(counter_ref);
    });
  }
}
