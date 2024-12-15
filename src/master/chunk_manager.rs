/*
*   Manages and allocates chunks and chunk handles
*   also manages leases.
*
*
*
*   Data Structures :
*       1.  Chunk Map           : Map (Chunk Handle -> Chunk Info)
*       2.  Chunk Info {
*               Version         : uint
*               Locations       : List<Server Locations (IP String)>
*               Size            : int
*               Last Modified   : DateTime
*               Valid Lease     : bool
*               Primary Server  : Server Location (IP String)
*           }
*       3. Chunkserver Map      : Map (IP (String) -> List<Uuid>)
*
*/
#![allow(unused)]

use crate::safe_map::SafeMap;
use chrono::{DateTime, Utc};
use rand::prelude::*;
use rand::seq::SliceRandom;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use uuid::Uuid;

static CHUNK_MAP: SafeMap<Uuid, String> = SafeMap::new();
static SERVER_MAP: SafeMap<String, Vec<Uuid>> = SafeMap::new();

// pub struct ChunkInfo {
//     version: u16,
//     locations: Vec<String>,
//     size: i64,
//     last_modified: DateTime<Utc>,
//     primary_server: String,
// }

// impl ChunkInfo {
//     pub fn new(locations : Vec<String>, ) -> Self{

//     }
// }

pub fn chunk_manager_init() {
    SERVER_MAP.init();
    CHUNK_MAP.init();
    load_chunk_map();
    load_server_map();
    /*
     *  Some dummy servers init
     */
    // SERVER_MAP.insert("localhost".to_string(), Vec::new());
    // SERVER_MAP.insert("host1".to_string(), Vec::new());
    // SERVER_MAP.insert("host2".to_string(), Vec::new());
    // SERVER_MAP.insert("host3".to_string(), Vec::new());
    // SERVER_MAP.insert("host4".to_string(), Vec::new());
}

// TODO: pass CHUNK_FILE from main
pub async fn save_chunk_map() {
    let mut file = OpenOptions::new()
        .write(true).create(true)
        .open("chunk.json").await.unwrap();

    for (uuid, id) in CHUNK_MAP.to_map().iter() {
        file.write_all(format!("{},{}\n", uuid.to_string(), id)
            .as_bytes()).await.unwrap();
    }
}

pub async fn load_chunk_map() {
    let mut file = OpenOptions::new()
        .read(true).create(true)
        .open("chunk.json").await.unwrap();

    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line_result) = lines.next_line().await.unwrap() {
        // stores as <uuid>,<string> at each line
        let mut parts = line_result.split(",");
        let uuid = Uuid::parse_str(parts.next().unwrap()).unwrap();
        CHUNK_MAP.insert(uuid, parts.next().unwrap().to_string());
    }
}

// TODO: pass SERVER_FILE from main
pub async fn save_server_map() {
    let mut file = OpenOptions::new()
        .write(true).create(true)
        .open("server.json").await.unwrap();

    for (server, uuids) in SERVER_MAP.to_map().iter() {
        let mut line = format!("{},", server);
        uuids.iter().for_each(|uuid| {
            line.push_str(&uuid.to_string());
            line.push(',');
        });
        file.write_all(line.as_bytes()).await.unwrap();
    }
}

pub async fn load_server_map() {
    let mut file = OpenOptions::new()
        .read(true).create(true)
        .open("server.json").await.unwrap();

    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line_result) = lines.next_line().await.unwrap() {
        // stores as <string>,<uuid>,<uuid>,... at each line
        let mut parts = line_result.split(",");
        let server = parts.next().unwrap().to_string();
        let mut uuids = Vec::new();
        for part in parts {
            uuids.push(Uuid::parse_str(part).unwrap());
        }
        SERVER_MAP.insert(server, uuids);
    }
}

/*
*   Function that returns the best chunks to hold the
*   data
*/
pub fn write_chunks(_size: usize) -> Vec<(Uuid, String)> {
    /*
     *  1. Find best chunkservers (At the moment we do it randomly)
     *  2. Generate chunk handles
     *  3. Send chunk handles to chunkservers
     *  4. Assign chunk informations
     *  5. Return chunk handles + chunk locations
     */
    let mut thingy: Vec<(Uuid, String)> = Vec::new(); // Initialize as mutable
    let all_keys = SERVER_MAP.keys(); // Collect keys into a Vec first

    // Sample 3 random keys
    let sample = all_keys
        .choose_multiple(&mut rand::thread_rng(), 3)
        .collect::<Vec<_>>();

    // Generate 3 UUIDs
    let uuids: Vec<Uuid> = (0..3).map(|_| Uuid::new_v4()).collect();

    // Zip sample and UUIDs together and process them
    for (uuid, key) in uuids.iter().zip(sample.iter()) {
        if let Some(server) = SERVER_MAP.get(key) {
            if let Ok(mut server_write) = server.write() {
                server_write.push(*uuid); // Assuming join takes a UUID
                thingy.push((*uuid, key.to_string())); // Store the UUID and key
            }
        }
    }

    for thing in thingy.clone() {
        CHUNK_MAP.insert(thing.0, thing.1);
    }

    save_chunk_map();
    save_server_map();

    /*
     *  TODO : Send the update to the actual chunkservers
     */
    return thingy;
}

pub fn get_chunks(chunk_handles : Vec<Uuid>) -> Vec<(Uuid, String)>{
    let mut tuples: Vec<(Uuid, String)> = Vec::new();
    for uuid in chunk_handles {
        if let Some(id) = CHUNK_MAP.get(&uuid) {
            if let Ok(id_read) = id.read() {
                tuples.push((uuid, id_read.to_string()));
            }
        }
    }
    return tuples;
}