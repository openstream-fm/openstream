use crate::params::Params;
use bytes::Bytes;
use hyper::body::HttpBody;
use hyper::header::{HeaderName, AUTHORIZATION, HOST};
use hyper::http::Extensions;
use hyper::{self, HeaderMap, Uri, Version};
use hyper::{Body, Method};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::net::{IpAddr, SocketAddr};

#[allow(clippy::declare_interior_mutable_const)]
const X_OPENSTREAM_FORWARDED_IP: HeaderName = HeaderName::from_static(constants::FORWARD_IP_HEADER);

#[allow(clippy::declare_interior_mutable_const)]
const X_REAL_IP: HeaderName = HeaderName::from_static(constants::REAL_IP_HEADER);

pub fn is_trusted_ip(ip: IpAddr) -> bool {
  !ip_rfc::global(&ip)
}

#[derive(Debug)]
pub struct Parts {
  pub local_addr: SocketAddr,
  pub remote_addr: SocketAddr,
  pub proxy_protocol_ip: Option<IpAddr>,
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
  pub(crate) proxy_protocol_ip: Option<IpAddr>,
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
      proxy_protocol_ip: parts.proxy_protocol_ip,
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
      proxy_protocol_ip: self.proxy_protocol_ip,
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
      if let Some(proxy_ip) = self.proxy_protocol_ip {
        ip = proxy_ip;
      }
    }

    if is_trusted_ip(ip) {
      if let Some(v) = self.headers().get(X_REAL_IP) {
        if let Ok(v) = v.to_str() {
          if let Ok(client_ip) = v.parse() {
            ip = client_ip;
          }
        }
      }
    }

    if is_trusted_ip(ip) {
      if let Some(v) = self.headers.get(X_OPENSTREAM_FORWARDED_IP) {
        if let Ok(v) = v.to_str() {
          if let Ok(forward_ip) = v.parse() {
            ip = forward_ip;
          }
        }
      }
    }

    ip
  }

  #[inline]
  pub fn host(&self) -> Option<&str> {
    self.headers.get(HOST)?.to_str().ok()
  }

  #[inline]
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

  pub fn qs<'de, T: Deserialize<'de>>(&'de self) -> Result<T, serde_qs::Error> {
    let qs = self.uri().query().unwrap_or("");
    // serde_qs::from_str(qs)
    serde_qs::Config::new(10, false).deserialize_str(qs)
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

#[cfg(test)]
mod tests {
  use super::*;
  use std::str::FromStr;

  fn mock_parts() -> Parts {
    Parts {
      local_addr: SocketAddr::from_str("127.0.0.1:8080").unwrap(),
      remote_addr: SocketAddr::from_str("127.0.0.1:12345").unwrap(),
      proxy_protocol_ip: None,
      method: Method::GET,
      uri: Uri::from_static("http://localhost"),
      version: Version::HTTP_11,
      headers: HeaderMap::new(),
      extensions: Extensions::new(),
      params: Params::new(),
      body: Body::empty(),
    }
  }

  #[test]
  fn is_trusted_ip() {
    assert!(super::is_trusted_ip(IpAddr::from_str("127.0.0.1").unwrap()));
    assert!(!super::is_trusted_ip(IpAddr::from_str("8.8.8.8").unwrap()));
  }

  #[test_util::async_test]
  async fn read_body_json() {
    let mut request = Request::from_parts(mock_parts());
    *request.body_mut() = Body::from(r#"{"key": "value"}"#);

    let json: std::collections::HashMap<String, String> =
      request.read_body_json(1024).await.unwrap();
    assert_eq!(json.get("key").unwrap(), "value");
  }

  #[test_util::async_test]
  async fn read_body_bytes() {
    let mut request = Request::from_parts(mock_parts());
    *request.body_mut() = Body::from("test body");

    let bytes: Result<Bytes, _> = request.read_body_bytes(1024).await;
    assert!(bytes.is_ok());
    assert_eq!(bytes.unwrap(), Bytes::from("test body"));
  }

  #[test]
  fn basic_auth() {
    let mut parts = mock_parts();
    parts.headers.insert(
      AUTHORIZATION,
      "Basic dXNlcm5hbWU6cGFzc3dvcmQ=".parse().unwrap(),
    );
    let request = Request::from_parts(parts);

    let auth = request.basic_auth();
    assert!(auth.is_some());
    let auth = auth.unwrap();
    assert_eq!(auth.user, "username");
    assert_eq!(auth.password, "password");
  }

  #[test]
  fn request_getters() {
    let request = Request::from_parts(mock_parts());
    assert_eq!(
      request.local_addr(),
      SocketAddr::from_str("127.0.0.1:8080").unwrap()
    );
    assert_eq!(
      request.remote_addr(),
      SocketAddr::from_str("127.0.0.1:12345").unwrap()
    );
    assert_eq!(request.method(), &Method::GET);
    assert_eq!(request.uri(), &Uri::from_static("http://localhost"));
    assert_eq!(request.version(), Version::HTTP_11);
    assert_eq!(request.headers().len(), 0);
  }

  #[test]
  fn request_setters() {
    let mut request = Request::from_parts(mock_parts());
    *request.local_addr_mut() = SocketAddr::from_str("127.0.0.1:9000").unwrap();
    *request.remote_addr_mut() = SocketAddr::from_str("127.0.0.1:6789").unwrap();
    *request.method_mut() = Method::POST;
    *request.uri_mut() = Uri::from_static("http://example.com");
    *request.version_mut() = Version::HTTP_2;

    assert_eq!(
      request.local_addr(),
      SocketAddr::from_str("127.0.0.1:9000").unwrap()
    );
    assert_eq!(
      request.remote_addr(),
      SocketAddr::from_str("127.0.0.1:6789").unwrap()
    );
    assert_eq!(request.method(), &Method::POST);
    assert_eq!(request.uri(), &Uri::from_static("http://example.com"));
    assert_eq!(request.version(), Version::HTTP_2);
  }

  #[test]
  fn request_param() {
    let mut parts = mock_parts();
    parts.params.set("key".to_string(), "value".to_string());
    let request = Request::from_parts(parts);

    assert_eq!(request.param("key"), Some("value"));
    assert_eq!(request.param("nonexistent"), None);
  }

  #[test]
  fn test_parts_from_request() {
    let request = Request::from_parts(mock_parts());
    let parts = Parts {
      local_addr: request.local_addr,
      remote_addr: request.remote_addr,
      proxy_protocol_ip: None,
      method: request.method,
      uri: request.uri,
      version: request.version,
      headers: request.headers,
      extensions: request.extensions,
      params: request.params,
      body: request.body,
    };

    assert_eq!(
      parts.local_addr,
      SocketAddr::from_str("127.0.0.1:8080").unwrap()
    );
    assert_eq!(
      parts.remote_addr,
      SocketAddr::from_str("127.0.0.1:12345").unwrap()
    );
    assert_eq!(parts.method, Method::GET);
    assert_eq!(parts.uri, Uri::from_static("http://localhost"));
    assert_eq!(parts.version, Version::HTTP_11);
    assert_eq!(parts.headers.len(), 0);
  }

  #[test_util::async_test]
  async fn test_read_body_json_error_too_large() {
    let mut request = Request::from_parts(mock_parts());
    *request.body_mut() = Body::from(r#"{"key": "value"}"#);

    let json: Result<std::collections::HashMap<String, String>, _> =
      request.read_body_json(5).await;
    assert!(matches!(json.unwrap_err(), ReadBodyJsonError::TooLarge(5)));
  }

  #[test_util::async_test]
  async fn test_read_body_bytes_error_too_large() {
    let mut request = Request::from_parts(mock_parts());
    *request.body_mut() = Body::from("test body");

    let bytes: Result<Bytes, _> = request.read_body_bytes(5).await;
    assert!(matches!(
      bytes.unwrap_err(),
      ReadBodyBytesError::TooLarge(5)
    ));
  }

  #[test_util::async_test]
  async fn test_read_body_json_error_malformed_json() {
    let mut request = Request::from_parts(mock_parts());
    *request.body_mut() = Body::from(r#"{"key": "value",}"#); // Note the extra comma

    let json: Result<std::collections::HashMap<String, String>, _> =
      request.read_body_json(1024).await;
    assert!(matches!(json.unwrap_err(), ReadBodyJsonError::Json(_)));
  }
}
