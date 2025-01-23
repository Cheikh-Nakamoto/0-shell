use std::fs::create_dir;
use std::path::Path;
use crate::utils::error::ShellError;
use crate::utils::messages::NO_DIRECTORY_SPECIFIED;

/**
    * Create directories.
    *
    * # Arguments
    * * `current_dir` - The current directory.
    * * `args` - The arguments passed to the command.
    *
    * # Examples
    * ```
    * use std::path::Path;
    * use shell::commands::mkdir;
    *
    * let current_dir = Path::new("/tmp");
    * let args = ["dir1", "dir2"];
    * let result = mkdir(current_dir, &args);
    * assert!(result.is_ok());
    * ```
*/
pub fn mkdir(current_dir: &Path, args: &[&str]) -> Result<(), ShellError> {
    if args.is_empty() {
        return Err(ShellError::InvalidArguments(NO_DIRECTORY_SPECIFIED.to_owned()))
    }

    for dir_name in args {
        let new_dir = current_dir.join(dir_name);
        create_dir(&new_dir)?;
    }

    Ok(())
}