use std::fmt::Display;

use crate::http::error::{ReadHeadError, WriteHeadError};

#[derive(Debug)]
pub enum HandlerError {
  Io(std::io::Error),
  ReadHead(ReadHeadError),
  WriteHead(WriteHeadError),
}

impl Display for HandlerError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Io(e) => write!(f, "{}", e),
      Self::ReadHead(e) => write!(f, "{}", e),
      Self::WriteHead(e) => write!(f, "{}", e),
    }
  }
}

impl From<std::io::Error> for HandlerError {
  fn from(inner: std::io::Error) -> Self {
    Self::Io(inner)
  }
}

impl From<ReadHeadError> for HandlerError {
  fn from(inner: ReadHeadError) -> Self {
    Self::ReadHead(inner)
  }
}

impl From<WriteHeadError> for HandlerError {
  fn from(inner: WriteHeadError) -> Self {
    Self::WriteHead(inner)
  }
}

impl std::error::Error for HandlerError {
  fn cause(&self) -> Option<&dyn std::error::Error> {
    match &self {
      Self::Io(e) => Some(e),
      Self::ReadHead(e) => Some(e),
      Self::WriteHead(e) => Some(e),
    }
  }
}
