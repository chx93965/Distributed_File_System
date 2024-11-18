use rocket::{
    get,
    tokio
};
mod heartbeat_manager;
mod chunk_manager;
use lib::shared::log_manager;
#[macro_use] extern crate rocket;

const CHUNKS_DIR: &str = "chunks";

#[rocket::main]
async fn main() {
    // Set the logging level and format
    log_manager::set_logging(
        &[
            log::Level::Info,
            //log::Level::Debug,
            log::Level::Warn,
            log::Level::Error,
        ]
    );

    // Initialize the chunk manager
    chunk_manager::ChunkManager::new(1024, CHUNKS_DIR.to_string());

    // Start the heartbeat manager in the background
    tokio::spawn(heartbeat_manager::heartbeat());

    // Launch the Rocket server
    let app = rocket::build().mount("/", routes![hello]);

    // Start the Rocket server
    app.launch().await.unwrap();

}

#[get("/")]
async fn hello() -> &'static str {
    "Hello, world! from Chunk Server\n"
}

