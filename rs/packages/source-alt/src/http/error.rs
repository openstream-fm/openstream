use std::fmt::Display;

use hyper::http::uri::InvalidUri;

#[derive(Debug)]
pub enum ReadHeadError {
  Io(std::io::Error),
  Hyper(hyper::Error),
  InvalidUri(InvalidUri),
  SizeExceeded,
  NoHeadLine,
  NoMethod,
  InvalidMethod,
  NoUri,
  NoVersion,
  InvalidVersion(String),
  VersionMethodMismatch,
}

impl Display for ReadHeadError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Io(inner) => inner.fmt(f),
      Self::Hyper(inner) => inner.fmt(f),
      Self::InvalidUri(_) => write!(f, "invalid uri"),
      Self::SizeExceeded => write!(f, "request head size exceeded"),
      Self::NoHeadLine => write!(f, "request head doesn't have a head line"),
      Self::NoMethod => write!(f, "request method not found"),
      Self::InvalidMethod => write!(f, "request method is invalid"),
      Self::NoUri => write!(f, "request uri not found"),
      Self::NoVersion => write!(f, "request version not found"),
      Self::InvalidVersion(ver) => write!(f, "request version is invalid, version = '{ver}'"),
      Self::VersionMethodMismatch => write!(
        f,
        "request version and method mismatch, HTTP/0.9 only allows GET requests"
      ),
    }
  }
}

impl std::error::Error for ReadHeadError {
  fn cause(&self) -> Option<&dyn std::error::Error> {
    match self {
      Self::Io(inner) => Some(inner),
      Self::Hyper(inner) => Some(inner),
      _ => None,
    }
  }
}

impl From<std::io::Error> for ReadHeadError {
  fn from(inner: std::io::Error) -> Self {
    Self::Io(inner)
  }
}

impl From<InvalidUri> for ReadHeadError {
  fn from(inner: InvalidUri) -> Self {
    Self::InvalidUri(inner)
  }
}

impl From<hyper::Error> for ReadHeadError {
  fn from(inner: hyper::Error) -> Self {
    Self::Hyper(inner)
  }
}

#[derive(Debug)]
pub enum WriteHeadError {
  Io(std::io::Error),
  SizeExceeded,
  UnsupportedVersion,
}

impl Display for WriteHeadError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Io(inner) => inner.fmt(f),
      Self::SizeExceeded => write!(f, "Response head size excedded"),
      Self::UnsupportedVersion => write!(f, "Response write, unsopported (non 1.0) version"),
    }
  }
}

impl std::error::Error for WriteHeadError {
  fn cause(&self) -> Option<&dyn std::error::Error> {
    match self {
      Self::Io(inner) => Some(inner),
      _ => None,
    }
  }
}

impl From<std::io::Error> for WriteHeadError {
  fn from(inner: std::io::Error) -> Self {
    Self::Io(inner)
  }
}
