// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::{Context, Result};

use std::{
    fs,
    path::{Path, PathBuf},
};

// ─── < Public Functions > ────────────────────────────────────────────────────

pub fn atomic_write(path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> Result<()> {
    let path = path.as_ref();

    let tmp_path = tmp_path_for(path);

    fs::write(&tmp_path, content).with_context(|| {
        format!(
            "no se pudo escribir archivo temporal {}",
            tmp_path.display()
        )
    })?;

    fs::rename(&tmp_path, path)
        .with_context(|| format!("no se pudo reemplazar {}", path.display()))?;

    Ok(())
}

// ─── < Private Functions > ────────────────────────────────────────────────────

fn tmp_path_for(path: &Path) -> PathBuf {
    let mut tmp = path.to_path_buf();

    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("colors");

    tmp.set_file_name(format!("{file_name}.tmp"));

    tmp
}
