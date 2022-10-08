use std::net::SocketAddr;

use crate::server::*;
use hyper::{StatusCode, header::{ALLOW, CONTENT_LENGTH, CONTENT_TYPE}};
use tokio::net::{TcpListener, TcpStream};

pub async fn start(addr: impl Into<SocketAddr>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  
  let addr = addr.into();

  let listener = TcpListener::bind(addr).await?;

  loop {
    let (socket, addr) = listener.accept().await?;
    debug_println!("accept: {}", addr);
    let _ = tokio::spawn(handle_connection(socket, addr));
  };
}

pub async fn handle_connection(mut socket: TcpStream, _addr: SocketAddr) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  
  let head = read_request_head(&mut socket).await?;
  debug_println!("head readed");

  let version = Version::HTTP_10;  

  if head.method != Method::GET {
    debug_println!("not GET");
    let status = StatusCode::METHOD_NOT_ALLOWED;
    let body = b"405 Method Not Allowed";
    let mut headers = HeaderMap::with_capacity(3);
    headers.append(ALLOW, HeaderValue::from_static("GET"));
    headers.append(CONTENT_LENGTH, HeaderValue::from_str(body.len().to_string().as_str()).unwrap());
    headers.append(CONTENT_TYPE, HeaderValue::from_static("text/plain;charset=utf-8"));

    let head = ResponseHead { version, status, headers };
    
    write_response_head(&mut socket, head, true).await?;
    
    debug_println!("writing body to socket");
    socket.write_all(body).await?;
    debug_println!("shutting down socket");
    socket.shutdown().await?;

    return Ok(())
  }

  if head.uri == "/status" {
    debug_println!("GET /status");
    let status = StatusCode::OK;
    let body = b"200 OK";
    let mut headers = HeaderMap::with_capacity(2);
    headers.append(CONTENT_TYPE, HeaderValue::from_static("text/plain;charset=utf-8"));
    headers.append(CONTENT_LENGTH, HeaderValue::from_str(body.len().to_string().as_str()).unwrap());
    let head = ResponseHead { version, status, headers };

    write_response_head(&mut socket, head, true).await?;
    debug_println!("writing body to socket");
    socket.write_all(body).await?;
    debug_println!("shutting down socket");
    socket.shutdown().await?;
    return Ok(());
  }

  debug_println!("response 404");
  let status = StatusCode::NOT_FOUND;
  let body = format!("Cannot {} {}", head.method, head.uri);
  let body = body.as_bytes();
  let mut headers = HeaderMap::with_capacity(2);
  headers.append(CONTENT_TYPE, HeaderValue::from_static("text/plain;chatset=utf-8"));
  headers.append(CONTENT_LENGTH, HeaderValue::from_str(body.len().to_string().as_str()).unwrap());
  let head = ResponseHead { version, status, headers };  
  
  write_response_head(&mut socket, head, true).await?;
  debug_println!("writing body to socket");
  socket.write_all(body).await?;
  debug_println!("shutting down socket");
  socket.shutdown().await?;
  Ok(())

}