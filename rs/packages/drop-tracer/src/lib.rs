use log::*;
// use parking_lot::{Condvar, Mutex};
use std::{
  sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
  },
  thread,
  time::Duration,
};

#[derive(Debug, Clone)]
pub struct DropTracer {
  inner: Arc<TracerInner>,
}

impl DropTracer {
  #[allow(unused)]
  fn increment(&self) -> u64 {
    self.inner.0.increment()
  }

  #[allow(unused)]
  fn decrement(&self) -> u64 {
    self.inner.0.decrement()
  }

  // fn wait(&self) {
  //   self.inner.0.wait()
  // }

  pub fn token(&self) -> Token {
    Token::new(self.inner.0.clone())
  }
}

// #[derive(Debug)]
// struct Inner {
//   condvar: Condvar,
//   count: Mutex<i128>,
// }

#[derive(Debug)]
struct Inner {
  title: &'static str,
  counter: AtomicU64,
}

impl Inner {
  fn new(title: &'static str) -> Self {
    Self {
      title,
      counter: AtomicU64::new(0),
    }
  }

  // fn increment(&self) {
  //   *self.count.lock() += 1;
  // }

  fn increment(&self) -> u64 {
    self.counter.fetch_add(1, Ordering::SeqCst) + 1
    // let mut lock = self.count.lock();
    // *lock += 1;
    // trace!("drop tracer increment: {} handles", *lock);
    // *lock
  }

  // fn decrement(&self) {
  //   let mut lock = self.count.lock();
  //   *lock -= 1;
  //   if *lock <= 0 {
  //     self.condvar.notify_all();
  //   }
  // }

  fn decrement(&self) -> u64 {
    self.counter.fetch_sub(1, Ordering::SeqCst) - 1
    // let mut lock = self.count.lock();
    // *lock -= 1;
    // info!("drop tracer decrement: {} handles", *lock);
    // if *lock <= 0 {
    //   self.condvar.notify_all();
    // }
    // *lock
  }

  fn wait(&self) {
    tokio::task::block_in_place(|| {
      let mut i = 0usize;
      loop {
        let value = self.counter.load(Ordering::SeqCst);
        if value == 0 {
          info!(
            "drop tracer '{}' wait ({}): 0 handles left, end",
            self.title, i
          );
          return;
        // log every 1 sec
        } else if i % 10 == 0 {
          info!(
            "drop tracer '{}' wait ({}): {} handles",
            self.title, i, value
          )
        }

        thread::sleep(Duration::from_millis(100));
        i += 1;
      }
    });

    // let mut lock = self.count.lock();
    // loop {
    //   if *lock <= 0 {
    //     return;
    //   }
    //   tokio::task::block_in_place(|| {
    //     self.condvar.wait(&mut lock);
    //   })
    // }
  }
}

#[derive(Debug)]
struct TracerInner(Arc<Inner>);

impl Drop for TracerInner {
  fn drop(&mut self) {
    self.0.wait();
    info!("drop tracer '{}' dropped", self.0.title);
  }
}

impl DropTracer {
  pub fn new(title: &'static str) -> Self {
    Self {
      inner: Arc::new(TracerInner(Arc::new(Inner::new(title)))),
    }
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
    token: Token,
  }

  impl Drop for TestDrop {
    fn drop(&mut self) {
      let token = self.token.clone();
      tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(1)).await;
        drop(token);
      });
    }
  }

  #[test_util::async_test]
  async fn wait_for_tokens() {
    logger::init();

    let tracer = DropTracer::new("test");
    let mut droppers = Vec::new();
    for _ in 0usize..100 {
      let dropper = TestDrop {
        token: tracer.token(),
      };

      droppers.push(dropper);
    }

    tokio::spawn(async move {
      tokio::time::sleep(Duration::from_millis(50)).await;
      drop(droppers);
    });

    drop(tracer.clone());
    drop(tracer);
  }
}
