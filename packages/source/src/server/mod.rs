pub mod error;

mod server;
pub use server::*;

use std::str::FromStr;
use error::*;
use tokio::io::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt};
use hyper::{Version, HeaderMap, Method, header::HeaderName, http::HeaderValue};

use debug_print::*;

pub const MAX_REQUEST_HEAD_SIZE: usize = 10 * 1024;
pub const MAX_RESPONSE_HEAD_SIZE: usize = 10 * 1024;

pub struct RequestHead {
  pub version: Version,
  pub method: Method,
  pub uri: String,
  pub headers: HeaderMap,
}

pub struct ResponseHead {
  pub version: hyper::Version,
  pub status: hyper::StatusCode,
  pub headers: HeaderMap,
}


pub async fn read_request_head<R: AsyncRead + Unpin>(reader: &mut R) -> Result<RequestHead, ReadHeadError> {

  debug_println!("read_request_head");

  let mut buf = [0u8; MAX_REQUEST_HEAD_SIZE];
  let mut i = 0usize;
  
  let slice = loop {
    if i >= MAX_REQUEST_HEAD_SIZE {
      return Err(ReadHeadError::SizeExceeded);
    }

    let byte = reader.read_u8().await?;
    
    if byte == b'\n' && i >= 3 && &buf[(i - 3)..i] == b"\r\n\r" {
      break &buf[0..i - 3];
    }

    buf[i] = byte;
    i += 1;
  };

  debug_println!("head size => {i} bytes");

  parse_request_head(slice).await
}

pub async fn parse_request_head(buf: &[u8]) -> Result<RequestHead, ReadHeadError> {
  
  debug_println!("parse_request_head");

  let string = String::from_utf8_lossy(buf);

  let mut lines = string.split_terminator("\r\n");

  let (method, uri, version) = match lines.next() {
    None => return Err(ReadHeadError::NoHeadLine),
    Some(line) => parse_head_line(line)?
  };

  debug_println!("leading => {method} {uri} {version:?}");

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

        debug_println!("header => {name}: {}", value.to_str().unwrap());
        headers.append(name, value);
      }
    }
  }

  let head = RequestHead {
    version,
    method,
    uri: String::from(uri),
    headers,
  };

  Ok(head)

}

pub fn parse_head_line(line: &str) -> Result<(hyper::Method, &str, hyper::Version), ReadHeadError> {

  let mut parts = line.split_ascii_whitespace();
  
  let method = match parts.next() {
    None => return Err(ReadHeadError::NoMethod),
    Some(method) => match Method::from_bytes(method.trim().as_bytes()) {
      Err(_) => return Err(ReadHeadError::InvalidMethod),
      Ok(method) => method,
    }
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
      _ => return Err(ReadHeadError::InvalidVersion)
    }
  };

  if version == Version::HTTP_09 && method != Method::GET {
    return Err(ReadHeadError::VersionMethodMismatch);
  }

  Ok((method, uri, version))

}

pub async fn write_response_head<W: AsyncWrite + Unpin>(writer: &mut W, head: ResponseHead, add_trailing_newline: bool) -> Result<(), WriteHeadError> {
  
  debug_println!("serializing head");

  let mut buf = [0u8; MAX_RESPONSE_HEAD_SIZE];
  let mut len = 0usize;

  macro_rules! write {
    ($expr:expr) => {
      {
        let item: &[u8] = $expr;
        if len + item.len() > MAX_RESPONSE_HEAD_SIZE {
          return Err(WriteHeadError::SizeExceeded)
        }
        
        for byte in item {
          buf[len] = *byte;
          len += 1;
        }
      }
    }
  }

  match head.version {
    Version::HTTP_10 => write!(b"HTTP/1.0"),
    _ => return Err(WriteHeadError::UnsupportedVersion)
  };

  write!(b" ");

  write!(head.status.as_str().as_bytes());
  write!(b" "); 

  match head.status.canonical_reason() {
    Some(reason) => write!(reason.as_bytes()),
    None => write!(b"Unknown"),
  };
  
  write!(b"\r\n");

  for (name, value) in head.headers.iter() {
    write!(name.as_ref());
    write!(b": ");
    write!(value.as_bytes());
    write!(b"\r\n");
  }

  if add_trailing_newline {
    write!(b"\r\n");
  }

  debug_println!("writing response head to socket: len => {len}");
  writer.write_all(&buf[0..len]).await?;
  
  Ok(())

}

pub async fn write_headers<W: AsyncWrite + Unpin>(writer: &mut W, headers: &HeaderMap) -> Result<(), std::io::Error> {
  for (name, value) in headers.iter() {
    writer.write(name.as_ref()).await?;
    writer.write(b"\r\n").await?;
    writer.write(value.as_bytes()).await?;
  }
  
  Ok(())
}

#[cfg(test)]
mod test {
  use std::str::FromStr;

use super::parse_head_line;
  use hyper::{Method, Version};

  #[test]
  fn head_line_parse() {
    let ok = vec![
      ("GET /", (Method::GET, "/", Version::HTTP_09)),
      ("GET /path", (Method::GET, "/path", Version::HTTP_09)),
      ("GET /path?query=asd", (Method::GET, "/path?query=asd", Version::HTTP_09)),

      ("GET / HTTP/1.0", (Method::GET, "/", Version::HTTP_10)),
      ("PUT /path HTTP/1.1", (Method::PUT, "/path", Version::HTTP_11)),
      ("SOURCE /path?query=asd HTTP/1.0", (Method::from_str("SOURCE").unwrap(), "/path?query=asd", Version::HTTP_10)),

      ("GET / HTTP/1.0", (Method::GET, "/", Version::HTTP_10)),
      ("POST / HTTP/1.0", (Method::POST, "/", Version::HTTP_10)),
      ("PUT / HTTP/1.0", (Method::PUT, "/", Version::HTTP_10)),
      ("PATCH / HTTP/1.0", (Method::PATCH, "/", Version::HTTP_10)),
      ("DELETE / HTTP/1.0", (Method::DELETE, "/", Version::HTTP_10)),
      ("OPTIONS / HTTP/1.0", (Method::OPTIONS, "/", Version::HTTP_10)),
      ("CONNECT / HTTP/1.0", (Method::CONNECT, "/", Version::HTTP_10)),
      ("SOURCE / HTTP/1.0", (Method::from_str("SOURCE").unwrap(), "/", Version::HTTP_10)),

      ("GET / HTTP/1.1", (Method::GET, "/", Version::HTTP_11)),
      ("POST / HTTP/1.1", (Method::POST, "/", Version::HTTP_11)),
      ("PUT / HTTP/1.1", (Method::PUT, "/", Version::HTTP_11)),
      ("PATCH / HTTP/1.1", (Method::PATCH, "/", Version::HTTP_11)),
      ("DELETE / HTTP/1.1", (Method::DELETE, "/", Version::HTTP_11)),
      ("OPTIONS / HTTP/1.1", (Method::OPTIONS, "/", Version::HTTP_11)),
      ("CONNECT / HTTP/1.1", (Method::CONNECT, "/", Version::HTTP_11)),
      ("SOURCE / HTTP/1.1", (Method::from_str("SOURCE").unwrap(), "/", Version::HTTP_11)),
    ];

    for (line, expected) in ok.into_iter() {
      assert_eq!(parse_head_line(line).unwrap(), expected)
    }

  }
}