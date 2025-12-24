use std::fs;
use std::path::{Path, PathBuf};

/// Expand tilde (~) to home directory
pub fn expand_tilde(path: &str) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(stripped);
        }
    } else if path == "~" {
        if let Some(home) = dirs::home_dir() {
            return home;
        }
    }
    PathBuf::from(path)
}

/// Get the size of a file or directory in bytes
pub fn get_size(path: &Path) -> std::io::Result<u64> {
    if path.is_file() {
        Ok(fs::metadata(path)?.len())
    } else if path.is_dir() {
        get_dir_size(path)
    } else {
        Ok(0)
    }
}

/// Get the size of a directory recursively
pub fn get_dir_size(path: &Path) -> std::io::Result<u64> {
    let mut total_size = 0u64;

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                total_size += fs::metadata(&path)?.len();
            } else if path.is_dir() {
                total_size += get_dir_size(&path)?;
            }
        }
    }

    Ok(total_size)
}

/// Check if a path exists
pub fn path_exists(path: &str) -> bool {
    expand_tilde(path).exists()
}

/// Check if a path is a directory
pub fn is_directory(path: &str) -> bool {
    expand_tilde(path).is_dir()
}

/// Check if a path is a file
pub fn is_file(path: &str) -> bool {
    expand_tilde(path).is_file()
}

/// Get the last modified time of a file as a Unix timestamp
pub fn get_modified_time(path: &Path) -> std::io::Result<i64> {
    let metadata = fs::metadata(path)?;
    let modified = metadata.modified()?;
    let duration = modified
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    Ok(duration.as_secs() as i64)
}

/// Get the last accessed time of a file as a Unix timestamp
pub fn get_accessed_time(path: &Path) -> std::io::Result<Option<i64>> {
    let metadata = fs::metadata(path)?;
    match metadata.accessed() {
        Ok(accessed) => {
            let duration = accessed
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default();
            Ok(Some(duration.as_secs() as i64))
        }
        Err(_) => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_expand_tilde_with_path() {
        let result = expand_tilde("~/Documents");
        assert!(result.to_string_lossy().ends_with("Documents"));
        assert!(!result.to_string_lossy().starts_with("~"));
    }

    #[test]
    fn test_expand_tilde_just_tilde() {
        let result = expand_tilde("~");
        assert!(!result.to_string_lossy().contains("~"));
    }

    #[test]
    fn test_expand_tilde_no_tilde() {
        let result = expand_tilde("/usr/local");
        assert_eq!(result, PathBuf::from("/usr/local"));
    }

    #[test]
    fn test_get_size_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"Hello, World!").unwrap();

        let size = get_size(&file_path).unwrap();
        assert_eq!(size, 13); // "Hello, World!" is 13 bytes
    }

    #[test]
    fn test_get_size_directory() {
        let dir = tempdir().unwrap();

        // Create some files
        let file1_path = dir.path().join("file1.txt");
        let mut file1 = File::create(&file1_path).unwrap();
        file1.write_all(b"Hello").unwrap();

        let file2_path = dir.path().join("file2.txt");
        let mut file2 = File::create(&file2_path).unwrap();
        file2.write_all(b"World").unwrap();

        let size = get_size(dir.path()).unwrap();
        assert_eq!(size, 10); // 5 + 5 bytes
    }

    #[test]
    fn test_get_dir_size_nested() {
        let dir = tempdir().unwrap();

        // Create nested structure
        let subdir = dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();

        let file1_path = dir.path().join("file1.txt");
        let mut file1 = File::create(&file1_path).unwrap();
        file1.write_all(b"Root file").unwrap();

        let file2_path = subdir.join("file2.txt");
        let mut file2 = File::create(&file2_path).unwrap();
        file2.write_all(b"Nested file").unwrap();

        let size = get_dir_size(dir.path()).unwrap();
        assert_eq!(size, 20); // 9 + 11 bytes
    }

    #[test]
    fn test_path_exists() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("exists.txt");
        File::create(&file_path).unwrap();

        assert!(path_exists(file_path.to_str().unwrap()));
        assert!(!path_exists("/nonexistent/path/file.txt"));
    }

    #[test]
    fn test_is_directory() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("file.txt");
        File::create(&file_path).unwrap();

        assert!(is_directory(dir.path().to_str().unwrap()));
        assert!(!is_directory(file_path.to_str().unwrap()));
    }

    #[test]
    fn test_is_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("file.txt");
        File::create(&file_path).unwrap();

        assert!(is_file(file_path.to_str().unwrap()));
        assert!(!is_file(dir.path().to_str().unwrap()));
    }

    #[test]
    fn test_get_modified_time() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("file.txt");
        File::create(&file_path).unwrap();

        let time = get_modified_time(&file_path).unwrap();
        assert!(time > 0);
    }
}
