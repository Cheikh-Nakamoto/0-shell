use std::os::unix::prelude::PermissionsExt;

/**
 * Get the color of a file based on its type and permissions.
 *
 * # Arguments
 * * `file_type` - The type of the file.
 * * `permissions` - The permissions of the file.
 *
 * # Example
 * ```rust
 * use std::fs::Permissions;
 * use shell::utils::color::get_color;
 *
 * let permissions = Permissions::from_mode(0o755);
 * let color = get_color("f", &permissions);
 * ```
 */
pub fn get_color(file_type: &str, permissions: &std::fs::Permissions) -> String {
    let color = match file_type {
        "d" => "\x1b[1;34m", // Blue
        "l" => "\x1b[36m", // Cyan
        "c" => "\x1b[33m", // Jaune
        "b" => "\x1b[35m", // Magenta
        _ => {
            // If the file is executable
            if permissions.mode() & 0o111 != 0 {
                "\x1b[1;32m"
            } else {
                "\x1b[0m"
            }
        }
    };

    color.to_string()
}