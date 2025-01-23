use std::fs::{metadata, read_dir, read_link, DirEntry, Metadata};
use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use crate::utils::color::get_color;
use crate::utils::date::format_datetime;
use crate::utils::error::ShellError;
use crate::utils::messages::{INVALID_FLAG, NOTHING};
use crate::utils::utils::{gid_to_name, permissions_string, uid_to_name};

/**
 * A custom entry struct to store the name and metadata of a directory entry.
 */
struct CustomEntry {
    name: String,
    metadata: Metadata,
}

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
 *
 * # Output
    * ```sh
    * $ ls -a
    * .  ..  file.txt  directory/
    * ```
 */
pub fn ls(current_dir: &Path, args: &[&str]) -> Result<(), ShellError> {
    let mut show_hidden = false;
    let mut long_format = false;
    let mut show_indicator = false;

    // Parse the arguments to determine the options
    for arg in args {
        match *arg {
            "-a" => show_hidden = true,
            "-l" => long_format = true,
            "-F" => show_indicator = true,
            _ => return Err(ShellError::InvalidArguments(INVALID_FLAG.to_string())),
        }
    }

    // Read the directory entries
    let entries: Vec<DirEntry> = read_dir(current_dir)?
        .filter_map(|entry| entry.ok())
        .collect();

    let dotdot_entry = CustomEntry {
        name: "..".to_string(),
        metadata: metadata(current_dir.join(".."))?,
    };
    let dot_entry = CustomEntry {
        name: ".".to_string(),
        metadata: metadata(current_dir)?,
    };

    // Create a list of custom entries
    let mut all_entries: Vec<CustomEntry> = entries
        .into_iter()
        .map(|entry| CustomEntry {
            name: entry.file_name().to_string_lossy().to_string(),
            metadata: entry.metadata().unwrap(),
        })
        .collect();

    if show_hidden {
        all_entries.insert(0, dot_entry);
        all_entries.insert(0, dotdot_entry);
    }

    all_entries.sort_by(|a, b| a.name.cmp(&b.name));

    // Calculate the total block size
    let mut block_size = 0;
    for entry in &all_entries {
        block_size += entry.metadata.blocks() / 2;
    }

    if long_format {
        println!("total {}", block_size);
    }

    for entry in all_entries {
        let name = entry.name;
        let metadata = entry.metadata;

        if !show_hidden && name.starts_with(".") {
            continue;
        }

        if long_format {
            // Determine the file type
            let file_type = if metadata.is_dir() {
                "d"
            } else if metadata.file_type().is_symlink() {
                "l"
            } else if metadata.file_type().is_char_device() {
                "c"
            } else if metadata.file_type().is_block_device() {
                "b"
            } else {
                "-"
            };

            let permissions = permissions_string(&metadata.permissions());

            let mut name_display = name.to_string();
            if metadata.file_type().is_symlink() {
                let target = read_link(current_dir.join(&name))
                    .unwrap_or_else(|_| PathBuf::from(NOTHING))
                    .to_string_lossy()
                    .to_string();
                name_display = format!("{} -> {}", name, target);
            } else if show_indicator {
                if metadata.is_dir() {
                    name_display.push('/');
                } else if metadata.permissions().mode() & 0o111 != 0 {
                    name_display.push('*');
                }
            }

            let color = get_color(file_type, &metadata.permissions());
            let reset_color = "\x1b[0m";

            let nlink = metadata.nlink();
            let uid = metadata.uid();
            let gid = metadata.gid();
            let size = metadata.len();
            let modified = metadata.modified()?;
            let datetime = format_datetime(modified);

            println!(
                "{}{} {:>2} {:>8} {:>8} {:>8} {} {}{}{}",
                file_type,
                permissions,
                nlink,
                uid_to_name(uid),
                gid_to_name(gid),
                size,
                datetime,
                color,
                name_display,
                reset_color
            );
        } else {
            let file_type = if metadata.is_dir() {
                "d"
            } else if metadata.file_type().is_symlink() {
                "l"
            } else if metadata.file_type().is_char_device() {
                "c"
            } else if metadata.file_type().is_block_device() {
                "b"
            } else {
                "-"
            };

            let color = get_color(file_type, &metadata.permissions());
            let reset_color = "\x1b[0m";

            let mut name_display = name.to_string();
            if show_indicator {
                if metadata.is_dir() {
                    name_display.push('/');
                } else if metadata.permissions().mode() & 0o111 != 0 {
                    name_display.push('*');
                }
            }

            print!("{}{}{}  ", color, name_display, reset_color);
        }
    }

    if !long_format {
        println!();
    }

    Ok(())
}