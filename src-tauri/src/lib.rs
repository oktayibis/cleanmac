// CleanMac - macOS disk cleanup and optimization utility

pub mod models;
pub mod utils;

use utils::format::{format_bytes, format_relative_time};

/// Placeholder greeting command for initial testing
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to CleanMac.", name)
}

/// Get formatted size string
#[tauri::command]
fn format_size(bytes: u64) -> String {
    format_bytes(bytes)
}

/// Get relative time string
#[tauri::command]
fn get_relative_time(timestamp: i64) -> String {
    format_relative_time(timestamp)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            format_size,
            get_relative_time
        ])
        .run(tauri::generate_context!())
        .expect("error while running CleanMac application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello, World! Welcome to CleanMac.");
        assert_eq!(greet(""), "Hello, ! Welcome to CleanMac.");
        assert_eq!(greet("Test User"), "Hello, Test User! Welcome to CleanMac.");
    }

    #[test]
    fn test_format_size_command() {
        assert_eq!(format_size(1024), "1.00 KB");
    }

    #[test]
    fn test_get_relative_time_command() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        assert_eq!(get_relative_time(now), "Just now");
    }
}
