pub mod post {
  use db::models::station::Station;
  use db::models::station_files_pre_shuffle_checkpoint::StationFilesPreShuffleCheckpoint;
  use db::{audio_file::AudioFile, run_transaction};
  use db::{IdDocument, Model};
  use mongodb::bson::doc;
  use mongodb::options::{FindOneAndReplaceOptions, FindOptions};
  use prex::Request;
  use rand::seq::SliceRandom;
  use serde::{Deserialize, Serialize};
  use serde_util::empty_struct::EmptyStruct;
  use serde_util::DateTime;
  use ts_rs::TS;

  use crate::{
    error::ApiError,
    json::JsonHandler,
    request_ext::{self, GetAccessTokenScopeError},
  };

  #[derive(Debug, Clone)]
  pub struct Input {
    station_id: String,
  }

  #[derive(Debug, Clone, Copy, Serialize, Deserialize, TS)]
  #[ts(
    export,
    export_to = "../../../defs/api/stations/[station]/files/suffle/POST/"
  )]
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

        let station = match tx_try!(Station::get_with_session(doc!{ Station::KEY_ID: &station_id }, &mut session).await) {
          Some(station) => station,
          None => return Err(HandleError::StationNotFound(station_id))
        };

        if !station.playlist_is_randomly_shuffled {
          let update = doc!{ "$set": { Station::KEY_PLAYLIST_IS_RANDOMLY_SHUFFLED: true } };
          tx_try!(Station::update_by_id_with_session(&station_id, update, &mut session).await);

          let filter = doc!{ AudioFile::KEY_STATION_ID: &station_id };
          let sort = doc!{ AudioFile::KEY_ORDER: 1 };
          let options = FindOptions::builder().sort(sort).projection(IdDocument::projection()).build();
          let mut cursor = tx_try!(AudioFile::cl_as::<IdDocument>().find_with_session(filter, options, &mut session).await);

          let mut ids = vec![];
          while let Some(r) = cursor.next(&mut session).await {
            let doc = tx_try!(r);
            ids.push(doc.id);
          }

          let checkpoint = StationFilesPreShuffleCheckpoint {
            id: station_id.clone(),
            file_ids: ids,
            created_at: DateTime::now(),
          };

          let filter = doc!{ StationFilesPreShuffleCheckpoint::KEY_ID: &station_id };
          let options = FindOneAndReplaceOptions::builder().upsert(true).build();
          tx_try!(StationFilesPreShuffleCheckpoint::cl().find_one_and_replace_with_session(filter, &checkpoint, options, &mut session).await);
        }



        let filter = doc! { AudioFile::KEY_STATION_ID: &station_id };
        let mut ids = tx_try!(AudioFile::cl()
          .distinct_with_session(AudioFile::KEY_ID, filter, None, &mut session)
          .await);

        {
          let mut rng = rand::thread_rng();
          ids.shuffle(&mut rng);
        }

        for (i, id) in ids.iter().enumerate() {
          let order = ((i as f64 + 1.0) * 2.0) + rand::random::<f64>();
          let filter = doc! { AudioFile::KEY_ID: id };
          let update = doc! { "$set": { AudioFile::KEY_ORDER: order } };
          tx_try!(AudioFile::cl().update_one_with_session(filter, update, None, &mut session).await);
        }
      });

      Ok(Output(EmptyStruct(())))
    }
  }
}
