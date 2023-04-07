use crate::params::Params;
use bytes::Bytes;
use hyper::body::HttpBody;
use hyper::header::AUTHORIZATION;
use hyper::http::Extensions;
use hyper::{self, HeaderMap, Uri, Version};
use hyper::{Body, Method};
use serde::de::DeserializeOwned;
use std::net::{IpAddr, SocketAddr};

fn is_trusted_ip(ip: IpAddr) -> bool {
  ip_rfc::global(&ip)
}

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

#[derive(Debug, thiserror::Error)]
pub enum ReadBodyJsonError {
  #[error("payload too large: (max: {0})")]
  TooLarge(usize),
  #[error("hyper error: {0}")]
  Hyper(#[from] hyper::Error),
  #[error("json deserialize: {0}")]
  Json(#[from] serde_json::Error),
  #[error("payload invalid: {0}")]
  PayloadInvalid(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ReadBodyBytesError {
  #[error("payload too large: (max: {0})")]
  TooLarge(usize),
  #[error("hyper error: {0}")]
  Hyper(#[from] hyper::Error),
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
    let mut ip = self.remote_addr().ip();

    if is_trusted_ip(ip) {
      // nginx forwarded ip
      if let Some(v) = self.headers().get("x-real-ip") {
        if let Ok(v) = v.to_str() {
          if let Ok(client_ip) = v.parse() {
            ip = client_ip;
          }
        }
      }
    }

    if is_trusted_ip(ip) {
      // client forwarded ip
      if let Some(v) = self.headers.get("x-openstream-forwarded-ip") {
        if let Ok(v) = v.to_str() {
          if let Ok(forward_ip) = v.parse() {
            ip = forward_ip;
          }
        }
      }
    }

    ip
  }

  pub fn basic_auth(&self) -> Option<BasicAuth> {
    let header = self.headers().get(AUTHORIZATION)?.to_str().ok()?;
    let creds = http_auth_basic::Credentials::from_header(header.to_string()).ok()?;
    Some(BasicAuth {
      user: creds.user_id,
      password: creds.password,
    })
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

  pub async fn read_body_bytes(&mut self, maxlen: usize) -> Result<Bytes, ReadBodyBytesError> {
    let mut buf = vec![];
    loop {
      let data = self.body_mut().data().await;
      match data {
        None => break,
        Some(r) => {
          let bytes = r?;
          if (bytes.len() + buf.len()) > maxlen {
            return Err(ReadBodyBytesError::TooLarge(maxlen));
          }
          buf.extend_from_slice(bytes.as_ref());
        }
      }
    }

    Ok(Bytes::from(buf))
  }
}

impl Request {
  #[inline]
  pub fn param(&self, key: &str) -> Option<&str> {
    self.params.get(key)
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BasicAuth {
  pub user: String,
  pub password: String,
}
