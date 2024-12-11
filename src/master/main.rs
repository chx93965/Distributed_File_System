/*
*   The following is based on prof. Ashvin Goels Slides on GFS
*/
#![allow(unused)]
#[macro_use]
extern crate rocket;
use std::io::Error;
use rocket::{get, post, routes, State};
use rocket::serde::{json::Json, Serialize, Deserialize};
use uuid::Uuid;
use lib::shared::log_manager;
use namespace_manager::{directory_create, file_create, list_directory};

mod namespace_manager;
mod chunk_manager;
mod safe_map;
mod heartbeat_manager;

/*
*   Maintains filesystem's metadata in memory :
*       1. Chunk namespace, i.e., all chunk handles in the system
*           --> For each chunk : 
*                   I. Reference Count
*                   II. Version Number
*       
*       2.  File namespace, i.e., all file paths
*           --> For each file path:
*                   I. ACL
*                   II. File -> chunk_handle mappings
*
*   All the metatdata changes are logged to disk for persistency
*   Log is replicated to backup master nodes
*   Memory Structures changes are periodically checkpointed
*
*   Manage chunks and replicas :
*       1. Creates new chunks on chunkservers
*       2. Track chunk replicas by caching chunk locations, i.e., 
*           chunkservers on which a chunk is stored
*       3. Makes chunk replica placement decisions
*
*   Has "per-filepath" read-write locks : 
*       Example : 
*                To modify /a/b/c, 
*                acquire read locks on /a, /a/b, 
*                write lock on /a/b/c
* 
*   Communication with Chunkserver : 
*       HeartBeat messages :
*           Find chunk locations
*           Do lease management (Primary for a chunk)
*           Find stale chunk servers
*           Garbage collect orphaned and stale chunks
*
*/

#[derive(Serialize, Deserialize)]
struct ChunkInfo {
    uuid: String,
    content: String,
}

#[rocket::main]
async fn main() {
    log_manager::set_logging(&[
        log::Level::Info,
        log::Level::Debug,
        log::Level::Warn,
        log::Level::Error,
    ]);

    namespace_manager::namespace_manager_init();
    chunk_manager::chunk_manager_init();
    heartbeat_manager::heartbeat_manager_init();
    /*
    *   Input  : Get (file name, chunk index) from client
    *   Output : Ret (chunk handle, chunk locations) to client
    *   Get chunkserver state from chunkservers
    *   Ret Instruction to chunkservers
    */
    let config = rocket::Config {
        port: 8000,
        ..Default::default()
    };

    let app = rocket::build()
        .configure(config)
        .mount("/", routes![
            create_file,
            read_file,
            update_file,
            delete_file,
            create_directory,
            read_directory,
            delete_directory,
            chunkserver_heartbeat
        ]);
    app.launch().await.unwrap();
}


/*
*   All done by namespace manager : 
*       1. Lookup File Directory 
*       2. Acquire Directory Lock
*       3. Check Permissions
*       4. Delete File Entry
*       5. Release Directory Lock
*/

fn serialize_file(file: Vec<(Uuid, String)>) -> Vec<ChunkInfo>{
    let mut chunks = Vec::new();
    for (uuid, content) in file{
        chunks.push(ChunkInfo{uuid: uuid.to_string(), content: content});
    }
    chunks
}

#[post("/file/create?<path>")]
async fn create_file(path:String) -> Result<(), Error> {
    let chunks = namespace_manager::file_create(path);
    Ok(())
}

#[get("/file/read?<path>&<chunk>")]
async fn read_file(path:String, chunk:usize) -> Json<Vec<ChunkInfo>>{
    let chunks = namespace_manager::file_read(path, chunk).unwrap();
    Json(serialize_file(chunks))
    // TODO: error handling for non-existent chunk index
}

#[post("/file/update?<path>&<size>")]
async fn update_file(path:String, size:usize) -> Json<Vec<ChunkInfo>>{
    let chunks = namespace_manager::file_write(path, size).unwrap();
    Json(serialize_file(chunks))
}

#[get("/file/delete?<path>")]
async fn delete_file(path:String) -> Result<(), Error> {
    namespace_manager::file_delete(path);
    Ok(())
}


#[post("/dir/create?<path>")]
async fn create_directory(path:String) -> Result<(), Error> {
    println!("{}", path);
    namespace_manager::directory_create(path);
    Ok(())
}

#[get("/dir/read?<path>")]
async fn read_directory(path:String) -> Result<(), Error> {
    namespace_manager::list_directory(path);
    Ok(())
}

#[post("/dir/delete?<path>")]
async fn delete_directory(path:String) -> Result<(), Error> {
    namespace_manager::directory_delete(path);
    Ok(())
}


fn update_namespace(){

}

/*
*   Heartbeat received from chunkservers.
*   Called by chunkservers with : Chunkserver ID + Chunks + Chunk Info
*   Input : 
*   Output : Chunk Location - Send Data to Chunk
*/
#[post("/heartbeat", format = "json", data = "<metadata>")]
async fn chunkserver_heartbeat(metadata: Json<heartbeat_manager::Metadata>) -> Result<(), Error> {
    // debug!("{:?}", metadata);
    heartbeat_manager::receive_heartbeat(metadata);
    Ok(())
}