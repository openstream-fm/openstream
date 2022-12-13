use async_trait::async_trait;
use channels::ChannelMap;
use futures::stream::FuturesUnordered;
use futures::TryStreamExt;
use hyper::header::{ACCEPT_RANGES, CACHE_CONTROL};
use hyper::{header::CONTENT_TYPE, http::HeaderValue, Body, Server, StatusCode};
use log::*;
use owo_colors::*;
use prex::{handler::Handler, Next, Request, Response};
use serde::{Deserialize, Serialize};
use shutdown::Shutdown;
use std::future::Future;
use std::net::SocketAddr;
use tokio::sync::broadcast::error::RecvError;

#[derive(Debug)]
pub struct StreamServer {
  addrs: Vec<SocketAddr>,
  channels: ChannelMap,
  shutdown: Shutdown,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Status {
  status: usize,
}

#[derive(Debug)]
struct StreamServerInner {}

impl StreamServer {
  pub fn new(addrs: Vec<SocketAddr>, shutdown: Shutdown) -> Self {
    let channels = ChannelMap::new();

    Self {
      addrs,
      shutdown,
      channels,
    }
  }

  pub fn start(
    self,
  ) -> Result<impl Future<Output = Result<(), hyper::Error>> + 'static, hyper::Error> {
    let mut app = prex::prex();

    app.with(http::middleware::server);
    app.get("/status", http::middleware::status);

    app.get(
      "/stream/:id",
      StreamHandler::new(self.channels.clone(), self.shutdown.clone()),
    );

    let app = app.build().expect("prex app build stream");

    let futs = FuturesUnordered::new();

    for addr in &self.addrs {
      let server = Server::try_bind(addr)?
        .http1_only(true)
        .http1_title_case_headers(false)
        .http1_preserve_header_case(false)
        .http1_keepalive(false);

      info!("stream server bound to {}", addr.yellow());

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

impl Drop for StreamServer {
  fn drop(&mut self) {
    info!("stream server dropped");
  }
}

#[derive(Debug, Clone)]
struct StreamHandler {
  channels: ChannelMap,
  shutdown: Shutdown,
}

impl StreamHandler {
  pub fn new(channels: ChannelMap, shutdown: Shutdown) -> Self {
    Self { channels, shutdown }
  }
}

#[async_trait]
impl Handler for StreamHandler {
  async fn call(&self, req: Request, _: Next) -> Response {
    // unwrap: "id" is a required param in path definition
    let id = req.param("id").unwrap();

    let mut stream = match self.channels.subscribe(id) {
      Some(stream) => stream,
      None => {
        let mut res = Response::new(StatusCode::NOT_FOUND);
        *res.body_mut() = Body::from(format!(
          "stream with id {id} is not actively streaming right now"
        ));
        return res;
      }
    };

    let (mut body_sender, response_body) = Body::channel();

    tokio::spawn({
      let shutdown = self.shutdown.clone();
      async move {
        loop {
          let r = stream.recv().await;

          if shutdown.is_closed() {
            break;
          }

          match r {
            // if lagged we ignore the error and continue with the oldest message buffered in the channel
            // TODO: maybe we should advance to the newest message with stream.resubscribe
            Err(RecvError::Lagged(_)) => continue,

            // Here the channel has been dropped
            Err(RecvError::Closed) => break,

            // Receive bytes and pass it to response body
            Ok(bytes) => {
              match body_sender.send_data(bytes).await {
                Err(_) => break,
                Ok(()) => continue,
              };
            }
          }
        }
      }
    });

    let mut res = Response::new(StatusCode::OK);
    res
      .headers_mut()
      .append(CONTENT_TYPE, HeaderValue::from_static("audio/mpeg"));

    res
      .headers_mut()
      .append(ACCEPT_RANGES, HeaderValue::from_static("none"));

    res
      .headers_mut()
      .append(CACHE_CONTROL, HeaderValue::from_static("no-cache"));

    *res.body_mut() = response_body;

    res
  }
}
