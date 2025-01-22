use std::fs::{copy, create_dir_all};
use std::io::{Error, ErrorKind};
use std::path::Path;
use crate::utils::error::ShellError;

/**
    * Copy a file or directory to another location.
    *
    * # Arguments
    * * `current_dir` - The current directory.
    * * `args` - The arguments passed to the command.
    *
    * # Example
    * ```rust
    * use std::path::Path;
    * use shell::commands::cp;
    *
    * let current_dir = Path::new("/home/user");
    * let args = vec!["file.txt", "new_file.txt"];
    *
    * cp(current_dir, &args);
    * ```
*/
pub fn cp(current_dir: &Path, args: &[&str]) -> Result<(), ShellError> {
    if args.len() != 2 {
        return Err(ShellError::InvalidArguments("cp: requires source and destination".to_owned()));
    }

    let source = current_dir.join(args[0]);
    let destination = current_dir.join(args[1]);

    if !source.exists() {
        return Err(ShellError::IoError(Error::new(
            ErrorKind::NotFound,
            format!("cp: cannot stat '{}': No such file or directory", args[0]),
        )));
    }

    let destination = if destination.is_dir() {
        let file_name = source.file_name().ok_or_else(|| {
            ShellError::IoError(Error::new(
                ErrorKind::InvalidInput,
                "cp: source has no file name",
            ))
        })?;
        destination.join(file_name)
    } else {
        destination
    };

    if let Some(parent) = destination.parent() {
        if !parent.exists() {
            create_dir_all(parent).map_err(|e| ShellError::IoError(Error::new(
                e.kind(),
                format!("cp: failed to create directory '{}'", parent.display()),
            )))?;
        }
    }

    copy(&source, &destination).map_err(|e| ShellError::IoError(Error::new(
        e.kind(),
        format!("cp: failed to copy '{}' to '{}'", args[0], args[1]),
    )))?;

    Ok(())
}