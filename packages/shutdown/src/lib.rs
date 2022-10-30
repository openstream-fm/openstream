use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Notify;

const CLOSED: bool = true;
const OPEN: bool = false;

#[derive(Debug, Clone)]
pub struct Shutdown {
  inner: Arc<Inner>,
}

#[derive(Debug)]
struct Inner {
  notify: Notify,
  closed: AtomicBool,
}

impl Shutdown {
  pub fn new() -> Self {
    Self {
      inner: Arc::new(Inner {
        notify: Notify::new(),
        closed: AtomicBool::new(OPEN),
      }),
    }
  }

  pub fn shutdown(&self) {
    self.inner.closed.store(CLOSED, Ordering::SeqCst);
    self.inner.notify.notify_waiters();
  }

  pub async fn notified(&self) {
    self.inner.notify.notified().await;
  }

  pub fn is_closed(&self) -> bool {
    self.inner.closed.load(Ordering::SeqCst) == CLOSED
  }

  pub fn is_open(&self) -> bool {
    self.inner.closed.load(Ordering::SeqCst) == OPEN
  }
}
