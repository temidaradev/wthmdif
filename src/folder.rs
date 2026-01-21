use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use jwalk::WalkDir;

use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::utils::{get_entry_color, human_size, is_system_dir, create_bar};

struct EntryInfo {
    name: String,
    size: u64,
    is_dir: bool,
}

struct DetailedEntry {
    path: PathBuf,
    size: u64,
    is_dir: bool,
}

fn get_dir_size(path: &Path) -> u64 {
    if is_system_dir(path) {
        return 0;
    }

    WalkDir::new(path)
        .skip_hidden(false)
        .process_read_dir(|_, parent, _, children| {
            children.retain(|entry_result| {
                if let Ok(entry) = entry_result {
                    let path = parent.join(&entry.file_name);
                    !is_system_dir(&path)
                } else {
                    true
                }
            });
        })
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.metadata().map(|m| m.len()).unwrap_or(0))
        .sum()
}

fn collect_recursive(path: &Path, list: &mut Vec<DetailedEntry>) -> anyhow::Result<u64> {
    if is_system_dir(path) {
        return Ok(0);
    }
    
    let mut total_size = 0;
    
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            let meta = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue, 
            };
            
            let size = if meta.is_dir() {
                collect_recursive(&path, list)?
            } else {
                meta.len()
            };
            
            total_size += size;
            
            list.push(DetailedEntry {
                path,
                size,
                is_dir: meta.is_dir(),
            });
        }
    }
    
    Ok(total_size)
}

pub fn run_dir(path: &PathBuf, detail: bool) -> anyhow::Result<()> {
    if detail {
        let pb_scan = ProgressBar::new_spinner();
        pb_scan.set_style(
            ProgressStyle::with_template("{spinner:.green} {msg}")
                .unwrap()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
        );
        pb_scan.set_message(format!("Scanning detailed tree for {}...", path.display()));
        pb_scan.enable_steady_tick(Duration::from_millis(100));

        let mut entries = Vec::new();
        let total = collect_recursive(path, &mut entries)?;
        
        pb_scan.finish_and_clear();

        println!("Scanning detailed tree for {:?}... Done", path);
        println!("\nTotal size: {}", human_size(total).bold());
        
        entries.sort_by(|a, b| b.size.cmp(&a.size));
        
        for entry in entries {
            let size_str = human_size(entry.size);
            let (colored_size, kind) = get_entry_color(&size_str, entry.is_dir);
            println!("{}  {}  {:?}", colored_size, kind, entry.path);
        }

        return Ok(());
    }

    let pb_scan = ProgressBar::new_spinner();
    pb_scan.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
    );
    pb_scan.set_message(format!("Scanning {}...", path.display()));
    pb_scan.enable_steady_tick(Duration::from_millis(100));

    let entries: Vec<_> = fs::read_dir(path)?.filter_map(Result::ok).collect();
    let folder_size = get_dir_size(path);
    
    pb_scan.finish_and_clear();

    let bytes = folder_size as f64;
    let kb = bytes / 1024.0;
    let mb = kb / 1024.0;
    let gb = mb / 1024.0;

    println!(
        "{}: {:?}\n{} | {} | {} | {}",
        "Path".bold().blue(),
        path.display(),
        format!("{:.2} GB", gb).red().bold(),
        format!("{:.2} MB", mb).yellow(),
        format!("{:.2} KB", kb).cyan(),
        format!("{:.2} B", bytes).white().dimmed(),
    );

    let pb_process = ProgressBar::new(entries.len() as u64);
    pb_process.set_style(
        ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut infos: Vec<EntryInfo> = entries
        .iter()
        .map(|entry| {
            pb_process.inc(1);
            let path = entry.path();
            let meta = entry.metadata().unwrap();

            let size = if meta.is_dir() {
                get_dir_size(&path)
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
    
    pb_process.finish_and_clear();

    infos.sort_by(|a, b| b.size.cmp(&a.size));

    let max_size = infos.first().map(|i| i.size).unwrap_or(0).max(1);
    let bar_total_width = 30;

    for info in infos {
        let size_str = human_size(info.size);
        let (colored_size, kind) = get_entry_color(&size_str, info.is_dir);

        let ratio = info.size as f64 / max_size as f64;
        
        let colored_bar = if size_str.ends_with("TB") || size_str.ends_with("GB") {
            create_bar(ratio, bar_total_width).red()
        } else if size_str.ends_with("MB") {
            create_bar(ratio, bar_total_width).yellow()
        } else {
            create_bar(ratio, bar_total_width).cyan()
        };

        println!("{}  {}  {}  {}", colored_size, kind, colored_bar, info.name);
    }

    Ok(())
}
