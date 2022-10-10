use debug_print::debug_println;
use hyper::{http::HeaderValue, StatusCode, header::{ALLOW, CONTENT_LENGTH, CONTENT_TYPE}, Version, Method};
use tokio::{net::TcpStream, io::AsyncWriteExt};

use crate::{http::{RequestHead, ResponseHead, write_response_head}, error::HandlerError, headers, text_plain};

pub async fn method_not_allowed(mut socket: TcpStream, req: RequestHead, allow: HeaderValue) -> Result<(), HandlerError> {
  
  debug_println!("method_not_allowed: {} {}", req.method, req.uri);
  
  let status = StatusCode::METHOD_NOT_ALLOWED;
  let body = b"405 Method Not Allowed";
  let mut headers = headers!(3);
  headers.append(ALLOW, allow);
  headers.append(CONTENT_LENGTH, HeaderValue::from_str(body.len().to_string().as_str()).unwrap());
  headers.append(CONTENT_TYPE, text_plain!());

  let head = ResponseHead { version: Version::HTTP_10, status, headers };
  
  write_response_head(&mut socket, head, true).await?;
  
  if req.method != &Method::HEAD {
    debug_println!("writing body to socket");
    socket.write_all(body).await?;
  }
  debug_println!("shutting down socket");
  socket.flush().await?;

  Ok(())
}