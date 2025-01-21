use crate::utils::error::ShellError;

pub fn echo(args: &[&str]) -> Result<(), ShellError> {
    if args.is_empty() {
        println!();
        return Ok(())
    }

    let mut text = args.join(" ");
    if text.starts_with('"') && text.ends_with('"') {
        text = text[1..text.len() - 1].to_string();
    }
    println!( "{}", text);
    Ok(())
}