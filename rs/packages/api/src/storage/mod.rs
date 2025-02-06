pub mod routes;

use futures::stream::FuturesUnordered;
use futures::TryStreamExt;
use hyper::Server;
use log::*;
use serde::{Deserialize, Serialize};
use shutdown::Shutdown;
use socket2::{Domain, Protocol, Socket, Type};
use std::future::Future;
use std::net::SocketAddr;

#[derive(Debug)]
pub struct StorageServer {
  #[allow(unused)]
  deployment_id: String,
  addrs: Vec<SocketAddr>,
  shutdown: Shutdown,
}

#[derive(Debug, thiserror::Error)]
pub enum StorageServerError {
  #[error("io error: {0}")]
  Io(#[from] std::io::Error),
  #[error("hyper error: {0}")]
  Hyper(#[from] hyper::Error),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Status {
  status: usize,
}

impl StorageServer {
  pub fn new(deployment_id: String, addrs: Vec<SocketAddr>, shutdown: Shutdown) -> Self {
    Self {
      deployment_id,
      addrs,
      shutdown,
    }
  }

  #[allow(dependency_on_unit_never_type_fallback)]
  pub fn start(
    self,
  ) -> Result<impl Future<Output = Result<(), hyper::Error>> + 'static, StorageServerError> {
    let mut app = prex::prex();

    app.with(http::middleware::server);
    app.get("/status", http::middleware::status);

    app.at("/").nest(routes::router());

    let app = app.build().expect("storage server prex build");

    let futs = FuturesUnordered::new();

    for addr in self.addrs.iter().copied() {
      let domain = match addr {
        SocketAddr::V4(_) => Domain::IPV4,
        SocketAddr::V6(_) => Domain::IPV6,
      };

      let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP))?;

      if addr.is_ipv6() {
        socket.set_only_v6(true)?;
      }

      socket.set_nonblocking(true)?;
      socket.set_reuse_address(true)?;
      // socket.set_reuse_port(true)?;

      socket.bind(&addr.into())?;
      socket.listen(1024)?;

      let tcp = socket.into();

      let server = Server::from_tcp(tcp)?
        .http1_only(true)
        .http1_title_case_headers(false)
        .http1_preserve_header_case(false)
        .http1_keepalive(false);

      {
        use owo_colors::*;
        info!(target: "storage", "storage server bound to {}", addr.yellow());
      }

      let fut = server
        .serve(app.clone())
        .with_graceful_shutdown(self.shutdown.signal());

      futs.push(fut);
    }

    Ok(async move {
      futs.try_collect().await?;
      drop(self);
      Ok(())
    })
  }
}

impl Drop for StorageServer {
  fn drop(&mut self) {
    info!(target: "storage", "storage server dropped");
  }
}
