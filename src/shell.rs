use std::env;
use std::fs::metadata;
use std::io::Error;
use std::path::PathBuf;
use crate::commands::{
    ls::ls,
    pwd::pwd,
    echo::echo,
    mkdir::mkdir,
    cat::cat,
    cp::cp,
    mv::mv,
    rm::rm
};
use crate::commands::exit::clear;
use crate::utils::error::ShellError;
use crate::utils::messages::{CANNOT_ACCESS, DIR_NOT_FOUND, MANY_ARGS, NOT_A_DIRECTORY, NO_SUCH_FILE};
use crate::utils::path::get_home_dir;

/**
 * The shell struct.
 *
 * The shell keeps track of the current directory and executes commands.
 */
pub struct Shell {
    current_dir: PathBuf
}

impl Shell {
    /**
     * Create a new shell instance with the current directory set to the current working directory.
     */
    pub fn new() -> Self {
        Shell {
            current_dir: env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        }
    }

    /**
     * Execute a command in the shell.
     *
     * The command is split into parts by whitespace, and the first part is used as the command name.
     */
    pub fn execute(&mut self, input: &str) -> Result<(), ShellError> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }

        match parts[0] {
            "cd" => {
                if parts.len() > 1 {
                    let new_dir = parts[1];
                    if let Err(_) = env::set_current_dir(new_dir) {
                        return Err(ShellError::InvalidArguments(format!("cd: {}", MANY_ARGS)));
                    }
                } else {
                    let home_dir = env::var("HOME").map_err(|_| ShellError::HomeDirNotFound)?;
                    if let Err(e) = env::set_current_dir(home_dir) {
                        return Err(ShellError::IoError(e));
                    }
                    self.cd(&parts[1..])?;
                }
                Ok(())
            },
            "pwd" => pwd(&self.current_dir),
            "ls" => ls(&self.current_dir, &parts[1..]),
            "echo" => echo(&parts[1..]),
            "mkdir" => mkdir(&self.current_dir, &parts[1..]),
            "cat" => cat(&self.current_dir, &parts[1..]),
            "cp" => cp(&self.current_dir, &parts[1..]),
            "mv" => mv(&self.current_dir, &parts[1..]),
            "rm" => rm(&self.current_dir, &parts[1..]),
            "clear" => Ok(clear()),
            _ => Err(ShellError::CommandNotFound(parts[0].to_owned())),
        }?;

        Ok(())
    }

    /**
     * Change the current directory of the shell.
     *
     * If no arguments are provided, the shell will change to the home directory.
     */
    pub fn cd(&mut self, args: &[&str]) -> Result<(), ShellError> {
        let new_dir = match args.get(0) {
            Some(&"") | None => {
                get_home_dir()?
            },
            Some(&path) => {
                let new_path = if path.starts_with('/') {
                    PathBuf::from(path)
                } else {
                    self.current_dir.join(path)
                };
                new_path
            }
        };

        let resolved_dir = new_dir.canonicalize().map_err(|e| {
            ShellError::IoError(Error::new(
                e.kind(),
                format!("cd: {CANNOT_ACCESS} '{}': {NO_SUCH_FILE}", new_dir.display()),
            ))
        })?;

        if let Ok(metadata) = metadata(&resolved_dir) {
            if metadata.is_dir() {
                self.current_dir = resolved_dir;
                Ok(())
            } else {
                Err(ShellError::InvalidArguments(NOT_A_DIRECTORY.to_owned()))
            }
        } else {
            Err(ShellError::InvalidArguments(DIR_NOT_FOUND.to_owned()))
        }
    }
}
