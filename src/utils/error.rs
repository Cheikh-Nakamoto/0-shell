use std::fmt::{Display, Formatter};
use std::io::Error;

#[derive(Debug)]
pub enum ShellError {
    IoError(Error),
    CommandNotFound(String),
    InvalidArguments(String),
}

impl From<Error> for ShellError {
    fn from(err: Error) -> Self {
        ShellError::IoError(err)
    }
}

impl Display for ShellError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellError::IoError(e) => write!(f, "IO Error: {}", e),
            ShellError::CommandNotFound(cmd) => write!(f, "Command <{}> not found", cmd),
            ShellError::InvalidArguments(msg) => write!(f, "Invalid arguments: {}", msg),
        }
    }
}