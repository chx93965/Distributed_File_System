use crate::chunk_server_client::ChunkServerClient;
use clap::Parser;
mod chunk_server_client;
mod master_client;

#[derive(Parser, Debug)]
#[command(name = "url")]
struct Opt {
    /// master or chunkserver
    #[arg(short, long)]
    target: String,
    /// chunk ID
    #[arg(short, long)]
    id: u16,
    /// operation to perform
    #[arg(short, long)]
    action: String
}

enum Target {
    Master,
    ChunkServer
}

enum Action {
    Contact,
    AddChunk,
    AppendChunk,
    GetChunk,
    GetChunkList
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::parse();
    let target = match opt.target.as_str() {
        "master" => Target::Master,
        "chunkserver" => Target::ChunkServer,
        _ => panic!("Invalid target")
    };
    let action = match opt.action.as_str() {
        "contact" => Action::Contact,
        "add_chunk" => Action::AddChunk,
        "append_chunk" => Action::AppendChunk,
        "get_chunk" => Action::GetChunk,
        "get_chunk_list" => Action::GetChunkList,
        _ => panic!("Invalid action")
    };
    match target {
        Target::Master => {
            let _master_client = master_client::MasterClient::new("http://localhost:8080");
        }
        Target::ChunkServer => {
            let chunk_server_client = ChunkServerClient::new("http://localhost:8081");
            match action {
                Action::Contact => {
                    let response = chunk_server_client.contact().await?;
                    println!("{}", response);
                }
                Action::AddChunk => {
                    let data = vec![0, 1, 2, 3, 4, 5];
                    chunk_server_client.add_chunk(&opt.id.to_string(), &data).await?;
                }
                Action::AppendChunk => {
                    let data = vec![6, 7, 8, 9, 10];
                    chunk_server_client.append_chunk(&opt.id.to_string(), &data).await?;
                }
                Action::GetChunk => {
                    let response = chunk_server_client.get_chunk(&opt.id.to_string()).await?;
                    println!("{:?}", response);
                }
                Action::GetChunkList => {
                    let response = chunk_server_client.get_chunk_list().await?;
                    println!("{:?}", response);
                }
            }
        }
    }

    Ok(())
}
