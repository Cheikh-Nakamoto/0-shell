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
        if stdin().read_line(&mut input)? == 0 {
            println!();
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        if let Err(e) = shell.execute(input) {
            eprintln!("{}", e);
        }

        if input == "exit" {
            exit();
        }
    }

    Ok(())
}