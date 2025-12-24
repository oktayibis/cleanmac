use crate::utils::permissions;
use serde::Serialize;
#[cfg(not(target_os = "macos"))]
use std::path::PathBuf;
use sysinfo::Disks;
use tauri::command;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskInfo {
    pub total_space: u64,
    pub available_space: u64,
    pub used_space: u64,
    pub mount_point: String,
    pub name: String,
}

#[command]
pub fn get_disk_info() -> Result<DiskInfo, String> {
    let disks = Disks::new_with_refreshed_list();

    // On macOS, the system disk is usually mounted at "/"
    // We'll find the disk mounted at "/" or take the first one
    let disk = disks
        .list()
        .iter()
        .find(|d| d.mount_point().to_string_lossy() == "/")
        .or_else(|| disks.list().first())
        .ok_or("No disks found")?;

    let total_space = disk.total_space();
    let available_space = disk.available_space();
    let used_space = total_space.saturating_sub(available_space);

    Ok(DiskInfo {
        total_space,
        available_space,
        used_space,
        mount_point: disk.mount_point().to_string_lossy().to_string(),
        name: disk.name().to_string_lossy().to_string(),
    })
}

#[command]
pub fn check_full_disk_access() -> bool {
    permissions::check_full_disk_access()
}

#[command]
pub fn open_full_disk_access_settings() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        // Deep link to Full Disk Access settings
        // macOS 13+ (Ventura) uses a different URL scheme than older versions, but this usually redirects correctly
        // x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles
        open::that("x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles")
            .map_err(|e| e.to_string())
    }
    #[cfg(not(target_os = "macos"))]
    {
        Ok(())
    }
}

#[command]
pub fn reveal_in_finder(path: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        // Use "open -R <path>" to reveal in Finder
        std::process::Command::new("open")
            .arg("-R")
            .arg(path)
            .spawn()
            .map_err(|e| e.to_string())?;
        Ok(())
    }
    #[cfg(not(target_os = "macos"))]
    {
        // Fallback for other OS (open parent dir)
        let p = PathBuf::from(path);
        if let Some(parent) = p.parent() {
            open::that(parent).map_err(|e| e.to_string())
        } else {
            Ok(())
        }
    }
}

#[command]
pub fn open_file(path: String) -> Result<(), String> {
    open::that(path).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_disk_info() {
        // This test mostly checks that the function doesn't panic
        // It might return Err if no disk is found (e.g. in some containers), which is handled
        let result = get_disk_info();
        println!("Disk info result: {:?}", result);
        // We don't assert ok/err because it depends on the environment,
        // but we ensure the code path is executable.
    }
}
