use crate::error::HandlerError;
use crate::http::{write_response_head, RequestHead, ResponseHead};
use crate::{content_length, headers, text_plain};
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::{Method, StatusCode, Version};
use log::*;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn not_found(mut socket: TcpStream, req: RequestHead) -> Result<(), HandlerError> {
  trace!("not_found: {} {}", req.method, req.uri);

  let status = StatusCode::METHOD_NOT_ALLOWED;
  let body = b"404 Not Found";

  let mut headers = headers!(2);
  headers.append(CONTENT_TYPE, text_plain!());
  headers.append(CONTENT_LENGTH, content_length!(body));

  let head = ResponseHead {
    version: Version::HTTP_10,
    status,
    headers,
  };

  write_response_head(&mut socket, head, true).await?;

  trace!("writing body to socket");
  if req.method != &Method::HEAD {
    socket.write_all(body).await?;
    trace!("shutting down socket");
  }

  socket.flush().await?;

  Ok(())
}
