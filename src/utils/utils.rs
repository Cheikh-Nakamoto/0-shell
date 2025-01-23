use std::fs::{File, Permissions};
use std::io::{BufRead, BufReader};
use std::os::unix::fs::PermissionsExt;

/// Convert a UID to a username by reading /etc/passwd
pub fn uid_to_name(uid: u32) -> String {
    let file = match File::open("/etc/passwd") {
        Ok(file) => file,
        Err(_) => return "ERROR".to_string(),
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 3 {
                let file_uid = parts[2].parse::<u32>().ok();
                if file_uid == Option::from(uid) {
                    return parts[0].to_string()
                }
            }
        }
    }
    "ERROR".to_string()
}

/// Convert a GID to a group name by reading /etc/group
pub fn gid_to_name(gid: u32) -> String {
    let file = match File::open("/etc/group") {
        Ok(file) => file,
        Err(_) => return "ERROR".to_string(),
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 3 {
                let file_gid = parts[2].parse::<u32>().ok();
                if file_gid == Option::from(gid) {
                    return parts[0].to_string()
                }
            }
        }
    }
    "ERROR".to_string()
}

/// Convert a Permissions struct to a string representation of the permissions
pub fn permissions_string(permissions: &Permissions) -> String {
    let mode = permissions.mode();
    let mut result = String::with_capacity(10);

    for i in (0..3).rev() {
        let offset = i * 3;
        result.push(if mode & (1 << (offset + 2)) != 0 { 'r' } else { '-' });
        result.push(if mode & (1 << (offset + 1)) != 0 { 'w' } else { '-' });
        result.push(if mode & (1 << offset) != 0 { 'x' } else { '-' });
    }

    result
}