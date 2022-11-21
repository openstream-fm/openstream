use parking_lot::RwLock;
use static_init::dynamic;
use std::collections::{btree_map::Entry, BTreeMap};
use std::net::IpAddr;
use std::time::Duration;

#[dynamic]
static IP_LIMIT_MAP: RwLock<BTreeMap<IpAddr, usize>> = RwLock::new(BTreeMap::new());

pub const LIMIT: usize = 60;
pub const LIMIT_RESET_SECS: u64 = 60;

pub fn get(ip: IpAddr) -> usize {
  let map = IP_LIMIT_MAP.read();
  *map.get(&ip).unwrap_or(&0)
}

pub fn should_reject(ip: IpAddr) -> bool {
  get(ip) >= LIMIT
}

pub fn hit(ip: IpAddr) -> usize {
  let v = increment(ip);
  tokio::spawn(async move {
    tokio::time::sleep(Duration::from_secs(LIMIT_RESET_SECS)).await;
    decrement(ip);
  });
  v
}

fn increment(ip: IpAddr) -> usize {
  let mut map = IP_LIMIT_MAP.write();
  match map.entry(ip) {
    Entry::Vacant(entry) => {
      entry.insert(1);
      1
    }
    Entry::Occupied(mut entry) => {
      let v = entry.get_mut();
      *v += 1;
      *v
    }
  }
}

fn decrement(ip: IpAddr) -> usize {
  let mut map = IP_LIMIT_MAP.write();
  if let Entry::Occupied(mut entry) = map.entry(ip) {
    let v = entry.get_mut();
    if *v <= 1 {
      entry.remove();
      0
    } else {
      *v = usize::min(0, *v - 1);
      *v
    }
  } else {
    0
  }
}
