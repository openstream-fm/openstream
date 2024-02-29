pub mod get {
  use db::models::station_slug::StationSlug;
  use modify::Modify;
  use prex::Request;
  use schemars::JsonSchema;
  use serde::{Deserialize, Serialize};
  use ts_rs::TS;
  use validator::Validate;

  use crate::json::JsonHandler;

  #[derive(Debug, Clone)]
  pub struct Endpoint {}

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema, Modify, Validate)]
  #[ts(
    export,
    export_to = "../../../defs/api/stations/is-slug-available/GET/"
  )]
  #[macros::schema_ts_export]
  pub struct Query {
    station_id: Option<String>,
    slug: String,
  }
  pub struct Input {
    station_id: Option<String>,
    slug: String,
  }

  #[derive(Debug, Clone, Serialize, Deserialize, TS, JsonSchema)]
  #[ts(
    export,
    export_to = "../../../defs/api/stations/is-slug-available/GET/"
  )]
  #[macros::schema_ts_export]
  pub struct Output {
    is_available: bool,
  }

  #[async_trait::async_trait]
  impl JsonHandler for Endpoint {
    type Input = Input;
    type Output = Output;
    type ParseError = serde_qs::Error;
    type HandleError = mongodb::error::Error;

    async fn parse(&self, req: Request) -> Result<Input, Self::ParseError> {
      let Query { station_id, slug } = req.qs()?;
      Ok(Input { station_id, slug })
    }

    async fn perform(&self, input: Input) -> Result<Output, Self::HandleError> {
      let Input { station_id, slug } = input;

      let is_available =
        StationSlug::is_slug_available_for_station(station_id.as_deref(), &slug).await?;

      Ok(Output { is_available })
    }
  }
}
