use std::fs::rename;
use std::path::Path;
use crate::utils::error::ShellError;

/**
    * Move a file or directory from one location to another.
    *
    * # Arguments
    * * `current_dir` - A reference to the current directory.
    * * `args` - A slice of arguments. The first argument is the source file or directory, and the second argument is the destination file or directory.
    *
    * # Examples
    * ```
    * mv(&Path::new("/"), &["file.txt", "new_file.txt"]);
    * ```
*/
pub fn mv(current_dir: &Path, args: &[&str]) -> Result<(), ShellError> {
    if args.len() != 2 {
        return Err(ShellError::InvalidArguments("'mv' requires source and destination".to_owned()));
    }

    let source = current_dir.join(args[0]);
    let destination = current_dir.join(args[1]);

    rename(&source, &destination)?;

    Ok(())
}