use crate::models::scan_result::CacheScanResult;
use crate::scanner::cache_scanner::CacheScanner;
use tauri::{Runtime, Window};

#[tauri::command]
pub async fn scan_caches<R: Runtime>(window: Window<R>) -> Result<CacheScanResult, String> {
    let scanner = CacheScanner::new();

    // Run scanning in a blocking thread to avoid blocking the async runtime
    let result = tauri::async_runtime::spawn_blocking(move || scanner.scan(&window))
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}

#[tauri::command]
pub async fn cancel_scan() -> Result<(), String> {
    // Placeholder for cancellation logic
    // In a real implementation, we would toggle an AtomicBool stored in AppState
    println!("Cancellation requested (not yet implemented)");
    Ok(())
}
