use crate::json::JsonHandler;
use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::audio_chunk::AudioChunk;
use db::audio_file::AudioFile;
use db::station::Station;
use db::Model;
use hyper::header::HeaderValue;
use mongodb::bson::doc;
use prex::{Request, Response};
use serde::{Deserialize, Serialize};

#[allow(clippy::declare_interior_mutable_const)]
const VARY_RANGE_X_ACCESS_TOKEN: HeaderValue = HeaderValue::from_static("range,x-access-token");

#[allow(clippy::declare_interior_mutable_const)]
const ACCEPT_RANGES_BYTES: HeaderValue = HeaderValue::from_static("bytes");

#[allow(clippy::declare_interior_mutable_const)]
const CONTENT_TYPE_AUDIO_MPEG: HeaderValue = HeaderValue::from_static("audio/mpeg");

pub mod get {

  use ts_rs::TS;

  use crate::error::ApiError;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    station: Station,
    #[allow(unused)]
    access_token_scope: AccessTokenScope,
    file_id: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(
    export,
    export_to = "../../../defs/api/stations/[station]/files/[file]/GET/"
  )]
  #[serde(rename_all = "snake_case")]
  pub struct Output {
    item: AudioFile,
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongodb: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("file not found: {0}")]
    FileNotFound(String),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::FileNotFound(id) => Self::AudioFileNotFound(id),
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = GetAccessTokenScopeError;
    type HandleError = HandleError;

    async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError> {
      let station_id = req.param("station").unwrap();
      let file_id = req.param("file").unwrap();

      let access_token_scope = request_ext::get_access_token_scope(&req).await?;

      let station = access_token_scope.grant_station_scope(station_id).await?;

      Ok(Self::Input {
        access_token_scope,
        station,
        file_id: file_id.to_string(),
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        access_token_scope: _,
        station,
        file_id,
      } = input;

      let filter = doc! { AudioFile::KEY_ID: &file_id, AudioFile::KEY_STATION_ID: station.id };

      let item = match AudioFile::cl().find_one(filter, None).await? {
        None => return Err(HandleError::FileNotFound(file_id)),
        Some(f) => f,
      };

      Ok(Output { item })
    }
  }
}

pub mod stream {

  use http_range::HttpRange;
  use hyper::{
    header::{
      ACCEPT_RANGES, CONTENT_LENGTH, CONTENT_RANGE, CONTENT_TYPE, ETAG, IF_NONE_MATCH, RANGE, VARY,
    },
    Body, StatusCode,
  };

  use crate::error::ApiError;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Handler {}

