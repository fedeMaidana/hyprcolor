// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::Result;

use std::path::Path;

use crate::{fs::atomic_write, palette::Palette};

// ─── < Public Functions > ────────────────────────────────────────────────────

pub fn export(output_dir: &Path, palette: &Palette) -> Result<()> {
    let content = format!(
        concat!(
            "HYPRCOLORS_WALLPAPER=\"{}\"\n",
            "HYPRCOLORS_BACKGROUND=\"{}\"\n",
            "HYPRCOLORS_FOREGROUND=\"{}\"\n",
            "HYPRCOLORS_ACCENT=\"{}\"\n",
            "HYPRCOLORS_ACCENT_1=\"{}\"\n",
            "HYPRCOLORS_ACCENT_2=\"{}\"\n",
            "HYPRCOLORS_ACCENT_3=\"{}\"\n",
            "HYPRCOLORS_SURFACE=\"{}\"\n",
            "HYPRCOLORS_SURFACE_VARIANT=\"{}\"\n",
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
