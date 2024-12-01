use reqwest::Client;
use std::error::Error;

pub struct MasterClient {
    master_url: String,
    client: Client,
}

impl MasterClient {
    pub fn new(base_url: &str) -> Self {
        MasterClient {
            master_url: base_url.to_string(),
            client: Client::new(),
        }
    }

    // /// Fetch hello message from the server.
    // pub async fn hello(&self) -> Result<String, Box<dyn Error>> {
    //     let url = format!("{}/", self.master_url);
    //     let response = self.client.get(&url).send().await?;
    //     Ok(response.text().await?)
    // }
    //
    // /// Add a chunk to the server an ID and binary data.
    // pub async fn add_chunk(&self, id: &str, data: &[u8]) -> Result<(), Box<dyn Error>> {
    //     let url = format!("{}/add_chunk", self.master_url);
    //     let response = self
    //         .client
    //         .post(&url)
    //         .query(&[("id", id)])
    //         .body(data.to_vec())
    //         .send()
    //         .await?;
    //
    //     if response.status().is_success() {
    //         Ok(())
    //     } else {
    //         Err(format!("Failed to add chunk: {}", response.text().await?).into())
    //     }
    // }
    //
    // /// Append data to an existing chunk with its ID.
    // pub async fn append_chunk(&self, id: &str, data: &[u8]) -> Result<(), Box<dyn Error>> {
    //     let url = format!("{}/append_chunk", self.master_url);
    //     let response = self
    //         .client
    //         .post(&url)
    //         .query(&[("id", id)])
    //         .body(data.to_vec())
    //         .send()
    //         .await?;
    //
    //     if response.status().is_success() {
    //         Ok(())
    //     } else {
    //         Err(format!("Failed to append chunk: {}", response.text().await?).into())
    //     }
    // }
    //
    // /// Retrieve a chunk's data with its ID.
    // pub async fn get_chunk(&self, id: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    //     let url = format!("{}/get_chunk", self.master_url);
    //     let response = self
    //         .client
    //         .get(&url)
    //         .query(&[("id", id)])
    //         .send()
    //         .await?;
    //
    //     if response.status().is_success() {
    //         Ok(response.bytes().await?.to_vec())
    //     } else {
    //         Err(format!("Failed to get chunk: {}", response.text().await?).into())
    //     }
    // }
    //
    // /// Retrieve all chunk IDs.
    // pub async fn get_chunk_list(&self) -> Result<Vec<String>, Box<dyn Error>> {
    //     let url = format!("{}/get_chunk_list", self.master_url);
    //     let response = self.client.get(&url).send().await?;
    //
    //     if response.status().is_success() {
    //         Ok(response.json().await?)
    //     } else {
    //         Err(format!("Failed to get chunk list: {}", response.text().await?).into())
    //     }
    // }
}
