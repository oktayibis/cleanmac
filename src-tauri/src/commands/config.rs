use crate::models::config::AppConfig;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, Runtime};

fn get_config_path<R: Runtime>(app: &AppHandle<R>) -> PathBuf {
    app.path().app_data_dir().unwrap().join("config.json")
}

#[tauri::command]
pub async fn get_config<R: Runtime>(app: AppHandle<R>) -> Result<AppConfig, String> {
    let path = get_config_path(&app);
    if path.exists() {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let config: AppConfig = serde_json::from_str(&content).unwrap_or_default();
        Ok(config)
    } else {
        Ok(AppConfig::default())
    }
}

#[tauri::command]
pub async fn save_config<R: Runtime>(app: AppHandle<R>, config: AppConfig) -> Result<(), String> {
    let path = get_config_path(&app);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let content = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    let mut file = fs::File::create(path).map_err(|e| e.to_string())?;
    file.write_all(content.as_bytes())
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn add_exclusion<R: Runtime>(
    app: AppHandle<R>,
    path: String,
) -> Result<AppConfig, String> {
    let mut config = get_config(app.clone()).await?;
    let p = PathBuf::from(path);
    if !config.excluded_paths.contains(&p) {
        config.excluded_paths.push(p);
        save_config(app, config.clone()).await?;
    }
    Ok(config)
}

#[tauri::command]
pub async fn remove_exclusion<R: Runtime>(
    app: AppHandle<R>,
    path: String,
) -> Result<AppConfig, String> {
    let mut config = get_config(app.clone()).await?;
    let p = PathBuf::from(path);
    if let Some(pos) = config.excluded_paths.iter().position(|x| *x == p) {
        config.excluded_paths.remove(pos);
        save_config(app, config.clone()).await?;
    }
    Ok(config)
}
