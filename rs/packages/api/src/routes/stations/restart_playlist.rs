use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use async_trait::async_trait;
use db::station::Station;
use mongodb::bson::doc;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod post {

  use db::{deployment::Deployment, Model};
  use drop_tracer::DropTracer;
  use hyper::http::HeaderValue;
  use schemars::JsonSchema;
  use serde_util::empty_struct::EmptyStruct;
  use shutdown::Shutdown;

  use crate::request_ext::X_ACCESS_TOKEN;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {
    pub deployment_id: String,
    pub drop_tracer: DropTracer,
    pub shutdown: Shutdown,
    pub media_sessions: media::MediaSessionMap,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    access_token_header: Option<HeaderValue>,
    station: Station,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(
    export,
    export_to = "../../../defs/api/stations/[station]/restart-playlist/POST/"
  )]
  #[macros::schema_ts_export]
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

  // impl From<HandleError> for ApiError {
  //   fn from(e: HandleError) -> Self {
  //     match e {
  //       HandleError::Db(e) => e.into(),
  //       HandleError::StationIsLive => ApiError::PlaylistStartIsLive,
  //       HandleError::StationIsExternalRelay => ApiError::PlaylistStartIsExternalRelay,
  //       HandleError::NoFiles => ApiError::PlaylistStartNoFiles,
  //     }
  //   }
  // }

  #[async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = GetAccessTokenScopeError;
    type HandleError = mongodb::error::Error;

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

      match station.owner_deployment_info {
        None => {
          let _ = self.media_sessions.playlist_restart(&station.id).await;
        }

        Some(info) => {
          if info.deployment_id == self.deployment_id {
            let _ = self.media_sessions.playlist_restart(&station.id).await;
          } else {
            #[allow(clippy::collapsible_else_if)]
            if let Some(deployment) = Deployment::get_by_id(&info.deployment_id).await? {
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
      }

      Ok(Output(EmptyStruct(())))
    }
  }
}
