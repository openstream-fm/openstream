use tokio::runtime;

fn main() {

    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .build().expect("tokio runtime build");

    rt.block_on(async_main());
}


async fn async_main() {
    source::start().await;
}