use parking_lot::Mutex;
use pin_project::pin_project;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
use tokio_stream::Stream;

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
  let state = Arc::new(Mutex::new(State {
    item: None,
    recv_dropped: false,
    send_dropped: false,
    recv_waker: None,
    send_waker: None,
  }));

  let sender = Sender {
    state: state.clone(),
  };

  let receiver = Receiver { state };

  (sender, receiver)
}

#[derive(Debug, Clone, Copy)]
pub struct SendError<T>(T);

impl<T> Display for SendError<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "The channel is closed")
  }
}

impl<T: Debug> Error for SendError<T> {}

#[derive(Debug, Clone, Copy)]
pub enum TrySendErr<T> {
  Full(T),
  Closed(T),
}

impl<T> Display for TrySendErr<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Full(_) => write!(f, "The channel is full"),
      Self::Closed(_) => write!(f, "The channel is closed"),
    }
  }
}

impl<T: Debug> Error for TrySendErr<T> {}

#[derive(Debug, Clone, Copy)]
pub enum TryRecvError {
  Empty,
  Closed,
}

impl Display for TryRecvError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Empty => write!(f, "The channel is empty"),
      Self::Closed => write!(f, "The channel is closed"),
    }
  }
}

impl Error for TryRecvError {}

#[derive(Debug)]
struct State<T> {
  item: Option<T>,
  send_dropped: bool,
  recv_dropped: bool,
  recv_waker: Option<Waker>,
  send_waker: Option<Waker>,
}

pub struct Sender<T> {
  state: Arc<Mutex<State<T>>>,
}

impl<T> Sender<T> {
  pub fn send(&self, item: T) -> Send<T> {
    Send {
      item: Some(item),
      state: &self.state,
    }
  }
}

impl<T> Drop for Sender<T> {
  fn drop(&mut self) {
    let mut state = self.state.lock();
    state.send_dropped = true;
    if let Some(waker) = state.recv_waker.take() {
      waker.wake();
    }
  }
}

#[pin_project]
pub struct Send<'a, T> {
  item: Option<T>,
  state: &'a Mutex<State<T>>,
}

impl<'a, T> Future for Send<'a, T> {
  type Output = Result<(), SendError<T>>;

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let me = self.project();

    let mut state = me.state.lock();

    if state.recv_dropped {
      let item = me
        .item
        .take()
        .expect("spsc send future polled after completion");

      return Poll::Ready(Err(SendError(item)));
    }

    if state.item.is_none() {
      let item = me
        .item
        .take()
        .expect("spsc send future polled after completion");

      let _ = state.item.insert(item);

      if let Some(waker) = state.recv_waker.take() {
        waker.wake();
      };

      Poll::Ready(Ok(()))
    } else {
      let _ = state.send_waker.insert(cx.waker().clone());
      Poll::Pending
    }
  }
}

pub struct Receiver<T> {
  state: Arc<Mutex<State<T>>>,
}

impl<T> Receiver<T> {
  pub fn recv(&self) -> Recv<T> {
    Recv { state: &self.state }
  }
}

impl<T> Stream for Receiver<T> {
  type Item = T;
  fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    let mut state = self.state.lock();
    match state.item.take() {
      None => {
        if state.send_dropped {
          return Poll::Ready(None);
        };

        let _ = state.recv_waker.insert(cx.waker().clone());
        Poll::Pending
      }

      Some(item) => {
        if let Some(waker) = state.send_waker.take() {
          waker.wake();
        }

        Poll::Ready(Some(item))
      }
    }
  }
}

#[derive(Debug)]
pub struct Recv<'a, T> {
  state: &'a Mutex<State<T>>,
}

impl<'a, T> Future for Recv<'a, T> {
  type Output = Option<T>;

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let mut state = self.state.lock();

    match state.item.take() {
      None => {
        if state.send_dropped {
          return Poll::Ready(None);
        };

        let _ = state.recv_waker.insert(cx.waker().clone());
        Poll::Pending
      }

      Some(item) => {
        if let Some(waker) = state.send_waker.take() {
          waker.wake();
        }

        Poll::Ready(Some(item))
      }
    }
  }
}

impl<T> Drop for Receiver<T> {
  fn drop(&mut self) {
    let mut state = self.state.lock();
    state.recv_dropped = true;
    if let Some(waker) = state.send_waker.take() {
      waker.wake();
    }
  }
}

#[cfg(test)]
mod test {
  use std::time::Instant;
  use tokio_stream::StreamExt;

  const N: usize = 100_000;

  #[tokio::test]
  async fn channel() {
    let (tx, rx) = super::channel::<usize>();

    let start = Instant::now();

    tokio::spawn(async move {
      for i in 0..N {
        tx.send(i).await.unwrap();
      }
    });

    for i in 0..N {
      assert_eq!(rx.recv().await.unwrap(), i);
    }

    assert!(rx.recv().await.is_none());

    let elapsed = start.elapsed().as_millis();

    eprintln!("spsc: send and recv {N} items in {elapsed}ms")
  }

  #[tokio::test]
  async fn channel_as_stream() {
    let (tx, mut rx) = super::channel::<usize>();

    let start = Instant::now();

    tokio::spawn(async move {
      for i in 0..N {
        tx.send(i).await.unwrap();
      }
    });

    for i in 0..N {
      assert_eq!(rx.next().await.unwrap(), i);
    }

    assert!(rx.recv().await.is_none());

    let elapsed = start.elapsed().as_millis();

    eprintln!("spsc as stream: send and recv {N} items in {elapsed}ms")
  }

  #[tokio::test]
  async fn tokio_channel() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<usize>(1);

    let start = Instant::now();

    tokio::spawn(async move {
      for i in 0..N {
        tx.send(i).await.unwrap();
      }
    });

    for i in 0..N {
      assert_eq!(rx.recv().await.unwrap(), i);
    }

    assert!(rx.recv().await.is_none());

    let elapsed = start.elapsed().as_millis();

    eprintln!("tokio mpsc: send and recv {N} items in {elapsed}ms")
  }

  #[tokio::test]
  async fn async_stream() {
    let start = Instant::now();

    let stream = async_stream::stream! {
      for i in 0..N {
        yield i;
      }
    };

    tokio::pin!(stream);

    for i in 0..N {
      assert_eq!(stream.next().await.unwrap(), i)
    }

    assert!(stream.next().await.is_none());

    let elapsed = start.elapsed().as_millis();

    eprintln!("tokio stream: send and recv {N} items in {elapsed}ms");
  }
}
