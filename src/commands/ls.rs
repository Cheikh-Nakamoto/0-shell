use std::fs::{read_dir, Permissions};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use crate::utils::error::ShellError;

pub fn ls(current_dir: &Path, args: &[&str]) -> Result<(), ShellError> {
    let mut show_hidden = false;
    let mut long_format = false;
    let mut show_indicator = false;

    for arg in args {
        match *arg {
            "-a" => show_hidden = true,
            "-l" => long_format = true,
            "-F" => show_indicator = true,
            _ => return Err(ShellError::InvalidArguments("Invalid flag".to_string())),
        }
    }
    let entries = read_dir(current_dir)?;
    let mut entries: Vec<_> = entries
        .filter_map(|entry| entry.ok())
        .collect();
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        if !show_hidden && name_str.starts_with(".") {
            continue;
        }

        if long_format {
            let metadata = entry.metadata()?;
            let file_type = if metadata.is_dir() { "d" } else { "-" };
            let size = metadata.len();
            let permissions = metadata.permissions();
            println!(
                "{}{} {} {}",
                file_type,
                permissions_string(&permissions),
                size,
                name_str
            );
        } else {
            let mut name_display = name_str.to_string();
            if show_indicator {
                let metadata = entry.metadata()?;
                if metadata.is_dir() {
                    name_display.push('/');
                }
            }
            print!("{} ", name_display);
        }
    }

    if !long_format {
        println!();
    }
    Ok(())
}

fn permissions_string(permissions: &Permissions) -> String {
    let mode = permissions.mode();
    let mut result = String::with_capacity(9);

    for i in (0..3).rev() {
        let offset = i * 3;
        result.push(if mode & (1 << (offset + 2)) != 0 { 'r' } else { '-' });
        result.push(if mode & (1 << (offset + 1)) != 0 { 'w' } else { '-' });
        result.push(if mode & (1 << offset) != 0 { 'x' } else { '-' });
    }

    result
}