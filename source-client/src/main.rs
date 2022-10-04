use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::time::{Duration, interval, Instant};

static BYTES_TRANSFERED: AtomicUsize = AtomicUsize::new(0);
static ERRORS: AtomicUsize = AtomicUsize::new(0);
static CURRENT_CLIENTS: AtomicUsize = AtomicUsize::new(0);
static HSTORIC_CLIENTS: AtomicUsize = AtomicUsize::new(0);

#[tokio::main]
async fn main() {
    /*
    for id in 0..1000 {
        let id = id.to_string();
        create_client(id);
    }
    */

    stats_logger().await;
}

fn create_client(id: String) {}

fn human_time(dur: Duration) -> String {
    
    const S: usize = 1;
    const M: usize = S * 60;
    const H: usize = M * 60;
    const D: usize = H * 24;
    
    let dur: usize = dur.as_secs() as f64 as usize;
    
    let days = dur / D;
    let hours = (dur % D) / H;  
    let mins = (dur % H) / M;  
    let secs = (dur % M) / S;  

    if days != 0 {
        format!("{days} d, {hours} h, {mins} m, {secs} s")
    } else if hours != 0 {
        format!("{hours} h, {mins} m, {secs} s")
    } else if mins != 0 {
        format!("{mins} m, {secs} s")
    } else {
        format!("{secs} s")
    }
}


async fn stats_logger() {
    
    let start = Instant::now();
    
    let mut interval = interval(Duration::from_millis(1000));

    loop {
        // first tick is instantaneous
        interval.tick().await;
        log(start.elapsed());
    }
}

fn log(dur: Duration) {
    println!("uptime: {}", human_time(dur));
}