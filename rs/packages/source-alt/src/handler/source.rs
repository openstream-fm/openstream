#![allow(clippy::useless_format)]

use std::{collections::btree_map::Entry, net::SocketAddr};

use db::{station::Station, Model};
use drop_tracer::DropTracer;
use hyper::{
  header::{CONTENT_LENGTH, CONTENT_TYPE, WWW_AUTHENTICATE},
  http::HeaderValue,
  HeaderMap, Method, StatusCode, Version,
};
use log::*;
use media_sessions::{live::LiveError, MediaSessionKind, MediaSessionMap};
use shutdown::Shutdown;
use stream_util::IntoTryBytesStream;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::{
  error::HandlerError,
  headers,
  http::{write_response_head, RequestHead, ResponseHead},
};

#[allow(clippy::too_many_arguments)]
pub async fn source(
  mut socket: TcpStream,
  local_addr: SocketAddr,
  remote_addr: SocketAddr,
  head: RequestHead,
  id: String,
  media_sessions: MediaSessionMap,
  drop_tracer: DropTracer,
  shutdown: Shutdown,
) -> Result<(), HandlerError> {
  trace!("source: {} {} => id: {}", head.method, head.uri, id);

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

  let content_type = match head.headers.get(CONTENT_TYPE).and_then(|t| t.to_str().ok()) {
    None => {
      let body = "content-type header is required";
      let mut headers = headers!();
      headers.append(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
      headers.append(
        CONTENT_LENGTH,
        HeaderValue::from_str(body.len().to_string().as_str()).unwrap(),
      );
      let response = ResponseHead {
        version: Version::HTTP_10,
        status: StatusCode::BAD_REQUEST,
        headers,
      };

      write_response_head(&mut socket, response, false).await?;
      socket.write_all(body.as_bytes()).await?;
      socket.flush().await?;
      return Ok(());
    }

    Some(t) => t.to_string(),
  };

  let station = match Station::get_by_id(&id).await {
    Err(_e) => {
      let body = "internal error (db)";
      let mut headers = headers!();
      headers.append(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
      headers.append(
        CONTENT_LENGTH,
        HeaderValue::from_str(body.len().to_string().as_str()).unwrap(),
      );
      let response = ResponseHead {
        version: Version::HTTP_10,
        status: StatusCode::INTERNAL_SERVER_ERROR,
        headers,
      };

      write_response_head(&mut socket, response, false).await?;
      socket.write_all(body.as_bytes()).await?;
      socket.flush().await?;
      return Ok(());
    }
    Ok(None) => {
      let body = format!("station with id {id} not found");
      let mut headers = headers!();
      headers.append(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
      headers.append(
        CONTENT_LENGTH,
        HeaderValue::from_str(body.len().to_string().as_str()).unwrap(),
      );
      let response = ResponseHead {
        version: Version::HTTP_10,
        status: StatusCode::NOT_FOUND,
        headers,
      };

      write_response_head(&mut socket, response, false).await?;
      socket.write_all(body.as_bytes()).await?;
      socket.flush().await?;
      return Ok(());
    }
    Ok(Some(station)) => station,
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
    None => {
      let body = "Authorization missing or invalid";
      let mut headers = headers!();
      headers.append(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
      headers.append(
        CONTENT_LENGTH,
        HeaderValue::from_str(body.len().to_string().as_str()).unwrap(),
      );
      headers.append(
        WWW_AUTHENTICATE,
        HeaderValue::from_static(r#"Basic realm="authentication", charset="UTF-8"#),
      );
      let res = ResponseHead {
        version: Version::HTTP_10,
        status: StatusCode::UNAUTHORIZED,
        headers,
      };

      write_response_head(&mut socket, res, true).await?;
      socket.write_all(body.as_bytes()).await?;
      socket.flush().await?;
      return Ok(());
    }

    Some(creds) => (creds.user_id, creds.password),
  };

  if auth_user != "source" || auth_password != password {
    let body = "basic auth username/password mismatch";
    let mut headers = headers!();
    headers.append(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
    headers.append(
      CONTENT_LENGTH,
      HeaderValue::from_str(body.len().to_string().as_str()).unwrap(),
    );
    headers.append(
      WWW_AUTHENTICATE,
      HeaderValue::from_static(r#"Basic realm="authentication", charset="UTF-8"#),
    );

    let res = ResponseHead {
      version: Version::HTTP_10,
      status: StatusCode::UNAUTHORIZED,
      headers,
    };

    write_response_head(&mut socket, res, true).await?;
    socket.write_all(body.as_bytes()).await?;
    socket.flush().await?;
    return Ok(());
  };

  let tx = {
    let mut map = media_sessions.write();
    match map.entry(&station.id) {
      Entry::Vacant(_) => Some(map.transmit(&station.id, MediaSessionKind::Live { content_type })),
      Entry::Occupied(entry) => {
        let session = entry.get();
        match session.kind() {
          MediaSessionKind::Live { .. } => None,
          MediaSessionKind::Playlist { .. } => {
            Some(map.transmit(&station.id, MediaSessionKind::Live { content_type }))
          }
        }
      }
    }
  };

  let tx = match tx {
    Some(tx) => tx,
    None => {
      let body = "this mountpoint is already in use, try again later";
      let mut headers = headers!();
      headers.append(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
      headers.append(
        CONTENT_LENGTH,
        HeaderValue::from_str(body.len().to_string().as_str()).unwrap(),
      );
      let res = ResponseHead {
        version: Version::HTTP_10,
        status: StatusCode::UNAUTHORIZED,
        headers,
      };

      write_response_head(&mut socket, res, true).await?;
      socket.write_all(body.as_bytes()).await?;
      socket.flush().await?;
      return Ok(());
    }
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

  tokio::spawn(async move {
    let (reader, mut writer) = socket.into_split();
    let shutdown = shutdown.clone();
    let drop_tracer = drop_tracer.clone();

    let user_agent = head
      .headers
      .get("user-agent")
      .and_then(|h| h.to_str().ok())
      .map(user_agent::UserAgent::parse)
      .unwrap_or_else(user_agent::UserAgent::default);

    let request_document = db::http::Request {
      local_addr: db::http::SocketAddr::from_http(local_addr),
      remote_addr: db::http::SocketAddr::from_http(remote_addr),
      real_ip,
      version: db::http::Version::from_http(head.version),
      method: db::http::Method::from_http(&head.method),
      uri: db::http::Uri::from_http(&head.uri),
      headers: db::http::Headers::from_http(&head.headers),
      user_agent,
    };

    let r = media_sessions::live::run_live_session(
      tx,
      reader.into_bytes_stream(1000),
      request_document,
      shutdown,
      drop_tracer,
    )
    .await;

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
        // let (status, body) = match e {
        //   LiveError::Db(_) => (StatusCode::INTERNAL_SERVER_ERROR, String::from("internal error creating live media session, try again later or report it to the administrators")),
        //   LiveError::Spawn(_) | LiveError::ExitIo(_) | LiveError::StderrError(_) => (StatusCode::INTERNAL_SERVER_ERROR, String::from("error allocating internal stream converter, try again later or report it to the administrators")),
        //   LiveError::ExitNotOk { stderr } => (StatusCode::FORBIDDEN, format!("error converting the audio stream (exit), possibly the audio is corrupted or is using a not supported format: {stderr}")),
        // };

        // let (status, message) = match e {
        //   LiveError::Db(_) => (StatusCode::INTERNAL_SERVER_ERROR, String::from("internal error creating live media session, try again later or report it to the administrators")),
        //   LiveError::Probe(e) => {
        //     let status = StatusCode::FORBIDDEN;
        //     let message = match e {
        //       mp3::ProbeError::NoDefaultTrack => String::from("unsupported stream: incomming audio stream does not have a default track"),
        //       mp3::ProbeError::NotMP3 => String::from("unsopported stream: incoming audio stream is not MP3"),
        //       mp3::ProbeError::NotSupported(e) => format!("unsupported stream: {e}")
        //     };
        //     (status, message)
        //   }
        //   LiveError::Play(e) => {
        //     let status = StatusCode::FORBIDDEN;
        //     let message = match e {
        //       mp3::PlayError::Packet(e) => format!("play packet error: {e}"),
        //       mp3::PlayError::Reset(e) => format!("play reset error: {e}"),
        //       mp3::PlayError::ResetNoDefaultTrack => String::from("play reset error: no default track after reset"),
        //       mp3::PlayError::ResetTrackNotMP3 => String::from("play reset error: new default track is not MP3"),
        //       mp3::PlayError::MissingTimeBase => String::from("internal error: missing track time base"),
        //     };
        //     (status, message)
        //   }
        // };

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

    Ok::<(), HandlerError>(())
  });

  Ok(())
}