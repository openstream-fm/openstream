use log::*;
use parking_lot::{Condvar, Mutex};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DropTracer {
  inner: Arc<TracerInner>,
}

#[derive(Debug)]
struct TracerInner(Arc<Inner>);

impl DropTracer {
  #[allow(unused)]
  fn increment(&self) -> i128 {
    self.inner.0.increment()
  }

  #[allow(unused)]
  fn decrement(&self) -> i128 {
    self.inner.0.decrement()
  }

  pub fn wait(&self) {
    self.inner.0.wait()
  }

  pub fn token(&self) -> Token {
    Token::new(self.inner.0.clone())
  }
}

#[derive(Debug)]
struct Inner {
  condvar: Condvar,
  count: Mutex<i128>,
}

impl Inner {
  // fn increment(&self) {
  //   *self.count.lock() += 1;
  // }

  fn increment(&self) -> i128 {
    let mut lock = self.count.lock();
    *lock += 1;
    trace!("drop tracer increment: {} handles", *lock);
    *lock
  }

  // fn decrement(&self) {
  //   let mut lock = self.count.lock();
  //   *lock -= 1;
  //   if *lock <= 0 {
  //     self.condvar.notify_all();
  //   }
  // }

  fn decrement(&self) -> i128 {
    let mut lock = self.count.lock();
    *lock -= 1;
    info!("drop tracer decrement: {} handles", *lock);
    if *lock <= 0 {
      self.condvar.notify_all();
    }
    *lock
  }

  fn wait(&self) {
    let mut lock = self.count.lock();
    loop {
      if *lock <= 0 {
        return;
      }
      tokio::task::block_in_place(|| {
        self.condvar.wait(&mut lock);
      })
    }
  }
}

impl Drop for TracerInner {
  fn drop(&mut self) {
    if log::log_enabled!(Level::Info) {
      info!(
        "drop tracer dropped, waiting for resources cleanup: {} handles",
        *self.0.count.lock()
      );
    }
    self.0.wait();
  }
}

impl DropTracer {
  pub fn new() -> Self {
    Self {
      inner: Arc::new(TracerInner(Arc::new(Inner {
        condvar: Condvar::new(),
        count: Mutex::new(0),
      }))),
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
struct TokenInner {
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
    let n = self.counter.decrement();
    trace!("drop tracer token (inner) dropped: {n} handles");
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::time::Duration;

  #[derive(Debug)]
  struct TestDrop {
    i: usize,
    token: Token,
  }

  impl Drop for TestDrop {
    fn drop(&mut self) {
      let i = self.i;
      let token = self.token.clone();
      tokio::spawn(async move {
        info!("dropper {i} dropped");
        tokio::time::sleep(Duration::from_secs(1)).await;
        drop(token);
        info!("token {i} dropped");
      });
    }
  }

  #[test_util::async_test]
  async fn wait_for_tokens() {
    logger::init();

    let tracer = DropTracer::new();
    let mut droppers = Vec::new();
    for i in 0usize..100 {
      let dropper = TestDrop {
        i,
        token: tracer.token(),
      };

      droppers.push(dropper);
    }

    tokio::spawn(async move {
      drop(droppers);
    });

    drop(tracer);
  }
}
