use db::{station::Station, ws_stats_connection::WsStatsConnection, Model};
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
  app_kind: Option<String>,

  #[ts(optional)]
  #[serde(skip_serializing_if = "Option::is_none")]
  app_version: Option<f64>,
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

    if !Station::exists(qs.station_id.clone()).await? {
      return Err(WsConnectionHandlerError::StationNotFound(qs.station_id));
    }

    let (res, stream_future) = prex::ws::upgrade(&mut req, None)?;

    tokio::spawn(async move {
      let mut stream = match stream_future.await {
        Ok(stream) => stream,
        Err(_) => {
          // TODO: log
          return;
        }
      };

      let Query {
        connection_id: prev_id,
        station_id,
        app_kind,
        app_version,
      } = qs;

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
        "OPEN ws-stats connection {connection_id} for station {station_id} ({reconnections})"
      );

      'start: {
        macro_rules! send {
          ($event:expr) => {{
            let event = serde_json::to_string(&$event).unwrap();
            let r = tokio::select! {
              _ = shutdown.signal() => break 'start,
              r = stream.send(Message::text(event)) => r
            };

            match r {
              Ok(_) => {}
              _ => break 'start,
            }
          }};
        }

        send!(ServerEvent::Start {
          connection_id: connection_id.clone(),
        });

        'messages: loop {
          let msg = tokio::select! {
            _ = shutdown.signal() => {
              break 'messages;
            }

            msg = stream.next() => msg,
          };

          let msg = match msg {
            Some(Ok(msg)) => msg,
            _ => break 'messages,
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

            _ => continue 'messages,
          }
        }
      }

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
        "CLOSE ws-stats connection {connection_id} for station {station_id} ({reconnections}) in {duration}",
        duration=FormatDuration(duration_ms),
      );
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
