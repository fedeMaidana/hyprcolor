// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::Result;

use std::path::Path;

use crate::{fs::atomic_write, palette::Palette};

// ─── < Public Functions > ────────────────────────────────────────────────────

pub fn export(output_dir: &Path, palette: &Palette) -> Result<()> {
    let content = format!(
        concat!(
            "HYPRCOLOR_WALLPAPER=\"{}\"\n",
            "HYPRCOLOR_BACKGROUND=\"{}\"\n",
            "HYPRCOLOR_FOREGROUND=\"{}\"\n",
            "HYPRCOLOR_ACCENT=\"{}\"\n",
            "HYPRCOLOR_ACCENT_1=\"{}\"\n",
            "HYPRCOLOR_ACCENT_2=\"{}\"\n",
            "HYPRCOLOR_ACCENT_3=\"{}\"\n",
            "HYPRCOLOR_SURFACE=\"{}\"\n",
            "HYPRCOLOR_SURFACE_VARIANT=\"{}\"\n",
            "\n",
        ),
        palette.wallpaper.display(),
        palette.background,
        palette.foreground,
        palette.accent,
        palette.accent_1,
        palette.accent_2,
        palette.accent_3,
        palette.surface,
        palette.surface_variant,
    );

    atomic_write(output_dir.join("colors.env"), content)
}
