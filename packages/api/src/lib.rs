#![feature(never_type)]
#![feature(exhaustive_patterns)]

pub mod error;
pub mod ip_limit;
pub mod json;
pub mod request_ext;
pub mod routes;

use async_trait::async_trait;
use futures::stream::FuturesUnordered;
use futures::TryStreamExt;
use hyper::header::CONTENT_TYPE;
use hyper::{http::HeaderValue, Body, Server, StatusCode};
use log::*;
use owo_colors::*;
use prex::{handler::Handler, Next, Request, Response};
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
    let futs = FuturesUnordered::new();

    for addr in &self.addrs {
      let server = Server::try_bind(addr)?
        .http1_only(true)
        .http1_title_case_headers(false)
        .http1_preserve_header_case(false)
        .http1_keepalive(false);

      info!("api server bound to {}", addr.yellow());

      let mut app = prex::prex();

      app.get("/status", StatusHandler::new());

      app.at("/").nest(routes::router());

      let app = app.build().expect("prex app build stream");

      let fut = server
        .serve(app)
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

#[derive(Debug)]
struct StatusHandler;

#[async_trait]
impl Handler for StatusHandler {
  async fn call(&self, _: Request, _: Next) -> Response {
    let mut res = Response::new(StatusCode::OK);
    let body = Body::from(r#"{"status":200}"#);
    res.headers_mut().append(
      CONTENT_TYPE,
      HeaderValue::from_static("application/json;charset=utf-8"),
    );
    *res.body_mut() = body;
    res
  }
}

impl StatusHandler {
  fn new() -> Self {
    Self {}
  }
}
