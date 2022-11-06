use async_trait::async_trait;
use channels::ChannelMap;
use cond_count::CondCount;
use futures::future::try_join_all;
use hyper::{header::CONTENT_TYPE, http::HeaderValue, Body, Server, StatusCode};
use log::*;
use owo::*;
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
  condcount: CondCount,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Status {
  status: usize,
}

#[derive(Debug)]
struct StreamServerInner {}

impl StreamServer {
  pub fn new(addrs: Vec<SocketAddr>, shutdown: Shutdown) -> Self {
    let condcount = CondCount::new();
    let channels = ChannelMap::new(condcount.clone());

    Self {
      addrs,
      shutdown,
      channels,
      condcount,
    }
  }

  pub fn start(
    self,
  ) -> Result<impl Future<Output = Result<(), hyper::Error>> + 'static, hyper::Error> {
    let mut futs = vec![];

    for addr in &self.addrs {
      let server = Server::try_bind(&addr)?
        .http1_only(true)
        .http1_title_case_headers(false)
        .http1_preserve_header_case(false);

      info!("stream server bound to {}", addr.yellow());

      let mut app = prex::prex();

      app.get("/status", StatusHandler::new());

      app.get(
        "/stream/:id",
        StreamHandler::new(self.channels.clone(), self.shutdown.clone()),
      );

      let app = app.build().expect("prex app build stream");

      let fut = server
        .serve(app)
        .with_graceful_shutdown(self.shutdown.signal());

      futs.push(fut);
    }

    Ok(async move {
      try_join_all(futs).await?;
      drop(self);
      Ok(())
    })
  }
}

impl Drop for StreamServer {
  fn drop(&mut self) {
    info!("stream server dropped, waiting for resources cleanup");
    self.condcount.wait();
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
      .insert(CONTENT_TYPE, HeaderValue::from_static("audio/mpeg"));
    *res.body_mut() = response_body;

    res
  }
}

#[derive(Debug)]
struct StatusHandler;

#[async_trait]
impl Handler for StatusHandler {
  async fn call(&self, _: Request, _: Next) -> Response {
    let mut res = Response::new(StatusCode::OK);
    let body = Body::from(r#"{"status":200}"#);
    res.set_content_type(HeaderValue::from_static("application/json"));
    res.set_charset(HeaderValue::from_static("utf-8"));
    *res.body_mut() = body;
    res
  }
}

impl StatusHandler {
  fn new() -> Self {
    Self {}
  }
}
