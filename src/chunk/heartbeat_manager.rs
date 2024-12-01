use rocket::tokio::time::{sleep, Duration};
use serde::{Deserialize, Serialize};
use std::{path::Path, time::SystemTime};
use sysinfo::{Disks, System};

const HEARTBEAT_INTERVAL: u64 = 2;

#[derive(Serialize, Deserialize)]
struct Disk {
    name: String,
    kind: String,
    file_system: String,
    mount_point: String,
    total_space: u64,
    available_space: u64,
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    os_name: String,
    os_version: String,
    host_name: String,
    chunkserver_id: u32,
    last_heartbeat: u64,
    disk_info: Disk,
}

///
/// Periodically sends a heartbeat to the master server.
/// The heartbeat contains information about the chunkserver.
/// The heartbeat interval is defined by `HEARTBEAT_INTERVAL`.
///
pub async fn heartbeat() {
    info!("Starting Chunkserver heartbeat...");
    let interval = Duration::from_secs(HEARTBEAT_INTERVAL);

    let mut sys = System::new_all();
    sys.refresh_all();

    // Select the disk mounted at `/`
    let mut disks = Disks::new_with_refreshed_list();
    let mut selected_disk = None;
    for disk in disks.list_mut() {
        if disk.mount_point() == Path::new("/") {
            selected_disk = Some(disk);
            break;
        }
    }
    let mut metadata = Metadata {
        os_name: System::name().unwrap_or_else(|| "Unknown".to_string()),
        os_version: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
        host_name: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
        chunkserver_id: 1,
        last_heartbeat: 0,
        disk_info: Disk {
            name: selected_disk
                .as_ref()
                .unwrap()
                .name()
                .to_str()
                .unwrap_or("Unknown")
                .to_string(),
            kind: selected_disk.as_ref().unwrap().kind().to_string(),
            file_system: selected_disk
                .as_ref()
                .unwrap()
                .file_system()
                .to_str()
                .unwrap_or("Unknown")
                .to_string(),
            mount_point: selected_disk
                .as_ref()
                .unwrap()
                .mount_point()
                .to_str()
                .unwrap_or("Unknown")
                .to_string(),
            total_space: selected_disk.as_ref().unwrap().total_space(),
            available_space: selected_disk.as_ref().unwrap().available_space(),
        },
    };

    // print metadata
    info!("OS Name: {}", metadata.os_name);
    info!("OS Version: {}", metadata.os_version);
    info!("Host Name: {}", metadata.host_name);
    info!("Chunkserver ID: {}", metadata.chunkserver_id);
    info!("Disk Name: {}", metadata.disk_info.name);
    info!("Disk Kind: {}", metadata.disk_info.kind);
    info!("File System: {}", metadata.disk_info.file_system);
    info!("Mount Point: {}", metadata.disk_info.mount_point);
    info!("Total Space: {}", metadata.disk_info.total_space);
    info!("Available Space: {}", metadata.disk_info.available_space);

    loop {
        metadata.last_heartbeat = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        info!("Sending heartbeat...");
        // send heartbeat to master server

        // update disk info
        disks.refresh();
        selected_disk = None;
        for disk in disks.list_mut() {
            if disk.mount_point() == Path::new("/") {
                selected_disk = Some(disk);
                break;
            }
        }
        metadata.disk_info.available_space = selected_disk.as_ref().unwrap().available_space();

        // convert metadata to JSON
        let metadata_json = serde_json::to_string(&metadata).unwrap();
        debug!("Metadata: {}", metadata_json);

        // Todo: send metadata to master server

        sleep(interval).await;
    }
}
