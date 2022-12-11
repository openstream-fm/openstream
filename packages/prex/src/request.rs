use crate::params::Params;
use hyper::body::HttpBody;
use hyper::http::Extensions;
use hyper::{self, HeaderMap, Uri, Version};
use hyper::{Body, Method};
use serde::de::DeserializeOwned;
use std::net::{IpAddr, SocketAddr};

#[derive(Debug)]
pub struct Parts {
  pub local_addr: SocketAddr,
  pub remote_addr: SocketAddr,
  pub method: Method,
  pub uri: Uri,
  pub version: Version,
  pub headers: HeaderMap,
  pub extensions: Extensions,
  pub params: Params,
  pub body: Body,
}

#[derive(Debug)]
pub struct Request {
  pub(crate) local_addr: SocketAddr,
  pub(crate) remote_addr: SocketAddr,
  pub(crate) method: Method,
  pub(crate) uri: Uri,
  pub(crate) version: Version,
  pub(crate) headers: HeaderMap,
  pub(crate) extensions: Extensions,
  pub(crate) params: Params,
  pub(crate) body: Body,
}

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

impl Request {
  #[inline]
  pub fn from_parts(parts: Parts) -> Self {
    Self {
      local_addr: parts.local_addr,
      remote_addr: parts.remote_addr,
      method: parts.method,
      uri: parts.uri,
      headers: parts.headers,
      extensions: parts.extensions,
      version: parts.version,
      params: parts.params,
      body: parts.body,
    }
  }

  #[inline]
  pub fn into_parts(self) -> Parts {
    Parts {
      local_addr: self.local_addr,
      remote_addr: self.remote_addr,
      method: self.method,
      uri: self.uri,
      headers: self.headers,
      extensions: self.extensions,
      version: self.version,
      params: self.params,
      body: self.body,
    }
  }

  #[inline]
  pub fn local_addr(&self) -> SocketAddr {
    self.local_addr
  }

  #[inline]
  pub fn local_addr_mut(&mut self) -> &mut SocketAddr {
    &mut self.local_addr
  }

  #[inline]
  pub fn remote_addr(&self) -> SocketAddr {
    self.remote_addr
  }

  #[inline]
  pub fn remote_addr_mut(&mut self) -> &mut SocketAddr {
    &mut self.remote_addr
  }

  #[inline]
  pub fn method(&self) -> &Method {
    &self.method
  }

  #[inline]
  pub fn method_mut(&mut self) -> &mut Method {
    &mut self.method
  }

  #[inline]
  pub fn uri(&self) -> &Uri {
    &self.uri
  }

  #[inline]
  pub fn uri_mut(&mut self) -> &mut Uri {
    &mut self.uri
  }

  #[inline]
  pub fn headers(&self) -> &HeaderMap {
    &self.headers
  }

  #[inline]
  pub fn headers_mut(&mut self) -> &mut HeaderMap {
    &mut self.headers
  }

  #[inline]
  pub fn extensions(&self) -> &Extensions {
    &self.extensions
  }

  #[inline]
  pub fn extensions_mut(&mut self) -> &mut Extensions {
    &mut self.extensions
  }

  #[inline]
  pub fn version(&self) -> Version {
    self.version
  }

  #[inline]
  pub fn version_mut(&mut self) -> &mut Version {
    &mut self.version
  }

  #[inline]
  pub fn params(&self) -> &Params {
    &self.params
  }

  #[inline]
  pub fn params_mut(&mut self) -> &mut Params {
    &mut self.params
  }

  #[inline]
  pub fn body(&self) -> &Body {
    &self.body
  }

  #[inline]
  pub fn body_mut(&mut self) -> &mut Body {
    &mut self.body
  }

  /// takes the body of this request replacing it with Body::empty
  #[inline]
  pub fn take_body(&mut self) -> Body {
    let mut body = Body::empty();
    std::mem::swap(self.body_mut(), &mut body);
    body
  }

  #[inline]
  pub fn into_body(self) -> Body {
    self.body
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
