use crate::commands::exit::exit;
use crate::shell::Shell;
use crate::utils::messages::CANNOT_READ_INPUT;
use std::io::{stdin, stdout, Write};
use std::{env, io};

mod commands;
mod utils;
mod shell;

fn main() -> io::Result<()> {
    let mut shell = Shell::new();

    loop {
        let current_dir = env::current_dir()?;
        let current_dir_display = current_dir.to_string_lossy();

        let home_dir = env::var("HOME").unwrap_or_else(|_| "".to_string());
        let prompt = if current_dir_display.starts_with(&home_dir) {
            format!("~{}", &current_dir_display[home_dir.len()..])
        } else {
            current_dir_display.to_string()
        };

        print!("\x1b[1;33;48;5;238m{}\x1b[0m $ \x1b[38;5;238m\x1b[0m", prompt);
        stdout().flush()?;

        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(0) => {
                println!();
                exit();
            }
            Ok(_) => {
                let input = input.trim();
                if !input.is_empty() {
                    if input == "exit" {
                        exit();
                    }
                    match shell.execute(input) {
                        Ok(_) => (),
                        Err(e) => eprintln!("{}", e),
                    }
                }
            }
            Err(e) => {
                eprintln!("\x1b[31m{CANNOT_READ_INPUT}: {}\x1b[0m", e);
                break;
            }
        }
    }

    Ok(())
}