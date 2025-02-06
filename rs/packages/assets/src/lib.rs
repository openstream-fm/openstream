#![allow(clippy::useless_format)]

use async_trait::async_trait;
use futures_util::stream::{FuturesUnordered, TryStreamExt};
use hyper::header::{HeaderValue, CONTENT_TYPE, ETAG, IF_NONE_MATCH};
use hyper::{Body, Server, StatusCode};
use log::*;
use prex::{handler::Handler, Next, Request, Response};
use rust_embed::RustEmbed;
use shutdown::Shutdown;
use socket2::{Domain, Protocol, Socket, Type};
use std::future::Future;
use std::net::SocketAddr;

#[derive(RustEmbed)]
#[folder = "../../../static/static/"]
struct Assets;

#[derive(Debug, thiserror::Error)]
pub enum StaticServerError {
  #[error("io error: {0}")]
  Io(#[from] std::io::Error),
  #[error("hyper error: {0}")]
  Hyper(#[from] hyper::Error),
}

#[derive(Debug)]
pub struct StaticServer {
  addrs: Vec<SocketAddr>,
  shutdown: Shutdown,
}

impl StaticServer {
  pub fn new(addrs: Vec<SocketAddr>, shutdown: Shutdown) -> Self {
    Self { addrs, shutdown }
  }

  #[allow(dependency_on_unit_never_type_fallback)]
  pub fn start(
    self,
  ) -> Result<impl Future<Output = Result<(), hyper::Error>> + 'static, StaticServerError> {
    let mut app = prex::prex();

    app.with(http::middleware::server);
    app.get("/status", http::middleware::status);
    app.get("/:path(.+)", StaticHandler::new());

    let app = app.build().expect("prex app build source");

    let futs = FuturesUnordered::new();

    for addr in self.addrs.iter().cloned() {
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
        info!("static server bound to {}", addr.yellow());
      }

      futs.push({
        let signal = self.shutdown.signal();
        let server = server.serve(app.clone());

        async move {
          tokio::select! {
            _ = signal => Ok(()),
            r = server => r
          }
        }
      });

      //   server
      //     .serve(app.clone())
      //     .with_graceful_shutdown(self.shutdown.signal()),
      // )
    }

    Ok(async move {
      futs.try_collect().await?;
      drop(self);
      Ok(())
    })
  }
}

impl Drop for StaticServer {
  fn drop(&mut self) {
    info!("static server dropped");
  }
}

pub struct StaticHandler {}

impl StaticHandler {
  fn new() -> Self {
    Self {}
  }

  async fn handle(&self, req: Request) -> prex::Response {
    let path = req.param("path").unwrap();

    let entry = match Assets::get(path) {
      None => return prex::Response::new(StatusCode::NOT_FOUND),
      Some(entry) => entry,
    };

    let etag = format!(r#""{}""#, base64::encode(&entry.metadata.sha256_hash()));

    if let Some(req_etag) = req.headers().get(IF_NONE_MATCH) {
      if req_etag.as_bytes() == etag.as_bytes() {
        return prex::Response::new(StatusCode::NOT_MODIFIED);
      }
    }

    let mut res = Response::new(StatusCode::OK);

    let content_type = match mime_guess::from_path(path).first() {
      Some(mime) => HeaderValue::from_str(mime.as_ref()).unwrap(),
      None => HeaderValue::from_static("application/octet-stream"),
    };

    let etag = HeaderValue::from_str(&etag).unwrap();

    res.headers_mut().insert(CONTENT_TYPE, content_type);
    res.headers_mut().insert(ETAG, etag);

    *res.body_mut() = Body::from(entry.data);

    res
  }
}

#[async_trait]
impl Handler for StaticHandler {
  async fn call(&self, req: Request, _: Next) -> prex::Response {
    self.handle(req).await
  }
}
