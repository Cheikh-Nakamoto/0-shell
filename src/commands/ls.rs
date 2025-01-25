use crate::utils::color::get_color;
use crate::utils::date::format_datetime;
use crate::utils::error::ShellError;
use crate::utils::messages::{CANNOT_ACCESS, INVALID_FLAG, NOTHING, NO_SUCH_FILE};
use crate::utils::utils::{extrac_guid_from_file, permissions_string};
use std::fs::{metadata, read_dir, read_link, DirEntry, Metadata};
use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};

/**
 * A custom entry struct to store the name and metadata of a directory entry.
 */
struct CustomEntry {
    name: String,
    metadata: Metadata,
}

/**
 * List files and directories.
 *
 * # Arguments
 * * `base_dir` - The base directory.
 * * `args` - The arguments passed to the command.
 *
 * # Example
 * ```rust
 * use std::path::Path;
 * use shell::commands::ls;
 *
 * let base_dir = Path::new("/home/user");
 * let args = vec!["-a"];
 *
 * ls(base_dir, &args);
 * ```
 */
pub fn ls(base_dir: &Path, args: &[&str]) -> Result<(), ShellError> {
    let mut show_hidden = false;
    let mut long_format = false;
    let mut show_indicator = false;
    let mut target_dir = base_dir;

    for arg in args {
        match *arg {
            "-a" => show_hidden = true,
            "-l" => long_format = true,
            "-F" => show_indicator = true,
            _ if arg.starts_with('-') => {
                return Err(ShellError::InvalidArguments(INVALID_FLAG.to_string()));
            }
            _ => {
                let path = Path::new(arg);
                if path.is_dir() {
                    target_dir = path;
                } else {
                    return Err(ShellError::InvalidArguments(format!(
                        "ls: {CANNOT_ACCESS} '{}': {NO_SUCH_FILE}",
                        arg
                    )));
                }
            }
        }
    }

    let entries: Vec<DirEntry> = read_dir(target_dir)?
        .filter_map(|entry| entry.ok())
        .collect();

    let mut all_entries: Vec<CustomEntry> = entries
        .into_iter()
        .map(|entry| CustomEntry {
            name: entry.file_name().to_string_lossy().to_string(),
            metadata: entry.metadata().unwrap(),
        })
        .collect();

    if show_hidden {
        all_entries.insert(
            0,
            CustomEntry {
                name: "..".to_string(),
                metadata: metadata(target_dir.join(".."))?,
            },
        );
        all_entries.insert(
            0,
            CustomEntry {
                name: ".".to_string(),
                metadata: metadata(target_dir)?,
            },
        );
    }

    all_entries.sort_by(|a, b| a.name.cmp(&b.name));

    let mut block_size = 0;
    for entry in &all_entries {
        if !show_hidden && entry.name.starts_with('.') {
            continue;
        }
        block_size += entry.metadata.blocks();
    }

    if long_format {
        println!("total {}", block_size / 2);
    }

    for entry in all_entries {
        let name = entry.name;
        let metadata = entry.metadata;

        if !show_hidden && name.starts_with('.') {
            continue;
        }

        let file_type = if metadata.is_dir() {
            "d"
        } else if metadata.file_type().is_symlink() {
            "l"
        } else if metadata.file_type().is_char_device() {
            "c"
        } else if metadata.file_type().is_block_device() {
            "b"
        } else if metadata.file_type().is_fifo() {
            "p"
        } else if metadata.file_type().is_socket() {
            "s"
        } else {
            "-"
        };

        let color = get_color(file_type, &metadata.permissions());
        let reset_color = "\x1b[0m";

        let permissions = permissions_string(&metadata.permissions());
        let nlink = metadata.nlink();
        let uid = metadata.uid();
        let gid = metadata.gid();
        let size = if file_type == "c" || file_type == "b" {
            let major = (metadata.rdev() >> 8) & 0xfff;
            let minor = metadata.rdev() & 0xff;
            format!("{:>3}, {:>8}", major, minor)
        } else {
            format!("{:>13}", metadata.len())
        };

        let modified = metadata.modified()?;
        let datetime = format_datetime(modified);

        let mut name_display = name.clone();
        if metadata.file_type().is_symlink() {
            let target = read_link(target_dir.join(&name))
                .unwrap_or_else(|_| PathBuf::from(NOTHING))
                .to_string_lossy()
                .to_string();
            if long_format {
                name_display = format!("{} -> {}", name, target);
            } else {
                name_display = format!("{}", name);
            }
        } else if show_indicator {
            if metadata.is_dir() {
                name_display.push('/');
            } else if metadata.permissions().mode() & 0o111 != 0 {
                name_display.push('*');
            }
        }

        if long_format {
            println!(
                "{:<1}{:<9} {:>2} {:<7} {:<7} {} {} {}{}{}",
                file_type, permissions, nlink, extrac_guid_from_file(uid, "/etc/passwd"), extrac_guid_from_file(gid, "/etc/group"), size, datetime, color, name_display, reset_color
            );
        } else {
            print!("{}{}{}  ", color, name_display, reset_color);
        }
    }

    if !long_format {
        println!();
    }

    Ok(())
}
