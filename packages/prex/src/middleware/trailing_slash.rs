use async_trait::async_trait;
use std::fmt::{self, Display};

use hyper::StatusCode;
use hyper::header::HeaderValue;

use crate::request::Request;
use crate::response::Response;
use crate::next::Next;

use crate::handler::Handler;

pub fn append() -> TrailingSlash {
  TrailingSlash { variant: TrailingSlashVariant::Append }
}

pub fn trim() -> TrailingSlash { 
  TrailingSlash { variant: TrailingSlashVariant::Trim }
}

#[derive(Clone, Debug)]
pub struct TrailingSlash {
  variant: TrailingSlashVariant
}

fn query_str(query: Option<&str>) -> QueryStr {
  QueryStr(query)
}

struct QueryStr<'a>(Option<&'a str>);

impl<'a> Display for QueryStr<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.0 {
      Some(str) => write!(f, "?{str}"),
      None => write!(f, "")
    }
}}


#[async_trait]
impl Handler for TrailingSlash {
  
  async fn call(&self, req: Request, next: Next) -> Response {
  
    let uri = req.uri();
    
    let path = match uri.path() {
      "" => "/",
      path => path
    };

    match self.variant {
      
      TrailingSlashVariant::Append => {
        if !path.ends_with('/') {
          let location = format!("{}/{}", path, query_str(uri.query()));
          return Response::redirect(StatusCode::MOVED_PERMANENTLY, HeaderValue::from_str(location.as_str()).unwrap());
        }
      },

      TrailingSlashVariant::Trim => {
        if path != "/" && path.ends_with('/') {
          let new_path = path.trim_end_matches('/');
          let location = format!("{}{}", new_path, query_str(uri.query()));
          return Response::redirect(StatusCode::MOVED_PERMANENTLY, HeaderValue::from_str(location.as_str()).unwrap());
        }
      }
    }

    next.run(req).await
  }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum TrailingSlashVariant {
  Append,
  Trim
}

