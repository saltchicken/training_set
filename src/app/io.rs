
use anyhow::{Context, Result};
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;

/// Ensures the target directory exists, creating it if necessary
pub fn ensure_directory_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)
            .with_context(|| format!("Failed to create directory: {:?}", path))?;
    }
    Ok(())
}

/// Async write to disk
pub async fn write_file(path: &Path, data: &[u8]) -> Result<()> {
    let mut file = fs::File::create(path)
        .await
        .with_context(|| format!("Failed to create file: {:?}", path))?;

    file.write_all(data)
        .await
        .with_context(|| format!("Failed to write bytes to: {:?}", path))?;

    Ok(())
}