use rocket::tokio::time::{sleep, Duration};


pub async fn heartbeat() {
    info!("Starting Chunkserver heartbeat...");
    let interval = Duration::from_secs(2);

    loop {
        info!("Chunkserver heartbeat...");
        sleep(interval).await;
    }
}