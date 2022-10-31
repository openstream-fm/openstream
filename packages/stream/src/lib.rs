use async_trait::async_trait;
use channels::ChannelMap;
use cond_count::CondCount;
use hyper::{header::CONTENT_TYPE, http::HeaderValue, Body, Server, StatusCode};
use log::*;
use owo::*;
use prex::{handler::Handler, Next, Request, Response};
use shutdown::Shutdown;
use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::broadcast::error::RecvError;

#[derive(Debug)]
pub struct StreamServer {
  addr: SocketAddr,
  channels: Arc<ChannelMap>,
  shutdown: Shutdown,
  condcount: CondCount,
}

#[derive(Debug)]
struct StreamServerInner {}

impl StreamServer {
  pub fn new<A: Into<SocketAddr>>(
    addr: A,
    channels: Arc<ChannelMap>,
    shutdown: Shutdown,
    condcount: CondCount,
  ) -> Self {
    Self {
      addr: addr.into(),
      channels,
      shutdown,
      condcount,
    }
  }

  pub fn start(
    self,
  ) -> Result<impl Future<Output = Result<(), hyper::Error>> + 'static, hyper::Error> {
    let mut app = prex::prex();

    let handle = StreamHandler::new(self.channels.clone(), self.shutdown.clone());

    app.get("/stream/:id", handle);

    let app = app.build().expect("prex app build stream");

    let server = Server::try_bind(&self.addr)?
      .http1_only(true)
      .http1_title_case_headers(false)
      .http1_preserve_header_case(false);

    info!("stream server bound to {}", self.addr.yellow());

    Ok(async move {
      server
        .serve(app)
        .with_graceful_shutdown(self.shutdown.signal())
        .await?;
      drop(self);
      Ok(())
    })
  }
}

impl Drop for StreamServer {
  fn drop(&mut self) {
    info!("stream server stopped, waiting for resources cleanup");
    self.condcount.wait();
  }
}

#[derive(Debug, Clone)]
struct StreamHandler {
  channels: Arc<ChannelMap>,
  shutdown: Shutdown,
}

impl StreamHandler {
  pub fn new(channels: Arc<ChannelMap>, shutdown: Shutdown) -> Self {
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
