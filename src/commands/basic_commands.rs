use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "nyx")]
#[command(version = "1.0")]
#[command(about = "nyx", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    //setup the nyx-manager 
    Init {
        #[arg(default_value = "this")]
        server_path: String,
    },

    /// Downolad files
    Run,
    /// update files
    Update {
        #[arg(default_value = "files")]
        type_: String,
    },

}

pub fn engine_commands(){
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { server_path } => {
            println!("Initializing nyx-manager at: {server_path}");
        }
        Commands::Run => {
            println!("Downolading...");
        }
        Commands::Update { type_ } => {
            println!("Updating: {type_}");
        }
    }
}
