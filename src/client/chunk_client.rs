use std::io::Error;

pub struct ChunkClient {
    client: reqwest::Client,
    base_url: String,
}

impl ChunkClient {
    pub fn new(base_url: &str) -> Self {
        ChunkClient {
            client: reqwest::Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn add_chunk(&self, id: &str, data: Vec<u8>) -> Result<String, Error> {
        let url = format!("https://{}/add_chunk?id={}", self.base_url, id);
        println!("{}", url);
        let response = self.client.post(&url).body(data).send()
            .await.expect("Request failed");
        if response.status().is_success() {
            Ok(response.status().to_string())
        } else {
            Err(Error::new(std::io::ErrorKind::Other, String::from(response.status().as_str())))
        }
    }

    pub async fn append_chunk(&self, id: &str, data: Vec<u8>) -> Result<String, Error> {
        let url = format!("https://{}/append_chunk?id={}", self.base_url, id);
        let response = self.client.post(&url).body(data).send()
            .await.expect("Request failed");
        if response.status().is_success() {
            Ok(response.status().to_string())
        } else {
            Err(Error::new(std::io::ErrorKind::Other, String::from(response.status().as_str())))
        }
    }

    pub async fn get_chunk(&self, id: &str) -> Result<Vec<u8>, Error> {
        let url = format!("https://{}/get_chunk?id={}", self.base_url, id);
        let response = self.client.get(&url).send().await.expect("Request failed");
        if response.status().is_success() {
            let result = response.bytes().await.expect("Failed to parse response");
            Ok(result.to_vec())
        } else {
            Err(Error::new(std::io::ErrorKind::Other, String::from(response.status().as_str())))
        }
    }

    pub async fn delete_chunk(&self, id: &str) -> Result<String, Error> {
        let url = format!("https://{}/delete_chunk?id={}", self.base_url, id);
        let response = self.client.post(&url).send().await.expect("Request failed");
        if response.status().is_success() {
            Ok(response.status().to_string())
        } else {
            Err(Error::new(std::io::ErrorKind::Other, String::from(response.status().as_str())))
        }
    }

    #[allow(unused)]
    pub async fn get_chunk_list(&self) -> Result<Vec<String>, Error> {
        let url = format!("https://{}/get_chunk_list", self.base_url);
        let response = self.client.get(&url).send().await.expect("Request failed");
        if response.status().is_success() {
            let result = response.json::<Vec<String>>().await.expect("Failed to parse response");
            Ok(result)
        } else {
            Err(Error::new(std::io::ErrorKind::Other, String::from(response.status().as_str())))
        }
    }
}