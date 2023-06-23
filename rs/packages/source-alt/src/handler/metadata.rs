use crate::http::write_response_head;
use crate::RequestHead;
use crate::{error::HandlerError, http::ResponseHead};
use db::{
  media_session::MediaSession, play_history_item::PlayHistoryItem, station::Station, Model,
};
use hyper::header::{CONNECTION, CONTENT_TYPE, WWW_AUTHENTICATE};
use hyper::Version;
use hyper::{header::CONTENT_LENGTH, http::HeaderValue, HeaderMap, StatusCode};
use lazy_regex::{Lazy, Regex};
use mongodb::bson::doc;
use regex_static::lazy_regex;
use serde::{Deserialize, Serialize};
use serde_util::DateTime;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[derive(Debug, thiserror::Error)]
pub enum MetadataError {
  #[error("Station not found")]
  NotFound,
  #[error("Station is not live streaming")]
  NotLiveStreaming,
  #[error("Invalid credentials")]
  InvalidCredentials,
  #[error("Internal server error (db)")]
  Db(#[from] mongodb::error::Error),
  #[error("Invalid query string: {0}")]
  Query(#[from] serde_qs::Error),
  #[error("Invalid query string: {0}")]
  InvalidQuery(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataQueryString {
  mount: String,
  mode: String,
  song: String,
  artist: Option<String>,
}

pub async fn metadata(
  mut socket: TcpStream,
  local_addr: SocketAddr,
  remote_addr: SocketAddr,
  head: RequestHead,
  deployment_id: String,
) -> Result<(), HandlerError> {
  async fn metadata(
    _local_addr: SocketAddr,
    _remote_addr: SocketAddr,
    head: RequestHead,
    deployment_id: String,
  ) -> Result<(), MetadataError> {
    let mut query: MetadataQueryString = serde_qs::from_str(head.uri.query().unwrap_or(""))?;

    if query.mode != "updinfo" {
      return Err(MetadataError::InvalidQuery("mode must be 'updinfo'".into()));
    }

    query.song = query.song.trim().to_string();
    if query.song.is_empty() {
      return Err(MetadataError::InvalidQuery("song is required".into()));
    }

    query.artist = match query.artist {
      None => None,
      Some(artist) => {
        let artist = artist.trim().to_string();
        if artist.is_empty() {
          None
        } else {
          Some(artist)
        }
      }
    };

    let target_regex: Lazy<Regex> = lazy_regex!("/?(?P<station>[a-zA-Z0-9]+)/source/?");
    let station_id = match target_regex.captures(&query.mount) {
      None => {
        return Err(MetadataError::InvalidQuery(
          "invalid mount parameter".into(),
        ))
      }
      Some(caps) => caps.name("station").unwrap().as_str(),
    };

    let station = match Station::get_by_id(station_id).await? {
      None => return Err(MetadataError::NotFound),
      Some(station) => station,
    };

    let password = station.source_password;

    let basic_auth = match head.headers.get("authorization") {
      None => return Err(MetadataError::InvalidCredentials),
      Some(header) => match header.to_str() {
        Err(_) => return Err(MetadataError::InvalidCredentials),
        Ok(header) => match http_basic_auth::decode(header) {
          Err(_) => return Err(MetadataError::InvalidCredentials),
          Ok(creds) => creds,
        },
      },
    };

    if basic_auth.user_id != "source" || basic_auth.password != password {
      return Err(MetadataError::InvalidCredentials);
    }

    let media_session = match MediaSession::get_current_for_station(&station.id).await? {
      None => return Err(MetadataError::NotLiveStreaming),
      Some(session) => session,
    };

    use db::media_session::MediaSessionKind::*;
    use db::media_session::MediaSessionNowPlaying;
    let (media_session_id, current_now_playing) = match &media_session.kind {
      Playlist { .. } => return Err(MetadataError::NotLiveStreaming),
      Live { .. } => (media_session.id, media_session.now_playing),
    };

    let new_now_playing = MediaSessionNowPlaying {
      title: query.song,
      artist: query.artist,
    };

    if current_now_playing.as_ref() == Some(&new_now_playing) {
      return Ok(());
    }

    let now = DateTime::now();

    let play_history_item = PlayHistoryItem {
      id: PlayHistoryItem::uid(),
      station_id: station_id.to_string(),
      kind: db::play_history_item::Kind::Live,
      title: new_now_playing.title.clone(),
      artist: new_now_playing.artist.clone(),
      deployment_id: deployment_id.clone(),
      created_at: now,
    };

    let update = doc! {
      "$set": {
        MediaSession::KEY_NOW_PLAYING: new_now_playing,
        MediaSession::KEY_UPDATED_AT: now,
      }
    };
    MediaSession::update_by_id(&media_session_id, update).await?;
    PlayHistoryItem::insert(&play_history_item).await?;

    Ok(())
  }

  use MetadataError::*;
  let (status, message, is_auth_fail) =
    match metadata(local_addr, remote_addr, head, deployment_id).await {
      Ok(()) => (StatusCode::OK, String::from("metadata info updated"), false),
      Err(e) => {
        let message = format!("{}", e);
        let (status, is_auth_fail) = match e {
          Db(_) => (StatusCode::INTERNAL_SERVER_ERROR, false),
          InvalidCredentials => (StatusCode::UNAUTHORIZED, true),
          Query(_) => (StatusCode::BAD_REQUEST, false),
          InvalidQuery(_) => (StatusCode::BAD_REQUEST, false),
          NotFound => (StatusCode::NOT_FOUND, false),
          NotLiveStreaming => (StatusCode::BAD_REQUEST, false),
        };
        (status, message, is_auth_fail)
      }
    };

  let mut headers = HeaderMap::with_capacity(if is_auth_fail { 4 } else { 3 });
  headers.append(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
  headers.append(
    CONTENT_LENGTH,
    HeaderValue::from_str(message.len().to_string().as_str()).unwrap(),
  );
  headers.append(CONNECTION, HeaderValue::from_static("close"));

  if is_auth_fail {
    headers.append(
      WWW_AUTHENTICATE,
      HeaderValue::from_static(r#"Basic realm="authentication", charset="UTF-8"#),
    );
  }

  let response_head = ResponseHead {
    version: Version::HTTP_10,
    status,
    headers,
  };

  write_response_head(&mut socket, response_head, true).await?;
  socket.write_all(message.as_bytes()).await?;
  socket.flush().await?;

  Ok(())
}
