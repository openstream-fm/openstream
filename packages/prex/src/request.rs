use crate::params::Params;
use hyper;
use hyper::body::HttpBody;
use hyper::Body;
use serde::de::DeserializeOwned;
use std::net::{IpAddr, SocketAddr};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub enum ReadBodyJsonError {
  TooLarge(usize),
  Hyper(hyper::Error),
  Json(serde_json::Error),
  PayloadInvalid(String),
}

impl From<hyper::Error> for ReadBodyJsonError {
  fn from(e: hyper::Error) -> Self {
    Self::Hyper(e)
  }
}

impl From<serde_json::Error> for ReadBodyJsonError {
  fn from(e: serde_json::Error) -> Self {
    Self::Json(e)
  }
}

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

  /// takes the body of this request replacing it with Body::empty
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

  pub fn isomorphic_ip(&self) -> IpAddr {
    match self.headers().get("x-client-ip") {
      None => self.remote_addr().ip(),
      Some(ip) => match ip.to_str() {
        Err(_e) => self.remote_addr().ip(),
        Ok(ip) => match ip.parse() {
          Err(_e) => self.remote_addr().ip(),
          Ok(ip) => ip,
        },
      },
    }
  }

  pub async fn read_body_json<T: DeserializeOwned>(
    &mut self,
    maxlen: usize,
  ) -> Result<T, ReadBodyJsonError> {
    let mut buf = vec![];
    loop {
      let data = self.body_mut().data().await;
      match data {
        None => break,
        Some(r) => {
          let bytes = r?;
          if (bytes.len() + buf.len()) > maxlen {
            return Err(ReadBodyJsonError::TooLarge(maxlen));
          }
          buf.extend_from_slice(bytes.as_ref());
        }
      }
    }

    let value: T = serde_json::from_slice(&buf)?;

    Ok(value)
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
