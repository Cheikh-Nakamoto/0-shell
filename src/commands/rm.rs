use crate::utils::error::ShellError;
use crate::utils::messages::{CANNOT_REMOVE, IS_A_DIRECTORY, NOT_SPECIFIED};
use std::fs::{remove_dir_all, remove_file};
use std::path::Path;

/**
 * Remove files or directories.
 *
 * # Arguments
 * * `current_dir` - The current directory.
 * * `args` - The arguments passed to the command.
 *
 * # Examples
 * ```
 * use std::path::Path;
 * use shell::commands::rm;
 *
 * let current_dir = Path::new("/tmp");
 * let args = ["file.txt"];
 * let result = rm(current_dir, &args);
 * assert!(result.is_ok());
 * ```
*/
pub fn rm(current_dir: &Path, args: &[&str]) -> Result<(), ShellError> {
    if args.is_empty() {
        return Err(ShellError::InvalidArguments(format!("rm: {NOT_SPECIFIED}")));
    }

    let mut is_recursive = false;

    if args[0] == "-r" {
        is_recursive = true;
    }

    if is_recursive && args.len() < 2 {
        return Err(ShellError::InvalidArguments(format!("rm: {NOT_SPECIFIED}")));
    }

    let files = if is_recursive { &args[1..] } else { args };

    for file in files {
        let path = current_dir.join(file);
        if is_recursive {
            remove_dir_all(&path)?;
        } else {
            remove_file(&path).map_err(|_| ShellError::InvalidArguments(format!("rm: {CANNOT_REMOVE} '{}': {IS_A_DIRECTORY}", file)))?;
        }
    }

    Ok(())
}