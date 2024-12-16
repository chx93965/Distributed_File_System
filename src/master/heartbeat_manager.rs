/*
*   Manages Chunkserver heartbeats.
*   Receive and parse the Heartbeats.
*   Alert and update master.
*
*
*
*   Data Structures : 
*       1. Server Status        : Map (Server Id -> Server Status)
*
*   Functions : 
*       1. Update Server Status
*       2. Receive heartbeats 
*/
#![allow(unused)]

use crate::safe_map::SafeMap;
use std::collections::HashMap;
use std::time::SystemTime;
use std::time::Duration;
use sysinfo::{Disks, System};
use reqwest::{Error, Client};
use rocket::serde::json::Json;
pub use lib::shared::master_chunk_utils::{Disk, Metadata, HEARTBEAT_INTERVAL};
use crate::chunk_manager::SERVER_MAP;

#[path = "../shared/master_chunk_utils"]

// static SERVER_STATUS_MAP: SafeMap<u16, Metadata> = SafeMap::new();

// pub fn heartbeat_manager_init() {
//     SERVER_STATUS_MAP.init();
// }

pub async fn receive_heartbeat(metadata: Json<Metadata>) {
    let mut metadata = metadata.into_inner();
    // SERVER_STATUS_MAP.insert(metadata.chunkserver_id, metadata);
    let addr = format!("{}:{}", metadata.ip, metadata.chunkserver_id);
    SERVER_MAP.insert(addr, Vec::new());
}
