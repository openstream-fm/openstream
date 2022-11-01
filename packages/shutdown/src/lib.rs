use pin_project::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::sync::futures::Notified;
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

  pub fn signal<'a>(&'a self) -> Signal<'a> {
    Signal {
      closed: &self.inner.closed,
      notified: self.inner.notify.notified(),
    }
  }

  pub fn is_closed(&self) -> bool {
    self.inner.closed.load(Ordering::SeqCst) == CLOSED
  }

  pub fn is_open(&self) -> bool {
    self.inner.closed.load(Ordering::SeqCst) == OPEN
  }
}

#[derive(Debug)]
#[pin_project]
pub struct Signal<'a> {
  closed: &'a AtomicBool,
  #[pin]
  notified: Notified<'a>,
}

impl Future for Signal<'_> {
  type Output = ();
  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let me = self.project();
    match me.notified.poll(cx) {
      Poll::Ready(()) => Poll::Ready(()),
      Poll::Pending => {
        if me.closed.load(Ordering::SeqCst) == CLOSED {
          Poll::Ready(())
        } else {
          Poll::Pending
        }
      }
    }
  }
}
