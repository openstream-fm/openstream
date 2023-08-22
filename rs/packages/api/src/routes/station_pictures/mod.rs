use async_trait::async_trait;
use db::station_picture::StationPicture;
use db::station_picture_variant::{StationPictureVariant, StationPictureVariantFormat};
use db::Model;
use hyper::header::{
  ACCESS_CONTROL_ALLOW_ORIGIN, CACHE_CONTROL, /*CACHE_CONTROL,*/ CONTENT_LENGTH, CONTENT_TYPE,
  ETAG, IF_NONE_MATCH,
};
use hyper::http::HeaderValue;
use hyper::{Body, StatusCode};
use mongodb::bson::doc;
use prex::{handler::Handler, Request, Response};

use crate::error::ApiError;

#[derive(Debug, Clone, Copy)]
pub enum StationPicHandler {
  Webp(f64),
  Png(f64),
  Source,
}

#[async_trait]
impl Handler for StationPicHandler {
  async fn call(&self, req: Request, _next: prex::Next) -> Response {
    let id = req.param("picture").unwrap();

    #[derive(serde::Deserialize)]
    struct VQs {
      #[allow(unused)]
      v: Option<String>,
    }

    let has_version_qs = req.qs::<VQs>().is_ok();

    let filter = match *self {
      Self::Webp(size) => doc! {
        StationPictureVariant::KEY_PICTURE_ID: id,
        StationPictureVariant::KEY_FORMAT: StationPictureVariantFormat::Webp,
        StationPictureVariant::KEY_SIZE: size,
      },

      Self::Png(size) => doc! {
        StationPictureVariant::KEY_PICTURE_ID: id,
        StationPictureVariant::KEY_FORMAT: StationPictureVariantFormat::Png,
        StationPictureVariant::KEY_SIZE: size,
      },

      Self::Source => doc! {
        StationPictureVariant::KEY_PICTURE_ID: id,
        StationPictureVariant::KEY_FORMAT: StationPictureVariantFormat::Source,
      },
    };

    match StationPictureVariant::get(filter).await {
      Err(e) => ApiError::from(e).into_json_response(),
      Ok(r) => match r {
        None => ApiError::ResourceNotFound {}.into_json_response(),
        Some(doc) => {
          let response_etag = format!("W/\"{}-{}-{}\"", doc.id, doc.size, StationPicture::VERSION);

          let request_etag = match req.headers().get(IF_NONE_MATCH) {
            None => None,
            Some(v) => v.to_str().ok(),
          };

          let is_match = if let Some(v) = request_etag {
            v == response_etag
          } else {
            false
          };

          let res = if is_match {
            Response::new(StatusCode::NOT_MODIFIED)
          } else {
            let mut res = Response::new(StatusCode::OK);

            match HeaderValue::from_str(&doc.content_type) {
              Err(_) => res.headers_mut().append(
                CONTENT_TYPE,
                HeaderValue::from_static("application/octet-stream"),
              ),
              Ok(value) => res.headers_mut().append(CONTENT_TYPE, value),
            };

            if has_version_qs {
              res.headers_mut().append(
                CACHE_CONTROL,
                HeaderValue::from_static("public,max-age=31536000,immutable"), // 1 year
              );
            }

            res
              .headers_mut()
              .append(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));

            res
              .headers_mut()
              .append(ETAG, HeaderValue::from_str(&response_etag).unwrap());

            res.headers_mut().append(
              CONTENT_LENGTH,
              HeaderValue::from_str(doc.data.len().to_string().as_str()).unwrap(),
            );

            let body = Body::from(doc.data);

            *res.body_mut() = body;

            res
          };

          res
        }
      },
    }
  }
}

pub mod post {
  use super::*;
  use crate::{
    json::JsonHandler,
    request_ext::{self, GetAccessTokenScopeError},
  };
  use bytes::Bytes;
  use db::station_picture::{CreateStationPictureError, StationPicture};
  use prex::request::ReadBodyBytesError;
  use serde::{Deserialize, Serialize};
  use ts_rs::TS;

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/station-pictures/POST/")]
  pub struct Query {
    pub account_id: String,
    pub filename: String,
    pub content_type: String,
  }

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    query: Query,
    data: Bytes,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export, export_to = "../../../defs/api/station-pictures/POST/")]
  pub struct Output(StationPicture);

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("qs: {0}")]
    Query(#[from] serde_qs::Error),
    #[error("token: {0}")]
    Token(#[from] GetAccessTokenScopeError),
    #[error("payload: {0}")]
    Payload(#[from] ReadBodyBytesError),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> Self {
      match e {
        ParseError::Query(e) => e.into(),
        ParseError::Token(e) => e.into(),
        ParseError::Payload(e) => e.into(),
      }
    }
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("create: {0}")]
    Create(#[from] CreateStationPictureError),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Create(e) => e.into(),
      }
    }
  }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = HandleError;

    async fn parse(&self, mut req: Request) -> Result<Input, ParseError> {
      let query: Query = req.qs()?;

      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      let _account = access_token_scope
        .grant_account_scope(&query.account_id)
        .await?;

      let data = req.read_body_bytes(2_000_000).await?;

      Ok(Input { query, data })
    }

    async fn perform(&self, input: Input) -> Result<Output, HandleError> {
      let Input { query, data } = input;

      let (picture, _variants) =
        StationPicture::create(query.account_id, query.filename, query.content_type, data).await?;

      Ok(Output(picture))
    }
  }
}
