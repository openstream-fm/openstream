#![allow(clippy::useless_format)]

use async_trait::async_trait;
use db::station::Station;
use db::Model;
use drop_tracer::DropTracer;
use futures::stream::FuturesUnordered;
use futures::TryStreamExt;
use hyper::header::{HeaderValue, ALLOW, CONTENT_TYPE, WWW_AUTHENTICATE};
use hyper::HeaderMap;
use hyper::{Body, Method, Server, StatusCode};
use log::*;
use media_sessions::live::LiveError;
use media_sessions::{MediaSessionKind, MediaSessionMap};
use prex::{handler::Handler, Next, Request, Response};
use shutdown::Shutdown;
use socket2::{Domain, Protocol, Socket, Type};
use std::collections::btree_map::Entry;
use std::future::Future;
use std::net::SocketAddr;

#[allow(clippy::declare_interior_mutable_const)]
const TEXT_PLAIN_UTF8: HeaderValue = HeaderValue::from_static("text/plain;charset=utf8");

#[derive(Debug, thiserror::Error)]
pub enum SourceServerError {
  #[error("io error: {0}")]
  Io(#[from] std::io::Error),
  #[error("hyper error: {0}")]
  Hyper(#[from] hyper::Error),
}

#[derive(Debug)]
pub struct SourceServer {
  addrs: Vec<SocketAddr>,
  media_sessions: MediaSessionMap,
  drop_tracer: DropTracer,
  shutdown: Shutdown,
}

#[derive(Debug)]
struct SourceServerInner {}

impl SourceServer {
  pub fn new(
    addrs: Vec<SocketAddr>,
    media_sessions: MediaSessionMap,
    drop_tracer: DropTracer,
    shutdown: Shutdown,
  ) -> Self {
    Self {
      addrs,
      media_sessions,
      drop_tracer,
      shutdown,
    }
  }

  pub fn start(
    self,
  ) -> Result<impl Future<Output = Result<(), hyper::Error>> + 'static, SourceServerError> {
    let mut app = prex::prex();

    app.with(http::middleware::server);
    app.get("/status", http::middleware::status);

    app.any(
      "/:id/source",
      SourceHandler::new(
        self.media_sessions.clone(),
        self.drop_tracer.clone(),
        self.shutdown.clone(),
      ),
    );

    let app = app.build().expect("prex app build source");

    let futs = FuturesUnordered::new();

    for addr in self.addrs.iter().cloned() {
      let domain = match addr {
        SocketAddr::V4(_) => Domain::IPV4,
        SocketAddr::V6(_) => Domain::IPV6,
      };

      let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP))?;

      if addr.is_ipv6() {
        socket.set_only_v6(true)?;
      }

      // socket.set_reuse_address(true)?;
      // socket.set_reuse_port(true)?;

      socket.bind(&addr.into())?;
      socket.listen(1024)?;

      let tcp = socket.into();

      let source = Server::from_tcp(tcp)?
        .http1_only(true)
        .http1_title_case_headers(false)
        .http1_preserve_header_case(false)
        .http1_keepalive(false);

      {
        use owo_colors::*;
        info!("source server bound to {}", addr.yellow());
      }

      futs.push(
        source
          .serve(app.clone())
          .with_graceful_shutdown(self.shutdown.signal()),
      )
    }

    // let mut app = prex::prex();

    // app.get(
    //   "/broadcast/:id",
    //   BroadcastHandler::new(self.channels.clone(), self.shutdown.clone()),
    // );

    // let app = app.build().expect("prex app build source");

    // for addr in &self.broadcast_addrs {
    //   let broadcast = Server::try_bind(addr)?
    //     .http1_only(true)
    //     .http1_title_case_headers(false)
    //     .http1_preserve_header_case(false);

    //   {
    //     use owo_colors::*;
    //     info!("source broadcaster server bound to {}", addr.yellow());
    //   }

    //   futs.push(
    //     broadcast
    //       .serve(app.clone())
    //       .with_graceful_shutdown(self.shutdown.signal()),
    //   );
    // }

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

#[derive(Debug, Clone)]
struct SourceHandler {
  media_sessions: MediaSessionMap,
  drop_tracer: DropTracer,
  shutdown: Shutdown,
}

impl SourceHandler {
  fn new(media_sessions: MediaSessionMap, drop_tracer: DropTracer, shutdown: Shutdown) -> Self {
    Self {
      media_sessions,
      drop_tracer,
      shutdown,
    }
  }

