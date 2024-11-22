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

use crate::safe_map::SafeMap;
use chrono::{DateTime, Utc};
use rand::prelude::*;
use rand::seq::SliceRandom;
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
    /*
     *      Do initialization
     */

    SERVER_MAP.init();
    CHUNK_MAP.init();

    SERVER_MAP.insert("localhost".to_string(), Vec::new());
    SERVER_MAP.insert("host1".to_string(), Vec::new());
    SERVER_MAP.insert("host2".to_string(), Vec::new());
    SERVER_MAP.insert("host3".to_string(), Vec::new());
    SERVER_MAP.insert("host4".to_string(), Vec::new());
}

/*
*   Function that returns the best chunks to hold the
*   data
*/
pub fn write_chunks(size: i64) -> Vec<(Uuid, String)> {
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