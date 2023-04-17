mod error;
mod handler;
mod http;

use drop_tracer::DropTracer;
use error::HandlerError;
use http::RequestHead;
use hyper::{http::HeaderValue, Method};
use log::*;
use media_sessions::MediaSessionMap;
use owo_colors::*;
use shutdown::Shutdown;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

use crate::handler::{method_not_allowed, not_found, source, status};
use crate::http::read_request_head;

pub async fn start(
  addr: impl Into<SocketAddr>,
  media_sessions: MediaSessionMap,
  drop_tracer: DropTracer,
  shutdown: Shutdown,
) -> Result<(), std::io::Error> {
  let local_addr = addr.into();

  let listener = TcpListener::bind(local_addr).await?;

  info!("source server bound to {}", local_addr.yellow());

  loop {
    tokio::select! {

      r = listener.accept() => {

        let (socket, remote_addr) = r?;

        tokio::spawn(handle_connection(
          socket,
          local_addr,
          remote_addr,
          media_sessions.clone(),
          drop_tracer.clone(),
          shutdown.clone(),
        ));

        if shutdown.is_closed() {
          return Ok(())
        }
      },

      _ = shutdown.signal() => {
        return Ok(());
      }
    };
  }
}

pub async fn handle_connection(
  mut socket: TcpStream,
  local_addr: SocketAddr,
  remote_addr: SocketAddr,
  media_sessions: MediaSessionMap,
  drop_tracer: DropTracer,
  shutdown: Shutdown,
) -> Result<(), HandlerError> {
  // this increases performance by aprox 5%
  // we'll do infrequent large writes so this makes sense
  socket.set_nodelay(true)?;

  // using buf reader here increases performance by aprox 6%
  // TODO: use buffered reader?
  // let mut reader = tokio::io::BufReader::new(socket);

  let head = read_request_head(&mut socket).await?;
  trace!("head readed");

  // need to copy here because we'll use socket again as non buffered reader
  // and tokio doesn't provide a way to get the buffer as owned
  // let leading_buf = Vec::from(reader.buffer());
  // let socket = reader.into_inner();

  match (&head.method, head.uri.path()) {
    (&Method::GET, "/status") => status(socket, head).await,
    (_, "/status") => method_not_allowed(socket, head, HeaderValue::from_static("GET")).await,
    _ => {
      if let Some(id) = is_source_client_uri(&head) {
        if head.method == Method::PUT || head.method.as_str().eq_ignore_ascii_case("SOURCE") {
          source(
            socket,
            local_addr,
            remote_addr,
            head,
            id,
            media_sessions,
            drop_tracer,
            shutdown,
          )
          .await
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
  if let Some(caps) = re.captures(head.uri.path()) {
    let id = caps.get(1).unwrap().as_str();
    Some(id.to_string())
  } else {
    None
  }
}