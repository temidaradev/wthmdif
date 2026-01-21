use colored::*;
use libc::statvfs;
use std::ffi::CString;
use std::path::Path;

pub fn disk_usage(path: &Path) -> std::io::Result<(u64, u64, u64)> {
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

pub fn run_disk(path: &Path) -> anyhow::Result<()> {
    let (total, used, free) = disk_usage(path)?;
    let percent = (used as f64 / total as f64) * 100.0;

    println!("\n{}", "Disk Usage Analysis".bold().underline());
    println!("{:<10} {}", "Path:".bold(), path.display());
    println!("{:<10} {}", "Total:".bold(), crate::utils::human_size(total).bold());
    println!("{:<10} {}", "Free:".bold(), crate::utils::human_size(free).green());
    println!("{:<10} {}", "Used:".bold(), crate::utils::human_size(used).yellow());

    let bar_width: usize = 40;
    
    let colored_bar = if percent > 85.0 {
        crate::utils::create_bar(percent / 100.0, bar_width).red()
    } else if percent > 60.0 {
         crate::utils::create_bar(percent / 100.0, bar_width).yellow()
    } else {
         crate::utils::create_bar(percent / 100.0, bar_width).green()
    };

    println!(
        "\n{}  {:.2}%",
        colored_bar,
        percent
    );

    Ok(())
}
