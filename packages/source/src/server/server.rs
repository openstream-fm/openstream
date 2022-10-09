use std::{net::SocketAddr, fmt::Display, collections::hash_map::Entry, sync::atomic::Ordering, ops::{Deref, DerefMut}};

use crate::{server::*, CHANNELS, Channel, CHANNEL_COUNT};
use tokio::net::{TcpListener, TcpStream};

macro_rules! text_plain {
  () => {
    ::hyper::header::HeaderValue::from_static("text/plain;charset=utf-8")
  }
}

macro_rules! content_len {
  ($tt:expr) => {
    {
      let body: &[u8] = $tt.as_ref();
      ::hyper::header::HeaderValue::from_str(body.len().to_string().as_str()).unwrap()
    }
  }
}

#[derive(Debug)]
pub enum HandlerError {
  Io(std::io::Error),
  ReadHead(ReadHeadError),
  WriteHead(WriteHeadError)
}

impl Display for HandlerError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        Self::Io(e) => write!(f, "{}", e),
        Self::ReadHead(e) => write!(f, "{}", e),
        Self::WriteHead(e) => write!(f, "{}", e)
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

mod handlers {
  
  use debug_print::debug_println;
  use ffmpeg::{Ffmpeg, FfmpegConfig, FfmpegSpawn};
  use hyper::{http::HeaderValue, StatusCode, header::{ALLOW, CONTENT_LENGTH, CONTENT_TYPE}, Version, Method, HeaderMap};
  use stream_util::{IntoTryBytesStream, IntoTryBytesStreamRated};
  use tokio::{net::TcpStream, io::{AsyncWriteExt, AsyncReadExt}};
  use tokio_stream::StreamExt;
  use crate::{server::{RequestHead, ResponseHead, write_response_head, server::insert_new_source_client}};
  use super::HandlerError;

  macro_rules! headers {
    ($n:expr) => {
      {
        let n: usize = $n;
        let mut headers = ::hyper::HeaderMap::with_capacity(n + 1);
        headers.append(::hyper::header::CONNECTION, HeaderValue::from_static("close"));
        //headers.append(::hyper::header::ACCEPT_ENCODING, HeaderValue::from_static("identity"));
        //headers.append(::hyper::header::TRANSFER_ENCODING, HeaderValue::from_static("identity"));
        headers
      } 
    };

    () => {
      headers!(0)
    }
  }

  pub async fn method_not_allowed(mut socket: TcpStream, req: RequestHead, allow: HeaderValue) -> Result<(), HandlerError> {
    debug_println!("method_not_allowed: {} {}", req.method, req.uri);
    let status = StatusCode::METHOD_NOT_ALLOWED;
    let body = b"405 Method Not Allowed";
    let mut headers = headers!(3);
    headers.append(ALLOW, allow);
    headers.append(CONTENT_LENGTH, HeaderValue::from_str(body.len().to_string().as_str()).unwrap());
    headers.append(CONTENT_TYPE, HeaderValue::from_static("text/plain;charset=utf-8"));

    let head = ResponseHead { version: Version::HTTP_10, status, headers };
    
    write_response_head(&mut socket, head, true).await?;
    
    if req.method != &Method::HEAD {
      debug_println!("writing body to socket");
      socket.write_all(body).await?;
    }
    debug_println!("shutting down socket");
    socket.shutdown().await?;

    Ok(())
  }

  pub async fn not_found(mut socket: TcpStream, req: RequestHead) -> Result<(), HandlerError> {
    
    debug_println!("not_found: {} {}", req.method, req.uri);
    
    let status = StatusCode::METHOD_NOT_ALLOWED;
    let body = b"404 Not Found";

    let mut headers = headers!(2);
    headers.append(CONTENT_TYPE, text_plain!());
    headers.append(CONTENT_LENGTH, content_len!(body));

    let head = ResponseHead { version: Version::HTTP_10, status, headers };
    
    write_response_head(&mut socket, head, true).await?;
    
    debug_println!("writing body to socket");
    if req.method != &Method::HEAD {
      socket.write_all(body).await?;
      debug_println!("shutting down socket");
    }
    
    socket.shutdown().await?;

    Ok(())
  }

  pub async fn status(mut socket: TcpStream, req: RequestHead) -> Result<(), HandlerError> {

    debug_println!("status: {} {}", req.method, req.uri);
    let status = StatusCode::OK;
    let body = b"200 OK";
    let mut headers = headers!(2);
    headers.append(CONTENT_LENGTH, HeaderValue::from_str(body.len().to_string().as_str()).unwrap());
    headers.append(CONTENT_TYPE, HeaderValue::from_static("text/plain;charset=utf-8"));

    let head = ResponseHead { version: Version::HTTP_10, status, headers };
    
    write_response_head(&mut socket, head, true).await?;
    
    if req.method != &Method::HEAD {
      debug_println!("writing body to socket");
      socket.write_all(body).await?;
    }

    debug_println!("shutting down socket");
    socket.shutdown().await?;

    Ok(())
  }


