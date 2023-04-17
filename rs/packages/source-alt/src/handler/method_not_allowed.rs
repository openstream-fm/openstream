use hyper::{
  header::{ALLOW, CONTENT_LENGTH, CONTENT_TYPE},
  http::HeaderValue,
  Method, StatusCode, Version,
};
use log::*;
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::{
  error::HandlerError,
  headers,
  http::{write_response_head, RequestHead, ResponseHead},
  text_plain,
};

pub async fn method_not_allowed(
  mut socket: TcpStream,
  req: RequestHead,
  allow: HeaderValue,
) -> Result<(), HandlerError> {
  trace!("method_not_allowed: {} {}", req.method, req.uri);

  let status = StatusCode::METHOD_NOT_ALLOWED;
  let body = b"405 Method Not Allowed";
  let mut headers = headers!(3);
  headers.append(ALLOW, allow);
  headers.append(
    CONTENT_LENGTH,
    HeaderValue::from_str(body.len().to_string().as_str()).unwrap(),
  );
  headers.append(CONTENT_TYPE, text_plain!());

  let head = ResponseHead {
    version: Version::HTTP_10,
    status,
    headers,
  };

  write_response_head(&mut socket, head, true).await?;

  if req.method != Method::HEAD {
    trace!("writing body to socket");
    socket.write_all(body).await?;
  }
  trace!("shutting down socket");
  socket.flush().await?;

  Ok(())
}
