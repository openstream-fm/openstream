#![allow(clippy::useless_format)]

use async_trait::async_trait;
use channels::ChannelMap;
use constants::STREAM_CHUNK_SIZE;
use ffmpeg::{Ffmpeg, FfmpegConfig, FfmpegSpawn};
use futures::stream::FuturesUnordered;
use futures::{StreamExt, TryStreamExt};
use hyper::body::HttpBody;
use hyper::header::{HeaderValue, ALLOW, CONTENT_TYPE};
use hyper::HeaderMap;
use hyper::{Body, Method, Server, StatusCode};
use log::*;
use owo_colors::*;
use prex::{handler::Handler, Next, Request, Response};
use shutdown::Shutdown;
use std::future::Future;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::broadcast::error::RecvError;

#[derive(Debug)]
pub struct SourceServer {
  source_addrs: Vec<SocketAddr>,
  broadcast_addrs: Vec<SocketAddr>,
  channels: ChannelMap,
  shutdown: Shutdown,
}

#[derive(Debug)]
struct SourceServerInner {}

impl SourceServer {
  pub fn new(
    source_addrs: Vec<SocketAddr>,
    broadcast_addrs: Vec<SocketAddr>,
    shutdown: Shutdown,
  ) -> Self {
    let channels = ChannelMap::new();

    Self {
      source_addrs,
      broadcast_addrs,
      channels,
      shutdown,
    }
  }

  pub fn start(
    self,
  ) -> Result<impl Future<Output = Result<(), hyper::Error>> + 'static, hyper::Error> {
    let futs = FuturesUnordered::new();

    for addr in &self.source_addrs {
      let source = Server::try_bind(addr)?
        .http1_only(true)
        .http1_title_case_headers(false)
        .http1_preserve_header_case(false)
        .http1_keepalive(false);

      info!("source receiver server bound to {}", addr.yellow());

      let mut app = prex::prex();

      if log::log_enabled!(Level::Debug) {
        app.with(logger);
      }

      app.any(
        "/:id/source",
        SourceHandler::new(self.channels.clone(), self.shutdown.clone()),
      );

      let app = app.build().expect("prex app build source");

      futs.push(
        source
          .serve(app)
          .with_graceful_shutdown(self.shutdown.signal()),
      )
    }

    for addr in &self.broadcast_addrs {
      let broadcast = Server::try_bind(addr)?
        .http1_only(true)
        .http1_title_case_headers(false)
        .http1_preserve_header_case(false);

      info!("source broadcaster server bound to {}", addr.yellow());

      let mut app = prex::prex();

      app.get(
        "/broadcast/:id",
        BroadcastHandler::new(self.channels.clone(), self.shutdown.clone()),
      );

      let app = app.build().expect("prex app build source");

      futs.push(
        broadcast
          .serve(app)
          .with_graceful_shutdown(self.shutdown.signal()),
      );
    }

    Ok(async move {
      futs.try_collect().await?;
      drop(self);
      Ok(())
    })
  }
}

impl Drop for SourceServer {
  fn drop(&mut self) {
    info!("source server dropped");
  }
}

async fn logger(req: Request, next: Next) -> prex::Response {
  let start = tokio::time::Instant::now();

  let method = req.method().clone();
  let path = req.uri().path().to_string();

  let res = next.run(req).await;
  let elapsed = start.elapsed().as_millis();
  let status = res.status();

  debug!("[request] {method} {path} => {status} in {}ms", elapsed);

  res
}

#[derive(Debug, Clone)]
struct SourceHandler {
  channels: ChannelMap,
  shutdown: Shutdown,
}

impl SourceHandler {
  fn new(channels: ChannelMap, shutdown: Shutdown) -> Self {
    Self { channels, shutdown }
  }
}

