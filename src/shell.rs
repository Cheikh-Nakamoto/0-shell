use crate::commands::{
    cat::cat,
    cp::cp,
    echo::echo,
    exit::clear,
    ls::ls,
    mkdir::mkdir,
    mv::mv,
    pwd::pwd,
    rm::rm,
    touch::touch,
};
use crate::utils::error::ShellError;
use crate::utils::messages::{CANNOT_ACCESS, DIR_NOT_FOUND, MANY_ARGS, NOT_A_DIRECTORY, NO_SUCH_FILE};
use crate::utils::path::get_home_dir;
use std::env;
use std::fs::metadata;
use std::io::Error;
use std::path::PathBuf;

/**
 * The shell struct.
 *
 * The shell keeps track of the current directory and executes commands.
 */
pub struct Shell {
    current_dir: PathBuf,
}

impl Shell {
    /**
     * Create a new shell instance with the current directory set to the current working directory.
     */
    pub fn new() -> Self {
        Shell {
            current_dir: env::current_dir().unwrap_or_else(|_| PathBuf::from("/"))
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
            "cd" => self.cd(&parts[1..]),
            "pwd" => pwd(&self.current_dir),
            "ls" => ls(&self.current_dir, &parts[1..]),
            "echo" => echo(&parts[1..]),
            "mkdir" => mkdir(&self.current_dir, &parts[1..]),
            "touch" => touch(&self.current_dir, &parts[1..]),
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
        if args.len() > 1 {
            return Err(ShellError::InvalidArguments(format!("cd: {}", MANY_ARGS)));
        }

        let new_dir = match args.get(0) {
            Some(&"") | None => {
                get_home_dir()?
            }
            Some(&path) => {
                if path.starts_with('/') {
                    PathBuf::from(path)
                } else {
                    self.current_dir.join(path)
                }
            }
        };

        if let Ok(metadata) = metadata(&new_dir) {
            let mut back: Option<String> = None;
            if new_dir.ends_with("..") {
                let p = new_dir.to_str().unwrap();
                let mut path_parts: Vec<&str> = p.split("/").collect();
                if !path_parts.is_empty() {
                    path_parts.pop();
                    if !path_parts.is_empty() {
                        path_parts.pop();
                    }
                }
                back = Some(path_parts.join("/"));
            }
            
            if metadata.is_dir() {
                env::set_current_dir(&new_dir).map_err(|e| {
                    ShellError::IoError(Error::new(
                        e.kind(),
                        format!("cd: {CANNOT_ACCESS} '{}': {NO_SUCH_FILE}", new_dir.display()),
                    ))
                })?;

                if let Some(step_back) = back {
                    self.current_dir = PathBuf::from(step_back);
                } else {
                    self.current_dir = new_dir;
                }

                Ok(())
            } else {
                Err(ShellError::InvalidArguments(format!("cd: {}", NOT_A_DIRECTORY)))
            }
        } else {
            Err(ShellError::InvalidArguments(format!("cd: {}", DIR_NOT_FOUND)))
        }
    }
}
