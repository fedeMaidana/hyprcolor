// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::Result;

use std::{fs, path::Path};

// ─── < Public Functions > ────────────────────────────────────────────────────

pub fn ensure_dir(path: &Path) -> Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}