  async fn handle(&self, req: Request) -> prex::Response {
    debug!("source client connected");
    debug!("== source client headers ==");
    for (key, value) in req.headers().iter() {
      debug!("== {:?}: {:?}", key, value);
    }
    debug!("== end source client headers ==");

    enum SourceMethod {
      Put,
      Source,
    }

    let _method: SourceMethod = if req.method().eq(&Method::PUT) {
      SourceMethod::Put
    } else if req.method().as_str().eq_ignore_ascii_case("SOURCE") {
      SourceMethod::Source
    } else {
      let mut res = Response::new(StatusCode::METHOD_NOT_ALLOWED);
      res.headers_mut().insert(CONTENT_TYPE, TEXT_PLAIN_UTF8);
      res
        .headers_mut()
        .insert(ALLOW, HeaderValue::from_static("PUT,SOURCE"));
      *res.body_mut() = Body::from(format!(
        "method {} is not allowed, allowed methods are PUT or SOURCE",
        req.method().as_str()
      ));
      return res;
    };

    let content_type = match req
      .headers()
      .get(CONTENT_TYPE)
      .and_then(|t| t.to_str().ok())
    {
      None => {
        let mut res = Response::new(StatusCode::BAD_REQUEST);
        res.headers_mut().insert(CONTENT_TYPE, TEXT_PLAIN_UTF8);
        *res.body_mut() = Body::from("content-type header is required");
        return res;
      }

      Some(t) => t.to_string(),
    };
    // safety unwrap: param "id" is required in route defnition
    let id = req.param("id").unwrap().to_string();

    let station = match Station::get_by_id(&id).await {
      Err(_e) => {
        let mut res = Response::new(StatusCode::INTERNAL_SERVER_ERROR);
        res.headers_mut().insert(CONTENT_TYPE, TEXT_PLAIN_UTF8);
        *res.body_mut() = Body::from("internal server error (db)");
        return res;
      }
      Ok(None) => {
        let mut res = Response::new(StatusCode::NOT_FOUND);
        res.headers_mut().insert(CONTENT_TYPE, TEXT_PLAIN_UTF8);
        *res.body_mut() = Body::from(format!("station with id {id} not found"));
        return res;
      }
      Ok(Some(station)) => station,
    };

    let password = station.source_password;

    // TODO: implement ip limit security
    match req.basic_auth() {
      None => {
        let mut res = Response::new(StatusCode::UNAUTHORIZED);
        res.headers_mut().insert(CONTENT_TYPE, TEXT_PLAIN_UTF8);
        res.headers_mut().insert(
          WWW_AUTHENTICATE,
          HeaderValue::from_static(r#"Basic realm="authentication", charset="UTF-8"#),
        );
        *res.body_mut() = Body::from("basic auth not present or malformed");
        return res;
      }

      Some(auth) => {
        if auth.user != "source" || auth.password != password {
          let mut res = Response::new(StatusCode::UNAUTHORIZED);
          res.headers_mut().insert(CONTENT_TYPE, TEXT_PLAIN_UTF8);
          res.headers_mut().insert(
            WWW_AUTHENTICATE,
            HeaderValue::from_static(r#"Basic realm="authentication", charset="UTF-8"#),
          );
          *res.body_mut() = Body::from("basic auth user/password mismatch");
          return res;
        }
      }
    }

    let tx = {
      let mut map = self.media_sessions.write();
      match map.entry(&station.id) {
        Entry::Vacant(_) => map.transmit(&station.id, MediaSessionKind::Live { content_type }),
        Entry::Occupied(entry) => {
          let session = entry.get();
          match session.kind() {
            MediaSessionKind::Live { .. } => {
              let mut res = Response::new(StatusCode::UNPROCESSABLE_ENTITY);
              *res.body_mut() = Body::from("this mountpoint is already in use, try again later or change the source password from your station dashboard in openstream studio app");
              return res;
            }

            MediaSessionKind::Playlist { .. } | MediaSessionKind::Relay { .. } => {
              map.transmit(&station.id, MediaSessionKind::Live { content_type })
            }
          }
        }
      }
    };

    let result = tokio::spawn({
      let shutdown = self.shutdown.clone();
      let drop_tracer = self.drop_tracer.clone();
      let request_document = db::http::Request::from_http(&req);
      media_sessions::live::run_live_session(
        tx,
        req.into_body(),
        self.media_sessions.deployment_id.clone(),
        request_document,
        shutdown,
        drop_tracer,
      )
    })
    .await
    .unwrap();

    match result {
      Ok(()) => {
        let mut res = Response::new(StatusCode::OK);
        *res.body_mut() = Body::from("data streamed successfully");

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, TEXT_PLAIN_UTF8);

        res
      }

      Err(e) => {
        // let (status, body) = match e {
        //   LiveError::Db(_) => (StatusCode::INTERNAL_SERVER_ERROR, String::from("internal error creating live media session, try again later or report it to the administrators")),
        //   LiveError::Spawn(_) | LiveError::ExitIo(_) | LiveError::StderrError(_) => (StatusCode::INTERNAL_SERVER_ERROR, String::from("error allocating internal stream converter, try again later or report it to the administrators")),
        //   LiveError::ExitNotOk { stderr } => (StatusCode::FORBIDDEN, format!("error converting the audio stream (exit), possibly the audio is corrupted or is using a not supported format: {stderr}")),
        // };

        // let (status, message) = match e {
        //   LiveError::Db(_) => (StatusCode::INTERNAL_SERVER_ERROR, String::from("internal error creating live media session, try again later or report it to the administrators")),
        //   LiveError::Probe(e) => {
        //     let status = StatusCode::FORBIDDEN;
        //     let message = match e {
        //       mp3::ProbeError::NoDefaultTrack => String::from("unsupported stream: incomming audio stream does not have a default track"),
        //       mp3::ProbeError::NotMP3 => String::from("unsopported stream: incoming audio stream is not MP3"),
        //       mp3::ProbeError::NotSupported(e) => format!("unsupported stream: {e}")
        //     };
        //     (status, message)
        //   }
        //   LiveError::Play(e) => {
        //     let status = StatusCode::FORBIDDEN;
        //     let message = match e {
        //       mp3::PlayError::Packet(e) => format!("play packet error: {e}"),
        //       mp3::PlayError::Reset(e) => format!("play reset error: {e}"),
        //       mp3::PlayError::ResetNoDefaultTrack => String::from("play reset error: no default track after reset"),
        //       mp3::PlayError::ResetTrackNotMP3 => String::from("play reset error: new default track is not MP3"),
        //       mp3::PlayError::MissingTimeBase => String::from("internal error: missing track time base"),
        //     };
        //     (status, message)
        //   }
        // };

        let (status, message) = match e {
           LiveError::Db(_) => (StatusCode::INTERNAL_SERVER_ERROR, String::from("internal error creating live media session, try again later or report it to the administrators")),
           LiveError::Data(_) => (StatusCode::BAD_REQUEST, String::from("io error reading data")),
        };

        let mut res = Response::new(status);
        res.headers_mut().insert(CONTENT_TYPE, TEXT_PLAIN_UTF8);
        *res.body_mut() = Body::from(message);
        res
      }
    }
  }
}

#[async_trait]
impl Handler for SourceHandler {
  async fn call(&self, req: Request, _: Next) -> prex::Response {
    self.handle(req).await
  }
}

// #[derive(Debug)]
// struct BroadcastHandler {
//   channels: ChannelMap,
//   shutdown: Shutdown,
// }

// impl BroadcastHandler {
//   fn new(channels: ChannelMap, shutdown: Shutdown) -> Self {
//     Self { channels, shutdown }
//   }
// }

// #[async_trait]
// impl Handler for BroadcastHandler {
//   async fn call(&self, request: Request, _: Next) -> Response {
//     // unwrap: id is a required param in path defintion
//     let id = request.param("id").unwrap();

//     let mut rx = match self.channels.subscribe(id) {
//       None => {
//         let mut res = Response::new(StatusCode::NOT_FOUND);
//         *res.body_mut() = Body::from(format!(
//           "stream with id {id} not actively streaming from this server"
//         ));

//         return res;
//       }
//       Some(rx) => rx,
//     };

//     let (mut sender, body) = Body::channel();

//     tokio::spawn({
//       let shutdown = self.shutdown.clone();

//       async move {
//         loop {
//           let item = rx.recv().await;
//           if shutdown.is_closed() {
//             break;
//           };

//           match item {
//             Err(e) => match e {
//               RecvError::Closed => break,
//               RecvError::Lagged(_) => continue,
//             },
//             Ok(bytes) => match sender.send_data(bytes).await {
//               Err(_) => break,
//               Ok(()) => continue,
//             },
//           }
//         }
//       }
//     });

//     let mut res = Response::new(StatusCode::OK);
//     res
//       .headers_mut()
//       .append(CONTENT_TYPE, HeaderValue::from_static("audio/mpeg"));
//     *res.body_mut() = body;

//     res
//   }
// }
