use std::{fmt::Debug, net::IpAddr, str::FromStr};

use hyper::{
  header::{HeaderName, HeaderValue},
  HeaderMap, Method, Uri, Version,
};
use tokio::io::{AsyncRead, AsyncReadExt};

use self::error::ReadHeadError;

pub mod error;

pub const MAX_REQUEST_HEAD_SIZE: usize = 16 * 1024;
// pub const MAX_RESPONSE_HEAD_SIZE: usize = 8 * 1024;

pub struct RequestHead {
  pub proxy_protocol_ip: Option<IpAddr>,
  pub version: Version,
  pub method: Method,
  pub uri: Uri,
  pub headers: HeaderMap,
}

impl Debug for RequestHead {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("RequestHead")
      //.field("buffer", &String::from_utf8_lossy(&self.buffer))
      .field("proxy-protocol-ip", &self.proxy_protocol_ip)
      .field("version", &self.version)
      .field("method", &self.method)
      .field("uri", &format!("{}", self.uri))
      .field("headers", &self.headers)
      .finish()
  }
}
#[derive(Debug)]
pub struct ResponseHead {
  pub version: hyper::Version,
  pub status: hyper::StatusCode,
  pub headers: HeaderMap,
}

#[macro_export]
macro_rules! text_plain {
  () => {
    ::hyper::header::HeaderValue::from_static("text/plain;charset=utf-8")
  };
}

#[macro_export]
macro_rules! content_length {
  ($tt:expr) => {{
    let body: &[u8] = $tt.as_ref();
    ::hyper::header::HeaderValue::from_str(body.len().to_string().as_str()).unwrap()
  }};
}

pub async fn read_request_head<R: AsyncRead + Unpin>(
  reader: &mut R,
) -> Result<RequestHead, ReadHeadError> {
  let mut buf = [0u8; MAX_REQUEST_HEAD_SIZE];
  let mut i = 0usize;

  let slice = loop {
    if i >= MAX_REQUEST_HEAD_SIZE {
      return Err(ReadHeadError::SizeExceeded);
    }

    let byte = reader.read_u8().await?;

    if byte == b'\n'
      && ((i >= 3 && &buf[(i - 3)..i] == b"\r\n\r") || (i >= 1 && buf[i - 1] == b'\n'))
    {
      buf[i] = byte;
      break &buf[0..=i];
    }

    buf[i] = byte;
    i += 1;
  };

  parse_request_head(Vec::from(slice)).await
}

pub async fn parse_request_head(buffer: Vec<u8>) -> Result<RequestHead, ReadHeadError> {
  let string = String::from_utf8_lossy(buffer.as_ref());

  let mut lines = string.split_terminator('\n');

  let mut line = match lines.next() {
    None => return Err(ReadHeadError::NoHeadLine),
    Some(line) => line,
  };

  let mut proxy_protocol_ip = None;

  match proxy_protocol::v1::parse_ip_from_proxy_line(line) {
    None => {}
    Some(addr) => {
      proxy_protocol_ip = Some(addr);
      line = match lines.next() {
        Some(line) => line,
        None => {
          return Err(ReadHeadError::NoHeadLine);
        }
      }
    }
  }

  let (method, uri, version) = parse_head_line(line)?;

  let mut headers = HeaderMap::new();
  for line in lines {
    match line.split_once(':') {
      None => continue,
      Some((name, value)) => {
        let name = match HeaderName::from_str(name.trim()) {
          Err(_) => continue,
          Ok(name) => name,
        };

        let value = match HeaderValue::from_str(value.trim()) {
          Err(_) => continue,
          Ok(value) => value,
        };

        headers.append(name, value);
      }
    }
  }

  let uri = if uri.starts_with('/') {
    hyper::Uri::from_str(uri)?
  } else {
    hyper::Uri::from_str(&format!("/{uri}"))?
  };

  let head = RequestHead {
    proxy_protocol_ip,
    version,
    method,
    uri,
    headers,
  };

  Ok(head)
}

pub fn parse_head_line(line: &str) -> Result<(hyper::Method, &str, hyper::Version), ReadHeadError> {
  let mut parts = line.trim().split_ascii_whitespace();

  let method = match parts.next() {
    None => return Err(ReadHeadError::NoMethod),
    Some(method) => match Method::from_bytes(method.trim().as_bytes()) {
      Err(_) => return Err(ReadHeadError::InvalidMethod),
      Ok(method) => method,
    },
  };

  let uri = match parts.next() {
    None => return Err(ReadHeadError::NoUri),
    Some(s) => s,
  };

  let version = match parts.next() {
    None => Version::HTTP_09,
    Some(ver) => match ver.trim() {
      "" => Version::HTTP_09,
      "HTTP/1.0" => Version::HTTP_10,
      "HTTP/1.1" => Version::HTTP_11,
      ice if ice.starts_with("ICE/") || ice.starts_with("ICY/") => Version::HTTP_10,
      ver => return Err(ReadHeadError::InvalidVersion(ver.to_string())),
    },
  };

  if version == Version::HTTP_09 && method != Method::GET {
    return Err(ReadHeadError::VersionMethodMismatch);
  }

  Ok((method, uri, version))
}
