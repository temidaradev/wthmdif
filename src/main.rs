use clap::{Parser, Subcommand};
extern crate fs_extra;
use fs_extra::dir::get_size;
use libc::statvfs;
use std::ffi::CString;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[derive(Parser, Clone)]
#[command(version, about = "Inspects file metadata")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
    /// Filesystem disk usage (df-style, O(1))
    Disk { path: PathBuf },

    /// Folder inspection (du / ls style)
    Folder { path: PathBuf },
}

struct EntryInfo {
    name: String,
    size: u64,
    is_dir: bool,
}

fn disk_usage(path: &Path) -> std::io::Result<(u64, u64, u64)> {
    let c_path = CString::new(path.as_os_str().as_encoded_bytes()).unwrap();

    let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };

    let res = unsafe { statvfs(c_path.as_ptr(), &mut stat) };
    if res != 0 {
        return Err(std::io::Error::last_os_error());
    }

    let total = stat.f_blocks as u64 * stat.f_frsize as u64;
    let free = stat.f_bfree as u64 * stat.f_frsize as u64;
    let used = total - free;

    Ok((total, used, free))
}

fn human_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut i = 0;

    while size >= 1024.0 && i < UNITS.len() - 1 {
        size /= 1024.0;
        i += 1;
    }

    format!("{:.2} {}", size, UNITS[i])
}

fn run_dir(path: &PathBuf) -> anyhow::Result<()> {
    let entries: Vec<_> = fs::read_dir(path)?.filter_map(Result::ok).collect();
    let folder_size = get_size(path)?;

    let bytes = folder_size as f64;
    let kb = bytes / 1024.0;
    let mb = kb / 1024.0;
    let gb = mb / 1024.0;

    println!(
        "Path: {:?}\n{:.2} GB | {:.2} MB | {:.2} KB | {:.2} B",
        path.display(),
        gb,
        mb,
        kb,
        bytes,
    );

    let mut infos: Vec<EntryInfo> = entries
        .iter()
        .map(|entry| {
            let path = entry.path();
            let meta = entry.metadata().unwrap();

            let size = if meta.is_dir() {
                get_size(&path).unwrap()
            } else {
                meta.len()
            };

            EntryInfo {
                name: entry.file_name().to_string_lossy().into_owned(),
                size,
                is_dir: meta.is_dir(),
            }
        })
        .collect();

    infos.sort_by(|a, b| b.size.cmp(&a.size));

    for info in infos {
        let kind = if info.is_dir { "DIR " } else { "FILE" };

        println!("{:>8}  {:<4}  {}", human_size(info.size), kind, info.name);
    }

    Ok(())
}

fn run_disk(path: &Path) -> anyhow::Result<()> {
    let (total, used, free) = disk_usage(path)?;

    println!(
        "Disk usage:\n  Total: {}\n  Used:  {}\n  Free:  {}",
        human_size(total),
        human_size(used),
        human_size(free),
    );

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Disk { path } => {
            run_disk(&path)?;
        }
        Commands::Folder { path } => {
            run_dir(&path)?;
        }
    }
    Ok(())
}
