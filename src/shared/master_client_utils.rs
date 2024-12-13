use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChunkInfo {
    pub uuid: String,
    pub key: String,
}

impl ChunkInfo {
    pub fn serialize(file: Vec<(Uuid, String)>) -> Vec<ChunkInfo> {
        let mut chunks = Vec::new();
        for (uuid, key) in file{
            chunks.push(ChunkInfo{uuid: uuid.to_string(), key });
        }
        chunks
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    size: i32,
    creation_time: DateTime<Utc>,
    modification_time: DateTime<Utc>,
    permission: i32,
    owner: String,
    group: String,
}

impl Metadata {
    pub fn new(size: i32, permission: i32, owner: String, group: String) -> Self {
        let utc_now: DateTime<Utc> = Utc::now();
        Self {
            size: size,
            creation_time: utc_now,
            modification_time: utc_now,
            permission: permission,
            owner: owner,
            group: group,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileInfo {
    pub file_name: String,
    pub file_parent: String,
    pub file_metadata: Metadata,
    pub chunks: Vec<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DirectoryInfo {
    pub dir_name: String,
    pub dir_parent: String,
    pub dir_metadata: Metadata,
    pub files: HashMap<String, FileInfo>
}