use db::{ws_stats_connection::WsStatsConnection, Model};
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
  #[error("expecting websocket request")]
  NotWs,

  #[error("query string is invalid: {0}")]
  InvalidQs(#[from] serde_qs::Error),

  #[error("websocket protocol error: {0}")]
  ProtocolError(#[from] ProtocolError),
}

impl From<WsConnectionHandlerError> for Response {
  fn from(err: WsConnectionHandlerError) -> Self {
    let body = Body::from(format!("{}", err));

    let status = match err {
      WsConnectionHandlerError::NotWs => StatusCode::BAD_REQUEST,
      WsConnectionHandlerError::InvalidQs(_) => StatusCode::BAD_REQUEST,
      WsConnectionHandlerError::ProtocolError(_) => StatusCode::BAD_REQUEST,
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

      let connection_id: String;
      let created_at: DateTime;

      macro_rules! create {
        () => {{
          connection_id = WsStatsConnection::uid();
          created_at = DateTime::now();

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

        Some(prev_id) => match WsStatsConnection::get_by_id(&prev_id).await {
          Err(_) => return,

          Ok(None) => create!(),

          Ok(Some(connection)) => {
            connection_id = connection.id;
            created_at = connection.created_at;
          }
        },
      }

      'start: {
        let start_message = serde_json::to_string(&ServerEvent::Start {
          connection_id: connection_id.clone(),
        })
        .unwrap();

        let r = tokio::select! {
          _ = shutdown.signal() => break 'start,
          r = stream.send(Message::text(start_message)) => r
        };

        match r {
          Ok(_) => {}
          _ => break 'start,
        }

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
                  let pong = serde_json::to_string(&ServerEvent::Pong).unwrap();
                  let r = tokio::select! {
                    _ = shutdown.signal() => break 'messages,
                    r = stream.send(Message::text(pong)) => r
                  };

                  match r {
                    Ok(_) => {}
                    _ => break 'messages,
                  }
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
    });

    Ok(res)
  }
}

#[async_trait::async_trait]
impl Handler for WsConnectionHandler {
  async fn call(&self, req: Request, _: Next) -> Response {
    self.handle(req).await.into()
  }
}