  pub async fn source(mut socket: TcpStream, head: RequestHead, leading_buf: Vec<u8>, id: String) -> Result<(), HandlerError> {
    
    debug_println!("source: {} {} => id: {}", head.method, head.uri, id);

    // if not PUT is SOURCE checked in router
    let _is_put = head.method == Method::PUT; 

    let is_continue = match head.headers.get("expect") {
      None => false,
      Some(h) => h.as_bytes().eq_ignore_ascii_case(b"100-continue")
    };

    let channel = match insert_new_source_client(id) {
      Some(channel) => channel,
      None => {
        let body = b"This mountpoint is already in use, try again later";

        let mut headers = headers!(2);
        headers.append(CONTENT_TYPE, text_plain!());
        headers.append(CONTENT_TYPE, content_len!(body));

        // FORBIDEN (403) is used to communicate all sorts of errors
        let response = ResponseHead {
          status: StatusCode::FORBIDDEN,
          headers: headers!(),
          version: Version::HTTP_10
        };

        write_response_head(&mut socket, response, true).await?;
        socket.write_all(body).await?;
        socket.flush().await?;
    
        return Ok(())
      }
    };

    let ff_spawn  = match Ffmpeg::with_config(FfmpegConfig::default()).spawn() {
      
      Err(_) => {

          let body = b"Error allocating internal stream converter, try again later or report it to the administrators";

          let mut headers = headers!(2);
          headers.append(CONTENT_TYPE, text_plain!());
          headers.append(CONTENT_LENGTH, content_len!(body));

          let response = ResponseHead {
            version: Version::HTTP_10,
            status: StatusCode::INTERNAL_SERVER_ERROR,
            headers,
          };

          write_response_head(&mut socket, response, true).await?;
          socket.write_all(body).await?;
          socket.flush().await?;
          return Ok(())
      },
      
      Ok(spawn) => spawn,
    };

    let FfmpegSpawn {
        mut stderr,
        mut stdin,
        stdout,
        mut child,
        config: _,
    } = ff_spawn;

    if is_continue {
      let version = Version::HTTP_10;
      let status = StatusCode::CONTINUE;
      let headers = HeaderMap::new();

      let head = ResponseHead { version, status, headers };

      write_response_head(&mut socket, head, false).await?;
    }

    let (mut socket_read, mut socket_write) = socket.into_split();

    let _write_handle = tokio::spawn(async move {
      if leading_buf.len() != 0 {
        stdin.write_all(leading_buf.as_ref()).await?;
      };

      // TODO: implement chunked encoding 
      tokio::io::copy(&mut socket_read, &mut stdin).await.expect("io::copy to ffmpeg stdin");

      Result::<(), std::io::Error>::Ok(())
    });

    let stderr_handle = tokio::spawn(async move {
      let mut buf = vec![];
      stderr.read_to_end(&mut buf).await?;
      Result::<Vec<u8>, std::io::Error>::Ok(buf)
    });
    
    let _broadcast_handle = tokio::spawn(async move {
      let stream = stdout.into_bytes_stream(2048).rated(16 * 1024);
      tokio::pin!(stream);
      while let Some(result) = stream.next().await {
        let bytes = result?;
        // this only fails when no subscribers but that is ok
        let _ = channel.send(bytes);
      }

      Result::<(), std::io::Error>::Ok(())
    });

    let exit = child.wait().await?;

    if exit.success() {
      let body = b"Data streamed successfully";
      let version = Version::HTTP_10;
      let status = StatusCode::OK;
      let mut headers = headers!(2);
      headers.append(CONTENT_TYPE, text_plain!());
      headers.append(CONTENT_LENGTH, content_len!(body));

      let head = ResponseHead{ version, status, headers };
      
      write_response_head(&mut socket_write, head, true).await?;
      socket_write.write_all(body).await?;
      socket_write.flush().await?;
      
      Ok(())
    
    } else {
      let body = match stderr_handle.await {
        Err(_) => format!("Internal error allocating stream converter (stderr panic)"),
        Ok(r) => match r {
          Err(_) => format!("Internal error allocating stream converter (stderr error)"),
          Ok(v) => { 
            let stderr_out = String::from_utf8_lossy(v.as_ref());
            format!("Error converting the audio stream, possibly the audio is corrupted or is using a not supported format: {stderr_out}")
          }
        }
      };

      let version = Version::HTTP_10;
      let status = StatusCode::BAD_REQUEST;
      let mut headers = headers!(2);
      headers.append(CONTENT_TYPE, text_plain!());
      headers.append(CONTENT_LENGTH, content_len!(body.as_bytes()));
      
      let head = ResponseHead { version, status, headers };

      write_response_head(&mut socket_write, head, true).await?;
      socket_write.write_all(body.as_bytes()).await?;
      socket_write.flush().await?;
      Ok(())
    }
  }
}

// this should not be clonable nor with public fields
#[derive(Debug)]
struct OwnedDroppableChannel {
  id: String,
  channel: Channel
}

impl Deref for OwnedDroppableChannel {
  type Target = Channel;
  fn deref(&self) -> &Self::Target {
      &self.channel
  }
}

impl DerefMut for OwnedDroppableChannel {
  fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.channel
  }
}

impl Drop for OwnedDroppableChannel {
  fn drop(&mut self) {
    println!("[INFO]: dropping channel {}", self.id);
    let entry = CHANNELS.write().remove(&self.id);
    if entry.is_none() {
      eprintln!("[WARN]: dropped OwnedDroppableClient that is not present in CHANNELS map: id => {}", self.id);
    } else {
      let count = CHANNEL_COUNT.fetch_sub(1, Ordering::Relaxed) - 1;
      println!("[INFO]: {count} open channels");
    }
  }
}

fn insert_new_source_client(id: String) -> Option<OwnedDroppableChannel> {
  let mut lock = CHANNELS.write();
  match lock.entry(id.clone()) {
    Entry::Vacant(slot) => {
      let channel = Channel::new();
      slot.insert(channel.clone());
      let count = CHANNEL_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
      println!("[INFO]: created channel {id} => {count} open channels");
      Some(OwnedDroppableChannel { id, channel })
    },
    Entry::Occupied(_) => {
      println!("[INFO]: fail to create channel {id}, channel is occupied");
      None
    }
  }
}

use handlers::*;