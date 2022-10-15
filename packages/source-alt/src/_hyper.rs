// use std::{collections::HashMap, sync::atomic::AtomicUsize, net::SocketAddr, io::BufRead};
// use tokio::{sync::broadcast::{channel, Receiver, Sender}, io::{BufReader, AsyncBufReadExt, AsyncReadExt}};
// use std::sync::atomic::Ordering;
pub mod server;

use std::ops::DerefMut;
use std::{net::SocketAddr, ops::Deref};
use std::future::Future;
use hyper::Method;
use hyper::body::HttpBody;
use hyper::header::{ALLOW, ACCEPT_ENCODING};
use hyper::{Server, Version, StatusCode, header::{HeaderValue, CONNECTION}, Body};

use tokio::io::AsyncWriteExt;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use bytes::Bytes;
use std::collections::HashMap;
use parking_lot::RwLock;
use static_init::dynamic;
use std::sync::atomic::{AtomicUsize, Ordering};

use tokio::process::ChildStdin;
use ffmpeg::{FfmpegConfig, FfmpegSpawn, Ffmpeg};

use prex::{Request, Response, Next};


#[dynamic]
static CHANNELS: RwLock<HashMap<String, Channel>> = RwLock::new(HashMap::new());

static CHANNEL_COUNT: AtomicUsize = AtomicUsize::new(0);

// static BODY: &'static [u8] = &[0u8;1024 * 1024];  

#[derive(Debug, Clone)]
pub struct Channel {
    sender: Sender<Bytes>,
}

impl Channel {

    pub fn new() -> Self {
        // capacity must be power of 2
        let (sender, _) = channel(8);
        
        Self { sender }
    }

    pub fn sender(&self) -> &Sender<Bytes> {
        &self.sender
    }
}

impl Deref for Channel {
    
    type Target = Sender<Bytes>;
    
    fn deref(&self) -> &Self::Target {
        &self.sender
    }
}

impl DerefMut for Channel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sender
    }
}

pub fn start() -> impl Future<Output=()> {

    let addr = SocketAddr::from(([0, 0, 0, 0], 20500));

    let server = Server::try_bind(&addr).expect("hyper bind source");
        //.http1_half_close(true)
        //.http1_header_read_timeout(Duration::from_secs(5))
        //.http1_keepalive(false)
        //.http1_only(true)
        //.http1_title_case_headers(true)
        //.http1_preserve_header_case(false)
        //.tcp_sleep_on_accept_errors(true);

    info!("source server bound to {}", addr);

    let mut app = prex::prex();

    app.with(logger);
    //app.with(http_1_0_version);
    //app.with(connection_close);
    
    app.any("/:id/source", source_allow);
    app.any("/:id/source", source_accept);
    app.any("/:id/source", source);

    let app = app.build().expect("prex app build source");

    async move {
        server.serve(app).await.expect("hyper serve source");
    }
}

async fn logger(req: Request, next: Next) -> prex::Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    
    let res = next.run(req).await;
    
    let status = res.status();

    println!("{method} {path} => {status}");

    res
}

async fn _connection_close(req: Request, next: Next) -> prex::Response {
    let mut res = next.run(req).await;
    res.headers_mut().insert(CONNECTION, HeaderValue::from_static("close"));
    res
}

async fn _http_1_0_version(req: Request, next: Next) -> prex::Response {
    let mut res = next.run(req).await;
    *res.version_mut() = Version::HTTP_10;
    res
}

async fn source_allow(req: Request, next: Next) -> prex::Response {
    let mut res = next.run(req).await;
    res.headers_mut().entry(ALLOW).or_insert_with(|| HeaderValue::from_static("PUT, SOURCE"));
    res
}

async fn source_accept(req: Request, next: Next) -> prex::Response {
    let mut res = next.run(req).await;
    res.headers_mut().entry(ACCEPT_ENCODING).or_insert_with(|| HeaderValue::from_static("identity"));
    res
}

