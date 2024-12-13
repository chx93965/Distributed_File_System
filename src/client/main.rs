use std::path::Path;
use clap::Parser;
use master_client::MasterClient;

mod chunk_client;
mod master_client;

const MASTER_URL: &str = "http://localhost:8000";

#[derive(Parser, Debug)]
#[command(name = "client", about = "CRUD operations on files/directories")]
struct Opt {
    #[arg(short, long)]
    target: String,

    #[arg(short, long)]
    action: String,

    #[arg(short, long)]
    path: Option<String>,
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

    let path = opt.path.unwrap_or_default();
    let path = Path::new(&path).to_str().unwrap_or_default();
    // println!("Target: {:?}, Action: {:?}, Data: {}", target, action, path);

    let client = MasterClient::new(MASTER_URL);
    match target {
        Target::Directory => {
            match action {
                Action::Create => {
                    let result = client.create_directory(path).await?;
                    println!("{}", result);
                }
                Action::Read => {
                    let result = client.read_directory(path).await?;
                    println!("{:?}", result);
                }
                Action::Delete => {
                    // let result = client.delete_directory(path).await?;
                    // println!("{}", result);
                }
                _ => {}
            }
        }
        Target::File => {
            match action {
                Action::Create => {
                    let result = client.create_file(path).await?;
                    println!("{:?}", result);
                }
                Action::Read => {
                    let result = client.read_file(path).await?;
                    println!("{:?}", result);
                }
                Action::Update => {
                    let result = client.update_file(path, 0).await?;
                    println!("{:?}", result);
                }
                Action::Delete => {
                    // let result = client.delete_file(path).await?;
                    // println!("{}", result);
                }
            }
        }
    }

    Ok(())
}
