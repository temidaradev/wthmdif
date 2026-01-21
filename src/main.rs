mod cli;
mod disk;
mod folder;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Disk { path } => {
            disk::run_disk(&path)?;
        }
        Commands::Folder { path, detail } => {
            folder::run_dir(&path, detail)?;
        }
    }
    Ok(())
}
