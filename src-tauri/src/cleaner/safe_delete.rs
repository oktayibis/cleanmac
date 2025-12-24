use std::fs;
use std::path::Path;

pub fn delete_item(path: &Path, permanent: bool) -> Result<(), String> {
    if !path.exists() {
        return Ok(()); // Already gone
    }

    if permanent {
        if path.is_dir() {
            fs::remove_dir_all(path).map_err(|e| e.to_string())
        } else {
            fs::remove_file(path).map_err(|e| e.to_string())
        }
    } else {
        trash::delete(path).map_err(|e| e.to_string())
    }
}
