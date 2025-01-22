use std::fs::{copy, create_dir_all, remove_file, rename};
use std::io::{Error, ErrorKind};
use std::path::Path;
use crate::utils::error::ShellError;

/**
    * Move a file or directory to another location.
    *
    * # Arguments
    * * `current_dir` - The current directory.
    * * `args` - The arguments passed to the command.
    *
    * # Example
    * ```rust
    * use std::path::Path;
    * use shell::commands::mv;
    *
    * let current_dir = Path::new("/home/user");
    * let args = vec!["file.txt", "new_file.txt"];
    *
    * mv(current_dir, &args);
    * ```
*/
pub fn mv(current_dir: &Path, args: &[&str]) -> Result<(), ShellError> {
    if args.len() != 2 {
        return Err(ShellError::InvalidArguments("mv: requires source and destination".to_owned()));
    }

    let source = current_dir.join(args[0]);
    let destination = current_dir.join(args[1]);

    if !source.exists() {
        return Err(ShellError::IoError(Error::new(
            ErrorKind::NotFound,
            format!("mv: cannot stat '{}': No such file or directory", args[0]),
        )));
    }

    let destination = if destination.is_dir() {
        let file_name = source.file_name().ok_or_else(|| {
            ShellError::IoError(Error::new(
                ErrorKind::InvalidInput,
                "mv: source has no file name",
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
                format!("mv: failed to create directory '{}'", parent.display()),
            )))?;
        }
    }

    copy(&source, &destination).map_err(|e| ShellError::IoError(Error::new(
        e.kind(),
        format!("cp: failed to copy '{}' to '{}'", args[0], args[1]),
    )))?;

    if let Err(e) = rename(&source, &destination) {
        if e.kind() == ErrorKind::Other && e.to_string().contains("Cross-device link") {
            copy(&source, &destination).map_err(|e| ShellError::IoError(Error::new(
                e.kind(),
                format!("mv: failed to copy '{}' to '{}'", args[0], args[1]),
            )))?;
            remove_file(&source).map_err(|e| ShellError::IoError(Error::new(
                e.kind(),
                format!("mv: failed to remove '{}'", args[0]),
            )))?;
        } else {
            return Err(ShellError::IoError(Error::new(
                e.kind(),
                format!("mv: failed to move '{}' to '{}'", args[0], args[1]),
            )));
        }
    }

    Ok(())
}