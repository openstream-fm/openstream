use async_trait::async_trait;
use channels::ChannelMap;
use db::account::Account;
use db::stream_connection::StreamConnection;
use db::Model;
use drop_tracer::Token;
use futures::stream::FuturesUnordered;
use futures::TryStreamExt;
use hyper::header::{HeaderName, ACCEPT_RANGES, CACHE_CONTROL};
use hyper::{header::CONTENT_TYPE, http::HeaderValue, Body, Server, StatusCode};
use log::*;
use owo_colors::*;
use prex::{handler::Handler, Next, Request, Response};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
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

  async fn handle(&self, req: Request) -> Result<Response, StreamError> {
    // unwrap: "id" is a required param in path definition
    let id = req.param("id").unwrap();

    let account = match Account::get_by_id(id).await? {
      None => {
        return Err(StreamError::AccountNotFound(id.to_string()));
      }
      Some(account) => account,
    };

    let mut rx = match self.channels.subscribe(id) {
      None => {
        return Err(StreamError::NotStreaming(id.to_string()));
      }
      Some(rx) => rx,
    };

    let mut conn_doc = {
      let now = DateTime::now();
      StreamConnection {
        id: StreamConnection::uid(),
        account_id: account.id,
        connected_at: now,
        last_transfer_at: now,
        request: db::http::Request::from_http(&req),
        state: db::stream_connection::State::Open,
        transfer_bytes: 0,
      }
    };

    if let Err(_e) = StreamConnection::insert(&conn_doc).await {
      let mut res = Response::new(StatusCode::INTERNAL_SERVER_ERROR);
      *res.body_mut() = Body::from("internal server error (db)");
    };

    let connection_dropper = StreamConnectionDropper {
      id: conn_doc.id.clone(),
      token: self.channels.drop_token(),
    };

    let (mut body_sender, response_body) = Body::channel();

    tokio::spawn({
      let shutdown = self.shutdown.clone();

      async move {
        loop {
          let r = rx.recv().await;

          if shutdown.is_closed() {
            break;
          }

          match r {
            // if lagged we ignore the error and continue with the oldest message buffered in the channel
            // TODO: maybe we should advance to the newest message with stream.resubscribe()
            Err(RecvError::Lagged(_)) => continue,

            // Here the channel has been dropped
            Err(RecvError::Closed) => break,

            // Receive bytes and pass it to response body
            Ok(bytes) => {
              let len = bytes.len();
              match body_sender.send_data(bytes).await {
                Err(_) => break,
                Ok(()) => {
                  conn_doc.transfer_bytes += len as u64;
                  let _ =
                    StreamConnection::set_transfer_bytes(&conn_doc.id, conn_doc.transfer_bytes);
                }
              };
            }
          }
        }

        drop(connection_dropper);
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

    Ok(res)
  }
}

#[async_trait]
impl Handler for StreamHandler {
  async fn call(&self, req: Request, _: Next) -> Response {
    self.handle(req).await.into()
  }
}

#[derive(Debug)]
struct StreamConnectionDropper {
  id: String,
  token: Token,
}

impl Drop for StreamConnectionDropper {
  fn drop(&mut self) {
    let token = self.token.clone();
    let id = self.id.clone();
    tokio::spawn(async move {
      let _ = StreamConnection::set_closed(&id, None).await;
      drop(token);
    });
  }
}

pub enum StreamError {
  Db(mongodb::error::Error),
  AccountNotFound(String),
  NotStreaming(String),
}

impl From<mongodb::error::Error> for StreamError {
  fn from(e: mongodb::error::Error) -> Self {
    Self::Db(e)
  }
}

impl From<StreamError> for Response {
  fn from(e: StreamError) -> Self {
    let (status, code, message) = match e {
      StreamError::Db(_e) => (
        StatusCode::INTERNAL_SERVER_ERROR,
        "INTERNAL_DB",
        "internal server error".into(),
      ),

      StreamError::AccountNotFound(id) => (
        StatusCode::NOT_FOUND,
        "NO_ACCOUNT",
        format!("station with id {id} not found"),
      ),

      StreamError::NotStreaming(id) => (
        StatusCode::MISDIRECTED_REQUEST,
        "WRONG_SERVER",
        format!("station with id {id} is not streaming from this server"),
      ),
    };

    let mut res = Response::new(status);
    res.headers_mut().append(
      CONTENT_TYPE,
      HeaderValue::from_static("text/plain;charset=utf-8"),
    );

    res.headers_mut().append(
      HeaderName::from_static("x-openstream-rejection-code"),
      HeaderValue::from_static(code),
    );

    *res.body_mut() = Body::from(message);

    res
  }
}
