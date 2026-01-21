use colored::*;
use std::path::Path;

pub fn human_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut i = 0;

    while size >= 1024.0 && i < UNITS.len() - 1 {
        size /= 1024.0;
        i += 1;
    }

    format!("{:.2} {}", size, UNITS[i])
}

pub fn is_system_dir(path: &Path) -> bool {
    // Only verify strictly top-level system directories
    if let Some(parent) = path.parent() {
        if parent == Path::new("/") {
            if let Some(name) = path.file_name() {
                return matches!(name.to_str(), Some("proc" | "sys" | "dev" | "run"));
            }
        }
    }
    path == Path::new("/proc") || path == Path::new("/sys") || path == Path::new("/dev") || path == Path::new("/run")
}

pub fn get_entry_color(size_str: &str, is_dir: bool) -> (colored::ColoredString, colored::ColoredString) {
    let colored_size = if size_str.ends_with("TB") {
        format!("{:>10}", size_str).red().bold()
    } else if size_str.ends_with("GB") {
        format!("{:>10}", size_str).red()
    } else if size_str.ends_with("MB") {
        format!("{:>10}", size_str).yellow()
    } else if size_str.ends_with("KB") {
        format!("{:>10}", size_str).cyan()
    } else {
        format!("{:>10}", size_str).white()
    };
    
    let kind = if is_dir { "DIR ".blue().bold() } else { "FILE".white() };
    (colored_size, kind)
}

pub fn create_bar(percent: f64, width: usize) -> String {
    let fractional_blocks = [" ", "▏", "▎", "▍", "▌", "▋", "▊", "▉", "█"];
    let filled_width = (percent * width as f64).round();
    let full_blocks = (filled_width.floor() as usize).min(width);
    
    let mut bar = String::with_capacity(width);
    
    // Add full blocks
    for _ in 0..full_blocks {
        bar.push_str(fractional_blocks[8]);
    }
    
    // Add fractional part if strictly less than width (and non-zero remainder)
    if full_blocks < width {
        let remainder = filled_width - full_blocks as f64;
        let index = (remainder * 8.0).round() as usize;
        if index > 0 {
             bar.push_str(fractional_blocks[index]);
        } else {
             bar.push_str(fractional_blocks[0]); // Space for empty
        }
    }

    // Pad with spaces
    let current_len = bar.chars().count();
    if current_len < width {
        for _ in 0..(width - current_len) {
            bar.push_str(fractional_blocks[0]);
        }
    }
    
    bar
}
