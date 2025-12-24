use std::time::{SystemTime, UNIX_EPOCH};

const BYTES_PER_KB: u64 = 1024;
const BYTES_PER_MB: u64 = 1024 * 1024;
const BYTES_PER_GB: u64 = 1024 * 1024 * 1024;
const BYTES_PER_TB: u64 = 1024 * 1024 * 1024 * 1024;

/// Format bytes into a human-readable string
pub fn format_bytes(bytes: u64) -> String {
    if bytes == 0 {
        return "0 Bytes".to_string();
    }

    if bytes >= BYTES_PER_TB {
        format!("{:.2} TB", bytes as f64 / BYTES_PER_TB as f64)
    } else if bytes >= BYTES_PER_GB {
        format!("{:.2} GB", bytes as f64 / BYTES_PER_GB as f64)
    } else if bytes >= BYTES_PER_MB {
        format!("{:.2} MB", bytes as f64 / BYTES_PER_MB as f64)
    } else if bytes >= BYTES_PER_KB {
        format!("{:.2} KB", bytes as f64 / BYTES_PER_KB as f64)
    } else {
        format!("{} Bytes", bytes)
    }
}

/// Format a Unix timestamp into a relative time string
pub fn format_relative_time(timestamp: i64) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let diff = now - timestamp;

    if diff < 0 {
        return "In the future".to_string();
    }

    let seconds = diff;
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let days = hours / 24;
    let weeks = days / 7;
    let months = days / 30;
    let years = days / 365;

    if seconds < 60 {
        "Just now".to_string()
    } else if minutes < 60 {
        if minutes == 1 {
            "1 minute ago".to_string()
        } else {
            format!("{} minutes ago", minutes)
        }
    } else if hours < 24 {
        if hours == 1 {
            "1 hour ago".to_string()
        } else {
            format!("{} hours ago", hours)
        }
    } else if days < 7 {
        if days == 1 {
            "1 day ago".to_string()
        } else {
            format!("{} days ago", days)
        }
    } else if weeks < 4 {
        if weeks == 1 {
            "1 week ago".to_string()
        } else {
            format!("{} weeks ago", weeks)
        }
    } else if months < 12 {
        if months == 1 {
            "1 month ago".to_string()
        } else {
            format!("{} months ago", months)
        }
    } else if years == 1 {
        "1 year ago".to_string()
    } else {
        format!("{} years ago", years)
    }
}

/// Get file extension from a path string
pub fn get_file_extension(path: &str) -> Option<String> {
    std::path::Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
}

/// Get file name from a path string
pub fn get_file_name(path: &str) -> Option<String> {
    std::path::Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .map(|s| s.to_string())
}

/// Truncate a path for display
pub fn truncate_path(path: &str, max_length: usize) -> String {
    if path.len() <= max_length {
        return path.to_string();
    }

    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() <= 3 {
        return path.to_string();
    }

    let file_name = parts.last().unwrap_or(&"");
    let first_part = parts.iter().take(2).map(|s| *s).collect::<Vec<_>>().join("/");

    if file_name.len() + first_part.len() + 5 > max_length {
        format!(".../{}", &file_name[file_name.len().saturating_sub(max_length - 4)..])
    } else {
        format!("{}/.../{}", first_part, file_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes_zero() {
        assert_eq!(format_bytes(0), "0 Bytes");
    }

    #[test]
    fn test_format_bytes_bytes() {
        assert_eq!(format_bytes(500), "500 Bytes");
        assert_eq!(format_bytes(1023), "1023 Bytes");
    }

    #[test]
    fn test_format_bytes_kilobytes() {
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1536), "1.50 KB");
        assert_eq!(format_bytes(10240), "10.00 KB");
    }

    #[test]
    fn test_format_bytes_megabytes() {
        assert_eq!(format_bytes(1048576), "1.00 MB");
        assert_eq!(format_bytes(5242880), "5.00 MB");
    }

    #[test]
    fn test_format_bytes_gigabytes() {
        assert_eq!(format_bytes(1073741824), "1.00 GB");
        assert_eq!(format_bytes(10737418240), "10.00 GB");
    }

    #[test]
    fn test_format_bytes_terabytes() {
        assert_eq!(format_bytes(1099511627776), "1.00 TB");
    }

    #[test]
    fn test_get_file_extension() {
        assert_eq!(get_file_extension("file.txt"), Some("txt".to_string()));
        assert_eq!(get_file_extension("file.PNG"), Some("png".to_string()));
        assert_eq!(get_file_extension("file.tar.gz"), Some("gz".to_string()));
        assert_eq!(get_file_extension("Makefile"), None);
        assert_eq!(get_file_extension("/path/to/file.js"), Some("js".to_string()));
    }

    #[test]
    fn test_get_file_name() {
        assert_eq!(get_file_name("/path/to/file.txt"), Some("file.txt".to_string()));
        assert_eq!(get_file_name("file.txt"), Some("file.txt".to_string()));
        assert_eq!(get_file_name("/"), None);
    }

    #[test]
    fn test_truncate_path_short() {
        let path = "/Users/test/file.txt";
        assert_eq!(truncate_path(path, 50), path);
    }

    #[test]
    fn test_truncate_path_long() {
        let path = "/Users/username/Documents/Projects/very/long/path/to/file.txt";
        let result = truncate_path(path, 40);
        assert!(result.len() <= 40 || result.contains("..."));
    }

    #[test]
    fn test_format_relative_time_just_now() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        assert_eq!(format_relative_time(now), "Just now");
        assert_eq!(format_relative_time(now - 30), "Just now");
    }

    #[test]
    fn test_format_relative_time_minutes() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        assert_eq!(format_relative_time(now - 60), "1 minute ago");
        assert_eq!(format_relative_time(now - 300), "5 minutes ago");
    }

    #[test]
    fn test_format_relative_time_hours() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        assert_eq!(format_relative_time(now - 3600), "1 hour ago");
        assert_eq!(format_relative_time(now - 7200), "2 hours ago");
    }

    #[test]
    fn test_format_relative_time_days() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        assert_eq!(format_relative_time(now - 86400), "1 day ago");
        assert_eq!(format_relative_time(now - 172800), "2 days ago");
    }
}
