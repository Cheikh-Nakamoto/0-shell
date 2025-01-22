use std::fmt::{Display, Formatter};
use std::io::Error;
use std::error;

#[derive(Debug)]
pub enum ShellError {
    IoError(Error),
    CommandNotFound(String),
    InvalidArguments(String),
}

impl error::Error for ShellError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ShellError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<Error> for ShellError {
    fn from(err: Error) -> Self {
        ShellError::IoError(err)
    }
}

impl Display for ShellError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellError::IoError(e) => write!(f, "\x1b[31m{}\x1b[0m", e),
            ShellError::CommandNotFound(cmd) => write!(f, "\x1b[31mCommand '{}' not found\x1b[0m", cmd),
            ShellError::InvalidArguments(msg) => write!(f, "\x1b[31m{}\x1b[0m", msg),
        }
    }
}