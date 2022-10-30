use parking_lot::{Condvar, Mutex};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CondCount {
  inner: Arc<Inner>,
}

impl CondCount {
  pub fn increment(&self) {
    self.inner.increment();
  }

  pub fn decrement(&self) {
    self.inner.decrement();
  }

  pub fn wait(&self) {
    self.inner.wait()
  }

  pub fn instance(&self) -> Ref {
    Ref::new(self.inner.clone())
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

impl CondCount {
  pub fn new() -> Self {
    Self {
      inner: Arc::new(Inner {
        condvar: Condvar::new(),
        count: Mutex::new(0),
      }),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Ref {
  #[allow(unused)]
  inner: Arc<InnerRef>,
}

#[derive(Debug)]
pub struct InnerRef {
  counter: Arc<Inner>,
}

impl Ref {
  fn new(counter: Arc<Inner>) -> Self {
    counter.increment();
    Self {
      inner: Arc::new(InnerRef { counter }),
    }
  }
}

impl Drop for InnerRef {
  fn drop(&mut self) {
    self.counter.decrement();
  }
}
