// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::{Context, Result};
use std::{
    env, fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::Command,
};

use crate::palette::Palette;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn run_all(palette: &Palette) -> Result<()> {
    let Some(hooks_dir) = hooks_dir() else {
        return Ok(());
    };

    if !hooks_dir.exists() {
        return Ok(());
    }

    for entry in
        fs::read_dir(&hooks_dir).with_context(|| format!("no se pudo leer hooks dir {}", hooks_dir.display()))?
    {
        let path = entry?.path();

        if !path.is_file() || !is_executable(&path) {
            continue;
        }

        run_hook(&path, palette);
    }

    Ok(())
}

// ─── < Private Functions > ──────────────────────────────────────────

// ─── < Hooks Dir > ───────

fn hooks_dir() -> Option<PathBuf> {
    if let Some(config_home) = env::var_os("XDG_CONFIG_HOME") {
        return Some(PathBuf::from(config_home).join("hyprcolor/hooks"));
    }

    env::var_os("HOME").map(|home| PathBuf::from(home).join(".config").join("hyprcolor").join("hooks"))
}

// ─── < Is Executable > ───────

fn is_executable(path: &Path) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

// ─── < Run Hook > ───────

fn run_hook(path: &Path, palette: &Palette) {
    log::debug!("running hook: {}", path.display());

    let status = Command::new(path)
        .env("HYPRCOLOR_WALLPAPER", &palette.wallpaper)
        .env("HYPRCOLOR_BACKGROUND", &palette.background)
        .env("HYPRCOLOR_FOREGROUND", &palette.foreground)
        .env("HYPRCOLOR_ACCENT", &palette.accent)
        .env("HYPRCOLOR_ACCENT_1", &palette.accent_1)
        .env("HYPRCOLOR_ACCENT_2", &palette.accent_2)
        .env("HYPRCOLOR_ACCENT_3", &palette.accent_3)
        .env("HYPRCOLOR_SURFACE", &palette.surface)
        .env("HYPRCOLOR_SURFACE_VARIANT", &palette.surface_variant)
        .status();

    match status {
        Ok(status) if status.success() => {
            log::debug!("hook finished: {}", path.display());
        }
        Ok(status) => {
            log::warn!("hook failed with status {status}: {}", path.display());
        }
        Err(err) => {
            log::warn!("failed to run hook {}: {err}", path.display());
        }
    }
}
