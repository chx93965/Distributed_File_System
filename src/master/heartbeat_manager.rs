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
use heartbeat::{Disk, Metadata, HEARTBEAT_INTERVAL};

#[path = "../shared/heartbeat.rs"]
mod heartbeat;

static SERVER_STATUS_MAP: SafeMap<u32, Metadata> = SafeMap::new();

pub fn heartbeat_manager_init() {
    SERVER_STATUS_MAP.init();
}

pub async fn receive_heartbeat(metadata: Json<Metadata>) {
    let mut metadata = metadata.into_inner();
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    metadata.last_heartbeat = now;
    SERVER_STATUS_MAP.insert(metadata.chunkserver_id, metadata);
}
