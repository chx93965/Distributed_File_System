use rocket::{
    data::{ByteUnit, Data},
    get,
    http::Status,
    response::status,
    serde::json::Json,
    tokio::{self, io::AsyncReadExt, sync::Mutex},
    State,
};
use std::{
    sync::Arc,
    net::TcpListener,
    io::ErrorKind,
};

mod chunk_manager;
mod heartbeat_manager;
use lib::shared::log_manager;
use uuid::Uuid;
#[macro_use]
extern crate rocket;

const CHUNKS_DIR: &str = "chunks";
const CHUNK_SIZE_MAX: usize = 1024 * 1024 * 256; // 256 MB
const START_PORT: u16 = 8000;
const MAX_PORT: u16 = 8100;

type SharedChunkManager = Arc<Mutex<chunk_manager::ChunkManager>>;

#[rocket::main]
async fn main() {
    // Set the logging level and format
    log_manager::set_logging(&[
        log::Level::Info,
        log::Level::Debug,
        log::Level::Warn,
        log::Level::Error,
    ]);

    // Initialize the chunk manager
    let chunk_manager = Arc::new(Mutex::new(chunk_manager::ChunkManager::new(
        1024,
        CHUNKS_DIR.to_string(),
    )));

    // Start the heartbeat manager in the background
    tokio::spawn(heartbeat_manager::heartbeat());

    // Try to find an available port
    let port = find_available_port(START_PORT, MAX_PORT);

    // If no port was found, panic
    if port.is_none() {
        panic!("No available port found between {} and {}", START_PORT, MAX_PORT);
    }

    // Configure rocket config
    let config = rocket::Config {
        port: port.unwrap(),
        ..Default::default()
    };
    
    let app = rocket::build()
        .configure(config)
        .manage(chunk_manager)
        .register("/", catchers![bad_request, not_found, payload_too_large])
        .mount("/", routes![hello])
        .mount("/", routes![add_chunk])
        .mount("/", routes![get_chunk])
        .mount("/", routes![append_chunk])
        .mount("/", routes![get_chunk_list]);

    // Start the Rocket server
    app.launch().await.unwrap();
}

/// Attempts to find an available port between `start` and `end`.
/// Returns the first available port, or `None` if no port is available.
fn find_available_port(start: u16, end: u16) -> Option<u16> {
    for port in start..=end {
        if is_port_available(port) {
            return Some(port);
        }
    }
    None
}

/// Checks if a given port is available by attempting to bind to it.
fn is_port_available(port: u16) -> bool {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr);
    // unbind the listener
    match listener {
        Ok(_) => true,  // Port is available
        Err(e) if e.kind() == ErrorKind::AddrInUse => false,  // Port is in use
        Err(_) => false,  // Other errors
    }
}

#[get("/")]
async fn hello() -> &'static str {
    "Hello, world! from Chunk Server\n"
}

///
/// Adds a chunk to the chunk manager. This endpoint expects a POST request with binary data
/// as the body of the request and allows specifying a UUID to associate with the chunk.
///
/// ## Parameters
/// - `id`: A UUID string passed as a query parameter or within the request body. This UUID is
///   used to uniquely identify the chunk.
/// - `data`: The binary data to be stored in the chunk. It is expected to be in the request body
///   as raw binary data.
///
/// ## Request Examples
///
/// ```bash
/// curl -X POST "http://127.0.0.1:8000/add_chunk?id=<UUID>" \
///      -H "Content-Type: application/octet-stream" \
///      --data-binary @example.bin
/// ```
/// In this case, the UUID is passed as part of the query parameter (`id=<UUID>`), and the binary data
/// for the chunk is passed as the body of the request (using the `--data-binary` flag in `curl`).
///
/// ## Example Usage
/// - The client sends a chunk of data, associating it with a specific UUID for later retrieval.
/// - The chunk data can be stored in a persistent storage or a temporary in-memory store for later use.
///
/// ## Error Handling
/// - If the UUID provided is invalid or improperly formatted, the server responds with a `400 BadRequest` error.
/// - If the binary data exceeds the allowed limit, the server responds with a `413 Payload Too Large` error.
#[post("/add_chunk?<id>", data = "<data>")]
async fn add_chunk(
    state: &State<SharedChunkManager>,
    id: String, // UUID as a query parameter
    data: Data<'_>,
) -> Result<status::Created<&'static str>, Status> {
    let mut chunk_manager = state.lock().await;

    // Parse the UUID from the query parameter
    let id = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => {
            error!("Invalid UUID provided");
            return Err(Status::BadRequest);
        }
    };

    // Read binary data from the HTTP body
    let mut buffer = Vec::new();
    let limit = ByteUnit::Byte(CHUNK_SIZE_MAX as u64);
    let mut stream = data.open(limit);

    if let Err(e) = stream.read_to_end(&mut buffer).await {
        error!("Failed to read data from request: {}", e);
        return Err(Status::PayloadTooLarge);
    }

    // Add the chunk to the ChunkManager
    chunk_manager.add_chunk(buffer, id);

    // Log the addition and respond with success
    log::info!("Chunk added with ID: {}", id);
    Ok(status::Created::new("/").body("Chunk added\n"))
}

