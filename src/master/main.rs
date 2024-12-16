/*
*   The following is based on prof. Ashvin Goels Slides on GFS
*/
#![allow(unused)]
#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::io::Error;
use log::warn;
use rocket::{get, post, routes, State};
use rocket::futures::StreamExt;
use rocket::serde::{json::Json, Serialize, Deserialize};
use rocket::tokio::fs::OpenOptions;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::RwLock;
use uuid::Uuid;
use lib::shared::{log_manager, master_client_utils::ChunkInfo};
use lib::shared::master_client_utils::{DirectoryInfo, FileInfo, User};
use namespace_manager::{directory_create, directory_delete, list_directory,
                        file_create, file_read, file_write, file_delete};
use crate::namespace_manager::file_read_all;

mod namespace_manager;
mod chunk_manager;
mod safe_map;
mod heartbeat_manager;

const USER_INFO:&str = "users.json";
const DIR_FILE:&str = "dir.json";
const CHUNK_FILE:&str = "chunk.json";
const SERVER_FILE:&str = "server.json";

struct UserDatabase {
    users: RwLock<HashMap<String, String>>,
}

impl UserDatabase {
    async fn new() -> Self {
        let users = RwLock::new(HashMap::new());

        let file = OpenOptions::new()
            .read(true).write(true).create(true)
            .open(USER_INFO).await.unwrap();

        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        while let Some(line_result) = lines.next_line().await.unwrap() {
            let user: User = serde_json::from_str(&line_result).unwrap();
            users.write().await.insert(
                user.username.clone(),
                user.password.clone());
        }
        Self { users }
    }

    async fn save(&self, user: User) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .append(true).create(true)
            .open(USER_INFO).await.unwrap();

        let data = serde_json::to_string(&user)?;
        file.write_all(data.as_bytes()).await?;
        file.write_all(b"\n").await?;
        Ok(())
    }
}

async fn persistent_storage_init() {
    let file = OpenOptions::new()
        .read(true).write(true).create(true)
        .open(DIR_FILE).await.unwrap();
    let reader = BufReader::new(file);

    let file = OpenOptions::new()
        .read(true).write(true).create(true)
        .open(CHUNK_FILE).await.unwrap();

    let file = OpenOptions::new()
        .read(true).write(true).create(true)
        .open(SERVER_FILE).await.unwrap();
}

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


#[rocket::main]
async fn main() {
    log_manager::set_logging(&[
        //log::Level::Info,
        //log::Level::Debug,
        log::Level::Warn,
        log::Level::Error,
    ]);

    persistent_storage_init().await;

    namespace_manager::namespace_manager_init();
    chunk_manager::chunk_manager_init();
    // heartbeat_manager::heartbeat_manager_init();

    let user_db = UserDatabase::new().await;
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
        .manage(user_db)
        .mount("/", routes![
            register,
            login,
            create_file,
            read_file,
            read_all_file,
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


#[post("/file/create?<path>")]
async fn create_file(path:String) -> Result<Json<FileInfo>, Error> {
    match file_create(path) {
        Ok(file) => Ok(Json(file)),
        Err(e) => Err(e)
    }
}

#[get("/file/read?<path>")]
async fn read_file(path:String) -> Json<Vec<ChunkInfo>>{
    let chunks = file_read(path).unwrap();
    Json(ChunkInfo::serialize(chunks))
    // TODO: error handling for non-existent chunk index
}

#[get("/file/read/all?<path>")]
async fn read_all_file(path:String) -> Json<Vec<ChunkInfo>>{
    let chunks = file_read_all(path).unwrap();
    Json(ChunkInfo::serialize(chunks))
}

#[post("/file/update?<path>&<size>")]
async fn update_file(path:String, size:usize) -> Json<Vec<ChunkInfo>>{
    let chunks = file_write(path, size).unwrap();
    Json(ChunkInfo::serialize(chunks))
}

#[get("/file/delete?<path>")]
async fn delete_file(path:String) -> Result<(), Error> {
    let result = file_delete(path);
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::new(std::io::ErrorKind::NotFound, e))
    }
}

#[post("/dir/create?<path>")]
async fn create_directory(path:String) -> Result<String, Error> {
    println!("{}", path);
    let result = directory_create(path);
    Ok(result)
}

#[get("/dir/read?<path>")]
async fn read_directory(path:String) -> Result<Json<DirectoryInfo>, Error> {
    match list_directory(path) {
        Ok(dir) => Ok(Json(dir)),
        Err(e) => Err(e)
    }
}

#[post("/dir/delete?<path>")]
async fn delete_directory(path:String) -> Result<(), Error> {
    let result = directory_delete(path);
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::new(std::io::ErrorKind::NotFound, e))
    }
}

#[post("/user/register", data = "<user>")]
async fn register(user:Json<User>, user_db: &State<UserDatabase>) -> Result<(), Error> {
    let user = user.into_inner();
    let mut users_lock = user_db.users.write().await;

    // check if user already exists
    if users_lock.contains_key(&user.username) {
        return Err(Error::new(std::io::ErrorKind::AlreadyExists, "User already exists"));
    }
    users_lock.insert(user.username.clone(), user.password.clone());

    // write to persistent storage
    user_db.save(user).await?;
    Ok(())
}

#[post("/user/login", data = "<user>")]
async fn login(user:Json<User>, user_db: &State<UserDatabase>) -> Result<(), Error> {
    let user = user.into_inner();
    let users_lock = user_db.users.read().await;

    // check if user exists
    if !users_lock.contains_key(&user.username) {
        return Err(Error::new(std::io::ErrorKind::NotFound, "User not found"));
    }

    // check if password matches
    if users_lock.get(&user.username).unwrap() != &user.password {
        return Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid password"));
    }
    Ok(())
}


// fn update_namespace(){
//
// }

/*
*   Heartbeat received from chunkservers.
*   Called by chunkservers with : Chunkserver ID + Chunks + Chunk Info
*   Input : 
*   Output : Chunk Location - Send Data to Chunk
*/
#[post("/heartbeat", format = "json", data = "<metadata>")]
async fn chunkserver_heartbeat(metadata: Json<heartbeat_manager::Metadata>) -> Result<(), Error> {
    debug!("{:?}", metadata);
    debug!("Received heartbeat from chunkserver id: {}", metadata.chunkserver_id);
    heartbeat_manager::receive_heartbeat(metadata).await;
    Ok(())
}