  #[async_trait]
  impl prex::handler::Handler for Handler {
    async fn call(&self, request: Request, _: prex::Next) -> prex::Response {
      let station_id = request.param("station").unwrap();
      let file_id = request.param("file").unwrap();

      let scope = match request_ext::get_media_access_token_scope(&request).await {
        Ok(scope) => scope,
        Err(e) => return ApiError::from(e).into_json_response(),
      };

      let _station = match scope.grant_station_scope(station_id).await {
        Ok(station) => station,
        Err(e) => return ApiError::from(e).into_json_response(),
      };

      let filter = doc! { AudioFile::KEY_ID: file_id, AudioFile::KEY_STATION_ID: station_id };
      let file = match AudioFile::get(filter).await {
        Ok(Some(file)) => file,
        Ok(None) => return ApiError::ResourceNotFound.into_json_response(),
        Err(e) => return ApiError::from(e).into_json_response(),
      };

      let req_etag = request
        .headers()
        .get(IF_NONE_MATCH)
        .and_then(|v| v.to_str().ok());

      let res_etag = format!("\"{}\"", file.sha256);

      if let Some(req_etag) = req_etag {
        if req_etag == res_etag {
          return Response::new(StatusCode::NOT_MODIFIED);
        }
      }

      let is_range_request = request.headers().contains_key(RANGE);

      let range: Option<HttpRange> =
        match request.headers().get(RANGE).and_then(|v| v.to_str().ok()) {
          None => None,
          Some(v) => match http_range::HttpRange::parse(v, file.len) {
            Err(e) => return ApiError::from(e).into_json_response(),
            Ok(ranges) => ranges.get(0).copied(),
          },
        };

      let (start, length) = match range {
        None => (0, file.len),
        Some(range) => (range.start, range.length),
      };

      let end = start + length - 1;

      let file_len = file.len;
      let chunk_len = file.chunk_len;

      let start_i = start as usize / chunk_len;
      let skip_first_item_bytes = start as usize % chunk_len;

      let end_i = (start as usize + length as usize) / chunk_len;
      let end_item_len = (start as usize + length as usize) % chunk_len;

      let stream = async_stream::stream! {

        let mut i = start_i;

        loop {
          if i > end_i {
            break;
          }

          let filter = doc!{AudioChunk::KEY_AUDIO_FILE_ID: &file.id, AudioChunk::KEY_I: i as f64 };
          let data = match AudioChunk::get(filter).await {
            Err(e) => {
              yield Err(e);
              break;
            },
            Ok(None) => break,
            Ok(Some(chunk)) => {
              if i == end_i && i == start_i {
                chunk.data.slice(skip_first_item_bytes..end_item_len)
              } else if i == start_i {
                chunk.data.slice(skip_first_item_bytes..)
              } else if i == end_i {
                chunk.data.slice(..end_item_len)
              } else {
                chunk.data
              }
            }
          };

          i += 1;

          yield Ok(data)
        }
      };

      let mut response = {
        if is_range_request {
          let mut response = Response::new(StatusCode::OK);

          response.headers_mut().append(
            CONTENT_LENGTH,
            HeaderValue::from_str(&file_len.to_string()).unwrap(),
          );

          response
        } else {
          let mut response = Response::new(StatusCode::PARTIAL_CONTENT);
          response.headers_mut().append(
            CONTENT_LENGTH,
            HeaderValue::from_str(&length.to_string()).unwrap(),
          );

          response.headers_mut().append(
            CONTENT_RANGE,
            HeaderValue::from_str(&format!("bytes {start}-{end}/{file_len}")).unwrap(),
          );

          response
        }
      };

      response
        .headers_mut()
        .append(ETAG, HeaderValue::from_str(res_etag.as_str()).unwrap());

      response
        .headers_mut()
        .append(CONTENT_TYPE, CONTENT_TYPE_AUDIO_MPEG);

      response
        .headers_mut()
        .append(ACCEPT_RANGES, ACCEPT_RANGES_BYTES);

      response
        .headers_mut()
        .append(VARY, VARY_RANGE_X_ACCESS_TOKEN);

      let body = Body::wrap_stream(stream);

      *response.body_mut() = body;

      response
    }
  }
}

pub mod delete {
  use ts_rs::TS;

  use crate::error::ApiError;

  use super::*;
  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    station: Station,
    file_id: String,
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongodb: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("audio file not found: {0}")]
    FileNotFound(String),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::FileNotFound(id) => ApiError::AudioFileNotFound(id),
      }
    }
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(
    export,
    export_to = "../../../defs/api/stations/[station]/files/[file]/DELETE/"
  )]
  pub struct Output(AudioFile);

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = GetAccessTokenScopeError;
    type HandleError = HandleError;

    async fn parse(&self, request: Request) -> Result<Input, Self::ParseError> {
      let station_id = request.param("station").unwrap();
      let file_id = request.param("file").unwrap();
      let access_token_scope = request_ext::get_access_token_scope(&request).await?;
      let station = access_token_scope.grant_station_scope(station_id).await?;

      Ok(Input {
        station,
        file_id: file_id.to_string(),
      })
    }

    async fn perform(&self, input: Input) -> Result<Output, HandleError> {
      let Input { station, file_id } = input;
      match AudioFile::delete_audio_file(&station.id, &file_id).await? {
        None => Err(HandleError::FileNotFound(file_id)),
        Some(audio_file) => Ok(Output(audio_file)),
      }
    }
  }
}
