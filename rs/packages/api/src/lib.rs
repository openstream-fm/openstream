pub mod error;
pub mod ip_limit;
pub mod json;
pub mod me;
pub mod request_ext;
pub mod routes;
pub mod storage;

use payments::client::PaymentsClient;

use db::stream_connection::index::MemIndex;
use drop_tracer::DropTracer;
use futures::stream::FuturesUnordered;
use futures::TryStreamExt;
use hyper::Server;
use log::*;
use mailer::send::Mailer;
use media_sessions::MediaSessionMap;
use serde::{Deserialize, Serialize};
use shutdown::Shutdown;
use socket2::{Domain, Protocol, Socket, Type};
use std::future::Future;
use std::net::SocketAddr;

#[derive(Debug)]
pub struct ApiServer {
  deployment_id: String,
  addrs: Vec<SocketAddr>,
  shutdown: Shutdown,
  drop_tracer: DropTracer,
  media_sessions: MediaSessionMap,
  stream_connections_index: MemIndex,
  payments_client: PaymentsClient,
  mailer: Mailer,
}

#[derive(Debug, thiserror::Error)]
pub enum ApiServerError {
  #[error("io error: {0}")]
  Io(#[from] std::io::Error),
  #[error("hyper error: {0}")]
  Hyper(#[from] hyper::Error),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Status {
  status: usize,
}

impl ApiServer {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    deployment_id: String,
    addrs: Vec<SocketAddr>,
    shutdown: Shutdown,
    drop_tracer: DropTracer,
    media_sessions: MediaSessionMap,
    stream_connections_index: MemIndex,
    payments_client: PaymentsClient,
    mailer: Mailer,
  ) -> Self {
    Self {
      deployment_id,
      addrs,
      shutdown,
      drop_tracer,
      media_sessions,
      stream_connections_index,
      payments_client,
      mailer,
    }
  }

  pub fn start(
    self,
  ) -> Result<impl Future<Output = Result<(), hyper::Error>> + 'static, ApiServerError> {
    let mut app = prex::prex();

    app.with(http::middleware::server);
    app.get("/status", http::middleware::status);

    app.at("/").nest(routes::router(
      self.deployment_id.clone(),
      self.media_sessions.clone(),
      self.shutdown.clone(),
      self.drop_tracer.clone(),
      self.stream_connections_index.clone(),
      self.payments_client.clone(),
      self.mailer.clone(),
    ));

    let app = app.build().expect("prex app build api");

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

      match socket.bind(&addr.into()) {
        Ok(()) => {}
        Err(e) => {
          error!("error binding to addr {} => {}", addr, e);
          return Err(e.into());
        }
      };

      socket.listen(1024)?;

      let tcp = socket.into();

      let server = Server::from_tcp(tcp)?
        .http1_only(true)
        .http1_title_case_headers(false)
        .http1_preserve_header_case(false)
        .http1_keepalive(false);

      {
        use owo_colors::*;
        info!("api server bound to {}", addr.yellow());
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

impl Drop for ApiServer {
  fn drop(&mut self) {
    info!("api server dropped");
  }
}
