use async_trait::async_trait;
use db::account::Account;
use db::audio_file::AudioFile;
use db::stream_connection::StreamConnection;
use db::Model;
use drop_tracer::{Token, DropTracer};
use futures::stream::FuturesUnordered;
use futures::TryStreamExt;
use hyper::header::{HeaderName, ACCEPT_RANGES, CACHE_CONTROL};
use hyper::{header::CONTENT_TYPE, http::HeaderValue, Body, Server, StatusCode};
use log::*;
use media_sessions::playlist::run_playlist_session;
use media_sessions::MediaSessionMap;
use media_sessions::RecvError;
use mongodb::bson::doc;
use prex::{handler::Handler, Next, Request, Response};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use shutdown::Shutdown;
use transfer_map::TransferTracer;
use std::future::Future;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering, AtomicBool};
use std::sync::Arc;
use socket2::{Domain, Protocol, Socket, Type};
use ip_counter::IpCounter;

pub mod transfer_map;

#[allow(clippy::declare_interior_mutable_const)]
const X_OPENSTREAM_REJECTION_CODE: HeaderName =
  HeaderName::from_static("x-openstream-rejection-code");

#[allow(clippy::declare_interior_mutable_const)]
const CONTENT_TYPE_MPEG: HeaderValue = HeaderValue::from_static("audio/mpeg");

#[allow(clippy::declare_interior_mutable_const)]
const ACCEPT_RANGES_NONE: HeaderValue = HeaderValue::from_static("none");

#[allow(clippy::declare_interior_mutable_const)]
const CACHE_CONTROL_NO_CACHE: HeaderValue = HeaderValue::from_static("no-cache");

#[allow(clippy::declare_interior_mutable_const)]
const TEXT_PLAIN_UTF8: HeaderValue = HeaderValue::from_static("text/plain;charset=utf-8");

