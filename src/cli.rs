use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Clone)]
#[command(version, about = "Inspects file metadata")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Filesystem disk usage (df-style, O(1))
    Disk { path: PathBuf },

    /// Folder inspection (du / ls style)
    Folder {
        path: PathBuf,
        #[arg(short, long)]
        detail: bool,
    },
}
