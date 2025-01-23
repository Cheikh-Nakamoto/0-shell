use std::path::Path;
use crate::utils::error::ShellError;

/**
    * Print the current directory.
    *
    * # Arguments
    * * `current_dir` - The current directory.
    *
    * # Examples
    * ```
    * use std::path::Path;
    * use shell::commands::pwd;
    *
    * let current_dir = Path::new("/tmp");
    * let result = pwd(current_dir);
    * assert!(result.is_ok());
    * ```
*/
pub fn pwd(current_dir: &Path) -> Result<(), ShellError> {
    println!("{}", current_dir.display());
    Ok(())
}