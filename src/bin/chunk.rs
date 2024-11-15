use env_logger::{Builder, Env};
use log::{debug, error, info};
use rocket::{get, routes, tokio::net::TcpListener, Build, Rocket};
use std::io::Write;

#[macro_use] extern crate rocket;

fn set_logging() {
    Builder::from_env(Env::default().default_filter_or("trace"))
        .format(|buf, record| {
            // Set the logging filter level:
            // 5 LevelFilter::Trace - Very detailed, often used for debugging fine details.
            // 4 LevelFilter::Debug - General debugging information.
            // 3 LevelFilter::Info  - High-level application progress messages.
            // 2 LevelFilter::Warn  - Indicates potential issues or warnings.
            // 1 LevelFilter::Error - Only logs error messages.
            // 0 LevelFilter::Off   - Disables all logging.
            if record.level() == log::Level::Info || record.level() == log::Level::Error {
                writeln!(
                    buf,
                    "{} [{}] - {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    record.args()
                )
            } else {
                Ok(()) // Skip other levels
            }
        })
        .init();
}

#[launch]
fn rocket() -> _ {
    set_logging();

    rocket::build()
        .mount("/", routes![hello])
}

#[get("/")]
async fn hello() -> &'static str {
    "Hello, world! from Chunk Server\n"
}
