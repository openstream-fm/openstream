use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::station::Station;
use mongodb::bson::doc;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod post {

  use db::{
    audio_file::AudioFile,
    deployment::Deployment,
    media_session::{MediaSession, MediaSessionKind},
    Model,
  };
  use drop_tracer::DropTracer;
  use hyper::http::HeaderValue;
  use serde_util::empty_struct::EmptyStruct;
  use shutdown::Shutdown;

  use crate::{error::ApiError, request_ext::X_ACCESS_TOKEN};

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {
    pub deployment_id: String,
    pub drop_tracer: DropTracer,
    pub shutdown: Shutdown,
    pub media_sessions: media_sessions::MediaSessionMap,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    access_token_header: Option<HeaderValue>,
    station: Station,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../../defs/api/stations/[station]/restart-playlist/POST/")]
  pub struct Output(EmptyStruct);

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongo: {0}")]
    Db(#[from] mongodb::error::Error),
    #[error("station is currently live streaming")]
    StationIsLive,
    #[error("station is currently streaming from external relay")]
    StationIsExternalRelay,
    #[error("no media files in station")]
    NoFiles,
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
        HandleError::StationIsLive => ApiError::PlaylistStartIsLive,
        HandleError::StationIsExternalRelay => ApiError::PlaylistStartIsExternalRelay,
        HandleError::NoFiles => ApiError::PlaylistStartNoFiles,
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
      let access_token_header = req.headers().get(X_ACCESS_TOKEN).cloned();
      let access_token_scope = request_ext::get_access_token_scope(&req).await?;
      let station = access_token_scope.grant_station_scope(station_id).await?;
      Ok(Self::Input {
        access_token_header,
        station,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Self::Input {
        access_token_header,
        station,
      } = input;

      match MediaSession::get_current_for_station(&station.id).await? {
        None => {
          let filter = doc! { AudioFile::KEY_STATION_ID: &station.id };
          if !AudioFile::exists(filter).await? {
            return Err(HandleError::NoFiles);
          }
        }

        Some(media_session) => match media_session.kind {
          MediaSessionKind::Live { .. } => return Err(HandleError::StationIsLive),
          MediaSessionKind::ExternalRelay { .. } => {
            return Err(HandleError::StationIsExternalRelay)
          }
          MediaSessionKind::Playlist { .. } => {
            if media_session.deployment_id == self.deployment_id {
              let mut lock = self.media_sessions.write();
              let _ = lock.restart(&station.id, self.shutdown.clone(), self.drop_tracer.clone());
            } else {
              #[allow(clippy::collapsible_else_if)]
              if let Some(deployment) = Deployment::get_by_id(&media_session.deployment_id).await? {
                use rand::seq::SliceRandom;
                let port = deployment.api_ports.choose(&mut rand::thread_rng());
                if let Some(port) = port {
                  let client = hyper::Client::default();
                  let addr = deployment.local_ip;
                  let uri = format!(
                    "http://{}:{}/runtime/restart-playlist/{}",
                    addr, port, station.id
                  );

                  let mut req = hyper::Request::builder()
                    .method(hyper::http::Method::POST)
                    .uri(uri);

                  if let Some(v) = access_token_header {
                    if let Ok(v) = v.to_str() {
                      req = req.header(X_ACCESS_TOKEN, v);
                    }
                  }

                  if let Ok(req) = req.body(hyper::Body::empty()) {
                    tokio::spawn(async move {
                      let _ = client.request(req).await;
                    });
                  }
                }
              }
            }
          }
        },
      };

      Ok(Output(EmptyStruct(())))
    }
  }
}
