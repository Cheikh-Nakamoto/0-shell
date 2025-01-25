/**
 * Exit command
 *
 * This command is used to exit the shell
 */
pub fn exit() -> ! {
    std::process::exit(0);
}

/**
 * Clear command
 *
 * This command is used to clear the terminal screen
 */
pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}