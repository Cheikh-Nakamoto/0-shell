use std::fs::{copy, create_dir_all, remove_dir_all, remove_file, rename};
use std::io::{Error, ErrorKind};
use std::path::Path;
use crate::commands::cp::copy_dir_all;
use crate::utils::error::ShellError;
use crate::utils::messages::{CANNOT_CREATE_DIR, CANNOT_MOVE, CANNOT_REMOVE, CANNOT_STAT, NO_SUCH_FILE, REQUIRE_SOURCE_DEST, SOURCE_HAS_NO_FILE_NAME};

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
        return Err(ShellError::InvalidArguments(format!("mv: {REQUIRE_SOURCE_DEST}")));
    }

    let source = current_dir.join(args[0]);
    let destination = current_dir.join(args[1]);

    if !source.exists() {
        return Err(ShellError::IoError(Error::new(
            ErrorKind::NotFound,
            format!("mv: {CANNOT_STAT} '{}': {NO_SUCH_FILE}", args[0]),
        )));
    }

    let destination = if destination.is_dir() {
        let file_name = source.file_name().ok_or_else(|| {
            ShellError::IoError(Error::new(
                ErrorKind::InvalidInput,
                format!("mv: {SOURCE_HAS_NO_FILE_NAME}"),
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
                format!("mv: {CANNOT_CREATE_DIR} '{}'", parent.display()),
            )))?;
        }
    }

    if let Err(e) = rename(&source, &destination) {
        if e.kind() == ErrorKind::Other && e.to_string().contains("Cross-device link") {
            if source.is_dir() {
                copy_dir_all(&source, &destination).map_err(|e| ShellError::IoError(Error::new(
                    e.kind(),
                    format!("mv: {CANNOT_MOVE} '{}' to '{}'", args[0], args[1]),
                )))?;
                remove_dir_all(&source).map_err(|e| ShellError::IoError(Error::new(
                    e.kind(),
                    format!("mv: {CANNOT_REMOVE} '{}'", args[0]),
                )))?;
            } else {
                copy(&source, &destination).map_err(|e| ShellError::IoError(Error::new(
                    e.kind(),
                    format!("mv: {CANNOT_MOVE} '{}' to '{}'", args[0], args[1]),
                )))?;
                remove_file(&source).map_err(|e| ShellError::IoError(Error::new(
                    e.kind(),
                    format!("mv: {CANNOT_REMOVE} '{}'", args[0]),
                )))?;
            }
        } else {
            return Err(ShellError::IoError(Error::new(
                e.kind(),
                format!("mv: {CANNOT_MOVE} '{}' to '{}'", args[0], args[1]),
            )));
        }
    }

    Ok(())
}