async fn source(req: Request, _next: Next) -> prex::Response {
    
    println!("source");

    enum SourceMethod {
        PUT,
        SOURCE,
    }

    let _method: SourceMethod = if req.method().eq(&Method::PUT) {
        SourceMethod::PUT
    } else if req.method().as_str().eq_ignore_ascii_case("SOURCE") {
        SourceMethod::SOURCE
    } else {
        let mut res = Response::new(StatusCode::METHOD_NOT_ALLOWED);
        *res.body_mut() = Body::from(format!("Method {} is not allowed, allowed methods are PUT or SOURCE", req.method().as_str()));
        return res;
    };

    // safety unwrap: param "id" is required in route defnition
    let id = req.param("id").unwrap();

    let channel = {
        let mut map = CHANNELS.write();
        
        if map.contains_key(id) {
            drop(map);
            // FORBIDEN (403) is used to communicate all sorts of errors
            let mut res = Response::new(StatusCode::FORBIDDEN);
            *res.body_mut() = Body::from("This source is already in use, try again later");
            return res;
        }

        let channel = Channel::new();

        CHANNEL_COUNT.fetch_add(1, Ordering::Relaxed);
        let _ = map.insert(id.to_string(), channel.clone());
        
        channel
    };
    
    let is_continue = match req.headers().get("expect") {
        None => false,
        Some(v) => {
            match v.to_str() {
                Err(_) => false,
                Ok(v) => v.trim().eq_ignore_ascii_case("100-continue"),
            }
        }
    };
    
    // need cloning here because of 'static requirements of future
    let id = id.to_string();

    let ffmpeg_config = FfmpegConfig {
        readrate: true,
        ..FfmpegConfig::default()
    };

    let ff_spawn  = match Ffmpeg::with_config(ffmpeg_config).spawn() {
        Err(_) => {
            // FORBIDEN (403) is used to communicate all sorts of errors
            let mut res = Response::new(StatusCode::INTERNAL_SERVER_ERROR);
            *res.body_mut() = Body::from("Error allocating internal stream converter, try again later or report it to the administrators");
            return res;
        },
        Ok(spawn) => spawn,
    };

    let FfmpegSpawn {
        stderr: _,
        stdin,
        stdout,
        child: _,
        config,
    } = ff_spawn;

    tokio::spawn(async move {
        use stream_util::*;
        use tokio_stream::StreamExt;
        
        let chunks = stdout.into_bytes_stream(STREAM_CHUNK_SIZE);
        
        tokio::pin!(chunks);

        loop {
            match chunks.next().await {
                None => {
                    println!("ffmpeg stdout end");
                    break
                },
                Some(Err(e)) => {
                    println!("ffmpeg stdout err: {e}");
                    break
                },
                Some(Ok(bytes)) => {
                    println!("ffmpeg stdout data");
                    // only fails if there are no receivers but that is ok
                    let _ = channel.sender().send(bytes);
                } 
            }
        }

        {
            // remove the channel to mark this source as available to stream again for another client
            let mut map = CHANNELS.write();
            
            map.remove(&id);
        }

        CHANNEL_COUNT.fetch_sub(1, Ordering::Relaxed);    
    });

    println!("is_continue: {is_continue}");

    if is_continue {
        // if Continue is expected we process the request and then send the response
        // hyper will send 100 Continue automatically when start reading from the body
        handle_request_body(req, stdin).await;
        let res = Response::new(StatusCode::OK);
        res
    } else {
        // if not expecting Continue we return the response right away
        // and spawn the handling of the incoming body
        tokio::spawn(handle_request_body(req, stdin));
        let res = Response::new(StatusCode::OK);
        res
    }
}

async fn handle_request_body(mut req: Request, mut stdin: ChildStdin) {
    
    println!("source body read start");

    loop {
        match req.data().await {
            
            None => {
                println!("source body end");
                break;
            }
            
            Some(Err(e)) => {
                println!("source body err: {e}");
                break;
            }

            Some(Ok(data)) => {
                println!("source body data");
                match stdin.write_all(data.as_ref()).await {
                    Ok(_) => continue,
                    Err(e) => {
                        println!("ffmpeg write err: {e}");
                        break;
                    },
                };
            }
        }
    }
}


pub fn subscribe(id: &str) -> Option<Receiver<Bytes>> {
    let map = CHANNELS.read();
    let channel = map.get(id)?;
    Some(channel.subscribe())
}
