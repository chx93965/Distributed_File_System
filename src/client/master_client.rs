use reqwest::{Client, StatusCode};
use std::io::Error;
use uuid::Uuid;
use lib::shared::master_client_utils::{ChunkInfo, DirectoryInfo, FileInfo};

pub struct MasterClient {
    base_url: String,
    client: Client,
}


impl MasterClient {
    pub fn new(base_url: &str) -> Self {
        MasterClient {
            base_url: base_url.to_string(),
            client: Client::new(),
        }
    }

    pub async fn create_file(&self, path: &str) -> Result<FileInfo, std::io::Error> {
        let url = format!("{}/file/create?path=/{}", self.base_url, path);
        let response = self.client.post(&url).send().await.expect("Request failed");
        if response.status().is_success() {
            let result = response.json::<FileInfo>().await.expect("Failed to parse response");
            Ok(result)
        } else {
            Err(Error::new(std::io::ErrorKind::Other, String::from("Failed to create file")))
        }
    }

    pub async fn read_file(&self, path: &str) -> Result<Vec<ChunkInfo>, Error> {
        let url = format!("{}/file/read?path=/{}", self.base_url, path);
        let response = self.client.get(&url).send().await.expect("Request failed");
        if response.status().is_success() {
            let result = response.json::<Vec<ChunkInfo>>().await.expect("Failed to parse response");
            Ok(result)
        } else {
            Err(Error::new(std::io::ErrorKind::Other, String::from("Failed to read file")))
        }
    }

    pub async fn update_file(&self, path: &str, size: usize) -> Result<Vec<ChunkInfo>, Error> {
        let url = format!("{}/file/update?path=/{}&size={}", self.base_url, path, size);
        let response = self.client.post(&url).send().await.expect("Request failed");
        if response.status().is_success() {
            let result = response.json::<Vec<ChunkInfo>>().await.expect("Failed to parse response");
            Ok(result)
        } else {
            Err(Error::new(std::io::ErrorKind::Other, String::from("Failed to update file")))
        }
    }

    pub async fn create_directory(&self, path: &str) -> Result<String, Error> {
        let url = format!("{}/dir/create?path=/{}", self.base_url, path);
        println!("{}", url);
        let response = self.client.post(&url).send().await.expect("Request failed");
        if response.status().is_success() {
            let result = response.text().await.expect("Failed to parse response");
            Ok(result)
        } else {
            Err(Error::new(std::io::ErrorKind::Other, String::from("Failed to create directory")))
        }
    }

    pub async fn read_directory(&self, path: &str) -> Result<DirectoryInfo, Error> {
        let url = format!("{}/dir/read?path=/{}", self.base_url, path);
        let response = self.client.get(&url).send().await.expect("Request failed");
        if response.status().is_success() {
            let result = response.json::<DirectoryInfo>().await.expect("Failed to parse response");
            Ok(result)
        } else {
            Err(Error::new(std::io::ErrorKind::Other, String::from("Failed to read directory")))
        }
    }
}

