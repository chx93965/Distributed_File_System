use reqwest::Client;
use std::io::Error;
use lib::shared::master_client_utils::{ChunkInfo, DirectoryInfo, FileInfo, User};

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

    pub async fn user_authenticate(&self, user:&User) -> Result<String, Error> {
        // register if user does not exist
        let url = format!("{}/user/register", self.base_url);
        let response = self.client.post(&url).json(user).send().await.expect("Request failed");
        if response.status().is_success() {
            let result = response.text().await.expect("Failed to parse response");
            Ok(result)
        } else {
            // login if user exists
            let url = format!("{}/user/login", self.base_url);
            let response = self.client.post(&url).json(user).send().await.expect("Request failed");
            if response.status().is_success() {
                let result = response.text().await.expect("Failed to parse response");
                Ok(result)
            } else {
                Err(Error::new(std::io::ErrorKind::Other, String::from("Failed to authenticate user")))
            }
        }
    }

    pub async fn create_file(&self, path: &str) -> Result<FileInfo, Error> {
        let url = format!("{}/file/create?path=/{}", self.base_url, path);
        println!("{}", url);
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
        println!("{}", url);
        let response = self.client.post(&url).send().await.expect("Request failed");
        if response.status().is_success() {
            let result = response.json::<Vec<ChunkInfo>>().await.expect("Failed to parse response");
            Ok(result)
        } else {
            Err(Error::new(std::io::ErrorKind::Other, String::from("Failed to update file")))
        }
    }

    pub async fn delete_file(&self, path: &str) -> Result<(), Error> {
        let url = format!("{}/file/delete?path=/{}", self.base_url, path);
        let response = self.client.get(&url).send().await.expect("Request failed");
        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::new(std::io::ErrorKind::Other, String::from("Failed to delete file")))
        }
    }

    pub async fn create_directory(&self, path: &str) -> Result<String, Error> {
        let url = format!("{}/dir/create?path=/{}", self.base_url, path);
        let response = self.client.post(&url).send().await.expect("Request failed");
        if response.status().is_success() {
            let result = response.text().await.expect("Failed to parse response");
            Ok(result)
        } else {
            Err(Error::new(std::io::ErrorKind::Other, String::from("Failed to create directory")))
        }
    }

    pub async fn read_directory(&self, path: &str) -> Result<String, Error> {
        let url = format!("{}/dir/read?path=/{}", self.base_url, path);
        let response = self.client.get(&url).send().await.expect("Request failed");
        if response.status().is_success() {
            let result = response.json::<DirectoryInfo>().await.expect("Failed to parse response");
            // convert to Json string
            let result = serde_json::to_string(&result).expect("Failed to convert to JSON string");
            Ok(result)
        } else {
            Err(Error::new(std::io::ErrorKind::Other, String::from("Failed to read directory")))
        }
    }

    pub async fn delete_directory(&self, path: &str) -> Result<(), Error> {
        let url = format!("{}/dir/delete?path=/{}", self.base_url, path);
        let response = self.client.post(&url).send().await.expect("Request failed");
        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::new(std::io::ErrorKind::Other, String::from("Failed to delete directory")))
        }
    }
}

