use crate::json::JsonHandler;
use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::account::Account;
use db::audio_chunk::AudioChunk;
use db::audio_file::AudioFile;
use db::Model;
use hyper::header::HeaderValue;
use mongodb::bson::doc;
use prex::{Request, Response};
use serde::{Deserialize, Serialize};

pub mod get {

  use ts_rs::TS;

  use crate::error::{ApiError, Kind};

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    account: Account,
    #[allow(unused)]
    access_token_scope: AccessTokenScope,
    file_id: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../defs/api/accounts/[account]/files/[file]/GET/")]
  #[serde(rename_all = "camelCase")]
  pub struct Output {
    item: AudioFile,
  }

  #[derive(Debug)]
  pub enum HandleError {
    Db(mongodb::error::Error),
    FileNotFound(String),
  }

  impl From<mongodb::error::Error> for HandleError {
    fn from(e: mongodb::error::Error) -> Self {
      Self::Db(e)
    }
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::FileNotFound(id) => Self::from(Kind::AudioFileNotFound(id)),
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
      let account_id = req.param("account").unwrap();
      let file_id = req.param("file").unwrap();

      let access_token_scope = request_ext::get_access_token_scope(&req).await?;

      let account = access_token_scope.grant_scope(account_id).await?;

      Ok(Self::Input {
        access_token_scope,
        account,
        file_id: file_id.to_string(),
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        access_token_scope: _,
        account,
        file_id,
      } = input;

      let filter = doc! { "_id": &file_id, "accountId": account.id };

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

  use crate::{
    error::{ApiError, Kind},
    request_ext::get_access_token_scope,
  };

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Handler {}

  #[async_trait]
  impl prex::handler::Handler for Handler {
    async fn call(&self, request: Request, _: prex::Next) -> prex::Response {
      let account_id = request.param("account").unwrap();
      let file_id = request.param("file").unwrap();
      let scope = match get_access_token_scope(&request).await {
        Ok(scope) => scope,
        Err(e) => return ApiError::from(e).into_json_response(),
      };

      let _account = match scope.grant_scope(account_id).await {
        Ok(account) => account,
        Err(e) => return ApiError::from(e).into_json_response(),
      };

      let file = match AudioFile::get(doc! { "_id": file_id, "accountId": account_id }).await {
        Ok(Some(file)) => file,
        Ok(None) => return ApiError::from(Kind::ResourceNotFound).into_json_response(),
        Err(e) => return ApiError::from(e).into_json_response(),
      };

      let req_etag = request
        .headers()
        .get(IF_NONE_MATCH)
        .and_then(|v| v.to_str().ok());

      let res_etag = format!("\"{}\"", file.md5);

      if let Some(req_etag) = req_etag {
        if req_etag == res_etag {
          let mut response = Response::new(StatusCode::NOT_MODIFIED);
          response
            .headers_mut()
            .append(VARY, HeaderValue::from_static("range"));

          return response;
        }
      }

      // let chunk_count = file.chunk_count;
      let file_id = file.id.to_string();

      let is_range_request = request.headers().contains_key(RANGE);

      let range: Option<HttpRange> =
        match request.headers().get(RANGE).and_then(|v| v.to_str().ok()) {
          None => None,
          Some(v) => match http_range::HttpRange::parse(v, file.len as u64) {
            Err(e) => return ApiError::from(e).into_json_response(),
            Ok(ranges) => ranges.get(0).copied(),
          },
        };

      let (start, length) = match range {
        None => (0, file.len as u64),
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

          let data = match AudioChunk::get(doc!{ "audioFileId": &file_id, "i": i as f64 }).await {
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
                chunk.data.slice(0..end_item_len)
              } else {
                chunk.data
              }
            }
          };

          i += 1;

          yield Ok(data)
        }
      };

      let mut response = match is_range_request {
        false => {
          let mut response = Response::new(StatusCode::OK);

          response.headers_mut().append(
            CONTENT_LENGTH,
            HeaderValue::from_str(format!("{file_len}").as_str()).unwrap(),
          );

          response
        }

        true => {
          let mut response = Response::new(StatusCode::PARTIAL_CONTENT);
          response.headers_mut().append(
            CONTENT_LENGTH,
            HeaderValue::from_str(format!("{length}").as_str()).unwrap(),
          );

          response.headers_mut().append(
            CONTENT_RANGE,
            HeaderValue::from_str(format!("bytes {start}-{end}/{file_len}").as_str()).unwrap(),
          );

          response
        }
      };

      response
        .headers_mut()
        .append(ETAG, HeaderValue::from_str(res_etag.as_str()).unwrap());

      response
        .headers_mut()
        .append(CONTENT_TYPE, HeaderValue::from_static("audio/mpeg"));

      response
        .headers_mut()
        .append(ACCEPT_RANGES, HeaderValue::from_static("bytes"));

      response
        .headers_mut()
        .append(VARY, HeaderValue::from_static("range,x-access-token"));

      let body = Body::wrap_stream(stream);

      *response.body_mut() = body;

      response
    }
  }
}
