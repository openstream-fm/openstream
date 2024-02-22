pub mod post {
  use db::audio_file::AudioFile;
  use db::models::station::Station;
  use db::models::station_files_pre_shuffle_checkpoint::StationFilesPreShuffleCheckpoint;
  use db::{run_transaction, IdDocument, Model};
  use mongodb::bson::doc;
  use mongodb::options::FindOptions;
  use prex::Request;
  use schemars::JsonSchema;
  use serde::{Deserialize, Serialize};
  use serde_util::empty_struct::EmptyStruct;
  use ts_rs::TS;

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongo: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("station not found: {0}")]
    StationNotFound(String),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::StationNotFound(id) => ApiError::StationNotFound(id),
      }
    }
  }

  use crate::{
    error::ApiError,
    json::JsonHandler,
    request_ext::{self, GetAccessTokenScopeError},
  };

  #[derive(Debug, Clone)]
  pub struct Input {
    station_id: String,
  }

  #[derive(Debug, Clone, Copy, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(
    export,
    export_to = "../../../defs/api/stations/[station]/files/unshuffle/POST/"
  )]
  #[macros::schema_ts_export]
  pub struct Output(EmptyStruct);

  #[derive(Debug, thiserror::Error)]
  pub enum ParseError {
    #[error("mongo: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("token: {0}")]
    Token(#[from] GetAccessTokenScopeError),
  }

  impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> Self {
      match e {
        ParseError::Db(e) => e.into(),
        ParseError::Token(e) => e.into(),
      }
    }
  }

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[async_trait::async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = ParseError;
    type HandleError = HandleError;

    async fn parse(&self, request: Request) -> Result<Input, ParseError> {
      let station_id = request.param("station").unwrap();
      let access_token_scope = request_ext::get_access_token_scope(&request).await?;
      let station = access_token_scope.grant_station_scope(station_id).await?;
      Ok(Input {
        station_id: station.id,
      })
    }

    async fn perform(&self, input: Input) -> Result<Output, Self::HandleError> {
      let Input { station_id } = input;

      run_transaction!(session => {

        let station = match tx_try!(Station::get_by_id_with_session(&station_id, &mut session).await) {
          Some(station) => station,
          None => return Err(HandleError::StationNotFound(station_id)),
        };

        // if not randomly shuffle we ignore and return Ok
        if !station.playlist_is_randomly_shuffled {
          return Ok(Output(EmptyStruct(())));
        }

        let update = doc!{ "$set": { Station::KEY_PLAYLIST_IS_RANDOMLY_SHUFFLED: false } };
        tx_try!(Station::update_by_id_with_session(&station.id, update, &mut session).await);

        let sorted_ids = match tx_try!(StationFilesPreShuffleCheckpoint::get_by_id_with_session(&station_id, &mut session).await) {

          Some(doc) => {

            let mut sorted_ids = doc.file_ids;

            let filter = doc!{ AudioFile::KEY_STATION_ID: &station.id, AudioFile::KEY_ID: { "$nin": &sorted_ids } };
            let sort = doc!{ AudioFile::KEY_CREATED_AT: 1 };
            let options = FindOptions::builder().sort(sort).projection(IdDocument::projection()).build();
            let mut cursor = tx_try!(AudioFile::cl_as::<IdDocument>().find_with_session(filter, options, &mut session).await);

            while let Some(r) = cursor.next(&mut session).await {
              let doc = tx_try!(r);
              sorted_ids.push(doc.id);
            }

            sorted_ids
          },

          None => {

            let mut sorted_ids = vec![];

            let filter = doc!{ AudioFile::KEY_STATION_ID: &station.id };
            let sort = doc!{ AudioFile::KEY_CREATED_AT: 1 };
            let options = FindOptions::builder().sort(sort).projection(IdDocument::projection()).build();
            let mut cursor = tx_try!(AudioFile::cl_as::<IdDocument>().find_with_session(filter, options, &mut session).await);

            while let Some(r) = cursor.next(&mut session).await {
              let doc = tx_try!(r);
              sorted_ids.push(doc.id);
            }

            sorted_ids
          }
        };

        let mut i = 0.0;
        for id in sorted_ids {
          i += 1.0;
          let order = (i * 2.0) + rand::random::<f64>();
          let filter = doc! { AudioFile::KEY_ID: id, AudioFile::KEY_STATION_ID: &station.id };
          let update = doc! { "$set": { AudioFile::KEY_ORDER: order } };
          tx_try!(AudioFile::cl().update_one_with_session(filter, update, None, &mut session).await);
        }
      });

      Ok(Output(EmptyStruct(())))
    }
  }
}
