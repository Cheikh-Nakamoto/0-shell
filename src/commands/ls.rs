use std::fs::{read_dir, Permissions};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;
use crate::utils::date::format_datetime;
use crate::utils::error::ShellError;

/**
 * List the contents of a directory.
 *
 * # Arguments
 * * `current_dir` - The current directory.
 * * `args` - The arguments passed to the command.
 *
 * # Example
 * ```rust
 * use std::path::Path;
 * use shell::commands::ls;
 *
 * let current_dir = Path::new("/home/user");
 * let args = vec!["-a"];
 *
 * ls(current_dir, &args);
 * ```
 */
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

    let mut entries = read_dir(current_dir)?;
    let mut entries: Vec<_> = entries
        .filter_map(|entry| entry.ok())
        .collect();
    entries.sort_by_key(|entry| entry.file_name());

    if long_format {
        let total_blocks: u64 = entries
            .iter()
            .map(|entry| entry.metadata().map(|m| m.blocks()).unwrap_or(0))
            .sum();
        println!("total {}", total_blocks / 2);
    }

    for entry in entries {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        if !show_hidden && name_str.starts_with(".") {
            continue;
        }

        let metadata = entry.metadata()?;

        if long_format {
            let file_type = if metadata.is_dir() { "d" } else { "-" };
            let permissions = permissions_string(&metadata.permissions());
            let nlink = metadata.nlink();
            let uid = metadata.uid();
            let gid = metadata.gid();
            let size = metadata.len();
            let modified = metadata.modified()?;
            let datetime = format_datetime(modified);

            let mut name_display = name_str.to_string();
            if show_indicator {
                if metadata.is_dir() {
                    name_display.push('/');
                } else if metadata.permissions().mode() & 0o111 != 0 {
                    name_display.push('*');
                }
            }

            println!(
                "{}{} {:>2} {} {} {:>8} {} {}",
                file_type,
                permissions,
                nlink,
                uid,
                gid,
                size,
                datetime,
                name_display
            );
        } else {
            let mut name_display = name_str.to_string();
            if show_indicator {
                if metadata.is_dir() {
                    name_display.push('/');
                } else if metadata.permissions().mode() & 0o111 != 0 {
                    name_display.push('*');
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