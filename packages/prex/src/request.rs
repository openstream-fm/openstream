use crate::params::Params;
use hyper;
use hyper::Body;
use std::net::SocketAddr;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Parts {
  pub remote_addr: SocketAddr,
  pub request: hyper::Request<hyper::Body>,
  pub params: Params,
}

#[derive(Debug)]
pub struct Request {
  pub(crate) request: hyper::Request<hyper::Body>,
  pub(crate) params: Params,
  pub(crate) remote_addr: SocketAddr,
}

impl Request {
  #[inline]
  pub fn from_parts(parts: Parts) -> Self {
    Self {
      remote_addr: parts.remote_addr,
      request: parts.request,
      params: parts.params,
    }
  }

  #[inline]
  pub fn into_parts(self) -> Parts {
    Parts {
      remote_addr: self.remote_addr,
      request: self.request,
      params: self.params,
    }
  }

  /// consumes this request returning only the body
  #[inline]
  pub fn into_body(self) -> Body {
    self.request.into_body()
  }

  /// takes the body if this request replacing it with Body::empty
  #[inline]
  pub fn take_body(&mut self) -> Body {
    let mut body = Body::empty();
    std::mem::swap(self.body_mut(), &mut body);
    body
  }

  #[inline]
  pub fn remote_addr(&self) -> SocketAddr {
    self.remote_addr
  }

  #[inline]
  pub fn remote_addr_mut(&mut self) -> &mut SocketAddr {
    &mut self.remote_addr
  }
}

impl Request {
  #[inline]
  pub fn param(&self, key: &str) -> Option<&str> {
    self.params.get(key)
  }
}

impl Deref for Request {
  type Target = hyper::Request<hyper::Body>;
  fn deref(&self) -> &Self::Target {
    &self.request
  }
}

impl DerefMut for Request {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.request
  }
}
