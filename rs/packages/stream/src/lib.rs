use async_trait::async_trait;
use constants::HEADER_RELAY_SOURCE_DEPLOYMENT;
use db::account::Account;
use db::station::Station;
use db::stream_connection::lite::StreamConnectionLite;
use db::stream_connection::StreamConnection;
use db::Model;
use drop_tracer::{DropTracer, Token};
use futures::stream::FuturesUnordered;
use futures::TryStreamExt;
use hyper::header::{HeaderName, ACCEPT_RANGES, CACHE_CONTROL, RETRY_AFTER, LOCATION};
use hyper::{header::CONTENT_TYPE, http::HeaderValue, Body, Server, StatusCode};
use ip_counter::IpCounter;
use log::*;
use media::channel::RecvError;
use media::handle::internal_relay::GetInternalRelayError;
use media::{MediaSessionMap, SubscribeError};
use mongodb::bson::doc;
use parking_lot::Mutex;
use prex::{handler::Handler, Next, Request, Response};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use shutdown::Shutdown;
use socket2::{Domain, Protocol, Socket, Type};
use std::fmt::Display;
use std::future::Future;
use std::net::{IpAddr, SocketAddr};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Instant, SystemTime};
use tokio::time::{sleep, Duration};
use transfer_map::TransferTracer;

mod error;
pub mod transfer_map;

#[allow(clippy::declare_interior_mutable_const)]
const CONTENT_TYPE_MPEG: HeaderValue = HeaderValue::from_static("audio/mpeg");

#[allow(clippy::declare_interior_mutable_const)]
const CONTENT_TYPE_X_MPEG_URL: HeaderValue = HeaderValue::from_static("audio/x-mpegurl");

#[allow(clippy::declare_interior_mutable_const)]
const CONTENT_TYPE_PLS: HeaderValue = HeaderValue::from_static("audio/x-scpls");

#[allow(clippy::declare_interior_mutable_const)]
const ACCEPT_RANGES_NONE: HeaderValue = HeaderValue::from_static("none");

#[allow(clippy::declare_interior_mutable_const)]
const CACHE_CONTROL_NO_CACHE: HeaderValue = HeaderValue::from_static("no-cache");

#[allow(clippy::declare_interior_mutable_const)]
const TEXT_PLAIN_UTF8: HeaderValue = HeaderValue::from_static("text/plain;charset=utf-8");

