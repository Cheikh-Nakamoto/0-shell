use std::env;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use crate::utils::error::ShellError;
use crate::utils::messages::NO_HOME_DIRECTORY;

/**
    * Get the home directory of the current user.
*/
pub fn get_home_dir() -> Result<PathBuf, ShellError> {
    #[cfg(unix)]
    let home_var = "HOME";
    #[cfg(windows)]
    let home_var = "USERPROFILE";


    env::var_os(home_var)
        .map(PathBuf::from)
        .ok_or_else(|| ShellError::IoError(Error::new(
            ErrorKind::NotFound,
            NO_HOME_DIRECTORY,
        )))
}