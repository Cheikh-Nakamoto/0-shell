use std::path::Path;
use crate::utils::error::ShellError;

pub fn pwd(current_dir: &Path) -> Result<(), ShellError> {
    println!("{}", current_dir.display());
    Ok(())
}