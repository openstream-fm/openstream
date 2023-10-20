#![allow(clippy::useless_format)]

use std::net::SocketAddr;

use db::{
  deployment::Deployment,
  station::{OwnerDeploymentInfo, Station},
  Model,
};
use drop_tracer::DropTracer;
use hyper::{
  header::{CONTENT_LENGTH, CONTENT_TYPE, WWW_AUTHENTICATE},
  http::HeaderValue,
  HeaderMap, Method, StatusCode, Version,
};
use log::*;
use media::{
  channel::Sender, drop::MapEntryRelease, handle::live::LiveError, Handle, Info, Kind,
  MediaSessionMap,
};
use serde_util::DateTime;
use shutdown::Shutdown;
use stream_util::IntoTryBytesStream;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[allow(clippy::declare_interior_mutable_const)]
const WWW_AUTHENTICATE_BASIC_AUTH: HeaderValue =
  HeaderValue::from_static(r#"Basic realm="authentication", charset="UTF-8"#);

use crate::{
  error::HandlerError,
  headers,
  http::{error::WriteHeadError, write_response_head, RequestHead, ResponseHead},
};

#[derive(Debug, thiserror::Error)]
enum IntermediateError {
  #[error("io: {0}")]
  Io(#[from] std::io::Error),
  #[error("write head: {0}")]
  WriteHead(#[from] WriteHeadError),
  #[error("db: {0}")]
  Db(#[from] mongodb::error::Error),
  #[error("content type required")]
  ContentTypeRequired,
  #[error("station not found")]
  StationNotFound,
  #[error("deployment not found")]
  DeploymentNotFound,
  #[error("deployment port not found")]
  DeploymentNoPort,
  #[error("basic auth missing")]
  BasicAuthMissing,
  #[error("basic auth mismatch")]
  BasicAuthMismatch,
  #[error("mountpoint in use")]
  MountpoingInUse,
}

#[allow(clippy::too_many_arguments)]
pub async fn source(
  mut socket: TcpStream,
  local_addr: SocketAddr,
  remote_addr: SocketAddr,
  head: RequestHead,
  deployment_id: String,
  station_id: String,
  media_sessions: MediaSessionMap,
  drop_tracer: DropTracer,
  shutdown: Shutdown,
) -> Result<(), HandlerError> {
  trace!(
    "source: {} {} => station_id: {}",
    head.method,
    head.uri,
    station_id
  );

  let r = async {
    let content_type = match head.headers.get(CONTENT_TYPE).and_then(|t| t.to_str().ok()) {
      None => return Err(IntermediateError::ContentTypeRequired),
      Some(t) => t.to_string(),
    };

    let task_id = Station::random_owner_task_id();

    let info = OwnerDeploymentInfo {
      deployment_id: deployment_id.clone(),
      task_id: task_id.clone(),
      health_checked_at: Some(DateTime::now()),
      content_type: content_type.clone(),
    };

    let r = Station::try_set_owner_deployment_info(&station_id, info, drop_tracer.token()).await?;

    let (station, dropper) = match r {
      Err(None) => return Err(IntermediateError::StationNotFound),
      Err(Some((station, info))) => {
        if deployment_id == info.deployment_id {
          (station, None)
        } else {
          let deployment = match Deployment::get_by_id(&info.deployment_id).await? {
            None => return Err(IntermediateError::DeploymentNotFound),
            Some(deployment) => deployment,
          };

          use rand::seq::SliceRandom;
          let source_port = deployment.source_ports.choose(&mut rand::thread_rng());

          let port = match source_port {
            None => return Err(IntermediateError::DeploymentNoPort),
            Some(port) => *port,
          };

          let destination = SocketAddr::from((deployment.local_ip, port));

          passthrough(&mut socket, destination, head.buffer).await?;

          return Ok(None);
        }
      }

      Ok((station, dropper)) => (station, Some(dropper)),
    };

    // if not PUT is SOURCE checked in router
    let _is_put = head.method == Method::PUT;

    let real_ip = match prex::request::is_trusted_ip(remote_addr.ip()) {
      true => match head.proxy_protocol_ip {
        Some(ip) => ip,
        None => remote_addr.ip(),
      },
      false => remote_addr.ip(),
    };

    let is_continue = match head.headers.get("expect") {
      None => false,
      Some(h) => h.as_bytes().eq_ignore_ascii_case(b"100-continue"),
    };

    let password = station.source_password;

    let basic_auth = match head.headers.get("authorization") {
      None => None,
      Some(header) => match header.to_str() {
        Err(_) => None,
        Ok(header) => match http_basic_auth::decode(header) {
          Err(_) => None,
          Ok(creds) => Some(creds),
        },
      },
    };

    let (auth_user, auth_password) = match basic_auth {
      None => return Err(IntermediateError::BasicAuthMissing),
      Some(creds) => (creds.user_id, creds.password),
    };

    if auth_user != "source" || auth_password != password {
      return Err(IntermediateError::BasicAuthMismatch);
    };

    let (sender, map_entry_release) = {
      let mut lock = media_sessions.lock(&station_id).await;
      match &*lock {
        None => {}
        Some(handle) => match handle.info().kind {
          Kind::ExternalRelay => {}
          Kind::InternalRelay => {}
          Kind::Playlist => {}
          Kind::Live => {
            return Err(IntermediateError::MountpoingInUse);
          }
        },
      }

      let info = Info::new(Kind::Live, task_id.clone(), content_type.clone());
      let sender = Sender::new(station_id.clone(), info);
      let handle = Handle::new(sender.clone());
      let map_entry_release = MapEntryRelease::new(
        station_id.clone(),
        task_id.clone(),
        media_sessions.clone(),
        drop_tracer.token(),
      );
      *lock = Some(handle);

      (sender, map_entry_release)
    };

    if is_continue {
      let version = Version::HTTP_10;
      let status = StatusCode::CONTINUE;
      let headers = HeaderMap::new();

      let head = ResponseHead {
        version,
        status,
        headers,
      };
      // TODO: trailing newline should be false here?
      write_response_head(&mut socket, head, true).await?;
    } else {
      let version = Version::HTTP_10;
      let status = StatusCode::OK;
      let headers = headers!();

      let head = ResponseHead {
        version,
        status,
        headers,
      };

      write_response_head(&mut socket, head, true).await?;
    }

    Ok::<_, IntermediateError>(Some((
      sender,
      map_entry_release,
      task_id,
      real_ip,
      is_continue,
      dropper,
    )))
  }
  .await;

  match r {
    Err(e) => {
      use IntermediateError::*;

      log::warn!(
        target: "source-alt",
        "error handling source request: {e:?} => {e}"
      );

      let status: StatusCode;
      let message: String;
      let mut headers = headers!();
      headers.append("content-type", HeaderValue::from_static("text/plain"));

      match e {
        Io(e) => return Err(HandlerError::Io(e)),
        WriteHead(e) => return Err(HandlerError::WriteHead(e)),
        Db(_) => {
          status = StatusCode::INTERNAL_SERVER_ERROR;
          message = "internal server error (db)".into();
        }
        ContentTypeRequired => {
          status = StatusCode::FORBIDDEN;
          message = "content-type header is required".into();
        }
        BasicAuthMismatch => {
          status = StatusCode::UNAUTHORIZED;
          message = "authorization credentials are required".into();
          headers.append(WWW_AUTHENTICATE, WWW_AUTHENTICATE_BASIC_AUTH);
        }
        BasicAuthMissing => {
          status = StatusCode::UNAUTHORIZED;
          message = "authorization credentials doesn't match".into();
          headers.append(WWW_AUTHENTICATE, WWW_AUTHENTICATE_BASIC_AUTH);
        }
        DeploymentNoPort => {
          status = StatusCode::INTERNAL_SERVER_ERROR;
          message = "internal server error (depl no port)".into();
        }
        DeploymentNotFound => {
          status = StatusCode::INTERNAL_SERVER_ERROR;
          message = "internal server error (depl not found)".into();
        }
        MountpoingInUse => {
          status = StatusCode::FORBIDDEN;
          message = "mountpoint is already in use, try again later".into();
        }
        StationNotFound => {
          status = StatusCode::NOT_FOUND;
          message = "mountpoint not found in this server".into();
        }
      }

      headers.append(
        CONTENT_LENGTH,
        HeaderValue::from_str(message.len().to_string().as_str()).unwrap(),
      );

      let head = ResponseHead {
        version: Version::HTTP_10,
        status,
        headers,
      };

      write_response_head(&mut socket, head, true).await?;
      socket.write_all(message.as_bytes()).await?;
    }

    // passthrough
    Ok(None) => {}

    Ok(Some((sender, map_entry_release, task_id, real_ip, is_continue, dropper))) => {
      tokio::spawn(async move {
        let (reader, mut writer) = socket.into_split();

        let user_agent = head
          .headers
          .get("user-agent")
          .and_then(|h| h.to_str().ok())
          .map(user_agent::UserAgent::parse)
          .unwrap_or_else(user_agent::UserAgent::default);

        let request_document = db::http::Request {
          real_ip,
          country_code: geoip::ip_to_country_code(&real_ip),
          local_addr: db::http::SocketAddr::from_http(local_addr),
          remote_addr: db::http::SocketAddr::from_http(remote_addr),
          version: db::http::Version::from_http(head.version),
          method: db::http::Method::from_http(&head.method),
          uri: db::http::Uri::from_http(&head.uri),
          headers: db::http::Headers::from_http(&head.headers),
          user_agent,
        };

        let r = {
          let r = media::handle::run_live_source(
            sender,
            deployment_id,
            task_id,
            station_id,
            reader.into_bytes_stream(1000),
            request_document,
            shutdown,
            drop_tracer,
          )
          .await;
          drop(map_entry_release);
          r
        };

        match r {
          Ok(()) => {
            if is_continue {
              let body = "data streamed successfully";
              let mut headers = headers!();
              headers.append(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
              headers.append(
                CONTENT_LENGTH,
                HeaderValue::from_str(body.len().to_string().as_str()).unwrap(),
              );
              let res = ResponseHead {
                version: Version::HTTP_10,
                status: StatusCode::OK,
                headers,
              };

              write_response_head(&mut writer, res, true).await?;
              writer.write_all(body.as_bytes()).await?;
              writer.flush().await?;
            }
          }

          Err(e) => {
            let (status, body) = match e {
              LiveError::Db(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal error creating live media session, try again later or report it to the administrators"),
              LiveError::Data(_) => (StatusCode::FORBIDDEN, "io error reading data"),
            };

            if is_continue {
              let mut headers = headers!();
              headers.append(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
              headers.append(
                CONTENT_LENGTH,
                HeaderValue::from_str(body.len().to_string().as_str()).unwrap(),
              );
              let res = ResponseHead {
                version: Version::HTTP_10,
                status,
                headers,
              };

              write_response_head(&mut writer, res, true).await?;
              writer.write_all(body.as_bytes()).await?;
              writer.flush().await?;
            }
          }
        };

        drop(dropper);

        Ok::<(), HandlerError>(())
      });
    }
  };

  Ok(())
}

pub async fn passthrough(
  incoming: &mut TcpStream,
  destination: SocketAddr,
  head: Vec<u8>,
) -> Result<(), std::io::Error> {
  let mut outgoing = tokio::net::TcpStream::connect(destination).await?;
  outgoing.write_all(head.as_ref()).await?;
  tokio::io::copy_bidirectional(&mut outgoing, incoming).await?;
  Ok(())
}