#[derive(Debug)]
pub struct StreamServer {
  deployment_id: String,
  addrs: Vec<SocketAddr>,
  media_sessions: MediaSessionMap,
  shutdown: Shutdown,
  drop_tracer: DropTracer,
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
  Hyper(#[from] hyper::Error),
}

impl StreamServer {
  pub fn new(
    deployment_id: String,
    addrs: Vec<SocketAddr>,
    shutdown: Shutdown,
    drop_tracer: DropTracer,
    media_sessions: MediaSessionMap,
  ) -> Self {
    Self {
      deployment_id,
      addrs,
      shutdown,
      drop_tracer,
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

    let cors = prex::middleware::cors::cors()
      .allow_origin("*")
      .allow_methods("GET");

    app.with(cors);

    app.get("/status", http::middleware::status);

    app.get(
      "/stream/:id.:ext(m3u8?)",
      LinkHandler::new(LinkHandlerKind::M3u),
    );

    app.get("/stream/:id.pls", LinkHandler::new(LinkHandlerKind::Pls));

    app.get(
      "/stream/:id",
      StreamHandler::new(
        self.deployment_id.clone(),
        self.media_sessions.clone(),
        self.transfer_map.clone(),
        self.shutdown.clone(),
        self.drop_tracer.clone(),
        self.ip_counter.clone(),
      ),
    );

    app.get(
      "/relay/:id",
      RelayHandler::new(self.deployment_id.clone(), self.media_sessions.clone()),
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
struct RelayHandler {
  deployment_id: String,
  media_sessions: MediaSessionMap,
}

impl RelayHandler {
  pub fn new(deployment_id: String, media_sessions: MediaSessionMap) -> Self {
    Self {
      deployment_id,
      media_sessions,
    }
  }

  pub async fn handle(&self, req: Request) -> Result<Response, StreamError> {
    let station_id = req.param("id").unwrap().to_string();

    let source_deployment_id = match req.headers().get(HEADER_RELAY_SOURCE_DEPLOYMENT) {
      None => None,
      Some(v) => match v.to_str() {
        Err(_) => None,
        Ok(v) => Some(v),
      },
    };

    if let Some(id) = source_deployment_id {
      if id == self.deployment_id {
        return Err(StreamError::RelayDeploymentSourceIsTarget);
      }
    }

    // match req.headers().get(X_OPENSTREAM_RELAY_CODE) {
    //   None => return Err(RelayError::RelayCodeMismatch),
    //   Some(v) => match v.to_str() {
    //     Err(_) => {
    //       return Err(RelayError::RelayCodeMismatch);
    //     }
    //     Ok(v) => {
    //       if v != self.deployment_id {
    //         return Err(RelayError::RelayCodeMismatch);
    //       }
    //     }
    //   },
    // }

    let mut rx = self.media_sessions.subscribe(&station_id).await?;

    let content_type = rx.content_type().to_string();

    let (mut sender, body) = hyper::Body::channel();

    tokio::spawn(async move {
      loop {
        match rx.recv().await {
          Err(RecvError::Lagged(_)) => {
            warn!(
              target: "internal-relay-tx",
              "internal-relay session for station {} lagged",
              station_id,
            );
            continue;
          }

          Err(RecvError::Closed) => {
            info!(
              target: "internal-relay-tx",
              "internal-relay session for station {} recv closed",
              station_id,
            );
            break;
          }

          Ok(bytes) => match sender.send_data(bytes).await {
            Err(e) => {
              info!(
                target: "internal-relay-tx",
                "internal-relay session for station {} body send error: {} => {:?}",
                station_id,
                e,
                e
              );

              break;
            }

            Ok(()) => continue,
          },
        }
      }
    });

    let mut res = Response::new(StatusCode::OK);
    *res.body_mut() = body;
    if let Ok(v) = HeaderValue::from_str(&content_type) {
      res.headers_mut().append("content-type", v);
    }

    Ok(res)
  }
}

#[async_trait]
impl Handler for RelayHandler {
  async fn call(&self, req: Request, _: Next) -> Response {
    self.handle(req).await.into()
  }
}

#[derive(Debug, Clone)]
struct StreamHandler {
  deployment_id: String,
  media_sessions: MediaSessionMap,
  transfer_map: TransferTracer,
  shutdown: Shutdown,
  drop_tracer: DropTracer,
  ip_counter: IpCounter,
}

impl StreamHandler {
  pub fn new(
    deployment_id: String,
    media_sessions: MediaSessionMap,
    transfer_map: TransferTracer,
    shutdown: Shutdown,
    drop_tracer: DropTracer,
    ip_counter: IpCounter,
  ) -> Self {
    Self {
      deployment_id,
      media_sessions,
      transfer_map,
      shutdown,
      drop_tracer,
      ip_counter,
    }
  }

  async fn handle(self, req: Request) -> Result<Response, StreamError> {
    let start_time = SystemTime::now();

    let Self {
      deployment_id,
      media_sessions,
      drop_tracer,
      ip_counter,
      shutdown,
      transfer_map,
    } = self;

    let station_id = req.param("id").unwrap().to_string();

    tokio::spawn(async move {
      let ip = req.isomorphic_ip();

      // we do not trace ip counts from internal net or loopback ips (not global)

      match ip_rfc::global(&ip) {
        false => {}
        true => match ip_counter.increment_with_limit(ip, constants::STREAM_IP_CONNECTIONS_LIMIT) {
          Some(_) => {}
          None => return Err(StreamError::TooManyOpenIpConnections),
        },
      };

      let ip_decrementer = {
        defer::defer(move || {
          ip_counter.decrement(ip);
        })
      };

      let station = match Station::get_by_id(&station_id).await? {
        Some(station) => station,
        None => return Err(StreamError::StationNotFound(station_id.to_string())),
      };

      // TODO: check account limits
      let rx = media_sessions.subscribe(&station.id).await?;
      let content_type = rx.content_type().to_string();

      let is_mp3 = content_type == "audio/mpeg";

      let conn_doc = {
        let now = DateTime::now();
        let request = db::http::Request::from_http(&req);

        StreamConnection {
          id: StreamConnection::uid(),
          station_id: station.id.clone(),
          deployment_id: deployment_id.clone(),
          is_open: true,
          ip: request.real_ip,
          country_code: request.country_code,
          transfer_bytes: None,
          duration_ms: None,
          request,
          created_at: now,
          last_transfer_at: now,
          closed_at: None,
        }
      };

      let conn_doc_lite = StreamConnectionLite::from_stream_connection_ref(&conn_doc);

      StreamConnection::insert(&conn_doc).await?;
      debug!(
        "StreamConnection::insert called for station {station_id}, connection_id: {}",
        conn_doc.id
      );

      StreamConnectionLite::insert(&conn_doc_lite).await?;
      debug!(
        "StreamConnectionLite::insert called for station {station_id}, connection_id: {}",
        conn_doc_lite.id
      );

      Account::increment_used_listeners(&station.account_id).await?;

      let transfer_bytes = Arc::new(AtomicU64::new(0));

      let conn_id = conn_doc.id;
      let end_reason = Arc::new(Mutex::new(EndReason::None));

      let station_name = station.name;
      let ip = conn_doc.ip;
      let domain = conn_doc_lite.domain.clone();

      let connection_dropper = StreamConnectionDropper(Some(StreamConnectionDropperInner {
        id: conn_id.clone(),
        ip,
        station_id: station_id.clone(),
        station_name: station_name.clone(),
        domain: domain.clone(),
        account_id: station.account_id.clone(),
        transfer_bytes: transfer_bytes.clone(),
        token: drop_tracer.token(),
        start_time,
        end_reason: end_reason.clone(),
      }));

      let (mut body_sender, response_body) = Body::channel();

      tokio::spawn({
        let transfer_map = transfer_map.clone();
        let mut rx = rx;

        let mut loop_i = 0usize;

        async move {
          
          let task = async {
            
            let mut is_first_chunk = true;

            info!("START conn {conn_id} {ip} - {station_id} ({station_name}) in {domain}", domain = domain.as_deref().unwrap_or("???"),);

            'root: loop {
              let loop_start = Instant::now();
              let mut rx_had_data = false;

              if loop_i != 0 {
                info!("LOOP {loop_i} conn {conn_id} {ip} - {station_id} ({station_name})");
              }

              loop_i += 1;

              'recv: loop {
                let r = tokio::select! {
                  r = rx.recv() => r,
                  _ = sleep(Duration::from_millis(100_000)) => return EndReason::RecvTimeout,
                };

                match r {
                  // Here the channel has been dropped
                  Err(RecvError::Closed) => break 'recv,

                  // if lagged we ignore the error and continue with the oldest message buffered in the channel
                  // TODO: maybe we should advance to the newest message with stream.resubscribe()
                  Err(RecvError::Lagged(_)) => {
                    // break this receiver if lagged behind more than 1 minute
                    continue 'recv;
                  }

                  // Receive bytes and pass it to response body
                  Ok(bytes) => {
                    rx_had_data = true;

                    let len = bytes.len();

                    let bytes = {
                    
                      if is_mp3 && is_first_chunk {

                        is_first_chunk = false;
                    
                        match find_first_frame_index(&bytes) {
                          None => {
                            // info!("FRAME_START_SLICE conn {conn_id} {ip} - station: {station_id} ({station_name}) | frame start index not found");
                            bytes
                          },

                          Some(i) => {
                            // info!("FRAME_START_SLICE conn {conn_id} {ip} - station: {station_id} ({station_name}) - {i} bytes");
                            bytes.slice(i..)
                          }
                        }
                    
                      } else {
                        bytes
                      }
                    };

                    let r = tokio::select! {
                      _ = sleep(Duration::from_millis(100_000)) => return EndReason::BodySendTimeout,
                      r = body_sender.send_data(bytes) => r,
                    };

                    match r {
                      Err(e) => return EndReason::BodySendError(e),
                      Ok(()) => {
                        transfer_map.increment(&station.account_id, len);
                        transfer_bytes.fetch_add(len as u64, Ordering::Relaxed);
                      }
                    };
                  }
                }
              }

              // if the connection had last < 5 secs
              // or had no data we abort to
              // avoid creating infinite loops here
              // if (loop_start.elapsed().as_secs() > 5) && rx_had_data && (loop_i <= 60) {
              // } else {
              if loop_start.elapsed().as_secs() < 5 {
                return EndReason::NoRestartSecs
              } else if !rx_had_data {
                return EndReason::NoRestartData
              } else if loop_i > 60 {
                return EndReason::NoRestartLoop
              } else {
                let new_rx = match media_sessions.subscribe(&station.id).await {
                  Ok(rx) => rx,
                  Err(e) => {
                    return EndReason::Resubscribe(e)
                  }
                };
                rx = new_rx;
                continue 'root;  
              }
            };
          };

          let signal = shutdown.signal();
          let max_time_timer = tokio::time::sleep(Duration::from_secs(
            constants::STREAM_CONNECTION_MAX_DURATION_SECS,
          ));

          let end = tokio::select! {
            _ = signal => EndReason::Shutdown,
            _ = max_time_timer => EndReason::MaxTime,
            end = task => end,
          };

          {
            *end_reason.lock() = end;
          }

          drop(connection_dropper);
          drop(ip_decrementer);

          Ok::<(), StreamError>(())
        }
      });

      let mut res = Response::new(StatusCode::OK);
      res.headers_mut().append(
        CONTENT_TYPE,
        HeaderValue::from_str(&content_type).unwrap_or(CONTENT_TYPE_MPEG),
      );

      res.headers_mut().append(ACCEPT_RANGES, ACCEPT_RANGES_NONE);

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
struct StreamConnectionDropper(Option<StreamConnectionDropperInner>);

#[derive(Debug)]
struct StreamConnectionDropperInner {
  id: String,
  ip: IpAddr,
  domain: Option<String>,
  station_name: String,
  station_id: String,
  account_id: String,
  transfer_bytes: Arc<AtomicU64>,
  start_time: SystemTime,
  end_reason: Arc<Mutex<EndReason>>,
  token: Token,
}

#[derive(Debug)]
pub enum EndReason {
  None,
  Shutdown,
  RecvTimeout,
  BodySendTimeout,
  BodySendError(hyper::Error),
  MaxTime,
  NoRestartSecs,
  NoRestartData,
  NoRestartLoop,
  Resubscribe(SubscribeError),
}

impl Display for EndReason {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      EndReason::None => f.write_str("none"),
      EndReason::Shutdown => f.write_str("shutdown"),
      EndReason::RecvTimeout => f.write_str("recv-timeout"),
      EndReason::BodySendTimeout => f.write_str("body-send-timeout"),
      EndReason::BodySendError(e) => write!(f, "body-send-error: {e} => {e:?}"),
      EndReason::MaxTime => f.write_str("max-time"),
      EndReason::NoRestartSecs => f.write_str("no-restart-secs"),
      EndReason::NoRestartData => f.write_str("no-restart-data"),
      EndReason::NoRestartLoop => f.write_str("no-restart-loop"),
      EndReason::Resubscribe(e) => write!(f, "resubscribe: {}", e),
    }
  }
}

impl Drop for StreamConnectionDropper {
  fn drop(&mut self) {
    let StreamConnectionDropperInner {
      id,
      ip,
      station_name,
      station_id,
      domain,
      account_id,
      transfer_bytes,
      start_time,
      end_reason,
      token,
    } = match self.0.take() {
      None => return,
      Some(v) => v,
    };

    let transfer_bytes = transfer_bytes.load(Ordering::SeqCst);
    let duration_ms = start_time.elapsed().unwrap().as_millis() as u64;
    let now = DateTime::now();

    {
      let domain = domain.as_deref().unwrap_or("???");
      let s = duration_ms as f64 / 1_000.0;
      let end_reason = end_reason.lock();
      info!("END conn {id} {ip} - {station_id} ({station_name}) in {domain} | reason={end_reason} - {s} s");
    }

    tokio::spawn(async move {
      {
        let update = doc! {
          "$set": {
            StreamConnection::KEY_IS_OPEN: false,
            StreamConnection::KEY_DURATION_MS: duration_ms as f64,
            StreamConnection::KEY_TRANSFER_BYTES: transfer_bytes as f64,
            StreamConnection::KEY_CLOSED_AT: now,
          }
        };

        let r = StreamConnection::update_by_id(&id, update)
          .await
          .expect("error updatting StreamConnection document");

        debug!("StreamConnection closed for station {station_id}, matched: {matched}, modified: {modified}", matched=r.matched_count, modified=r.modified_count);
      }

      {
        let update_lite = doc! {
          "$set": {
            StreamConnectionLite::KEY_IS_OPEN: false,
            StreamConnectionLite::KEY_DURATION_MS: duration_ms as f64,
            StreamConnectionLite::KEY_TRANSFER_BYTES: transfer_bytes as f64,
            StreamConnectionLite::KEY_CLOSED_AT: now,
          }
        };

        let r = StreamConnectionLite::update_by_id(&id, update_lite)
          .await
          .expect("error updating StreamConnectionLite document");

        debug!("StreamConnectionLite closed for station {station_id}, matched: {matched}, modified: {modified}", matched=r.matched_count, modified=r.modified_count);
      }

      let r = Account::decrement_used_listeners(&account_id)
        .await
        .expect("error at Account::decrement_used_listeners");

      debug!("Account::decrement_used_listeners called for account {account_id}, matched: {matched}, modified: {modified}", matched=r.matched_count, modified=r.modified_count);

      drop(token);
    });
  }
}

#[derive(Debug, thiserror::Error)]
pub enum StreamError {
  #[error("db: {0}")]
  Db(#[from] mongodb::error::Error),
  #[error("station not found: {0}")]
  StationNotFound(String),
  #[error("to many open ip connections")]
  TooManyOpenIpConnections,
  #[error("subscribe: {0}")]
  Subscribe(#[from] SubscribeError),
  #[error("deployment target is source")]
  RelayDeploymentSourceIsTarget,
}

impl From<StreamError> for Response {
  fn from(e: StreamError) -> Self {
    let (status, code, message, retry_after_secs) = match e {
      
      StreamError::Db(_e) => (
        StatusCode::INTERNAL_SERVER_ERROR,
        "INTERNAL_DB",
        "internal server error (db1)".into(),
        None,
      ),

      StreamError::StationNotFound(id) => (
        StatusCode::NOT_FOUND,
        "STATION_NOT_FOUND",
        format!("station with id {id} not found"),
        None,
      ),

      StreamError::TooManyOpenIpConnections => (
        StatusCode::TOO_MANY_REQUESTS,
        "TOO_MANY_CONNECTIONS",
        "Too many open connections from your network, close some connections or try again later"
          .into(),
        Some(30u32),
      ),

      StreamError::RelayDeploymentSourceIsTarget => (
        StatusCode::SERVICE_UNAVAILABLE,
        "RELAY_SOURCE_IS_TARGET",
        "Service temporarily unavailable, try again in a few seconds".into(),
        Some(5u32),
      ),

      StreamError::Subscribe(e) => match e {
        
        SubscribeError::ExternalRelayRedirect(url) => {
          let mut res = Response::new(StatusCode::FOUND);
          if let Ok(v) = HeaderValue::from_str(&url) {
            res.headers_mut().append(LOCATION, v);
          }
          return res;
        }

        SubscribeError::Db(_e) => (
          StatusCode::INTERNAL_SERVER_ERROR,
          "INTERNAL_RELAY_DB",
          "internal server error (db2)".into(),
          None,
        ),

        SubscribeError::StationNotFound(id) => (
          StatusCode::NOT_FOUND,
          "STATION_NOT_FOUND",
          format!("station with id {id} not found"),
          None,
        ),

        SubscribeError::PlaylistEmpty => (
          StatusCode::SERVICE_UNAVAILABLE,
          "STATION_NOT_STREAMING",
          "station is not actively streaming, try again later".into(),
          Some(60u32),
        ),

        SubscribeError::InternalRelay(e) => match e {
          GetInternalRelayError::Db(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_RELAY_DB",
            "internal server error (db3)".into(),
            None,
          ),

          GetInternalRelayError::RelayTimeout => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_RELAY_TIMEOUT",
            "internal server error (to)".into(),
            None,
          ),

          GetInternalRelayError::CreateRequest(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_RELAY_CREATE_REQUEST",
            "internal server error (cr)".into(),
            None,
          ),

          GetInternalRelayError::SendRequest(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_RELAY_SEND_REQUEST",
            "internal server error (sr)".into(),
            None,
          ),

          GetInternalRelayError::DeploymentNoPort => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_DEPLOYMENT_NO_PORT",
            "internal server error (dnp)".into(),
            None,
          ),

          GetInternalRelayError::DeploymentNotFound(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_DEPLOYMENT_NOT_FOUND",
            "internal server error (dnf)".into(),
            None,
          ),

          GetInternalRelayError::RelayStatus(s, code) => {
            let status = s.as_u16();
            (
              StatusCode::SERVICE_UNAVAILABLE,
              "RELAY_STATUS",
              format!("relay server responded with not ok status code: {status} => {code:?}"),
              Some(60u32),
            )
          }
        },
      },
    };

    let mut res = Response::new(status);

    res.headers_mut().append(CONTENT_TYPE, TEXT_PLAIN_UTF8);

    res.headers_mut().append(
      HeaderName::from_static(constants::INTERNAL_RELAY_REJECTION_CODE_HEADER),
      HeaderValue::from_static(code),
    );

    if let Some(secs) = retry_after_secs {
      res.headers_mut().append(
        RETRY_AFTER,
        HeaderValue::from_str(&secs.to_string()).unwrap(),
      );
    }

    *res.body_mut() = Body::from(message);

    res
  }
}

#[derive(Debug, Clone, Copy)]
pub enum LinkHandlerKind {
  M3u,
  Pls,
}

#[derive(Debug, Clone)]
pub struct LinkHandler {
  kind: LinkHandlerKind,
}

impl LinkHandler {
  pub fn new(kind: LinkHandlerKind) -> Self {
    Self { kind }
  }

  async fn handle(&self, req: Request) -> Result<Response, StreamError> {
    let station_id = req.param("id").unwrap().to_string();

    let station = match Station::get_by_id(&station_id).await? {
      Some(station) => station,
      None => return Err(StreamError::StationNotFound(station_id.to_string())),
    };

    let host = req.host().unwrap_or("stream.openstream.fm");

    let target = format!("https://{}/stream/{}", host, station.id);

    let (body, content_type) = match self.kind {
      LinkHandlerKind::M3u => {
        let file = HlsContents {
          target: &target,
          title: &station.name,
        };

        let body = Body::from(file.to_string());
        let content_type = CONTENT_TYPE_X_MPEG_URL;
        (body, content_type)
      }

      LinkHandlerKind::Pls => {
        let file = PlsContents {
          target: &target,
          title: &station.name,
        };

        let body = Body::from(file.to_string());
        let content_type = CONTENT_TYPE_PLS;
        (body, content_type)
      }
    };

    let mut res = Response::new(StatusCode::OK);
    res.headers_mut().append(CONTENT_TYPE, content_type);

    res
      .headers_mut()
      .append(CACHE_CONTROL, CACHE_CONTROL_NO_CACHE);

    *res.body_mut() = body;

    Ok(res)
  }
}

#[async_trait]
impl Handler for LinkHandler {
  async fn call(&self, req: Request, _: Next) -> Response {
    self.handle(req).await.into()
  }
}

/// render a .m3u/.m3u8 file
/// #EXTM3U
/// https://stream.openstream.fm/stream/:station_id
#[derive(Debug, Clone)]
pub struct HlsContents<'a> {
  target: &'a str,
  #[allow(unused)]
  title: &'a str,
}

impl<'a> Display for HlsContents<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // writeln!(f, "#EXTM3U")?;
    // writeln!(f, "#EXTENC:UTF-8")?;
    // f.write_str("\n")?;
    // writeln!(f, "#EXTINF:1,{}", self.title)?;
    // writeln!(f, "#EXT-X-TARGETDURATION:3600")?;
    writeln!(f, "{}", self.target)?;
    Ok(())
  }
}

/// render a .pls file
/// [Playlist]
/// NumberOfEntries=1
/// File1=https://stream.openstream.fm/stream/:station_id
/// Title1=Station Name
/// Length=-1
/// Version=2
#[derive(Debug, Clone)]
pub struct PlsContents<'a> {
  pub target: &'a str,
  pub title: &'a str,
}

impl<'a> Display for PlsContents<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "[Playlist]")?;
    writeln!(f, "NumberOfEntries=1")?;
    writeln!(f, "File1={}", self.target)?;
    writeln!(f, "Title1={}", self.title)?;
    writeln!(f, "Length=-1")?;
    writeln!(f, "Version=2")?;
    Ok(())
  }
}


fn find_first_frame_index(data: &[u8]) -> Option<usize> {
  match mp3::read_from_slice(data) {
    Err(_) => None,
    Ok(meta) => { 
      // let offsets_str = meta.frames.windows(2).map(|fs| {
      //   let f1 = &fs[0];
      //   let f2 = &fs[1];
      //   format!("{start} - {size}", start = f1.offset,  size = f2.offset - f1.offset)
      // }).collect::<Vec<_>>().join("\n");
      // info!("== OFFSETS ==\n {}", offsets_str);
      meta.frames.windows(4).find_map(|w| {
        let f1 = &w[0];
        let f2 = &w[1];
        let f3 = &w[2];
        let f4 = &w[3];

        let n = f2.offset - f1.offset;
        if n == f3.offset - f2.offset && n == f4.offset - f3.offset {
          Some(f1.offset as usize)
        } else {
          None
        }
      })
    }
  }
}