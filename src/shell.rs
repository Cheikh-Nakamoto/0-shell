use std::env;
use std::fs::metadata;
use std::io::{Error, ErrorKind};
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
use crate::utils::error::ShellError;

pub struct Shell {
    current_dir: PathBuf
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            current_dir: env::current_dir().unwrap_or_else(|_| PathBuf::from("/"))
        }
    }

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
            "cat" => cat(&self.current_dir, &parts[1..]),
            "cp" => cp(&self.current_dir, &parts[1..]),
            "mv" => mv(&self.current_dir, &parts[1..]),
            "rm" => rm(&self.current_dir, &parts[1..]),
            "exit" => Ok(()),
            cmd => Err(ShellError::CommandNotFound(cmd.to_owned())),
        }.expect("Failed to execute command");

        Ok(())
    }

    pub fn cd(&mut self, args: &[&str]) -> Result<(), ShellError> {
        let new_dir = match args.get(0) {
            /*Some(&"") | None => dirs::home_dir().ok_or_else(|| {
                ShellError::IoError(Error::new(
                    ErrorKind::NotFound,
                    "No home directory found",
                ))
            })?,*/
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

        if let Ok(metadata) = metadata(&new_dir) {
            if metadata.is_dir() {
                self.current_dir = new_dir;
                Ok(())
            } else {
                Err(ShellError::InvalidArguments("Not a directory".to_owned()))
            }
        } else {
            Err(ShellError::InvalidArguments("Directory not found".to_owned()))
        }
    }
}

fn get_home_dir() -> Result<PathBuf, ShellError> {
    #[cfg(unix)]
    let home_var = "HOME";
    #[cfg(windows)]
    let home_var = "USERPROFILE";

    env::var_os(home_var)
        .map(PathBuf::from)
        .ok_or_else(|| ShellError::IoError(Error::new(
            ErrorKind::NotFound,
            "No home directory found",
        )))
}