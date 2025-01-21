use std::fs::{remove_dir_all, remove_file};
use std::path::Path;
use crate::utils::error::ShellError;

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
        return Err(ShellError::InvalidArguments("No file or directory specified".to_owned()));
    }

    let recursive = args[0] == "-r";
    let files = if recursive { &args[1..] } else { args };

    for file in files {
        let path = current_dir.join(file);
        if recursive {
            remove_dir_all(&path)?;
        } else {
            remove_file(&path)?;
        }
    }

    Ok(())
}