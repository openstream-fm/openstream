use log::*;
use parking_lot::{Condvar, Mutex};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DropTracer {
  inner: Arc<Inner>,
}

impl DropTracer {
  #[allow(unused)]
  fn increment(&self) {
    self.inner.increment();
  }

  #[allow(unused)]
  fn decrement(&self) {
    self.inner.decrement();
  }

  pub fn wait(&self) {
    self.inner.wait()
  }

  pub fn token(&self) -> Token {
    Token::new(self.inner.clone())
  }
}

#[derive(Debug)]
struct Inner {
  condvar: Condvar,
  count: Mutex<usize>,
}

impl Inner {
  fn increment(&self) {
    *self.count.lock() += 1;
  }

  fn decrement(&self) {
    let mut lock = self.count.lock();
    *lock -= 1;
    if *lock == 0 {
      self.condvar.notify_all();
    }
  }

  fn wait(&self) {
    let mut lock = self.count.lock();
    if *lock == 0 {
      return;
    }
    tokio::task::block_in_place(|| {
      self.condvar.wait(&mut lock);
    })
  }
}

impl Drop for Inner {
  fn drop(&mut self) {
    if log::log_enabled!(Level::Info) {
      info!(
        "drop tracer droppped, waiting for resources cleanup: {} handles",
        *self.count.lock()
      );
    }
    self.wait();
  }
}

impl DropTracer {
  pub fn new() -> Self {
    Self {
      inner: Arc::new(Inner {
        condvar: Condvar::new(),
        count: Mutex::new(0),
      }),
    }
  }
}

impl Default for DropTracer {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Debug, Clone)]
pub struct Token {
  #[allow(unused)]
  inner: Arc<TokenInner>,
}

#[derive(Debug)]
pub struct TokenInner {
  counter: Arc<Inner>,
}

impl Token {
  fn new(counter: Arc<Inner>) -> Self {
    counter.increment();
    Self {
      inner: Arc::new(TokenInner { counter }),
    }
  }
}

impl Drop for TokenInner {
  fn drop(&mut self) {
    self.counter.decrement();
  }
}
