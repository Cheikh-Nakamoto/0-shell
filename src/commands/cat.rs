use crate::utils::error::ShellError;
use crate::utils::messages::{NO_FILE_SPECIFIED, NO_SUCH_FILE};
use std::fs::File;
use std::io::Read;
use std::path::Path;

/**
 * Display the contents of a file.
 *
 * # Arguments
 * * `current_dir` - The current directory.
 * * `args` - The arguments passed to the command.
 *
 * # Example
 * ```rust
 * use std::path::Path;
 * use shell::commands::cat;
 *
 * let current_dir = Path::new("/home/user");
 * let args = vec!["file.txt"];
 *
 * cat(current_dir, &args);
 * ```
*/
pub fn cat(current_dir: &Path, args: &[&str]) -> Result<(), ShellError> {
    if args.is_empty() {
        return Err(ShellError::InvalidArguments(NO_FILE_SPECIFIED.to_owned()));
    }

    for file_name in args {
        let file_path = current_dir.join(file_name);
        let mut file = File::open(&file_path).map_err(|_| ShellError::InvalidArguments(format!("cat: {}: {NO_SUCH_FILE}", file_name)))?;
        let mut contents = String::new();

        file.read_to_string(&mut contents).map_err(|_| ShellError::InvalidArguments(format!("cat: {}: {NO_SUCH_FILE}", file_name)))?;
        print!("{}", contents);
    }
    println!();

    Ok(())
}