use std::io::{BufRead, BufReader};

pub fn uid_to_name(uid: u32) -> String {
    let file = match std::fs::File::open("/etc/passwd") {
        Ok(file) => file,
        Err(_) => return "Error".to_string(),
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
    "Error".to_string()
}

/// Convertit un GID en nom de groupe en lisant /etc/group
pub fn gid_to_name(gid: u32) -> String {
    let file = match std::fs::File::open("/etc/group") {
        Ok(file) => file,
        Err(_) => return "Error".to_string(),
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
    "Error".to_string()
}