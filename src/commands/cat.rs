use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::utils::error::ShellError;

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
        return Err(ShellError::InvalidArguments("No file specified".to_owned()));
    }

    for file_name in args {
        let file_path = current_dir.join(file_name);
        let mut file = File::open(&file_path)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;
        print!("{}", contents);
    }

    Ok(())
}