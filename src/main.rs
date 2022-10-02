mod accounts;
mod api;
mod db;

use clap::{Parser, Subcommand};
use std::path::Path;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Optional config path
    #[clap(short, long, value_parser)]
    config: Option<String>,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// batch add users
    Add {
        /// user file path
        #[clap(short, long, value_parser)]
        file: String,
    },
    /// batch get users
    Get {
        /// user class
        #[clap(short, long, value_parser)]
        class: Option<String>,
    },
    /// clear all users
    Clear,
    /// sync users
    Sync,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { file } => {
            let path = Path::new(&file);
            api::batch_add_users(path)?;
        }
        Commands::Get { class } => {
            api::batch_get_users(class.as_ref().map(|v| v as &str))?;
        }
        Commands::Clear => {
            api::clear_users()?;
        }
        Commands::Sync => {
            api::sync_users()?;
        }
    }
    Ok(())
}
