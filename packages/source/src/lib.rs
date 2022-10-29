use async_trait::async_trait;
use channels::ChannelMap;
use constants::STREAM_CHUNK_SIZE;
use ffmpeg::{Ffmpeg, FfmpegConfig, FfmpegSpawn};
use hyper::body::HttpBody;
use hyper::header::{HeaderValue, ALLOW, CONTENT_TYPE};
use hyper::HeaderMap;
use hyper::{Body, Method, Server, StatusCode};
use log::*;
use owo::*;
use prex::{handler::Handler, Next, Request, Response};
use std::future::Future;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Notify;

#[derive(Debug, Clone)]
pub struct SourceServer {
  inner: Arc<SourceServerInner>,
}

const OPEN: bool = false;
const CLOSED: bool = true;

#[derive(Debug)]
struct SourceServerInner {
  addr: SocketAddr,
  channels: Arc<ChannelMap>,
  shutdown_signal: Notify,
  closed: AtomicBool,
}

impl SourceServer {
  pub fn new(addr: impl Into<SocketAddr>, channels: Arc<ChannelMap>) -> Self {
    Self {
      inner: Arc::new(SourceServerInner {
        addr: addr.into(),
        channels,
        shutdown_signal: Notify::new(),
        closed: AtomicBool::new(OPEN),
      }),
    }
  }

  pub fn start(
    &self,
  ) -> Result<impl Future<Output = Result<(), hyper::Error>> + 'static, hyper::Error> {
    let server = Server::try_bind(&self.inner.addr)?
      .http1_only(true)
      .http1_header_read_timeout(Duration::from_secs(5))
      .http1_title_case_headers(false)
      .http1_preserve_header_case(false);

    info!("source server bound to {}", self.inner.addr.yellow());

    let mut app = prex::prex();

    app.with(logger);
    //app.with(http_1_0_version);
    //app.with(connection_close);
    //app.any("/:id/source", source_allow);
    //app.any("/:id/source", source_accept);
    app.any("/:id/source", SourceHandler::new(self.clone()));

    let app = app.build().expect("prex app build source");

    let signal = {
      let me = self.clone();
      async move {
        me.inner.shutdown_signal.notified().await;
      }
    };

    Ok(async move {
      server.serve(app).with_graceful_shutdown(signal).await?;
      Ok(())
    })
  }

  pub fn graceful_shutdown(&self) {
    self.inner.closed.store(CLOSED, Ordering::SeqCst);
    self.inner.shutdown_signal.notify_waiters();
  }
}

async fn logger(req: Request, next: Next) -> prex::Response {
  let method = req.method().clone();
  let path = req.uri().path().to_string();

  let res = next.run(req).await;

  let status = res.status();

  debug!("[request] {method} {path} => {status}");

  res
}

#[derive(Debug, Clone)]
struct SourceHandler {
  server: SourceServer,
}

impl SourceHandler {
  fn new(server: SourceServer) -> Self {
    Self { server }
  }
}

#[async_trait]
impl Handler for SourceHandler {
  async fn call(&self, mut req: Request, _next: Next) -> prex::Response {
    enum SourceMethod {
      PUT,
      SOURCE,
    }

    let _method: SourceMethod = if req.method().eq(&Method::PUT) {
      SourceMethod::PUT
    } else if req.method().as_str().eq_ignore_ascii_case("SOURCE") {
      SourceMethod::SOURCE
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

    let channel = match self.server.inner.channels.transmit(id.clone()) {
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

    let ff_spawn = match Ffmpeg::with_config(ffmpeg_config).spawn() {
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
        use tokio_stream::StreamExt;

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
          let data = req.data().await;

          if self.server.inner.closed.load(Ordering::SeqCst) == CLOSED {
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
