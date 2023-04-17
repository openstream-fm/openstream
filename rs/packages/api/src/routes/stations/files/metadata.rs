use crate::json::JsonHandler;
use crate::request_ext::{self, AccessTokenScope, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::audio_file::AudioFile;
use db::station::Station;
use db::Model;
use mongodb::bson::doc;
use prex::Request;
use serde::{Deserialize, Serialize};

pub mod patch {

  use db::run_transaction;
  use prex::request::ReadBodyJsonError;
  use ts_rs::TS;

  use crate::error::ApiError;
  use serde_util::map_some;

  use super::*;

  #[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone, TS)]
  #[ts(
    export,
    export_to = "../../../defs/api/stations/[station]/files/[file]/metadata/PATCH/"
  )]
  #[serde(rename_all = "snake_case")]
  pub struct Payload {
    #[ts(optional)]
    #[serde(
      default,
      deserialize_with = "map_some",
      skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<Option<String>>,

    #[ts(optional)]
    #[serde(
      default,
      deserialize_with = "map_some",
      skip_serializing_if = "Option::is_none"
    )]
    pub artist: Option<Option<String>>,

    #[ts(optional)]
    #[serde(
      default,
      deserialize_with = "map_some",
      skip_serializing_if = "Option::is_none"
    )]
    pub album: Option<Option<String>>,

    #[ts(optional)]
    #[serde(
      default,
      deserialize_with = "map_some",
      skip_serializing_if = "Option::is_none"
    )]
    pub album_artist: Option<Option<String>>,

    #[ts(optional)]
    #[serde(
      default,
      deserialize_with = "map_some",
      skip_serializing_if = "Option::is_none"
    )]
    pub genre: Option<Option<String>>,

    #[ts(optional)]
    #[serde(
      default,
      deserialize_with = "map_some",
      skip_serializing_if = "Option::is_none"
    )]
    pub year: Option<Option<i32>>,

    #[ts(optional)]
    #[serde(
      default,
      deserialize_with = "map_some",
      skip_serializing_if = "Option::is_none"
    )]
    pub comment: Option<Option<String>>,

    #[ts(optional)]
    #[serde(
      default,
      deserialize_with = "map_some",
      skip_serializing_if = "Option::is_none"
    )]
    pub track: Option<Option<u16>>,
  }

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone)]
  pub struct Input {
    station: Station,
    #[allow(unused)]
    access_token_scope: AccessTokenScope,
    file_id: String,
    payload: Payload,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(
    export,
    export_to = "../../../defs/api/stations/[station]/files/[file]/metadata/PATCH/"
  )]
  #[serde(rename_all = "snake_case")]
  pub struct Output {
    item: AudioFile,
  }

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("token: {0}")]
    Token(#[from] GetAccessTokenScopeError),
    #[error("payload: {0}")]
    Payload(#[from] ReadBodyJsonError),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> Self {
      match e {
        ParseError::Token(e) => e.into(),
        ParseError::Payload(e) => e.into(),
      }
    }
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
    type ParseError = ParseError;
    type HandleError = HandleError;

    async fn parse(&self, mut request: Request) -> Result<Self::Input, Self::ParseError> {
      let station_id = request.param("station").unwrap().to_string();

      let file_id = request.param("file").unwrap().to_string();

      let access_token_scope = request_ext::get_access_token_scope(&request).await?;

      let station = access_token_scope.grant_station_scope(&station_id).await?;

      let payload: Payload = request.read_body_json(10_000).await?;

      Ok(Self::Input {
        access_token_scope,
        station,
        file_id,
        payload,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        access_token_scope: _,
        station,
        file_id,
        payload,
      } = input;

      let document = run_transaction!(session => {
        let filter = doc!{ AudioFile::KEY_STATION_ID: &station.id, db::KEY_ID: &file_id };
        let mut file = match tx_try!(AudioFile::cl().find_one_with_session(filter, None, &mut session).await) {
          None => return Err(HandleError::FileNotFound(file_id)),
          Some(file) => file,
        };

        macro_rules! set_attr_num {
          ($name:ident) => {
            // check is a number
            if let Some(ref $name) = payload.$name {
              file.metadata.$name = $name.clone();
            }
          }
        }

        macro_rules! set_attr_str {
          ($name:ident) => {
            if let Some(ref $name) = payload.$name {
              let value = match $name {
                None => None,
                Some($name) => {
                  let $name = $name.trim();
                  if $name.is_empty() {
                    None
                  } else {
                    Some($name.to_string())
                  }
                }
              };

              file.metadata.$name = value;
            }
          }
        }

        set_attr_str!(title);
        set_attr_str!(artist);
        set_attr_str!(album);
        set_attr_str!(album_artist);
        set_attr_str!(genre);
        set_attr_str!(comment);
        set_attr_num!(year);
        set_attr_num!(track);

        tx_try!(AudioFile::replace_with_session(&file.id, &file, &mut session).await);

        file
      });

      let out = Output { item: document };

      Ok(out)
    }
  }
}
