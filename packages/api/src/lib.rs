pub mod error;
pub mod ip_limit;
pub mod json;
pub mod request_ext;
pub mod routes;

use futures::stream::FuturesUnordered;
use futures::TryStreamExt;
use hyper::Server;
use log::*;
use owo_colors::*;
use serde::{Deserialize, Serialize};
use shutdown::Shutdown;
use std::future::Future;
use std::net::SocketAddr;

#[derive(Debug)]
pub struct ApiServer {
  addrs: Vec<SocketAddr>,
  shutdown: Shutdown,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Status {
  status: usize,
}

impl ApiServer {
  pub fn new(addrs: Vec<SocketAddr>, shutdown: Shutdown) -> Self {
    Self { addrs, shutdown }
  }

  pub fn start(
    self,
  ) -> Result<impl Future<Output = Result<(), hyper::Error>> + 'static, hyper::Error> {
    let mut app = prex::prex();

    app.with(http::middleware::server);
    app.get("/status", http::middleware::status);

    app.at("/").nest(routes::router());

    let app = app.build().expect("prex app build api");

    let futs = FuturesUnordered::new();

    for addr in &self.addrs {
      let server = Server::try_bind(addr)?
        .http1_only(true)
        .http1_title_case_headers(false)
        .http1_preserve_header_case(false)
        .http1_keepalive(false);

      info!("api server bound to {}", addr.yellow());

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
