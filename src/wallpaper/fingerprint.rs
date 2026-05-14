// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::Result;

use std::{
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WallpaperFingerprint {
    path: PathBuf,
    len: u64,
    modified: Option<SystemTime>,
}

// ─── < Implementations > ────────────────────────────────────────────────────

impl WallpaperFingerprint {
    pub fn from_path(path: &Path) -> Result<Self> {
        let metadata = fs::metadata(path)?;

        Ok(Self {
            path: path.canonicalize().unwrap_or_else(|_| path.to_path_buf()),
            len: metadata.len(),
            modified: metadata.modified().ok(),
        })
    }
}
