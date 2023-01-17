use constants::TRANSFER_SAVE_INTERVAL_MILLIS;
use db::station::Station;
use log::*;
use parking_lot::RwLock;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::time::Duration;

const SAVE_INTERVAL: Duration = Duration::from_millis(TRANSFER_SAVE_INTERVAL_MILLIS);

#[derive(Debug, Default, Clone)]
pub struct TransferTracer {
  current: Arc<RwLock<BTreeMap<String, AtomicUsize>>>,
}

impl TransferTracer {
  #[inline]
  pub fn new() -> Self {
    Self::default()
  }

  pub fn increment(&self, id: &str, len: usize) -> usize {
    let lock = self.current.upgradable_read();
    match lock.get(id) {
      Some(value) => value.fetch_add(len, Ordering::AcqRel) + len,
      None => {
        let mut lock = parking_lot::RwLockUpgradableReadGuard::upgrade(lock);
        lock.insert(id.to_string(), AtomicUsize::new(len));
        len
      }
    }
  }

  pub fn start_background_task(&self) -> tokio::task::JoinHandle<()> {
    let me = self.clone();
    tokio::spawn(async move {
      loop {
        tokio::time::sleep(SAVE_INTERVAL).await;
        me.save().await;
      }
    })
  }

  async fn save(&self) {
    let mut now = BTreeMap::new();
    {
      let current = self.current.read();
      for (id, value) in current.iter() {
        let value = value.swap(0, Ordering::AcqRel);
        if value != 0 {
          now.insert(id.clone(), value);
        }
      }
    }

    if now.is_empty() {
      debug!("saving transfer, 0 stations to update");
    } else {
      debug!("saving transfer for {} stations", now.len());
    }

    for (id, value) in now.iter() {
      debug!("saving transfer for station {id} increment: {value}");
      let r = Station::increment_used_transfer(id, *value).await;
      match r {
        Err(e) => warn!("error saving transfer for station {id}: {e}"),
        Ok(r) => debug!(
          "transfer saved for station {id}, matched: {}, modified: {}",
          r.matched_count, r.modified_count
        ),
      }
    }
  }
}