#[post("/append_chunk?<id>", data = "<data>")]
async fn append_chunk(
    state: &State<SharedChunkManager>,
    id: String, // UUID as a query parameter
    data: Data<'_>,
) -> Result<status::Created<&'static str>, Status> {
    let mut chunk_manager = state.lock().await;

    // Parse the UUID from the query parameter
    let id = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => {
            error!("Invalid UUID provided");
            return Err(Status::BadRequest);
        }
    };

    // Read binary data from the HTTP body
    let mut buffer = Vec::new();
    let limit = ByteUnit::Byte(CHUNK_SIZE_MAX as u64);
    let mut stream = data.open(limit);

    if let Err(e) = stream.read_to_end(&mut buffer).await {
        error!("Failed to read data from request: {}", e);
        return Err(Status::PayloadTooLarge);
    }

    // Append the chunk to the ChunkManager
    chunk_manager.append_chunk(buffer, id);

    // Log the addition and respond with success
    log::info!("Chunk appended with ID: {}", id);
    Ok(status::Created::new("/").body("Chunk appended\n"))
}

///
/// Retrieves a chunk of data from the ChunkManager by its UUID.
///
/// This endpoint accepts a GET request with the UUID of the desired chunk as a query parameter.
/// The UUID is used to look up the chunk in the ChunkManager, and if found, the chunk data
/// is returned as the response body. If the UUID is invalid or the chunk is not found,
/// an appropriate HTTP status is returned.
///
/// ## Parameters
/// - `id`: A UUID string passed as a query parameter (`id=<UUID>`), which uniquely identifies
///   the chunk to retrieve.
///
/// ## Returns
/// - A `Vec<u8>` representing the binary data of the chunk, if found.
/// - HTTP status `400 BadRequest` if the UUID is invalid.
/// - HTTP status `404 NotFound` if the chunk with the given UUID is not found.
///
/// ## Example Usage
/// ```bash
/// curl -X GET "http://127.0.0.1:8000/get_chunk?id=<UUID>" --output chunk_output.bin
/// ```
/// This command retrieves the chunk associated with the given UUID and stores it as `chunk_output.bin`
/// on the local machine.
///
/// ## Error Handling
/// - If the UUID is invalid or improperly formatted, the server responds with a `400 BadRequest` error.
/// - If the chunk with the provided UUID is not found, the server responds with a `404 NotFound` error.
///
#[get("/get_chunk?<id>")]
async fn get_chunk(
    state: &State<SharedChunkManager>,
    id: String, // UUID as a query parameter
) -> Result<Vec<u8>, Status> {
    let chunk_manager = state.lock().await;

    println!("get_chunk: id={}", id);
    // Parse the UUID from the query parameter
    let id = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => {
            error!("Invalid UUID provided");
            return Err(Status::BadRequest);
        }
    };

    // Retrieve the chunk from the ChunkManager
    let chunk = match chunk_manager.get_chunk(id) {
        Ok(chunk) => chunk,
        Err(_) => {
            error!("Chunk not found");
            return Err(Status::NotFound);
        }
    };

    // Log the retrieval and respond with the chunk data
    log::info!("Chunk retrieved with ID: {}", id);
    Ok(chunk)
}

#[get("/get_chunk_list")]
async fn get_chunk_list(state: &State<SharedChunkManager>) -> Json<Vec<String>> {
    let chunk_manager = state.lock().await;
    let chunk_list = chunk_manager.get_chunk_list();
    let string_list: Vec<String> = chunk_list.iter().map(|id| id.to_string()).collect();
    Json(string_list)
}

#[catch(400)]
fn bad_request() -> &'static str {
    "400 Bad Request\n"
}

#[catch(404)]
fn not_found() -> &'static str {
    "404 Not Found\n"
}

#[catch(413)]
fn payload_too_large() -> &'static str {
    "413 Payload Too Large\n"
}
