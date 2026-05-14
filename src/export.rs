// ─── < Modules > ────────────────────────────────────────────────────

mod css;
mod env;
mod json;
mod writer;
mod zed;

// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::Result;
use std::path::PathBuf;

use crate::palette::Palette;

// ─── < Structs > ────────────────────────────────────────────────────

pub struct Exporters {
    output_dir: PathBuf,
}

// ─── < Implementations > ────────────────────────────────────────────────────

impl Exporters {
    pub fn new(output_dir: PathBuf) -> Self {
        Self { output_dir }
    }

    pub fn export_all(&self, palette: &Palette) -> Result<()> {
        writer::ensure_dir(&self.output_dir)?;

        env::export(&self.output_dir, palette)?;
        css::export(&self.output_dir, palette)?;
        json::export(&self.output_dir, palette)?;
        zed::export(palette)?;

        Ok(())
    }
}
