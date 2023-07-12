pub mod id;
pub mod metadata;
pub mod order;
pub mod shuffle;
pub mod unshuffle;

use crate::json::JsonHandler;
use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};
use mongodb::bson::doc;

use crate::error::ApiError;
use async_trait::async_trait;
use db::audio_file::AudioFile;
use db::station::Station;
use db::Model;
use db::Paged;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod get {

  use crate::qs::PaginationQs;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Serialize, Deserialize, Clone, TS)]
  #[ts(export, export_to = "../../../defs/api/stations/[station]/files/GET/")]
  pub struct Output {
    files: Paged<AudioFile>,
    playlist_is_randomly_shuffled: bool,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, Default, TS)]
  #[ts(export, export_to = "../../../defs/api/stations/[station]/files/GET/")]
  pub struct Query {
    #[serde(flatten)]
    pub page: PaginationQs,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    station: Station,
    #[allow(unused)]
    access_token_scope: AccessTokenScope,
    query: Query,
  }

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("access: {0}")]
    Access(#[from] GetAccessTokenScopeError),
    #[error("querystring: {0}")]
    QueryString(#[from] serde_qs::Error),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> ApiError {
      match e {
        ParseError::Access(e) => e.into(),
        ParseError::QueryString(e) => e.into(),
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = mongodb::error::Error;

    async fn parse(&self, req: Request) -> Result<Self::Input, Self::ParseError> {
      let station_id = req.param("station").unwrap();

      let access_token_scope = request_ext::get_access_token_scope(&req).await?;

      let station = access_token_scope.grant_station_scope(station_id).await?;

      let query: Query = req.qs()?;

      Ok(Self::Input {
        access_token_scope,
        station,
        query,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        access_token_scope: _,
        station,
        query: Query {
          page: PaginationQs { skip, limit },
        },
      } = input;

      let filter = doc! { AudioFile::KEY_STATION_ID: &station.id };
      let sort = doc! { AudioFile::KEY_ORDER: 1 };
      let page = AudioFile::paged(filter, sort, skip, limit).await?;

      Ok(Output {
        files: page,
        playlist_is_randomly_shuffled: station.playlist_is_randomly_shuffled,
      })
    }
  }
}

pub mod post {

  use bytes::Bytes;
  use futures::Stream;
  use hyper::header::CONTENT_LENGTH;
  use serde::de::Error;
  use upload::UploadError;

  use crate::request_ext::get_access_token_scope;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {
    pub deployment_id: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/stations/[station]/files/POST/")]
  pub struct Query {
    pub filename: String,
  }

  #[derive(Debug, Clone)]
  pub struct Input<S> {
    pub station: Station,
    pub filename: String,
    pub stream: S,
    pub size_hint: Option<u64>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/stations/[station]/files/POST/")]
  #[serde(rename_all = "snake_case")]
  pub struct Output {
    file: AudioFile,
  }

  impl Endpoint {
    pub async fn perform_stream<
      E: std::error::Error + Send + Sync + 'static,
      S: Stream<Item = Result<Bytes, E>> + Send + 'static,
    >(
      &self,
      input: Input<S>,
    ) -> Result<Output, upload::UploadError<E>> {
      // let size_limit = (input.station.limits.storage.avail as usize)
      //   .saturating_sub(input.station.limits.storage.used as usize);

      let file = upload::upload_audio_file(
        self.deployment_id.clone(),
        input.station.id,
        None,
        input.size_hint,
        input.filename,
        input.stream,
      )
      .await?;

      Ok(Output { file })
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("token: {0}")]
    Token(#[from] GetAccessTokenScopeError),
    #[error("querystring: {0}")]
    Query(#[from] serde_qs::Error),
    #[error("content length is required")]
    ContentLengthRequired,
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> Self {
      match e {
        ParseError::Token(e) => e.into(),
        ParseError::Query(e) => e.into(),
        ParseError::ContentLengthRequired => ApiError::ContentLengthRequired,
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input<hyper::Body>;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = UploadError<hyper::Error>;

    async fn parse(&self, request: Request) -> Result<Self::Input, ParseError> {
      let station_id = request.param("station").unwrap();
      let query: Query = request.qs()?;

      let filename = query.filename.trim();
      if filename.is_empty() {
        return Err(serde_qs::Error::custom("filename is required").into());
      }

      let content_length: u64 = match request.headers().get(CONTENT_LENGTH) {
        None => return Err(ParseError::ContentLengthRequired),
        Some(v) => match v.to_str() {
          Err(_e) => return Err(ParseError::ContentLengthRequired),
          Ok(s) => match s.parse() {
            Err(_e) => return Err(ParseError::ContentLengthRequired),
            Ok(v) => v,
          },
        },
      };

      let access_token_scope = get_access_token_scope(&request).await?;
      let station = access_token_scope.grant_station_scope(station_id).await?;

      let stream = request.into_body();

      let input = Self::Input {
        station,
        filename: filename.to_string(),
        stream,
        size_hint: Some(content_length),
      };

      Ok(input)
    }

    async fn perform(&self, input: Self::Input) -> Result<Output, Self::HandleError> {
      self.perform_stream(input).await
    }
  }
}
