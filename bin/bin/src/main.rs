use std::net::SocketAddr;

use tokio::runtime;

fn main() {

    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .build().expect("tokio runtime build");

    rt.block_on(async_main());
}


async fn async_main() {

    let source_addr = SocketAddr::from(([0, 0, 0, 0], 20500));
    let handle = tokio::spawn(source::server::start(source_addr));

    tokio::select! {
        r = handle => r.expect("source panicked").expect("source errored"),
        //r = handle2 => r.expect("stream panicked")
    };
}