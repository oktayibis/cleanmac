// CleanMac - macOS disk cleanup and optimization utility
// Module declarations will be added as features are implemented

// Placeholder greeting command for initial testing
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to CleanMac.", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running CleanMac application");
}
