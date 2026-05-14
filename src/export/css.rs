// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::Result;

use std::path::Path;

use crate::{fs::atomic_write, palette::Palette};

// ─── < Public Functions > ────────────────────────────────────────────────────

pub fn export(output_dir: &Path, palette: &Palette) -> Result<()> {
    let content = format!(
        concat!(
            "@define-color background {};\n",
            "@define-color foreground {};\n",
            "@define-color accent {};\n",
            "@define-color accent_1 {};\n",
            "@define-color accent_2 {};\n",
            "@define-color accent_3 {};\n",
            "@define-color surface {};\n",
            "@define-color surface_variant {};\n",
            "\n",
        ),
        palette.background,
        palette.foreground,
        palette.accent,
        palette.accent_1,
        palette.accent_2,
        palette.accent_3,
        palette.surface,
        palette.surface_variant,
    );

    atomic_write(output_dir.join("colors.css"), content)
}
