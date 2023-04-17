use parking_lot::{Mutex, MutexGuard};
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::net::IpAddr;
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
pub struct IpCounter {
  map: Arc<Mutex<BTreeMap<IpAddr, u64>>>,
}

impl IpCounter {
  #[inline]
  pub fn new() -> Self {
    Self::default()
  }

  #[inline]
  pub fn lock(&self) -> IpCounterLock<'_> {
    IpCounterLock {
      lock: self.map.lock(),
    }
  }

  #[inline]
  pub fn get(&self, ip: IpAddr) -> u64 {
    self.lock().get(ip)
  }

  #[inline]
  pub fn increment(&self, ip: IpAddr) -> u64 {
    self.lock().increment(ip)
  }

  #[inline]
  pub fn increment_with_limit(&self, ip: IpAddr, limit: u64) -> Option<u64> {
    self.lock().increment_with_limit(ip, limit)
  }

  #[inline]
  pub fn decrement(&self, ip: IpAddr) -> u64 {
    self.lock().decrement(ip)
  }
}

#[derive(Debug)]
pub struct IpCounterLock<'a> {
  lock: MutexGuard<'a, BTreeMap<IpAddr, u64>>,
}

impl<'a> IpCounterLock<'a> {
  #[inline]
  pub fn get(&self, ip: IpAddr) -> u64 {
    self.lock.get(&ip).copied().unwrap_or(0)
  }

  pub fn increment(&mut self, ip: IpAddr) -> u64 {
    match self.lock.entry(ip) {
      Entry::Occupied(mut entry) => {
        let v = entry.get_mut();
        let new_value = *v + 1;
        *v = new_value;
        new_value
      }

      Entry::Vacant(entry) => {
        entry.insert(1);
        1
      }
    }
  }

  #[must_use]
  pub fn increment_with_limit(&mut self, ip: IpAddr, limit: u64) -> Option<u64> {
    match self.lock.entry(ip) {
      Entry::Occupied(mut entry) => {
        let v = entry.get_mut();
        if *v >= limit {
          return None;
        }
        let new_value = *v + 1;
        *v = new_value;
        Some(new_value)
      }

      Entry::Vacant(entry) => {
        if 1 >= limit {
          return None;
        }

        entry.insert(1);
        Some(1)
      }
    }
  }

  pub fn decrement(&mut self, ip: IpAddr) -> u64 {
    match self.lock.entry(ip) {
      Entry::Occupied(mut entry) => {
        let v = entry.get_mut();
        *v -= 1;
        let new_v = *v;
        if new_v == 0 {
          entry.remove_entry();
        }
        new_v
      }

      Entry::Vacant(_) => 0,
    }
  }
}

// #[cfg(test)]
// mod tests {
//   use super::*;
// }
