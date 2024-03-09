use std::{
  sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
  },
  time::Instant,
};

use db::{station::Station, ws_stats_connection::WsStatsConnection, Model};
use drop_tracer::DropTracer;
use futures_util::{sink::SinkExt, stream::StreamExt};
use hyper::{Body, StatusCode};
use mongodb::bson::doc;
use prex::{
  handler::Handler,
  ws::tungstenite::{error::ProtocolError, Message},
  Next, Request, Response,
};
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use shutdown::Shutdown;
use ts_rs::TS;

#[derive(Debug, Clone)]
pub struct WsConnectionHandler {
  pub deployment_id: String,
  pub drop_tracer: DropTracer,
  pub shutdown: Shutdown,
}

#[derive(Debug, thiserror::Error)]
pub enum WsConnectionHandlerError {
  #[error("db: {0}")]
  Db(#[from] mongodb::error::Error),

  #[error("station with id {0} not found")]
  StationNotFound(String),

  #[error("expecting websocket request")]
  NotWs,

  #[error("query string is invalid: {0}")]
  InvalidQs(#[from] serde_qs::Error),

  #[error("websocket protocol error: {0}")]
  ProtocolError(#[from] ProtocolError),
}

impl From<WsConnectionHandlerError> for Response {
  fn from(err: WsConnectionHandlerError) -> Self {
    let status = match &err {
      WsConnectionHandlerError::Db(_) => StatusCode::INTERNAL_SERVER_ERROR,
      WsConnectionHandlerError::StationNotFound(_) => StatusCode::BAD_REQUEST,
      WsConnectionHandlerError::NotWs => StatusCode::BAD_REQUEST,
      WsConnectionHandlerError::InvalidQs(_) => StatusCode::BAD_REQUEST,
      WsConnectionHandlerError::ProtocolError(_) => StatusCode::BAD_REQUEST,
    };

    let body = match &err {
      WsConnectionHandlerError::Db(_) => Body::from("internal server error (db)"),
      _ => Body::from(format!("{}", err)),
    };

    let mut res = Response::new(status);
    *res.body_mut() = body;

    res
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
  export,
  export_to = "../../../defs/ws-stats/api/ws/stats/connection/WS/"
)]
pub struct Query {
  #[ts(optional)]
  #[serde(skip_serializing_if = "Option::is_none")]
  connection_id: Option<String>,

  station_id: String,

  #[ts(optional)]
  #[serde(skip_serializing_if = "Option::is_none")]
  user_id: Option<String>,

  #[ts(optional)]
  #[serde(skip_serializing_if = "Option::is_none")]
  app_kind: Option<String>,

  #[ts(optional)]
  #[serde(skip_serializing_if = "Option::is_none")]
  app_version: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
  export,
  export_to = "../../../defs/ws-stats/api/ws/stats/connection/WS/"
)]
#[serde(tag = "kind")]
pub enum ServerEvent {
  #[serde(rename = "ping")]
  Ping,
  #[serde(rename = "pong")]
  Pong,
  #[serde(rename = "start")]
  Start { connection_id: String },
}
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
  export,
  export_to = "../../../defs/ws-stats/api/ws/stats/connection/WS/"
)]
#[serde(tag = "kind")]
pub enum ClientEvent {
  #[serde(rename = "ping")]
  Ping,
  #[serde(rename = "pong")]
  Pong,
}

