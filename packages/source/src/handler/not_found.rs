use crate::error::HandlerError;
use crate::{headers, text_plain, content_length};
use crate::http::{RequestHead, ResponseHead, write_response_head};
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::{StatusCode, Version, Method};
use tokio::net::TcpStream;
use debug_print::debug_println;
use tokio::io::AsyncWriteExt;


pub async fn not_found(mut socket: TcpStream, req: RequestHead) -> Result<(), HandlerError> {
    
  debug_println!("not_found: {} {}", req.method, req.uri);
  
  let status = StatusCode::METHOD_NOT_ALLOWED;
  let body = b"404 Not Found";

  let mut headers = headers!(2);
  headers.append(CONTENT_TYPE, text_plain!());
  headers.append(CONTENT_LENGTH, content_length!(body));

  let head = ResponseHead { version: Version::HTTP_10, status, headers };
  
  write_response_head(&mut socket, head, true).await?;
  
  debug_println!("writing body to socket");
  if req.method != &Method::HEAD {
    socket.write_all(body).await?;
    debug_println!("shutting down socket");
  }
  
  socket.flush().await?;

  Ok(())
}