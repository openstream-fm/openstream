// use std::{collections::HashMap, sync::atomic::AtomicUsize, net::SocketAddr, io::BufRead};
// use static_init::dynamic;
// use tokio::{sync::broadcast::{channel, Receiver, Sender}, io::{BufReader, AsyncBufReadExt, AsyncReadExt}};
// use std::sync::atomic::Ordering;

use std::net::SocketAddr;
use std::time::Duration;
use std::future::Future;

use hyper::{Server, Version, header::{HeaderValue, CONNECTION}};


/*
#[dynamic]
static CHANNELS: RwLock<HashMap<String, Sender<Bytes>>> = RwLock::new(HashMap::new());
static CHANNEL_COUNT: AtomicUsize = AtomicUsize::new(0);
static BYTES_READED: AtomicUsize = AtomicUsize::new(0);
static LAGGED: AtomicUsize = AtomicUsize::new(0);
*/

pub fn start() -> impl Future<Output=()> {

    let addr = SocketAddr::from(([127, 0, 0, 1], 20200));

    let server = Server::try_bind(&addr).expect("hyper bind")
        .http1_half_close(true)
        .http1_header_read_timeout(Duration::from_secs(5))
        .http1_keepalive(false)
        .http1_only(true)
        .http1_title_case_headers(true)
        .http1_preserve_header_case(false)
        .tcp_sleep_on_accept_errors(true);

    println!("source server bound to {}", addr);

    let mut app = prex::prex();

    app.with(http_1_0_version);
    app.with(connection_close);

    app.with(|_, _| async { format!("hello") });

    let app = app.build().expect("prex app build");

    async move {
        server.serve(app).await.expect("hyper serve");
    }
}

async fn connection_close(req: prex::Request, next: prex::Next) -> prex::Response {
    let mut res = next.run(req).await;
    res.headers_mut().insert(CONNECTION, HeaderValue::from_static("close"));
    res
}

async fn http_1_0_version(req: prex::Request, next: prex::Next) -> prex::Response {
    let mut res = next.run(req).await;
    *res.version_mut() = Version::HTTP_10;
    res
}