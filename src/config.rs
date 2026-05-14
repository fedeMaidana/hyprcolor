// ─── < Imports > ────────────────────────────────────────────────────

use std::{env, path::PathBuf, time::Duration};

// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Config {
    pub output_dir: PathBuf,
    pub poll_interval: Duration,
}

// ─── < Implementations > ────────────────────────────────────────────

impl Config {
    pub fn load() -> Self {
        Self {
            output_dir: default_cache_dir(),
            poll_interval: Duration::from_secs(1),
        }
    }
}

// ─── < Private Functions > ──────────────────────────────────────────

fn default_cache_dir() -> PathBuf {
    if let Some(cache_home) = env::var_os("XDG_CACHE_HOME") {
        return PathBuf::from(cache_home).join("hyprcolors");
    }

    env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".cache")
        .join("hyprcolors")
}
