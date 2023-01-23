use parking_lot::RwLock;
use static_init::dynamic;
use std::collections::{btree_map::Entry, BTreeMap};
use std::net::IpAddr;
use std::time::Duration;

#[dynamic]
static IP_LIMIT_MAP: RwLock<BTreeMap<IpAddr, usize>> = RwLock::new(BTreeMap::new());

#[cfg(test)]
pub const LIMIT: usize = 60;
#[cfg(test)]
pub const LIMIT_DURATION: Duration = Duration::from_millis(100);

#[cfg(not(test))]
pub const LIMIT: usize = constants::API_IP_LIMIT;
#[cfg(not(test))]
pub const LIMIT_DURATION: Duration = constants::API_IP_LIMIT_DURATION;

pub fn get(ip: IpAddr) -> usize {
  let map = IP_LIMIT_MAP.read();
  *map.get(&ip).unwrap_or(&0)
}

pub fn should_reject(ip: IpAddr) -> bool {
  get(ip) >= LIMIT
}

pub fn hit(ip: IpAddr) -> usize {
  let v = increment(ip);
  let _handle = tokio::spawn(async move {
    tokio::time::sleep(LIMIT_DURATION).await;
    decrement(ip);
  });

  #[cfg(test)]
  eprintln!(
    "size of decrement handle: {} bytes",
    std::mem::size_of_val(&_handle)
  );

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
      *v = v.saturating_sub(1);
      *v
    }
  } else {
    0
  }
}

#[cfg(test)]
#[test_util::async_test]
async fn hit_count_and_reset() {
  let ip = IpAddr::from([1, 1, 1, 1]);

  for _ in 0..LIMIT {
    assert!(!should_reject(ip));
    hit(ip);
  }

  assert!(should_reject(ip));

  tokio::time::sleep(LIMIT_DURATION + Duration::from_millis(10)).await;

  assert_eq!(get(ip), 0);

  assert!(!should_reject(ip));

  assert!(IP_LIMIT_MAP.read().is_empty())
}