impl WsConnectionHandler {
  async fn handle(&self, mut req: Request) -> Result<Response, WsConnectionHandlerError> {
    if !prex::ws::is_upgrade_request(&req) {
      return Err(WsConnectionHandlerError::NotWs);
    }

    let shutdown = self.shutdown.clone();
    let deployment_id = self.deployment_id.clone();
    let qs = req.qs::<Query>()?;
    let ip = req.isomorphic_ip();
    let country_code = geoip::ip_to_country_code(&ip);

    let station_name = match Station::get_by_id(&qs.station_id).await? {
      Some(station) => station.name,
      None => return Err(WsConnectionHandlerError::StationNotFound(qs.station_id)),
    };

    let (res, stream_future) = prex::ws::upgrade(&mut req, None)?;

    let token = self.drop_tracer.token();
    tokio::spawn(async move {
      let Query {
        connection_id: prev_id,
        station_id,
        user_id,
        app_kind,
        app_version,
      } = qs;

      let stream = match stream_future.await {
        Ok(stream) => stream,
        Err(e) => {
          log::warn!(
            target: "ws-stats",
            "ERR ws-stats {station_id} ({station_name}) at {app_kind_version} => {e} {e:?}",
            app_kind_version=AppKindVersion {
              kind: app_kind.as_deref(),
              version: app_version,
            },
          );
          return;
        }
      };

      let reconnections: u16;
      let connection_id: String;
      let created_at: DateTime;

      macro_rules! create {
        () => {{
          connection_id = WsStatsConnection::uid();
          created_at = DateTime::now();
          reconnections = 0;

          let connection = WsStatsConnection {
            id: connection_id.clone(),
            station_id: station_id.clone(),
            deployment_id,
            duration_ms: None,
            is_open: true,
            country_code,
            ip,
            app_kind: app_kind.clone(),
            app_version,
            user_id,
            reconnections,
            created_at,
            closed_at: None,
          };

          match WsStatsConnection::insert(&connection).await {
            Ok(_) => {}
            Err(_) => return,
          };
        }};
      }

      match prev_id {
        None => create!(),

        Some(prev_id) => {
          let filter = doc! {
            WsStatsConnection::KEY_ID: prev_id,
          };

          let update = doc! {
            "$set": {
              WsStatsConnection::KEY_IS_OPEN: true,
              WsStatsConnection::KEY_CLOSED_AT: null,
            },
            "$inc": {
              WsStatsConnection::KEY_RECONNECTIONS: 1.0,
            }
          };

          match WsStatsConnection::cl()
            .find_one_and_update(filter, update, None)
            .await
          {
            Err(_) => return,

            Ok(None) => create!(),

            Ok(Some(connection)) => {
              reconnections = connection.reconnections;
              connection_id = connection.id;
              created_at = connection.created_at;
            }
          }
        }
      }

      log::info!(
        target: "ws-stats",
        "OPEN ws-stats conn {connection_id} for {station_id} ({station_name}) ({reconnections}) at {app_kind_version}",
        app_kind_version=AppKindVersion {
          kind: app_kind.as_deref(),
          version: app_version,
        },
      );

      let (mut sink, mut stream) = stream.split();

      let (tx, mut rx) = tokio::sync::mpsc::channel::<Message>(8);

      let start = Instant::now();
      let last_pong_timestamp = Arc::new(AtomicU64::new(0));

      macro_rules! send {
        ($event:expr) => {{
          let event: ServerEvent = $event;
          let text = serde_json::to_string(&event).unwrap();
          let message = Message::Text(text);
          match tx.send(message).await {
            Ok(_) => {}
            Err(_) => return,
          };
        }};
      }

      let write = async {
        loop {
          let message = match rx.recv().await {
            None => break,
            Some(event) => event,
          };

          // let event = serde_json::to_string(&event).unwrap();
          match sink.send(message).await {
            Ok(_) => {}
            _ => return,
          }
        }
      };

      let pong = {
        async {
          loop {
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;

            if start.elapsed().as_secs() - last_pong_timestamp.load(Ordering::Acquire) > 45 {
              break;
            }
          }
        }
      };

      let ping = {
        async {
          loop {
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;

            match tx.send(Message::Ping(vec![])).await {
              Ok(_) => {}
              Err(_) => return,
            }
          }
        }
      };

      let handle = async {
        send!(ServerEvent::Start {
          connection_id: connection_id.clone(),
        });

        'messages: loop {
          let msg = match stream.next().await {
            None => break 'messages,
            Some(Err(_)) => break 'messages,
            Some(Ok(msg)) => msg,
          };

          match msg {
            Message::Text(text) => {
              let event = match serde_json::from_str::<ClientEvent>(&text) {
                Ok(event) => event,
                Err(_) => continue 'messages,
              };

              match event {
                ClientEvent::Pong => {}
                ClientEvent::Ping => {
                  send!(ServerEvent::Pong);
                }
              }
            }

            Message::Pong(_) => {
              last_pong_timestamp.store(start.elapsed().as_secs(), Ordering::Release);
            }

            Message::Close(_) => break 'messages,

            _ => continue 'messages,
          }
        }
      };

      tokio::select! {
        _ = ping => {}
        _ = pong => {}
        _ = write => {}
        _ = handle => {}
        _ = shutdown.signal() => {}
      };

      let duration_ms = ((*DateTime::now() - *created_at).as_seconds_f64() * 1000.0).round();

      let update = doc! {
        "$set": {
          WsStatsConnection::KEY_IS_OPEN: false,
          WsStatsConnection::KEY_CLOSED_AT: DateTime::now(),
          WsStatsConnection::KEY_DURATION_MS: duration_ms,
        }
      };

      let _ = WsStatsConnection::update_by_id(&connection_id, update).await;

      log::info!(
        target: "ws-stats",
        "CLOSE ws-stats conn {connection_id} for {station_id} ({station_name}) ({reconnections}) at {app_kind_version} in {duration}",
        app_kind_version=AppKindVersion {
          kind: app_kind.as_deref(),
          version: app_version,
        },
        duration=FormatDuration(duration_ms),
      );

      drop(token)
    });

    Ok(res)
  }
}

#[async_trait::async_trait]
impl Handler for WsConnectionHandler {
  async fn call(&self, req: Request, _: Next) -> Response {
    let r = self.handle(req).await;
    match &r {
      Ok(_) => {
        log::info!(
          target: "ws-stats",
          "ws connections handle ok"
        )
      }

      Err(err) => {
        log::warn!(
          target: "ws-stats",
          "ws connections handle err: {}",
          err
        )
      }
    }
    r.into()
  }
}

struct FormatDuration(f64);
impl core::fmt::Display for FormatDuration {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    const S: u64 = 1000;
    const M: u64 = 60 * S;
    const H: u64 = 60 * M;

    let d = self.0.round() as u64;

    let h = d / H;
    let m = (d % H) / M;
    let s = (d % M) / S;

    if h != 0 {
      write!(f, "{h}h {m}m {s}s")
    } else if m != 0 {
      write!(f, "{m}m {s}s")
    } else {
      write!(f, "{s}s")
    }
  }
}

struct AppKindVersion<'a> {
  kind: Option<&'a str>,
  version: Option<u32>,
}

impl<'a> core::fmt::Display for AppKindVersion<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match (&self.kind, &self.version) {
      (None, None) => write!(f, "unknown"),
      (Some(k), None) => write!(f, "{k}"),
      (None, Some(v)) => write!(f, "@{v}"),
      (Some(k), Some(v)) => write!(f, "{k}@{v}"),
    }
  }
}
