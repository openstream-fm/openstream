use std::net::SocketAddr;
use std::future::Future;
use hyper::HeaderMap;
use hyper::body::HttpBody;
use hyper::header::{ALLOW, ACCEPT_ENCODING, CONTENT_TYPE};
use hyper::{Method, Server, Version, StatusCode, header::{HeaderValue, CONNECTION}, Body};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use ffmpeg::{FfmpegConfig, FfmpegSpawn, Ffmpeg};
use prex::{Request, Response, Next};

pub fn start() -> impl Future<Output=()> {

    let addr = SocketAddr::from(([0, 0, 0, 0], 20600));

    let server = Server::try_bind(&addr).expect("hyper bind source")
        .http1_only(true)
        .http1_keepalive(false);
        //.http1_half_close(true)
        //.http1_header_read_timeout(Duration::from_secs(5))
        //.http1_keepalive(false)
        //.http1_only(true)
        //.http1_title_case_headers(true)
        //.http1_preserve_header_case(false)
        //.tcp_sleep_on_accept_errors(true);

    println!("hyper source server bound to {}", addr);

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

async fn source(mut req: Request, _next: Next) -> prex::Response {

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
    let id = req.param("id").unwrap().to_string();

    let channel = match channels::transmit(id.clone()) {
        None => {
            let mut res = Response::new(StatusCode::FORBIDDEN);
            *res.body_mut() = Body::from("This source is already in use, try again later");
            return res;
        }

        Some(tx) => tx
    };

    // need cloning here because of 'static requirements of future

    let ff_spawn  = match Ffmpeg::with_config(FfmpegConfig::default()).spawn() {
        Err(_) => {
            // FORBIDEN (403) is used to communicate all sorts of errors
            let mut res = Response::new(StatusCode::INTERNAL_SERVER_ERROR);
            *res.body_mut() = Body::from("Error allocating internal stream converter, try again later or report it to the administrators");
            return res;
        },
        Ok(spawn) => spawn,
    };

    let FfmpegSpawn {
        mut stderr,
        mut stdin,
        stdout,
        mut child,
        config,
    } = ff_spawn;

    let stderr_handler = {
        tokio::spawn(async move {
            let mut data = Vec::new();
            stderr.read_to_end( &mut data).await?;
            Result::<Vec<u8>, std::io::Error>::Ok(data)
        })
    };

    let _stdout_handler = {
        
        let id = id.clone();

        tokio::spawn(async move {
            use stream_util::*;
            use tokio_stream::StreamExt;
            
            let chunks = stdout.into_bytes_stream(16 * 1024).rated(config.kbitrate as usize / 8 * 1024);
            tokio::pin!(chunks);

            loop {
                match chunks.next().await {
                    None => {
                        println!("[source]: channel {id}: ffmpeg stdout end");
                        break
                    },
                    Some(Err(e)) => {
                        println!("[sorce]: channel {id}: ffmpeg stdout error: {e}");
                        break
                    },
                    Some(Ok(bytes)) => {
                        println!("[source]: channel {id}: ffmpeg stdout data: {} bytes", bytes.len());
                        // only fails if there are no receivers but that is ok
                        let _ = channel.send(bytes);
                    } 
                }
            }
        })
    };

    loop {
        match req.data().await {
            None => {
                println!("[source] channel {id}: recv body end");
                break;
            },

            Some(Err(e)) => {
                println!("[source] channel {id}: recv body error: {e}");
                break;
            },

            Some(Ok(data)) => {
                println!("[source] channel {id}: recv body data: {} bytes", data.len());
                
                match stdin.write_all(data.as_ref()).await {
                    Err(e) => {
                        println!("[source] channel {id} stdin error: {e}");
                        break;
                    },

                    Ok(()) => {
                        println!("[source] channel {id} stdin write: {} bytes", data.len());
                    }
                }
            }
        }
    };

    let exit = match child.wait().await {
        Err(e) => {
            println!("[source] channel {id}: ffmpeg child error: {e}");
            let mut headers = HeaderMap::with_capacity(1);
            headers.append(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        
            let body = Body::from("Unexpected error allocating the stream converter, please report this to the administrators");
            
            let mut res = Response::new(StatusCode::INTERNAL_SERVER_ERROR);
            *res.headers_mut() = headers;
            *res.body_mut() = body;    

            return res;
        }

        Ok(exit) => exit
    };

    println!("[source] channel {id}: ffmpeg child end: exit {exit}");
  
    if exit.success() {
        let mut res = Response::new(StatusCode::OK);
        *res.body_mut() = Body::from("bData streamed successfully");
        
        let mut headers = HeaderMap::with_capacity(1);
        headers.append(CONTENT_TYPE, HeaderValue::from_static("text/plain"));

        res 

    } else {
        let body = match stderr_handler.await {
           
            Err(_) => format!("Internal error allocating stream converter (stderr panic)"),
           
            Ok(r) => match r {
                Err(_) => format!("Internal error allocating stream converter (stderr error)"),
                Ok(v) => { 
                    let stderr_out = String::from_utf8_lossy(v.as_ref());
                    format!("Error converting the audio stream, possibly the audio is corrupted or is using a not supported format: {stderr_out}")
                }
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