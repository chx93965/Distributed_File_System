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
use namespace_manager::{directory_create, file_create, list_directory};
use heartbeat::{Disk, Metadata};

mod namespace_manager;
mod chunk_manager;
mod safe_map;
mod heartbeat_manager;
#[path = "../shared/heartbeat.rs"] mod heartbeat;
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

#[launch]
fn rocket() -> _ {
    namespace_manager::namespace_manager_init();
    chunk_manager::chunk_manager_init();
    heartbeat_manager::heartbeat_manager_init();
    /*
    *   Input  : Get (file name, chunk index) from client
    *   Output : Ret (chunk handle, chunk locations) to client
    */

    /*
    *   Get chunkserver state from chunkservers
    *   Ret Instruction to chunkservers
    */
    rocket::build()
        .mount("/", routes![file_read, file_write, direcotry_create, receive_heartbeat])
}


/*
*   All done by namespace manager : 
*       1. Lookup File Directory 
*       2. Acquire Directory Lock
*       3. Check Permissions
*       4. Delete File Entry
*       5. Release Directory Lock
*/
fn file_delete(){
    
}

/*
*   All done by namespace manager : 
*       1. Lookup File Directory 
*       2. Acquire Directory Lock
*       3. Check Permissions
*       4. Release Directory Lock
*/

fn serialize_file(file: Vec<(Uuid, String)>) -> Vec<ChunkInfo>{
    let mut chunks = Vec::new();
    for (uuid, content) in file{
        chunks.push(ChunkInfo{uuid: uuid.to_string(), content: content});
    }
    chunks
}

#[get("/file_read/<file_name>/<chunk_index>")]
async fn file_read(file_name:String, chunk_index:usize) -> Json<Vec<ChunkInfo>>{
    // namespace_manager::file_read(file_name, chunk_index).unwrap()
    let chunks = namespace_manager::file_read(file_name, chunk_index).unwrap();
    Json(serialize_file(chunks))
}

/*
*   All done by namespace manager : 
*       1. Lookup File Directory 
*       2. Acquire Directory Lock
*       3. Check Permissions
*       4. Release Directory Lock
*/
#[post("/file_write/<file_name>/<size>")]
async fn file_write(file_name:String, size:usize) -> Json<Vec<ChunkInfo>>{
    // namespace_manager::file_write(file_name, size).unwrap()
    let chunks = namespace_manager::file_write(file_name, size).unwrap();
    Json(serialize_file(chunks))
}


/*
*   All done by namespace manager : 
*       1. Lookup Parent Directory 
*       2. Acquire Parent Lock
*       3. Check Permissions
*       4. Create Directory
*       5. Release Parent Lock
*/
#[post("/directory/<path>")]
async fn direcotry_create(path:String) -> Result<(), Error> {
    namespace_manager::directory_create(path);
    Ok(())
}

/*
*   All done by namespace manager : 
*       1. Lookup Parent Directory 
*       2. Acquire Parent Lock
*       3. Check Permissions
*       4. Delete Directory
*       5. Release Parent Lock
*/
fn directory_delete(){
    
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
async fn receive_heartbeat(metadata: Json<Metadata>) -> Result<(), Error> {
    heartbeat_manager::receive_heartbeat(metadata);
    Ok(())
}