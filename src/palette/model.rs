// ─── < Imports > ────────────────────────────────────────────────────

use serde::Serialize;
use std::path::PathBuf;

// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct Palette {
    pub wallpaper: PathBuf,

    pub background: String,
    pub foreground: String,
    pub surface: String,
    pub surface_variant: String,

    pub accent: String,
    pub accent_1: String,
    pub accent_2: String,
    pub accent_3: String,

    pub ui_accent: String,
    pub swatches: Vec<String>,
}
