use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use crate::error::ApiError;
use async_trait::async_trait;
use db::audio_file::AudioFile;
use db::audio_file::OrderDocument;
use db::run_transaction;
use db::station::Station;
use db::Model;
use mongodb::bson::doc;
use mongodb::options::FindOneOptions;
use prex::request::ReadBodyJsonError;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use db::models::increment_station_audio_file_order::IncrementStationAudioFileOrder;
use db::Incrementer;

pub mod swap {

  use super::*;

  pub mod post {

    use serde_util::empty_struct::EmptyStruct;

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, TS)]
    #[ts(
      export,
      export_to = "../../defs/api/stations/[station]/files/[file]/order/swap/POST/"
    )]
    #[serde(rename_all = "snake_case")]
    pub struct Payload {
      other_file_id: String,
    }

    #[derive(Debug, Clone)]
    pub struct Endpoint {}

    #[derive(Debug, Clone)]
    pub struct Input {
      station: Station,
      file_id: String,
      other_file_id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, TS)]
    #[ts(
      export,
      export_to = "../../defs/api/stations/[station]/files/[file]/order/swap/POST/"
    )]
    pub struct Output(EmptyStruct);

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

        let payload: Payload = request.read_body_json(1_000).await?;

        Ok(Self::Input {
          station,
          file_id,
          other_file_id: payload.other_file_id,
        })
      }

      async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
        let Self::Input {
          station,
          file_id,
          other_file_id,
        } = input;

        run_transaction!(session => {

          let file1 = {
            let filter = doc!{ AudioFile::KEY_STATION_ID: &station.id, AudioFile::KEY_ID: &file_id };
            let doc = tx_try!(AudioFile::get_with_session(filter, &mut session).await);
            match doc {
              None => return Err(HandleError::FileNotFound(file_id)),
              Some(doc) => doc
            }
          };

          let file2 = {
            let filter = doc!{ AudioFile::KEY_STATION_ID: &station.id, AudioFile::KEY_ID: &other_file_id };
            let doc = tx_try!(AudioFile::get_with_session(filter, &mut session).await);
            match doc {
              None => return Err(HandleError::FileNotFound(other_file_id)),
              Some(doc) => doc
            }
          };

          {
            let filter = doc!{ AudioFile::KEY_ID: &file1.id };
            let update = doc!{ "$set": { AudioFile::KEY_ORDER: file2.order } };
            tx_try!(AudioFile::cl().update_one_with_session(filter, update, None, &mut session).await);
          }

          {
            let filter = doc!{ AudioFile::KEY_ID: &file2.id };
            let update = doc!{ "$set": { AudioFile::KEY_ORDER: file1.order } };
            tx_try!(AudioFile::cl().update_one_with_session(filter, update, None, &mut session).await);
          }


        });

        let out = Output(EmptyStruct(()));

        Ok(out)
      }
    }
  }
}

pub mod move_to_first {

  use super::*;

  pub mod post {

    use super::*;

    #[derive(Debug, Clone)]
    pub struct Endpoint {}

    #[derive(Debug, Clone)]
    pub struct Input {
      station: Station,
      file_id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, TS)]
    #[ts(
      export,
      export_to = "../../defs/api/stations/[station]/files/[file]/order/move-to-first/POST/"
    )]
    #[serde(rename_all = "snake_case")]
    pub struct Output {
      order: f64,
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

      async fn parse(&self, request: Request) -> Result<Self::Input, Self::ParseError> {
        let station_id = request.param("station").unwrap().to_string();

        let file_id = request.param("file").unwrap().to_string();

        let access_token_scope = request_ext::get_access_token_scope(&request).await?;

        let station = access_token_scope.grant_station_scope(&station_id).await?;

        Ok(Self::Input { station, file_id })
      }

      async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
        let Self::Input { station, file_id } = input;

        let order = run_transaction!(session => {
          let filter = doc!{ AudioFile::KEY_STATION_ID: &station.id };
          let sort = doc!{ AudioFile::KEY_ORDER: 1 };
          let options = FindOneOptions::builder().sort(sort).build();
          let new_order = match tx_try!(AudioFile::cl_as::<OrderDocument>().find_one_with_session(filter, options, &mut session).await) {
            Some(doc) => doc.order - 1.0,
            None => -1.0
          };

          let filter = doc!{ AudioFile::KEY_ID: &file_id, AudioFile::KEY_STATION_ID: &station.id };
          let update = doc!{ "$set": { AudioFile::KEY_ORDER: new_order } };
          let r = tx_try!(AudioFile::cl().update_one_with_session(filter, update, None, &mut session).await);

          if r.matched_count == 0 {
            return Err(HandleError::FileNotFound(file_id));
          }

          new_order
        });

        let out = Output { order };

        Ok(out)
      }
    }
  }
}

pub mod move_to_last {

  use super::*;

  pub mod post {

    use super::*;

    #[derive(Debug, Clone)]
    pub struct Endpoint {}

    #[derive(Debug, Clone)]
    pub struct Input {
      station: Station,
      file_id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, TS)]
    #[ts(
      export,
      export_to = "../../defs/api/stations/[station]/files/[file]/order/move-to-last/POST/"
    )]
    #[serde(rename_all = "snake_case")]
    pub struct Output {
      order: f64,
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

      async fn parse(&self, request: Request) -> Result<Self::Input, Self::ParseError> {
        let station_id = request.param("station").unwrap().to_string();

        let file_id = request.param("file").unwrap().to_string();

        let access_token_scope = request_ext::get_access_token_scope(&request).await?;

        let station = access_token_scope.grant_station_scope(&station_id).await?;

        Ok(Self::Input { station, file_id })
      }

      async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
        let Self::Input { station, file_id } = input;

        let order = run_transaction!(session => {
          let new_order = tx_try!(IncrementStationAudioFileOrder::next_with_session(&station.id, &mut session).await);
          let filter = doc!{ AudioFile::KEY_STATION_ID: &station.id, AudioFile::KEY_ID: &file_id };
          let update = doc!{ "$set": { AudioFile::KEY_ORDER: new_order } };
          let r = tx_try!(AudioFile::cl().update_one_with_session(filter, update, None, &mut session).await);
          if r.matched_count == 0 {
            return Err(HandleError::FileNotFound(file_id));
          }
          new_order
        });

        let out = Output { order };

        Ok(out)
      }
    }
  }
}
