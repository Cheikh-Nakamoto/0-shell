use std::io;
use std::io::{stdin, stdout, Write};
use crate::commands::exit::exit;
use crate::shell::Shell;

mod commands;
mod utils;
mod shell;

fn main() -> io::Result<()> {
    let mut shell = Shell::new();

    loop {
        print!("$ ");
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
                eprintln!("\x1b[31mError reading input: {}\x1b[0m", e);
                break;
            }
        }
    }

    Ok(())
}