use crate::endpoint::Endpoint;
use crate::request::Request;
use crate::response::Response;

use std::sync::Arc;

pub struct Next {
  pub(crate) enpoints: Arc<Vec<Endpoint>>,
  pub(crate) index: usize,
}

impl Next {
  pub async fn run(mut self, mut req: Request) -> Response {
    for endpoint in self.enpoints.clone().iter().skip(self.index) {
      self.index += 1;
      if let Some(params) = endpoint.matcher.r#match(req.method(), req.uri().path()) {
        req.params = params;
        return endpoint.handler.call(req, self).await;
      }
    }

    Response::default_not_found(format!("Cannot {} {}", req.method(), req.uri().path()))
  }
}
