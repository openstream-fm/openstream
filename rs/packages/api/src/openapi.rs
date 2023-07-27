use crate::routes::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(admins::get::openapi))]
pub struct Api;

#[cfg(test)]
mod test {
  use super::*;

  use std::path::Path;

  #[test]
  fn export_openapi_definition() {
    let target = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../defs/api/openapi.json");
    let json = Api::openapi()
      .to_pretty_json()
      .expect("openapi.to_preety_json");

    std::fs::write(target, json).expect("fs.write");
  }
}
