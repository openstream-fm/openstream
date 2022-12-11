pub mod middleware;

// use hyper::server::accept::from_stream;
// use merge_streams::MergeStreams;
// use pin_project::pin_project;
use std::collections::BTreeSet;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
// use std::pin::Pin;
// use std::task::{Context, Poll};
// use tokio::net::{TcpListener, TcpStream};
// use tokio_stream::Stream;

pub fn to_socket_addrs<A: ToSocketAddrs, S: AsRef<[A]>>(
  addrs: S,
) -> Result<BTreeSet<SocketAddr>, std::io::Error> {
  let mut set: BTreeSet<SocketAddr> = BTreeSet::new();

  for to_addrs in addrs.as_ref() {
    for addr in to_addrs.to_socket_addrs()? {
      set.insert(addr);
    }
  }

  Ok(set)
}

/*
pub async fn incoming<A: ToSocketAddrs, S: AsRef<[A]>>(
  addrs: S,
) -> Result<
  impl hyper::server::accept::Accept + Send + Sync + std::fmt::Debug + 'static,
  std::io::Error,
> {
  let set = to_socket_addrs(addrs)?;

  if set.is_empty() {
    return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
  };

  let mut listeners = Vec::with_capacity(set.len());

  for addr in set {
    let listener = TcpListener::bind(addr).await?;
    listeners.push(AcceptStream { listener });
  }

  let stream = listeners.merge();

  let accept = Accept { incoming: stream };

  Ok(accept)
}
 */

/*
#[pin_project]
#[derive(Debug)]
pub struct Accept<I> {
  #[pin]
  incoming: I,
}

impl<I> Accept<I> {
  pub fn into_inner(self) -> I {
    self.incoming
  }
}

impl<I> AsRef<I> for Accept<I> {
  fn as_ref(&self) -> &I {
    &self.incoming
  }
}

impl<I> AsMut<I> for Accept<I> {
  fn as_mut(&mut self) -> &mut I {
    &mut self.incoming
  }
}

impl<I: Stream<Item = Result<TcpStream, std::io::Error>>> hyper::server::accept::Accept
  for Accept<I>
{
  type Conn = tokio::net::TcpStream;
  type Error = std::io::Error;
  fn poll_accept(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
    self.project().incoming.poll_next(cx)
  }
}

#[derive(Debug)]
struct AcceptStream {
  listener: TcpListener,
}

impl Stream for AcceptStream {
  type Item = Result<TcpStream, std::io::Error>;
  fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    match self.listener.poll_accept(cx) {
      Poll::Pending => Poll::Pending,
      Poll::Ready(Err(e)) => Poll::Ready(Some(Err(e))),
      Poll::Ready(Ok((stream, addr))) => Poll::Ready(Some(Ok(stream))),
    }
  }
}

#[cfg(test)]
pub mod test {
  // use prex::prex;
  // use crate::{incoming, AddrIncoming};

  #[test]
  fn to_socket_addrs() {
    let addrs = super::to_socket_addrs(vec!["localhost:7777"]).unwrap();
    eprintln!("to_socket_addrs(vec![\"localhost:7777\"]) -> {:?}", addrs)
  }

  /*
  #[tokio::test]
  async fn incoming_call_with_vec_string() {
    incoming(vec!["localhost:7777"]).await.unwrap();
  }

  #[tokio::test]
  async fn can_create_hyper_server() {
    //let server = hyper::Server::builder(incoming(vec!["localhost:7777"]).await.unwrap());
    let server = hyper::Server::builder(super::AddrIncoming {});
    let app = prex().build().unwrap();

    server.serve(app);
  }
   */
}
*/
