use std::fs;
use std::io::Error;
use std::path::Path;
use clap::Parser;
use master_client::MasterClient;
use chunk_client::ChunkClient;
use lib::shared::master_client_utils::{ChunkInfo, User};

mod chunk_client;
mod master_client;

const MASTER_URL: &str = "http://localhost:8000";

#[derive(Parser, Debug)]
#[command(name = "client", about = "CRUD operations on files/directories")]
struct Opt {
    #[arg(short, long)]
    username: String,

    #[arg(short, long)]
    password: String,

    #[arg(short, long)]
    target: String,

    #[arg(short, long)]
    action: String,

    #[arg(short, long)]
    local_path: Option<String>,

    #[arg(short, long)]
    remote_path: Option<String>,
}

#[derive(Debug)]
enum Target {
    Directory,
    File,
}

#[derive(Debug)]
enum Action {
    Create,
    Read,
    Update,
    Delete,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let opt = Opt::parse();
    let target = match opt.target.as_str() {
        "directory" => Target::Directory,
        "dir" => Target::Directory,
        "d" => Target::Directory,
        "file" => Target::File,
        "f" => Target::File,
        _ => {
            panic!("Invalid target");
        }
    };

    let action = match opt.action.as_str() {
        "create" => Action::Create,
        "c" => Action::Create,
        "read" => Action::Read,
        "r" => Action::Read,
        "update" => Action::Update,
        "u" => Action::Update,
        "delete" => Action::Delete,
        "d" => Action::Delete,
        _ => {
            panic!("Invalid action");
        }
    };

    let user = User {
        username: opt.username,
        password: opt.password,
    };

    let binding = opt.local_path.unwrap_or_default();
    let local_path = binding.as_str();
    let binding = opt.remote_path.unwrap_or_default();
    let remote_path = binding.as_str();

    let master_client = MasterClient::new(MASTER_URL);
    user_authenticate(&master_client, &user).await?;

    match target {
        Target::Directory => {
            match action {
                Action::Create => {
                    let result = master_client.create_directory(remote_path).await?;
                    println!("{}", result);
                }
                Action::Read => {
                    let result = master_client.read_directory(remote_path).await?;
                    println!("{}", result);
                }
                Action::Delete => {
                    let _result = master_client.delete_directory(remote_path).await?;
                }
                _ => {}
            }
        }
        Target::File => {
            match action {
                Action::Create => {
                    create_file(&master_client, local_path, remote_path).await?;
                }
                Action::Read => {
                    read_file(&master_client, local_path, remote_path).await?;
                }
                Action::Update => {
                    update_file(&master_client, local_path, remote_path).await?;
                }
                Action::Delete => {
                    delete_file(&master_client, remote_path).await?;
                }
            }
        }
    }

    Ok(())
}

async fn user_authenticate(master_client: &MasterClient, user: &User) -> Result<(), Error> {
    master_client.user_authenticate(user).await?;
    Ok(())
}

async fn create_file(master_client: &MasterClient, local_path: &str, remote_path: &str)
                     -> Result<(), Error> {
    // read local file
    let file: Vec<u8> = fs::read(local_path)?;
    let size = file.len();

    // Create remote file on master
    let _result = master_client.create_file(remote_path).await?;
    // println!("{:?}", result);

    // Signal master with file size
    let result: Vec<ChunkInfo> = master_client.update_file(remote_path, size).await?;

    // Write to all chunks
    for chunk in result.iter() {
        let chunk_client = ChunkClient::new(&chunk.server_ip.as_str());
        let _result = chunk_client.add_chunk(&chunk.uuid, file.clone()).await.unwrap();
        // println!("{}", result);
    }

    Ok(())
}

async fn read_file(master_client: &MasterClient, source_path: &str, destination_path: &str)
                   -> Result<(), Error> {
    let result = master_client.read_file(destination_path).await?;

    // pick the first chunk
    if result.is_empty() {
        return Err(Error::new(std::io::ErrorKind::NotFound, "File not found"));
    }
    let chunk = result.first().unwrap();

    // Read from chunk
    let chunk_client = ChunkClient::new(&chunk.server_ip.as_str());
    let result = chunk_client.get_chunk(&chunk.uuid).await.unwrap();

    // Write to local file
    let path = Path::new(source_path);
    fs::write(path, result).expect("Failed to write file");

    Ok(())
}

async fn update_file(master_client: &MasterClient, source_path: &str, destination_path: &str)
                     -> Result<(), Error> {
    let file: Vec<u8> = fs::read(source_path)?;
    let size = file.len();

    // Signal master with file size
    let result: Vec<ChunkInfo> = master_client.update_file(destination_path, size).await?;

    // update across all chunks
    for chunk in result.iter() {
        let chunk_client = ChunkClient::new(&chunk.server_ip.as_str());
        let _result = chunk_client.append_chunk(&chunk.uuid, file.clone()).await.unwrap();
        // println!("{}", result);
    }

    Ok(())
}

async fn delete_file(master_client: &MasterClient, remote_path: &str) -> Result<(), Error> {
    // get file info from master
    let result = master_client.read_file(remote_path).await?;

    // delete file from master
    let _result = master_client.delete_file(remote_path).await?;

    // delete file from all chunks
    for chunk in result.iter() {
        let chunk_client = ChunkClient::new(&chunk.server_ip.as_str());
        let _result = chunk_client.delete_chunk(&chunk.uuid).await.unwrap();
    }
    Ok(())
}

