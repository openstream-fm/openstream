use std::{fmt::Display, net::SocketAddr};

use hyper::{body::HttpBody, Body};
use prex::{request::Parts, Request};
use shutdown::Signal;
use tokio::{
  io::{AsyncRead, AsyncWriteExt},
  net::{tcp::OwnedWriteHalf, TcpListener},
};

use crate::http::{error::ReadHeadError, read_request_head, RequestHead};

pub struct Server {
  tcp: TcpListener,
  router: prex::Router,
  signal: Signal,
}

impl Server {
  pub fn new(tcp: TcpListener, router: prex::Router, signal: Signal) -> Self {
    Self {
      tcp,
      router,
      signal,
    }
  }

  pub async fn serve(self) -> std::io::Result<()> {
    let Self {
      tcp,
      router,
      signal,
    } = self;

    let task = async move {
      loop {
        let (socket, remote_addr) = tcp.accept().await?;
        let router = router.clone();
        tokio::spawn(async move {
          let local_addr = socket
            .local_addr()
            .unwrap_or_else(|_| SocketAddr::from(([1, 1, 1, 1], 1)));

          log::info!(
            target: "source-alt",
            "accept {} => {}", local_addr, remote_addr
          );

          let (read, write) = socket.into_split();
          let request = match read_request(local_addr, remote_addr, read).await {
            Ok(request) => {
              log::info!(
                target: "source-alt",
                "read request {local_addr} => {remote_addr} - {ip} {method} {uri}",
                ip=request.isomorphic_ip(),
                method=request.method(),
                uri=request.uri(),
              );
              request
            }
            Err(e) => {
              log::error!(target: "source-alt", "read request error {local_addr} => {remote_addr} - {e} {e:?}");
              return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
            }
          };

          let ip = request.isomorphic_ip();
          let method = request.method().clone();
          let uri = request.uri().clone();

          let response = router.handle(request).await;
          let status = response.status();

          log::info!(
            target: "source-alt",
            "write response {local_addr} => {remote_addr} - {ip} {method} {uri} => {status}",
          );

          match write_response(write, response).await {
            Ok(_) => {
              log::info!(
                target: "source-alt",
                "response end {local_addr} => {remote_addr} - {ip} {method} {uri} {status}",
              );
            }

            Err(e) => {
              log::error!(target: "source-alt", "write response error {local_addr} => {remote_addr} - {ip} {method} {uri} {status} => {e} {e:?}");
              return Err(e);
            }
          };
          Ok::<(), std::io::Error>(())
        });
      }
    };

    tokio::select! {
      r = task => r,
      _ = signal => Ok(())
    }
  }
}

async fn read_request<R: AsyncRead + Unpin>(
  local_addr: SocketAddr,
  remote_addr: SocketAddr,
  mut read: R,
) -> Result<prex::Request, ReadHeadError> {
  let RequestHead {
    proxy_protocol_ip,
    version,
    method,
    uri,
    headers,
  } = read_request_head(&mut read).await?;
  // TODO: implement request body
  let body = Body::empty();

  let request = Request::from_parts(Parts {
    local_addr,
    remote_addr,
    proxy_protocol_ip,
    method,
    uri,
    version,
    headers,
    body,
    params: prex::params::Params::new(),
    extensions: hyper::http::Extensions::new(),
  });

  Ok(request)
}

async fn write_response(write: OwnedWriteHalf, response: prex::Response) -> std::io::Result<()> {
  tokio::pin!(write);

  let head_fmt = HeadFmt {
    response: &response,
  };

  let head = format!("{head_fmt}");
  let mut body = response.into_body();

  write.write_all(head.as_bytes()).await?;

  loop {
    let item = body.data().await;
    match item {
      Some(Ok(bytes)) => {
        write.write_all(bytes.as_ref()).await?;
      }
      None => {
        write.shutdown().await?;
        break;
      }
      Some(Err(e)) => {
        log::info!(
          target: "source-alt",
          "body.data() error {e} {e:?}"
        );
        // we do not call .shutdown() because we want to shutdown in a not clean fashion
        break;
      }
    }
  }

  Ok(())
}

struct HeadFmt<'a> {
  response: &'a prex::Response,
}

impl<'a> Display for HeadFmt<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let response = self.response;
    let version_str = "HTTP/1.0";
    let status = response.status();
    let status_text = status.canonical_reason().unwrap_or("");
    let headers = response.headers();

    write!(f, "{version_str} {status} {status_text}\r\n",)?;

    for (key, value) in headers {
      if let Ok(value) = value.to_str() {
        let key = key.as_str();
        write!(f, "{key}: {value}\r\n")?;
      }
    }

    write!(f, "\r\n")?;

    Ok(())
  }
}
