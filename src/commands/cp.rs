use std::fs::{copy, create_dir_all, read_dir};
use std::io::{Error, ErrorKind};
use std::path::Path;
use crate::utils::error::ShellError;
use crate::utils::messages::{CANNOT_COPY, CANNOT_CREATE_DIR, CANNOT_STAT, NO_SUCH_FILE, REQUIRE_SOURCE_DEST, SOURCE_HAS_NO_FILE_NAME};

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
        return Err(ShellError::InvalidArguments(format!("cp: {REQUIRE_SOURCE_DEST}")));
    }

    let source = current_dir.join(args[0]);
    let destination = current_dir.join(args[1]);

    if !source.exists() {
        return Err(ShellError::IoError(Error::new(
            ErrorKind::NotFound,
            format!("cp: {CANNOT_STAT} '{}': {NO_SUCH_FILE}", args[0]),
        )));
    }

    let destination = if destination.is_dir() {
        let file_name = source.file_name().ok_or_else(|| {
            ShellError::IoError(Error::new(
                ErrorKind::InvalidInput,
                format!("cp: {SOURCE_HAS_NO_FILE_NAME}"),
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
                format!("cp: {CANNOT_CREATE_DIR} '{}'", parent.display()),
            )))?;
        }
    }

    if source.is_dir() {
        copy_dir_all(&source, &destination).map_err(|e| ShellError::IoError(Error::new(
            e.kind(),
            format!("cp: {CANNOT_COPY} '{}' to '{}'", args[0], args[1]),
        )))?;
    } else {
        copy(&source, &destination).map_err(|e| ShellError::IoError(Error::new(
            e.kind(),
            format!("cp: {CANNOT_COPY} '{}' to '{}'", args[0], args[1]),
        )))?;
    }

    Ok(())
}

pub fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), Error> {
    if !dst.exists() {
        create_dir_all(dst)?;
    }

    for entry in read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst_path)?;
        } else {
            copy(&entry.path(), &dst_path)?;
        }
    }

    Ok(())
}