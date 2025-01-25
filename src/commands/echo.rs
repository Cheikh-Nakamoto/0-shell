use crate::utils::error::ShellError;
use crate::utils::messages::COMMAND_NOT_FOUND;

/**
 * Print the given arguments to the console.
 *
 * # Arguments
 * * `args` - The arguments to print.
 *
 * # Example
 * ```rust
 * use shell::commands::echo;
 *
 * let args = vec!["Hello, world!"];
 *
 * echo(&args);
 * ```
 *
 * # Output
 * ```sh
 * Hello, world!
 * ```
*/
pub fn echo(args: &[&str]) -> Result<(), ShellError> {
    if args.is_empty() {
        println!();
        return Ok(());
    }

    let mut arguments = Vec::new();
    for arg in args {
        if arg.starts_with('"') && arg.ends_with('"') {
            arguments.push(&arg[1..arg.len() - 1]);
        } else {
            arguments.push(arg);
        }
    }

    let mut text = arguments.join(" ");
    if text.starts_with('"') && text.ends_with('"') {
        text = text[1..text.len() - 1].to_string();
    } else if text.starts_with('\'') && text.ends_with('\'') {
        text = text[1..text.len() - 1].to_string();
    } else if text.starts_with('`') && text.ends_with('`') {
        return Err(ShellError::InvalidArguments(format!("???: {COMMAND_NOT_FOUND}: {}", text)));
    }

    println!("{}", escape(text));
    Ok(())
}

fn escape(str: String) -> String {
    str.replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\r", "\r")
        .replace("\\0", "\0")
        .replace("!\"", "")
}