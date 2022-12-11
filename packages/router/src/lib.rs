use async_trait::async_trait;
use db::account::Account;
use db::Model;
use futures::stream::FuturesUnordered;
use futures::TryStreamExt;
use hyper::header::LOCATION;
use hyper::{header::CONTENT_TYPE, http::HeaderValue, Body, Server, StatusCode};
use log::*;
use owo_colors::*;
use prex::{handler::Handler, Next, Request, Response};
use serde::{Deserialize, Serialize};
use shutdown::Shutdown;
use std::future::Future;
use std::net::SocketAddr;

#[derive(Debug)]
pub struct RouterServer {
  addrs: Vec<SocketAddr>,
  shutdown: Shutdown,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Status {
  status: usize,
}

impl RouterServer {
  pub fn new(addrs: Vec<SocketAddr>, shutdown: Shutdown) -> Self {
    Self { addrs, shutdown }
  }

  pub fn start(
    self,
  ) -> Result<impl Future<Output = Result<(), hyper::Error>> + 'static, hyper::Error> {
    let futs = FuturesUnordered::new();

    let mut app = prex::prex();

    app.with(http::middleware::server);
    app.get("/status", http::middleware::status);

    app.get("/:id([a-zA-Z0-9]+)", RouterHandler::new());
    app.get("/:id([a-zA-Z0-9]+).m3u", HlsHandler::new());
    app.get("/:id([a-zA-Z0-9]+).pls", PlsHandler::new());

    let app = app.build().expect("prex app build router");

    for addr in &self.addrs {
      let server = Server::try_bind(addr)?
        .http1_only(true)
        .http1_title_case_headers(false)
        .http1_preserve_header_case(false)
        .http1_keepalive(false);

      info!("router server bound to {}", addr.yellow());

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

impl Drop for RouterServer {
  fn drop(&mut self) {
    info!("router server dropped");
  }
}

#[derive(Debug, Clone)]
struct HlsHandler {}

impl HlsHandler {
  fn new() -> Self {
    Self {}
  }
}

#[async_trait]
impl Handler for HlsHandler {
  async fn call(&self, req: Request, _: Next) -> Response {
    let id = req.param("id").unwrap();
    let mut res = Response::new(StatusCode::OK);
    res.headers_mut().append(
      CONTENT_TYPE,
      HeaderValue::from_static("application/vnd.apple.mpegurl"),
    );
    *res.body_mut() = Body::from(hls_contents(id));
    res
  }
}

#[derive(Debug, Clone)]
struct PlsHandler {}

impl PlsHandler {
  fn new() -> Self {
    Self {}
  }
}

#[async_trait]
impl Handler for PlsHandler {
  async fn call(&self, req: Request, _: Next) -> Response {
    let id = req.param("id").unwrap();
    let mut res = Response::new(StatusCode::OK);
    res
      .headers_mut()
      .append(CONTENT_TYPE, HeaderValue::from_static("audio/x-scpls"));
    *res.body_mut() = Body::from(pls_contents(id));
    res
  }
}

#[derive(Debug, Clone)]
struct RouterHandler {}

impl RouterHandler {
  fn new() -> Self {
    Self {}
  }
}

#[async_trait]
impl Handler for RouterHandler {
  async fn call(&self, req: Request, _: Next) -> Response {
    let id = req.param("id").unwrap();
    let url = match route(id).await {
      Err(e) => {
        let (status, message) = match e {
          RouteError::Db(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Internal Server Error (DB)"),
          ),
          RouteError::NotFound => (
            StatusCode::NOT_FOUND,
            format!("Station with id {id} not found"),
          ),
          RouteError::NotStreaming => (
            StatusCode::SERVICE_UNAVAILABLE,
            format!("Station with id {id} is not actively streaming right now"),
          ),
        };

        let mut res = Response::new(status);
        *res.body_mut() = Body::from(message);
        res.headers_mut().append(
          "content-type",
          HeaderValue::from_static("text/plain;charset=utf-8"),
        );

        return res;
      }
      Ok(url) => url,
    };

    let mut res = Response::new(StatusCode::FOUND);
    res
      .headers_mut()
      .append(LOCATION, HeaderValue::try_from(url).unwrap());
    res
  }
}

#[derive(Debug, Clone)]
pub enum RouteError {
  Db(mongodb::error::Error),
  NotFound,
  NotStreaming,
}

impl From<mongodb::error::Error> for RouteError {
  fn from(e: mongodb::error::Error) -> Self {
    Self::Db(e)
  }
}

pub async fn route(id: &str) -> Result<String, RouteError> {
  let account = Account::get_by_id(id).await?;

  let _account = match account {
    None => return Err(RouteError::NotFound),
    Some(account) => account,
  };

  let url = format!("https://stream-1.openstream.test/stream/{id}");
  Ok(url)
}

pub fn pls_contents(id: &str) -> String {
  format!(
    "\
[Playlist]
NumberOfEntries=1
File1=http://stream.openstream.test/{id}
Title1={id}
Length=-1
Version=2"
  )
}

pub fn hls_contents(id: &str) -> String {
  format!("http://stream.openstream.test/{id}")
}
