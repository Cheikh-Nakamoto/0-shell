use std::fs::create_dir;
use std::path::Path;
use crate::utils::error::ShellError;

pub fn mkdir(current_dir: &Path, args: &[&str]) -> Result<(), ShellError> {
    if args.is_empty() {
        return Err(ShellError::InvalidArguments("No directory name provided".to_owned()))
    }

    for dir_name in args {
        let new_dir = current_dir.join(dir_name);
        create_dir(&new_dir)?;
    }

    Ok(())
}