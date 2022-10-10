mod handler;
mod http;
mod error;

use std::{net::SocketAddr, str::FromStr};
use debug_print::debug_println;
use error::HandlerError;
use http::RequestHead;
use hyper::{Method, http::HeaderValue};
use tokio::net::{TcpListener, TcpStream};
use crate::{http::read_request_head, handler::{status, method_not_allowed, source, not_found}};

pub async fn start(addr: impl Into<SocketAddr>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  
  let addr = addr.into();

  let listener = TcpListener::bind(addr).await?;

  println!("source server bound to {addr}");

  loop {
    let (socket, addr) = listener.accept().await?;
    debug_println!("accept: {}", addr);
    let _ = tokio::spawn(handle_connection(socket, addr));
  };
}

pub async fn handle_connection(socket: TcpStream, _addr: SocketAddr) -> Result<(), HandlerError> {
  
  // this increases performance by aprox 5%
  // we'll do infrequent large writes so this makes sense 
  socket.set_nodelay(true)?;

  // using buf reader here increases performance by aprox 6% 
  let mut reader = tokio::io::BufReader::new(socket);

  let head = read_request_head(&mut reader).await?;
  debug_println!("head readed");

  // need to copy here because we'll use socket again as non buffered reader
  // and tokio doesn't provide a way to get the buffer as owned
  let leading_buf = Vec::from(reader.buffer());
  let socket = reader.into_inner();

  match (&head.method, head.uri.as_str()) {
    (&Method::GET, "/status") => status(socket, head).await,
    (_, "/status") => method_not_allowed(socket, head, HeaderValue::from_static("GET")).await, 
    _ => {
      if let Some(id) = is_source_client_uri(&head) {
        if head.method == Method::PUT || head.method == Method::from_str("SOURCE").unwrap() {
          source(socket, head, leading_buf, id).await
        } else {
          method_not_allowed(socket, head, HeaderValue::from_str("PUT, SOURCE").unwrap()).await
        }
      } else {
        not_found(socket, head).await
      }
    }
  }
}

fn is_source_client_uri(head: &RequestHead) -> Option<String> {
  let re = regex_static::static_regex!("^/?([^/]{1,20})/source/?$");
  if let Some(caps) = re.captures(head.uri.as_str()) {
    let id = caps.get(1).unwrap().as_str();
    Some(id.to_string())
  } else {
    None
  }
}