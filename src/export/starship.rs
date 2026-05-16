use anyhow::Result;
use std::{env, fs, path::PathBuf};

use crate::{fs::atomic_write, palette::Palette};

const PALETTE_HEADER: &str = "[palettes.hyprcolor]";

pub fn export(palette: &Palette) -> Result<()> {
    let path = starship_config_path();
    let existing = fs::read_to_string(&path).unwrap_or_default();
    atomic_write(&path, update_palette(&existing, palette))?;
    Ok(())
}

fn update_palette(content: &str, palette: &Palette) -> String {
    let block = palette_block(palette);

    if let Some(start) = content.find(PALETTE_HEADER) {
        let after = &content[start + PALETTE_HEADER.len()..];
        let end = after.find("\n[").map(|i| i + 1).unwrap_or(after.len());
        format!("{}{}{}", &content[..start], block, &after[end..])
    } else {
        format!("{}\n{}", content.trim_end(), block)
    }
}

fn palette_block(palette: &Palette) -> String {
    format!(
        "{}\n# Managed by hyprcolor\n\
         hy_accent = \"{}\"\n\
         hy_accent1 = \"{}\"\n\
         hy_accent2 = \"{}\"\n\
         hy_dir = \"{}\"\n\
         hy_surface = \"{}\"\n",
        PALETTE_HEADER,
        palette.ui_accent,
        palette.accent_1,
        palette.accent_2,
        palette.ui_accent,
        palette.surface_variant,
    )
}

fn starship_config_path() -> PathBuf {
    if let Some(p) = env::var_os("STARSHIP_CONFIG") {
        return PathBuf::from(p);
    }
    if let Some(xdg) = env::var_os("XDG_CONFIG_HOME") {
        return PathBuf::from(xdg).join("starship.toml");
    }
    env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config/starship.toml")
}