#[async_trait]
impl Handler for SourceHandler {
  async fn call(&self, mut req: Request, _next: Next) -> prex::Response {
    enum SourceMethod {
      Put,
      Source,
    }

    let _method: SourceMethod = if req.method().eq(&Method::PUT) {
      SourceMethod::Put
    } else if req.method().as_str().eq_ignore_ascii_case("SOURCE") {
      SourceMethod::Source
    } else {
      let mut headers = HeaderMap::with_capacity(2);
      headers.append(ALLOW, HeaderValue::from_static("PUT,SOURCE"));
      let mut res = Response::new(StatusCode::METHOD_NOT_ALLOWED);
      *res.headers_mut() = headers;
      *res.body_mut() = Body::from(format!(
        "method {} is not allowed, allowed methods are PUT or SOURCE",
        req.method().as_str()
      ));
      return res;
    };

    // safety unwrap: param "id" is required in route defnition
    let id = req.param("id").unwrap().to_string();

    let channel = match self.channels.transmit(id.clone()) {
      None => {
        let mut res = Response::new(StatusCode::FORBIDDEN);
        *res.body_mut() = Body::from("this source is already in use, try again later");
        return res;
      }

      Some(tx) => tx,
    };

    // need cloning here because of 'static requirements of future

    let ffmpeg_config = FfmpegConfig {
      readrate: true,
      ..FfmpegConfig::default()
    };

    let ff_spawn = match Ffmpeg::new(ffmpeg_config).spawn() {
      Err(_) => {
        // FORBIDEN (403) is used to communicate all sorts of errors
        let mut res = Response::new(StatusCode::INTERNAL_SERVER_ERROR);
        *res.body_mut() = Body::from("error allocating internal stream converter, try again later or report it to the administrators");
        return res;
      }
      Ok(spawn) => spawn,
    };

    let FfmpegSpawn {
      mut stderr,
      mut stdin,
      stdout,
      mut child,
      config: _,
    } = ff_spawn;

    let stderr_handle = async move {
      let mut data = Vec::new();
      stderr.read_to_end(&mut data).await?;
      Result::<Vec<u8>, std::io::Error>::Ok(data)
    };

    let stdout_handle = {
      let id = id.clone();

      async move {
        use stream_util::*;

        let chunks = stdout.into_bytes_stream(STREAM_CHUNK_SIZE);

        tokio::pin!(chunks);

        loop {
          match chunks.next().await {
            None => {
              debug!("[source]: channel {id}: ffmpeg stdout end");
              break;
            }
            Some(Err(e)) => {
              debug!("[sorce]: channel {id}: ffmpeg stdout error: {e}");
              break;
            }
            Some(Ok(bytes)) => {
              debug!(
                "[source]: channel {id}: ffmpeg stdout data: {} bytes",
                bytes.len()
              );
              // only fails if there are no receivers but that is ok
              let _ = channel.send(bytes);
            }
          }
        }
      }
    };

    let write_handle = {
      let id = id.clone();

      // move stdin to drop on close
      async move {
        loop {
          let data = req.body_mut().data().await;

          if self.shutdown.is_closed() {
            break;
          }

          match data {
            None => {
              debug!("[source] channel {id}: recv body end");
              break;
            }

            Some(Err(e)) => {
              debug!("[source] channel {id}: recv body error: {e}");
              break;
            }

            Some(Ok(data)) => {
              debug!(
                "[source] channel {id}: recv body data: {} bytes",
                data.len()
              );

              match stdin.write_all(data.as_ref()).await {
                Err(e) => {
                  debug!("[source] channel {id} stdin error: {e}");
                  break;
                }

                Ok(()) => {
                  debug!("[source] channel {id} stdin write: {} bytes", data.len());
                }
              }
            }
          }
        }
      }
    };

    let status_handle = async move { child.wait().await };

    let (status, _write, _stdout, stderr) =
      tokio::join!(status_handle, write_handle, stdout_handle, stderr_handle);

    let exit = match status {
      Err(e) => {
        debug!("[source] channel {id}: ffmpeg child error: {e}");
        let mut headers = HeaderMap::with_capacity(1);
        headers.append(CONTENT_TYPE, HeaderValue::from_static("text/plain"));

        let body = Body::from("unexpected error allocating the stream converter (exit 1), please report this to the administrators");

        let mut res = Response::new(StatusCode::INTERNAL_SERVER_ERROR);
        *res.headers_mut() = headers;
        *res.body_mut() = body;

        return res;
      }

      Ok(exit) => exit,
    };

    debug!("[source] channel {id}: ffmpeg child end: {exit}");

    if exit.success() {
      let mut res = Response::new(StatusCode::OK);
      *res.body_mut() = Body::from("data streamed successfully");

      let mut headers = HeaderMap::with_capacity(1);
      headers.append(CONTENT_TYPE, HeaderValue::from_static("text/plain"));

      res
    } else {
      let body = match stderr {
        Err(_) => format!("internal error allocating stream converter (stderr 1)"),

        Ok(v) => {
          let out = String::from_utf8_lossy(v.as_ref());
          format!("error converting the audio stream (exit), possibly the audio is corrupted or is using a not supported format: {out}")
        }
      };

      let body = Body::from(body);
      let mut headers = HeaderMap::with_capacity(1);
      headers.append(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
      let mut res = Response::new(StatusCode::OK);
      *res.headers_mut() = headers;
      *res.body_mut() = body;

      res
    }
  }
}

#[derive(Debug)]
struct BroadcastHandler {
  channels: ChannelMap,
  shutdown: Shutdown,
}

impl BroadcastHandler {
  fn new(channels: ChannelMap, shutdown: Shutdown) -> Self {
    Self { channels, shutdown }
  }
}

#[async_trait]
impl Handler for BroadcastHandler {
  async fn call(&self, request: Request, _: Next) -> Response {
    // unwrap: id is a required param in path defintion
    let id = request.param("id").unwrap();

    let mut rx = match self.channels.subscribe(id) {
      None => {
        let mut res = Response::new(StatusCode::NOT_FOUND);
        *res.body_mut() = Body::from(format!(
          "stream with id {id} not actively streaming from this server"
        ));

        return res;
      }
      Some(rx) => rx,
    };

    let (mut sender, body) = Body::channel();

    tokio::spawn({
      let shutdown = self.shutdown.clone();

      async move {
        loop {
          let item = rx.recv().await;
          if shutdown.is_closed() {
            break;
          };

          match item {
            Err(e) => match e {
              RecvError::Closed => break,
              RecvError::Lagged(_) => continue,
            },
            Ok(bytes) => match sender.send_data(bytes).await {
              Err(_) => break,
              Ok(()) => continue,
            },
          }
        }
      }
    });

    let mut res = Response::new(StatusCode::OK);
    res
      .headers_mut()
      .append(CONTENT_TYPE, HeaderValue::from_static("audio/mpeg"));
    *res.body_mut() = body;

    res
  }
}
