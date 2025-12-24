use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

/// Calculate SHA-256 hash of a file
pub fn calculate_hash(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

/// Calculate partial SHA-256 hash (first 4KB + last 4KB)
/// This is used for quick comparison of large files before full hashing
pub fn calculate_partial_hash(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let metadata = file.metadata()?;
    let size = metadata.len();
    let mut hasher = Sha256::new();
    let mut buffer = [0; 4096];

    // Read first 4KB
    let count = file.read(&mut buffer)?;
    hasher.update(&buffer[..count]);

    // If file is larger than 4KB, read last 4KB
    if size > 4096 {
        let seek_pos = if size > 8192 { size - 4096 } else { 4096 };

        file.seek(SeekFrom::Start(seek_pos))?;
        let count = file.read(&mut buffer)?;
        hasher.update(&buffer[..count]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_calculate_hash() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"Hello, World!").unwrap();

        let hash = calculate_hash(&file_path).unwrap();
        // echo -n "Hello, World!" | shasum -a 256
        assert_eq!(
            hash,
            "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f"
        );
    }

    #[test]
    fn test_calculate_partial_hash_small_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("small.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"Small file").unwrap();

        let full_hash = calculate_hash(&file_path).unwrap();
        let partial_hash = calculate_partial_hash(&file_path).unwrap();

        // For small files (< 4KB), partial hash should equal full hash
        assert_eq!(full_hash, partial_hash);
    }

    #[test]
    fn test_calculate_partial_hash_large_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("large.bin");
        let mut file = File::create(&file_path).unwrap();

        // Create a 10KB file
        let data = vec![0u8; 10240];
        file.write_all(&data).unwrap();

        let partial_hash = calculate_partial_hash(&file_path).unwrap();

        // Should calculate successfully
        assert_eq!(partial_hash.len(), 64);
    }
}
