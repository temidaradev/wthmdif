use clap::Parser;
use filesize::PathExt;
use std::path::Path;
extern crate fs_extra;
use fs_extra::dir::get_size;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "Inspects file metadata")]
struct Args {
    path: PathBuf,

    #[arg(short, long)]
    human_readable: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    // let metadata = std::fs::metadata(&args.path)?;
    let folder_size = get_size(&args.path)?;

    let bytes = folder_size as f64;
    let kb = bytes / 1024.0;
    let mb = kb / 1024.0;
    let gb = mb / 1024.0;

    println!(
        "Path: {:?}\n{:.2} GB | {:.2} MB | {:.2} KB | {:.2} B",
        args.path, gb, mb, kb, bytes
    );
    Ok(())
}
