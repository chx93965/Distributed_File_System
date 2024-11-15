use std::{io::Write, net::SocketAddr};

use axum::{
    routing::{get},
    Router,
};

use log::{debug, error, info};
use env_logger::{Builder, Env};

#[tokio::main]
async fn main() {
    Builder::from_env(Env::default().default_filter_or("trace")) // Set to a wide range initially
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

    let app: Router = Router::new().route("/", get(hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    info!("Listening on {}", addr);
    debug!("This is a debug message");
    error!("This is an error message");
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> &'static str {
    "Hello, world! from Chunk Server\n"
}
