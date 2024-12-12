use lib::shared::log_manager;
use clap::Parser;
use master_client::MasterClient;

mod chunk_client;
mod master_client;

#[derive(Parser, Debug)]
#[command(name = "client", about = "CRUD operations on files/directories")]
struct Opt {
    #[arg(short, long)]
    target: String,

    #[arg(short, long)]
    action: String,

    #[arg(short, long)]
    data: Option<String>,
}

enum Target {
    Directory,
    File,
}

enum Action {
    Create,
    Read,
    Update,
    Delete,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log_manager::set_logging(&[
        log::Level::Info,
        log::Level::Debug,
        log::Level::Warn,
        log::Level::Error,
    ]);

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

    let data = opt.data.unwrap_or_default();


    Ok(())
}
