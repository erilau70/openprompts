use crate::error::{map_err, AppResult};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct StoragePaths {
    pub root: PathBuf,
    pub prompts_dir: PathBuf,
    pub index_path: PathBuf,
    pub settings_path: PathBuf,
}

pub fn ensure_storage_dirs(paths: &StoragePaths) -> AppResult<()> {
    fs::create_dir_all(&paths.prompts_dir).map_err(map_err)?;
    Ok(())
}

/// Atomic write: write to .tmp file, then rename. Safe on NTFS.
pub fn atomic_write(path: &Path, content: &[u8]) -> AppResult<()> {
    let temp = path.with_extension("tmp");
    fs::write(&temp, content).map_err(map_err)?;
    fs::rename(&temp, path).map_err(map_err)?;
    Ok(())
}
