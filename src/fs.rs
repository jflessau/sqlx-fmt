use anyhow::{Result, bail};
use std::path::Path;
use walkdir::WalkDir;

pub fn find_rust_files(path: &str) -> Result<Vec<String>> {
    let path = Path::new(path);
    let mut rust_files = Vec::new();

    if path.is_file() {
        if let Some(extension) = path.extension()
            && extension == "rs"
        {
            rust_files.push(path.to_string_lossy().to_string());
        }
    } else if path.is_dir() {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            let file_path = entry.path();
            if file_path.is_file()
                && let Some(extension) = file_path.extension()
                && extension == "rs"
            {
                rust_files.push(file_path.to_string_lossy().to_string());
            }
        }
    } else {
        bail!("path '{}' does not exist", path.display());
    }

    Ok(rust_files)
}
