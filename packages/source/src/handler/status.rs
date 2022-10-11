use crate::error::HandlerError;
use crate::http::{write_response_head, RequestHead, ResponseHead};
use crate::{headers, text_plain};
use debug_print::debug_println;
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::http::HeaderValue;
use hyper::{Method, StatusCode, Version};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn status(mut socket: TcpStream, req: RequestHead) -> Result<(), HandlerError> {
  debug_println!("status: {} {}", req.method, req.uri);

  let status = StatusCode::OK;
  let body = b"200 OK";
  let mut headers = headers!(2);
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

  if req.method != &Method::HEAD {
    debug_println!("writing body to socket");
    socket.write_all(body).await?;
  }

  debug_println!("shutting down socket");
  socket.flush().await?;

  Ok(())
}
