use crate::cleaner::history::HistoryManager;
use crate::cleaner::safe_delete::delete_item;
use crate::models::history::{CleanedItem, CleaningEntry};
use crate::models::scan_result::CacheItem;
use chrono::Utc;
use tauri::{AppHandle, Manager, Runtime};
use uuid::Uuid;

#[tauri::command]
pub async fn clean_items<R: Runtime>(
    app: AppHandle<R>,
    items: Vec<CacheItem>,
) -> Result<CleaningEntry, String> {
    let mut total_reclaimed = 0;
    let mut cleaned_items = Vec::new();
    let start_time = std::time::Instant::now();

    for item in items {
        if let Err(e) = delete_item(&item.path, false) {
            println!("Failed to delete {}: {}", item.path.display(), e);
            continue;
        }
        total_reclaimed += item.size;
        cleaned_items.push(CleanedItem {
            path: item.path.clone(),
            size: item.size,
            category_id: "unknown".to_string(),
        });
    }

    let entry = CleaningEntry {
        id: Uuid::new_v4().to_string(),
        timestamp: Utc::now(),
        total_size_reclaimed: total_reclaimed,
        items_count: cleaned_items.len(),
        items: cleaned_items,
        duration_ms: start_time.elapsed().as_millis() as u64,
    };

    // Save history
    if let Ok(app_data_dir) = app.path().app_data_dir() {
        let history_manager = HistoryManager::new(&app_data_dir);
        let _ = history_manager.add_entry(entry.clone());
    }

    Ok(entry)
}

#[tauri::command]
pub async fn get_cleaning_history<R: Runtime>(
    app: AppHandle<R>,
) -> Result<crate::models::history::CleaningHistory, String> {
    if let Ok(app_data_dir) = app.path().app_data_dir() {
        let history_manager = HistoryManager::new(&app_data_dir);
        Ok(history_manager.load())
    } else {
        Ok(crate::models::history::CleaningHistory::default())
    }
}
