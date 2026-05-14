// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::Result;

use std::path::Path;

use crate::{fs::atomic_write, palette::Palette};

// ─── < Public Functions > ────────────────────────────────────────────────────

pub fn export(output_dir: &Path, palette: &Palette) -> Result<()> {
    let content = serde_json::to_string_pretty(palette)?;
    atomic_write(output_dir.join("colors.json"), content)
}
