use std::sync::Arc;
//use std::convert::Infallible;
//use std::net::SocketAddr;

//use core::task::Context;
//use core::task::Poll;

use crate::endpoint::Endpoint;
use crate::next::Next;
use crate::params::Params;
use crate::request::{Parts as RequestParts, Request};
use crate::response::{Parts as ResponseParts, Response};

use hyper;
//use hyper::service::Service;
use hyper::Body;

#[derive(Clone)]
pub struct Router {
  endpoints: Arc<Vec<Endpoint>>,
}

impl Router {
  pub fn new() -> Self {
    Self {
      endpoints: Arc::new(vec![]),
    }
  }

  pub async fn handle(&self, req: Request) -> Response {
    let next = Next {
      enpoints: self.endpoints.clone(),
      index: 0,
    };

    return next.run(req).await;
  }
}

impl Default for Router {
  fn default() -> Self {
    Self::new()
  }
}

pub mod builder {

  use crate::matcher::MatchType;
  use crate::matcher::Matcher;
  use crate::path;
  use hyper::Method;

  use crate::handler::Handler;

  use crate::error::RouterBuilderError;

  use std::sync::Arc;

  pub mod entry {

    use crate::handler::Handler;
    use crate::matcher::MatchType;

    pub struct Endpoint {
      pub method: Option<hyper::Method>,
      pub path: Option<String>,
      pub handler: Box<dyn Handler>,
      pub match_type: MatchType,
    }

    pub struct Builder {
      pub path: Option<String>,
      pub builder: super::Builder,
    }

    pub enum Entry {
      Endpoint(Endpoint),
      Builder(Builder),
    }
  }

