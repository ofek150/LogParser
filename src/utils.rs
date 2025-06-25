use anyhow::{anyhow, Result};
use std::fs::read_dir;
use std::path::{Path, PathBuf};

pub fn extract_file_name_from_path(file_path: &Path) -> Result<String> {
    let file_name = file_path
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow!("Invalid file name: {:?}", file_path))?
        .to_owned();
    Ok(file_name)
}

pub fn list_files_in_folder(path: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            files.push(path);
        }
    }

    Ok(files)
}
