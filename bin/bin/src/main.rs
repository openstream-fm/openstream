#[tokio::main]
async fn main() {

    let handle1 = tokio::spawn(source::server::start(([0, 0, 0, 0], 20500)));
    //let handle1 = tokio::spawn(source::start());
    let handle2 = tokio::spawn(stream::start());

    tokio::select! {
        r = handle1 => r.expect("source panicked").expect("source errored"),
        //r = handle1 => r.expect("source panicked"),
        r = handle2 => r.expect("stream panicked"),
    };
}