#[derive(Debug)]
pub struct StreamServer {
  addrs: Vec<SocketAddr>,
  media_sessions: MediaSessionMap,
  shutdown: Shutdown,
  drop_tracer:  DropTracer,
  transfer_map: TransferTracer,
  ip_counter: IpCounter,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Status {
  status: usize,
}

#[derive(Debug)]
struct StreamServerInner {}

#[derive(Debug, thiserror::Error)]
pub enum StreamServerError {
  #[error("io error: {0}")]
  Io(#[from] std::io::Error),
  #[error("hyper error: {0}")]
  Hyper(#[from] hyper::Error)
}

impl StreamServer {
  pub fn new(addrs: Vec<SocketAddr>, shutdown: Shutdown, media_sessions: MediaSessionMap) -> Self {
    Self {
      addrs,
      shutdown,
      drop_tracer: DropTracer::new(),
      media_sessions,
      transfer_map: TransferTracer::new(),
      ip_counter: IpCounter::new(),
    }
  }

  pub fn start(
    self,
  ) -> Result<impl Future<Output = Result<(), hyper::Error>> + 'static, StreamServerError> {
    let mut app = prex::prex();

    app.with(http::middleware::server);
    app.get("/status", http::middleware::status);

    app.get(
      "/stream/:id",
      StreamHandler::new(
        self.media_sessions.clone(),
        self.transfer_map.clone(),
        self.shutdown.clone(),
        self.drop_tracer.clone(),
        self.ip_counter.clone()
      ),
    );

    let app = app.build().expect("prex app build stream");

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

      socket.set_reuse_address(true)?;
      socket.set_reuse_port(true)?;

      socket.bind(&addr.into())?;
      socket.listen(128)?;

      let tcp = socket.into();


      let server = Server::from_tcp(tcp)?
        .http1_only(true)
        .http1_title_case_headers(false)
        .http1_preserve_header_case(false)
        .http1_keepalive(false);

      {
        use owo_colors::*;
        info!("stream server bound to {}", addr.yellow());
      }

      let fut = server
        .serve(app.clone())
        .with_graceful_shutdown(self.shutdown.signal());

      futs.push(fut);
    }

    Ok(async move {
      self.transfer_map.start_background_task();
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
  media_sessions: MediaSessionMap,
  transfer_map: TransferTracer,
  shutdown: Shutdown,
  drop_tracer: DropTracer,
  ip_counter: IpCounter,
}

impl StreamHandler {
  pub fn new(media_sessions: MediaSessionMap, transfer_map: TransferTracer, shutdown: Shutdown, drop_tracer: DropTracer, ip_counter: IpCounter) -> Self {
    Self {
      media_sessions,
      transfer_map,
      shutdown,
      drop_tracer,
      ip_counter,
    }
  }

  async fn handle(self, req: Request) -> Result<Response, StreamError> {
    let account_id = req.param("id").unwrap().to_string();

    let ip = req.isomorphic_ip();

    // we do not trace ip counts from internal net or loopback ips (not global)
    
    // TODO:
    // provide a way to whitelist global ips (from openstream itself or for test purposes)
    // maybe with a internal password for each stream in the provided
    // by the client as a request header
    let _ip_count = match ip_rfc::global(&ip) {
      false => 0,
      true => match self.ip_counter.increment_with_limit(ip, constants::STREAM_IP_CONNECTIONS_LIMIT) {
        Some(n) => n,
        None => {
          return Err(StreamError::TooManyOpenConnections)
        }
      }
    };

    let ip_decrementer = {
      let map = self.ip_counter.clone();
      defer::defer(move || {
        map.decrement(ip);
      })
    };

    tokio::spawn(async move {
      let account = match Account::get_by_id(&account_id).await? {
        Some(account) => account,
        None => return Err(StreamError::AccountNotFound(account_id.to_string())),
      };

      #[allow(clippy::collapsible_if)]
      if self.media_sessions.read().get(&account_id).is_none() {
        if !AudioFile::exists(doc! { AudioFile::KEY_ACCOUNT_ID: &account.id }).await? {
          return Err(StreamError::NotStreaming(account.id.clone()));
        }
      };

      let mut rx = {
        let lock = self.media_sessions.upgradable_read();

        match lock.get(&account_id) {
          Some(session) => session.subscribe(),

          None => {
            let mut lock = lock.upgrade();
            let tx = lock.transmit(&account_id, media_sessions::MediaSessionKind::Playlist {});
            let rx = tx.subscribe();
            run_playlist_session(tx, self.shutdown.clone(), self.drop_tracer.clone(), true);
            rx
          }
        }
      };

      let conn_doc = {
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

      let r = Account::increment_used_listeners(&account_id).await?;
      debug!("Account::increment_used_listeners called for account {account_id}, matched: {matched}, modified: {modified}", matched=r.matched_count, modified=r.modified_count);

      StreamConnection::insert(&conn_doc).await?;
      debug!("StreamConnection::insert called for account {account_id}, connection_id: {}", conn_doc.id);
      
      let transfer_bytes = Arc::new(AtomicU64::new(0));
      let closed = Arc::new(AtomicBool::new(false));

      let connection_dropper = StreamConnectionDropper {
        id: conn_doc.id.clone(),
        transfer_bytes: transfer_bytes.clone(),
        account_id: account_id.clone(),
        token: self.media_sessions.drop_token(),
      };

      // {
      //   let transfer_bytes = transfer_bytes.clone();
      //   let closed = closed.clone();
      //   let id = conn_doc.id;
      //   tokio::spawn(async move {
      //     use tokio::time::Duration;
      //     let mut last = 0;
      //     loop {
      //       tokio::time::sleep(Duration::from_secs(5)).await;
            
      //       if closed.load(Ordering::SeqCst) {
      //         break;
      //       }

      //       let v = transfer_bytes.load(Ordering::SeqCst);
      //       if v != last {
      //         last = v;
      //         let r = StreamConnection::set_transfer_bytes(&id, v).await;
      //         if let Err(e) = r {
      //           warn!("error calling StreamConnection::set_transfer_bytes for connection {id}: {e}");
      //         }
      //       };
      //     }
      //   });
      // }

      let (mut body_sender, response_body) = Body::channel();

      tokio::spawn({
        let shutdown = self.shutdown.clone();
        let transfer_map = self.transfer_map.clone();

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
                if shutdown.is_closed() {
                  break;
                }

                let len = bytes.len();
                match body_sender.send_data(bytes).await {
                  Err(_) => break,
                  Ok(()) => {
                    transfer_map.increment(&account_id, len);
                    transfer_bytes.fetch_add(len as u64, Ordering::SeqCst);
                  }
                };
              }
            }
          }

          closed.store(true, Ordering::SeqCst);
          drop(connection_dropper);
          drop(ip_decrementer);
        }
      });

      let mut res = Response::new(StatusCode::OK);
      res
        .headers_mut()
        .append(CONTENT_TYPE,  CONTENT_TYPE_MPEG);

      res
        .headers_mut()
        .append(ACCEPT_RANGES, ACCEPT_RANGES_NONE);

      res
        .headers_mut()
        .append(CACHE_CONTROL, CACHE_CONTROL_NO_CACHE);

      *res.body_mut() = response_body;

      Ok(res)
    })
    .await
    .unwrap()
  }
}

#[async_trait]
impl Handler for StreamHandler {
  async fn call(&self, req: Request, _: Next) -> Response {
    self.clone().handle(req).await.into()
  }
}

#[derive(Debug)]
struct StreamConnectionDropper {
  id: String,
  account_id: String,
  transfer_bytes: Arc<AtomicU64>,
  token: Token,
}

impl Drop for StreamConnectionDropper {
  fn drop(&mut self) {
    let token = self.token.clone();
    let id = self.id.clone();
    let account_id = self.account_id.clone();
    let transfer_bytes = self.transfer_bytes.load(Ordering::SeqCst);
    tokio::spawn(async move {
      let r = StreamConnection::set_closed(&id, Some(transfer_bytes))
        .await
        .expect("error at StreamConnection::set_closed");

      debug!("StreamConnection::set_closed called for account {account_id}, matched: {matched}, modified: {modified}", matched=r.matched_count, modified=r.modified_count);

      let r = Account::decrement_used_listeners(&account_id)
        .await
        .expect("error at Account::decrement_used_listeners");

      debug!("Account::decrement_used_listeners called for account {account_id}, matched: {matched}, modified: {modified}", matched=r.matched_count, modified=r.modified_count);

      drop(token);
    });
  }
}

pub enum StreamError {
  Db(mongodb::error::Error),
  AccountNotFound(String),
  NotStreaming(String),
  TooManyOpenConnections,
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
        "NOT_STREAMING",
        format!("station with id {id} is not streaming from this server"),
      ),

      StreamError::TooManyOpenConnections => (
        StatusCode::TOO_MANY_REQUESTS,
        "TOO_MANY_CONNECTIONS",
        "Too many open connections from your network, close some connections or try again later".into(),
      )
    };

    let mut res = Response::new(status);

    res.headers_mut().append(
      CONTENT_TYPE,
      TEXT_PLAIN_UTF8
    );

    res
      .headers_mut()
      .append(X_OPENSTREAM_REJECTION_CODE, HeaderValue::from_static(code));

    *res.body_mut() = Body::from(message);

    res
  }
}