  pub struct BuilderAt<'a> {
    builder: &'a mut Builder,
    path: String,
  }

  impl<'a> BuilderAt<'a> {
    fn add(
      &mut self,
      method: Option<Method>,
      handler: impl Handler,
      match_type: MatchType,
    ) -> &mut Self {
      self
        .builder
        .add(method, Some(self.path.clone()), handler, match_type);
      self
    }

    pub fn at(&'a mut self, path: impl AsRef<str>) -> BuilderAt<'a> {
      let new_path = path::join(&self.path, path.as_ref());

      BuilderAt {
        builder: self.builder,
        path: new_path,
      }
    }

    pub fn nest(&mut self, builder: Builder) -> &mut Self {
      let inner_entry = entry::Builder {
        path: Some(self.path.clone()),
        builder,
      };

      let entry = entry::Entry::Builder(inner_entry);

      self.builder.entries.push(entry);

      self
    }

    pub fn with(&mut self, handler: impl Handler) -> &mut Self {
      self.add(None, handler, MatchType::Scope);
      self
    }

    pub fn any(&mut self, handler: impl Handler) -> &mut Self {
      self.add(None, handler, MatchType::Exact)
    }

    pub fn get(&mut self, handler: impl Handler) -> &mut Self {
      self.add(Some(Method::GET), handler, MatchType::Exact)
    }

    pub fn post(&mut self, handler: impl Handler) -> &mut Self {
      self.add(Some(Method::POST), handler, MatchType::Exact)
    }

    pub fn put(&mut self, handler: impl Handler) -> &mut Self {
      self.add(Some(Method::PUT), handler, MatchType::Exact)
    }

    pub fn patch(&mut self, handler: impl Handler) -> &mut Self {
      self.add(Some(Method::PATCH), handler, MatchType::Exact)
    }

    pub fn delete(&mut self, handler: impl Handler) -> &mut Self {
      self.add(Some(Method::DELETE), handler, MatchType::Exact)
    }

    pub fn options(&mut self, handler: impl Handler) -> &mut Self {
      self.add(Some(Method::OPTIONS), handler, MatchType::Exact)
    }

    pub fn trace(&mut self, handler: impl Handler) -> &mut Self {
      self.add(Some(Method::TRACE), handler, MatchType::Exact)
    }

    pub fn connect(&mut self, handler: impl Handler) -> &mut Self {
      self.add(Some(Method::CONNECT), handler, MatchType::Exact)
    }

    pub fn method(&mut self, method: Method, handler: impl Handler) -> &mut Self {
      self.add(Some(method), handler, MatchType::Exact)
    }
  }

  pub struct Builder {
    entries: Vec<entry::Entry>,
  }

  impl Builder {
    fn add_endpoint(
      router: &mut super::Router,
      mountpoint: &str,
      entry: entry::Endpoint,
    ) -> Result<(), RouterBuilderError> {
      let pattern = Matcher::compile_pattern(
        path::join(mountpoint, entry.path.unwrap_or_default().as_str()).as_str(),
        entry.match_type,
      )?;

      let matcher = Matcher {
        pattern: Some(pattern),
        method: entry.method,
        match_type: entry.match_type,
      };

      let endpoint = crate::endpoint::Endpoint {
        matcher,
        handler: entry.handler,
      };

      let endpoints = Arc::get_mut(&mut router.endpoints).unwrap();

      endpoints.push(endpoint);

      Ok(())
    }

    fn add_builder(
      router: &mut super::Router,
      mountpoint: &str,
      builder: Builder,
    ) -> Result<(), RouterBuilderError> {
      for entry in builder.entries.into_iter() {
        match entry {
          entry::Entry::Endpoint(entry) => {
            Self::add_endpoint(router, mountpoint, entry)?;
          }
          entry::Entry::Builder(entry) => {
            Self::add_builder(
              router,
              path::join(mountpoint, entry.path.unwrap_or_default().as_str()).as_str(),
              entry.builder,
            )?;
          }
        };
      }

      Ok(())
    }

    pub fn new() -> Self {
      Self { entries: vec![] }
    }

    pub fn build(self) -> Result<super::Router, RouterBuilderError> {
      let mut router = super::Router::new();
      Self::add_builder(&mut router, "/", self)?;
      Ok(router)
    }

    pub fn mount(&mut self, builder: Builder) -> &mut Self {
      let inner_entry = entry::Builder {
        path: None,
        builder,
      };

      let entry = entry::Entry::Builder(inner_entry);

      self.entries.push(entry);

      self
    }

    pub fn at(&'_ mut self, path: impl ToString) -> BuilderAt<'_> {
      BuilderAt {
        path: path.to_string(),
        builder: self,
      }
    }

    fn add(
      &mut self,
      method: Option<Method>,
      path: Option<impl ToString>,
      handler: impl Handler,
      match_type: MatchType,
    ) -> &mut Self {
      let inner_entry = entry::Endpoint {
        method,
        path: path.map(|s| s.to_string()),
        handler: Box::new(handler),
        match_type,
      };

      let entry = entry::Entry::Endpoint(inner_entry);

      self.entries.push(entry);

      self
    }

    pub fn with(&mut self, handler: impl Handler) -> &mut Self {
      self.add(None, Option::<String>::None, handler, MatchType::Scope)
    }

    pub fn any(&mut self, path: impl ToString, handler: impl Handler) -> &mut Self {
      self.add(None, Some(path), handler, MatchType::Exact)
    }

    pub fn get(&mut self, path: impl ToString, handler: impl Handler) -> &mut Self {
      self.add(Some(Method::GET), Some(path), handler, MatchType::Exact)
    }

    pub fn post(&mut self, path: impl ToString, handler: impl Handler) -> &mut Self {
      self.add(Some(Method::POST), Some(path), handler, MatchType::Exact)
    }

    pub fn put(&mut self, path: impl ToString, handler: impl Handler) -> &mut Self {
      self.add(Some(Method::PUT), Some(path), handler, MatchType::Exact)
    }

    pub fn patch(&mut self, path: impl ToString, handler: impl Handler) -> &mut Self {
      self.add(Some(Method::PATCH), Some(path), handler, MatchType::Exact)
    }

    pub fn delete(&mut self, path: impl ToString, handler: impl Handler) -> &mut Self {
      self.add(Some(Method::DELETE), Some(path), handler, MatchType::Exact)
    }

    pub fn trace(&mut self, path: impl ToString, handler: impl Handler) -> &mut Self {
      self.add(Some(Method::TRACE), Some(path), handler, MatchType::Exact)
    }

    pub fn options(&mut self, path: impl ToString, handler: impl Handler) -> &mut Self {
      self.add(Some(Method::OPTIONS), Some(path), handler, MatchType::Exact)
    }
  }
}

use core::task::Context;
use core::task::Poll;
use futures::future::Future;
use hyper::server::conn::AddrStream;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::pin::Pin;
use tower::Service;

pub struct RouterService {
  router: Router,
  remote_addr: SocketAddr,
  local_addr: SocketAddr,
}

impl Service<hyper::Request<Body>> for RouterService {
  type Response = hyper::Response<Body>;
  type Error = Infallible;
  type Future =
    Pin<Box<dyn Future<Output = Result<hyper::Response<Body>, Infallible>> + Send + 'static>>;

  fn poll_ready<'a>(&mut self, _cx: &mut Context<'a>) -> Poll<Result<(), Self::Error>> {
    Poll::Ready(Ok(()))
  }

  fn call(&mut self, req: hyper::Request<Body>) -> Self::Future {
    let router = self.router.clone();

    let remote_addr = self.remote_addr;
    let local_addr = self.local_addr;

    let fut = async move {
      let (parts, body) = req.into_parts();

      let request = Request::from_parts(RequestParts {
        local_addr,
        remote_addr,
        method: parts.method,
        uri: parts.uri,
        headers: parts.headers,
        extensions: parts.extensions,
        version: parts.version,
        body,
        params: Params::new(),
      });

      let response = router.handle(request).await;

      let ResponseParts {
        status,
        version,
        headers,
        extensions,
        body,
      } = response.into_parts();

      let mut response = hyper::Response::new(body);
      *response.status_mut() = status;
      *response.version_mut() = version;
      *response.headers_mut() = headers;
      *response.extensions_mut() = extensions;

      Ok::<_, Infallible>(response)
    };

    Box::pin(fut)
  }
}

impl Service<&AddrStream> for Router {
  type Response = RouterService;

  type Error = Infallible;

  type Future = Pin<Box<dyn Future<Output = Result<RouterService, Infallible>> + Send + 'static>>;

  fn poll_ready<'a>(&mut self, _cx: &mut Context<'a>) -> Poll<Result<(), Self::Error>> {
    Poll::Ready(Ok(()))
  }

  fn call(&mut self, socket: &AddrStream) -> Self::Future {
    let router = self.clone();

    let service = RouterService {
      router,
      remote_addr: socket.remote_addr(),
      local_addr: socket.local_addr(),
    };

    let fut = async move { Ok::<_, Infallible>(service) };
    Box::pin(fut)
  }
}
