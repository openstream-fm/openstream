pub mod id;

use crate::json::JsonHandler;
use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

use crate::error::ApiError;
use async_trait::async_trait;
use db::account::Account;
use db::audio_file::AudioFile;
use db::Model;
use db::Paged;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod get {

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  pub const DEFAULT_SKIP: u64 = 0;
  pub const DEFAULT_LIMIT: i64 = 60;

  fn default_skip() -> u64 {
    DEFAULT_SKIP
  }

  fn default_limit() -> i64 {
    DEFAULT_LIMIT
  }

  #[derive(Debug, Serialize, Deserialize, Clone, TS)]
  #[ts(export, export_to = "../../defs/api/accounts/[account]/files/GET/")]
  pub struct Output(Paged<AudioFile>);

  #[derive(Debug, Serialize, Deserialize, Default, TS)]
  #[ts(export, export_to = "../../defs/api/accounts/[account]/files/GET/")]
  struct Query {
    #[serde(skip_serializing_if = "Option::is_none")]
    skip: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    account: Account,
    #[allow(unused)]
    access_token_scope: AccessTokenScope,
    skip: u64,
    limit: i64,
  }

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("access: {0}")]
    Access(#[from] GetAccessTokenScopeError),
    #[error("querystring: {0}")]
    QueryString(#[from] serde_querystring::Error),
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
      let account_id = req.param("account").unwrap();

      let access_token_scope = request_ext::get_access_token_scope(&req).await?;

      let account = access_token_scope.grant_account_scope(account_id).await?;

      let Query { skip, limit } = match req.uri().query() {
        None => Default::default(),
        Some(qs) => serde_querystring::from_str(qs)?,
      };

      Ok(Self::Input {
        access_token_scope,
        account,
        skip: skip.unwrap_or_else(default_skip),
        limit: limit.unwrap_or_else(default_limit),
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        access_token_scope: _,
        account,
        skip,
        limit,
      } = input;

      let filter = mongodb::bson::doc! { AudioFile::KEY_ACCOUNT_ID: account.id };
      let page = AudioFile::paged(filter, skip, limit).await?;

      Ok(Output(page))
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
  pub struct Endpoint {}

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../defs/api/accounts/[account]/files/POST/")]
  pub struct Query {
    pub filename: String,
  }

  #[derive(Debug, Clone)]
  pub struct Input<S> {
    pub account: Account,
    pub filename: String,
    pub stream: S,
    pub size_hint: Option<u64>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../defs/api/accounts/[account]/files/POST/")]
  #[serde(rename_all = "camelCase")]
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
      // let size_limit = (input.account.limits.storage.avail as usize)
      //   .saturating_sub(input.account.limits.storage.used as usize);

      let file = upload::upload_audio_file(
        input.account.id,
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
    Query(#[from] serde_querystring::Error),
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
      let account_id = request.param("account").unwrap();
      let query: Query = serde_querystring::from_str(request.uri().query().unwrap_or(""))?;

      let filename = query.filename.trim();
      if filename.is_empty() {
        return Err(serde_querystring::Error::custom("filename is required").into());
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
      let account = access_token_scope.grant_account_scope(account_id).await?;

      let stream = request.into_body();

      let input = Self::Input {
        account,
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
