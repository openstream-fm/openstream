use crate::json::JsonHandler;
use crate::request_ext::{self, GetAccessTokenScopeError};

use crate::error::ApiError;
use async_trait::async_trait;
use db::station::Station;
use mongodb::bson::doc;
use prex::Request;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
pub mod post {

  use db::{deployment::Deployment, Model};
  use hyper::{http::HeaderValue, Body};
  use media::MediaSessionMap;

  use crate::request_ext::X_ACCESS_TOKEN;

  use super::*;

  #[derive(Debug, Clone)]
  pub struct Endpoint {
    pub deployment_id: String,
    pub media_sessions: MediaSessionMap,
  }

  #[derive(Debug, Clone)]
  pub struct Input {
    access_token_header: Option<HeaderValue>,
    station: Station,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS)]
  #[ts(export)]
  #[ts(export_to = "../../../defs/api/stations/[station]/reset-source-password/POST/")]
  pub struct Output {
    new_password: String,
  }

  #[derive(Debug, thiserror::Error)]
  pub enum HandleError {
    #[error("mongo: {0}")]
    Db(#[from] mongodb::error::Error),
  }

  impl From<HandleError> for ApiError {
    fn from(e: HandleError) -> Self {
      match e {
        HandleError::Db(e) => e.into(),
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
      Ok(Input {
        station,
        access_token_header,
      })
    }

    async fn perform(&self, input: Self::Input) -> Result<Self::Output, Self::HandleError> {
      let Input {
        station,
        access_token_header,
      } = input;

      let new_password = Station::random_source_password();

      let update = doc! { "$set": { Station::KEY_SOURCE_PASSWORD: &new_password } };
      Station::update_by_id(&station.id, update).await?;

      let deployment_id = self.deployment_id.clone();
      let media_sessions = self.media_sessions.clone();
      tokio::spawn(async move {
        match station.owner_deployment_info {
          None => {}
          Some(info) => {
            if info.deployment_id == deployment_id {
              crate::routes::runtime::source_password_updated::station_id::perform(
                &media_sessions,
                station.id,
              )
              .await;
            } else {
              #[allow(clippy::collapsible_else_if)]
              if let Ok(Some(deployment)) = Deployment::get_by_id(&info.deployment_id).await {
                use rand::seq::SliceRandom;
                let addr = deployment.local_ip;
                let port = deployment.api_ports.choose(&mut rand::thread_rng());
                if let Some(port) = port {
                  let uri = format!(
                    "http://{}:{}/runtime/source-password-updated/{}",
                    addr, port, station.id
                  );

                  let client = hyper::Client::default();
                  let mut req = hyper::Request::builder()
                    .method(hyper::http::Method::POST)
                    .uri(uri);

                  if let Some(v) = access_token_header {
                    if let Ok(v) = v.to_str() {
                      req = req.header(X_ACCESS_TOKEN, v);
                    }
                  };

                  if let Ok(req) = req.body(Body::empty()) {
                    let _ = client.request(req).await;
                  }
                }
              }
            }
          }
        }
      });

      Ok(Output { new_password })
    }
  }
}
