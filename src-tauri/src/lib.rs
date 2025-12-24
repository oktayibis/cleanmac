// CleanMac - macOS disk cleanup and optimization utility
// Module declarations will be added as features are implemented
pub mod cleaner;
pub mod commands;
pub mod models;
pub mod scanner;
pub mod utils;

// Placeholder greeting command for initial testing
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to CleanMac.", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::scan::scan_caches,
            commands::scan::cancel_scan,
            commands::clean::clean_items,
            commands::clean::get_cleaning_history,
            commands::config::get_config,
            commands::config::save_config,
            commands::config::add_exclusion,
            commands::config::remove_exclusion,
            commands::system::get_disk_info,
            commands::system::check_full_disk_access,
            commands::system::open_full_disk_access_settings,
            commands::system::reveal_in_finder,
            commands::system::open_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running CleanMac application");
}
