use async_trait::async_trait;
use channels::ChannelMap;
use hyper::{header::CONTENT_TYPE, http::HeaderValue, Body, Server, StatusCode};
use log::*;
use owo::*;
use prex::{handler::Handler, Next, Request, Response};
use std::future::Future;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::{broadcast::error::RecvError, Notify};

const OPEN: bool = false;
const CLOSED: bool = true;

#[derive(Clone, Debug)]
pub struct StreamServer {
  inner: Arc<StreamServerInner>,
}

#[derive(Debug)]
struct StreamServerInner {
  addr: SocketAddr,
  channels: Arc<ChannelMap>,
  shutdown_signal: Notify,
  closed: AtomicBool,
}

impl StreamServer {
  pub fn new<A: Into<SocketAddr>>(addr: A, channels: Arc<ChannelMap>) -> Self {
    Self {
      inner: Arc::new(StreamServerInner {
        addr: addr.into(),
        channels,
        shutdown_signal: Notify::new(),
        closed: AtomicBool::new(OPEN),
      }),
    }
  }

  pub fn start(
    &self,
  ) -> Result<impl Future<Output = Result<(), hyper::Error>> + 'static, hyper::Error> {
    let mut app = prex::prex();

    let handle = StreamHandler::new(self.clone());

    app.get("/stream/:id", handle);

    let app = app.build().expect("prex app build stream");

    let signal = {
      let me = self.clone();
      async move {
        me.inner.shutdown_signal.notified().await;
      }
    };

    let server = Server::try_bind(&self.inner.addr)?;

    info!("stream server bound to {}", self.inner.addr.yellow());

    Ok(async move {
      server.serve(app).with_graceful_shutdown(signal).await?;
      Ok(())
    })
  }

  pub fn graceful_shutdown(&self) {
    self.inner.closed.store(CLOSED, Ordering::SeqCst);
    self.inner.shutdown_signal.notify_waiters();
  }
}

#[derive(Debug, Clone)]
struct StreamHandler {
  server: StreamServer,
}

impl StreamHandler {
  pub fn new(server: StreamServer) -> Self {
    Self { server }
  }
}

#[async_trait]
impl Handler for StreamHandler {
  async fn call(&self, req: Request, _: Next) -> Response {
    // unwrap: "id" is a required param in path definition
    let id = req.param("id").unwrap();

    let mut stream = match self.server.inner.channels.subscribe(id) {
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
      let me = self.clone();
      async move {
        loop {
          let r = stream.recv().await;

          if me.server.inner.closed.load(Ordering::SeqCst) == CLOSED {
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
