use rocket::serde::{Deserialize, Serialize};

pub const HEARTBEAT_INTERVAL: u64 = 2;

#[derive(Serialize, Deserialize, Debug)]
pub struct Disk {
    pub name: String,
    pub kind: String,
    pub file_system: String,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub os_name: String,
    pub os_version: String,
    pub host_name: String,
    pub chunkserver_id: u16,
    pub last_heartbeat: u64,
    pub disk_info: Disk,
}