use std::fs::copy;
use std::path::Path;
use crate::utils::error::ShellError;

/**
    * Copy a file from one location to another
    *
    * # Arguments
    * * `current_dir` - The current directory
    * * `args` - The arguments to the command
    *
    * # Example
    * ```rust
    * use std::path::Path;
    * use shell_emulator::commands::cp;
    *
    * let current_dir = Path::new("/home/user");
    * let args = vec!["file1.txt", "file2.txt"];
    *
    * cp(current_dir, &args);
    * ```
*/
pub fn cp(current_dir: &Path, args: &[&str]) -> Result<(), ShellError> {
    if args.len() != 2 {
        return Err(ShellError::InvalidArguments("'cp' requires source and destination".to_owned()));
    }

    let source = current_dir.join(args[0]);
    let destination = current_dir.join(args[1]);

    copy(&source, &destination)?;

    Ok(())
}