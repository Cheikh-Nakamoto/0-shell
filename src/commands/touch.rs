use crate::utils::error::ShellError;
use crate::utils::messages::{CANNOT_CREATE_FILE, CANNOT_UPDATE_FILE, NO_FILE_SPECIFIED};
use std::fs::OpenOptions;
use std::io::Error;
use std::path::Path;

/**
 * Create a file if it does not exist, or update the access and modification times if it does.
 *
 * # Arguments
 * * `current_dir` - The current directory.
 * * `args` - The arguments passed to the command.
 *
 * # Example
 * ```rust
 * use std::path::Path;
 * use shell::commands::touch;
 *
 * let current_dir = Path::new("/home/user");
 * let args = vec!["file.txt"];
 *
 * touch(current_dir, &args);
 * ```
 */
pub fn touch(current_dir: &Path, args: &[&str]) -> Result<(), ShellError> {
    if args.is_empty() {
        return Err(ShellError::InvalidArguments(format!("touch: {NO_FILE_SPECIFIED}")));
    }

    for file_name in args {
        let file_path = current_dir.join(file_name);

        if file_path.exists() {
            OpenOptions::new()
                .write(true)
                .open(&file_path)
                .map_err(|e| ShellError::IoError(Error::new(
                    e.kind(),
                    format!("touch: {CANNOT_UPDATE_FILE} {}", file_name),
                )))?;
        } else {
            OpenOptions::new()
                .create(true)
                .write(true)
                .open(&file_path)
                .map_err(|e| ShellError::IoError(Error::new(
                    e.kind(),
                    format!("touch: {CANNOT_CREATE_FILE} {}", file_name),
                )))?;
        }
    }

    Ok(())
}