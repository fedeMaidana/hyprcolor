// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::Result;

use std::path::Path;

use crate::{fs::atomic_write, palette::Palette};

// ─── < Public Functions > ───────────────────────────────────────────

pub fn export(output_dir: &Path, palette: &Palette) -> Result<()> {
    let content = format!(
        concat!(
            "return {{\n",
            "  wallpaper = {},\n",
            "\n",
            "  hex = {{\n",
            "    background = \"{}\",\n",
            "    foreground = \"{}\",\n",
            "    accent = \"{}\",\n",
            "    accent_1 = \"{}\",\n",
            "    accent_2 = \"{}\",\n",
            "    accent_3 = \"{}\",\n",
            "    surface = \"{}\",\n",
            "    surface_variant = \"{}\",\n",
            "  }},\n",
            "\n",
            "  rgb = {{\n",
            "    background = \"rgb({})\",\n",
            "    foreground = \"rgb({})\",\n",
            "    accent = \"rgb({})\",\n",
            "    accent_1 = \"rgb({})\",\n",
            "    accent_2 = \"rgb({})\",\n",
            "    accent_3 = \"rgb({})\",\n",
            "    surface = \"rgb({})\",\n",
            "    surface_variant = \"rgb({})\",\n",
            "  }},\n",
            "\n",
            "  rgba = {{\n",
            "    background = \"rgba({}ff)\",\n",
            "    foreground = \"rgba({}ff)\",\n",
            "    accent = \"rgba({}ff)\",\n",
            "    accent_1 = \"rgba({}ff)\",\n",
            "    accent_2 = \"rgba({}ff)\",\n",
            "    accent_3 = \"rgba({}ff)\",\n",
            "    surface = \"rgba({}ff)\",\n",
            "    surface_variant = \"rgba({}ff)\",\n",
            "  }},\n",
            "}}\n",
        ),
        lua_string(&palette.wallpaper.display().to_string()),
        palette.background,
        palette.foreground,
        palette.accent,
        palette.accent_1,
        palette.accent_2,
        palette.accent_3,
        palette.surface,
        palette.surface_variant,
        strip_hash(&palette.background),
        strip_hash(&palette.foreground),
        strip_hash(&palette.accent),
        strip_hash(&palette.accent_1),
        strip_hash(&palette.accent_2),
        strip_hash(&palette.accent_3),
        strip_hash(&palette.surface),
        strip_hash(&palette.surface_variant),
        strip_hash(&palette.background),
        strip_hash(&palette.foreground),
        strip_hash(&palette.accent),
        strip_hash(&palette.accent_1),
        strip_hash(&palette.accent_2),
        strip_hash(&palette.accent_3),
        strip_hash(&palette.surface),
        strip_hash(&palette.surface_variant),
    );

    atomic_write(output_dir.join("hyprland.lua"), content)
}

// ─── < Private Functions > ──────────────────────────────────────────

fn strip_hash(color: &str) -> &str {
    color.trim_start_matches('#')
}

fn lua_string(value: &str) -> String {
    let escaped = value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t");

    format!("\"{escaped}\"")
}
