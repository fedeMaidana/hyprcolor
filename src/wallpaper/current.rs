// ─── < Imports > ────────────────────────────────────────────────────

use std::{env, fs, path::PathBuf, process::Command};

// ─── < Public Functions > ────────────────────────────────────────────────────

pub fn current_wallpaper_path() -> Option<PathBuf> {
    current_from_hyprwall_cache()
        .or_else(current_from_swww)
        .or_else(current_from_hyprpaper)
}

// ─── < Private Functions > ────────────────────────────────────────────────────

// ─── < Current From Hyprpaper > ───────────

fn current_from_hyprpaper() -> Option<PathBuf> {
    let output = Command::new("hyprctl")
        .args(["hyprpaper", "listactive"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    first_existing_path_from_text(&String::from_utf8_lossy(&output.stdout))
}

// ─── < Current From Swww > ───────────

fn current_from_swww() -> Option<PathBuf> {
    let output = Command::new("swww").arg("query").output().ok()?;

    if !output.status.success() {
        return None;
    }

    first_existing_path_from_text(&String::from_utf8_lossy(&output.stdout))
}

// ─── < Current From Hyprwall Cache > ───────────

fn current_from_hyprwall_cache() -> Option<PathBuf> {
    let path = fs::read_to_string(hyprwall_cache_file()?).ok()?;
    let path = PathBuf::from(path.trim());

    path.exists().then_some(path)
}

// ─── < First Existing Path From Text > ───────────

fn first_existing_path_from_text(text: &str) -> Option<PathBuf> {
    text.lines().find_map(existing_path_from_line)
}

// ─── < Existing Path From Line > ───────────

fn existing_path_from_line(line: &str) -> Option<PathBuf> {
    [
        value_after(line, "image:"),
        value_after(line, "Image:"),
        value_after(line, "path:"),
        value_after(line, "Path:"),
        value_after(line, ":"),
        value_after(line, "="),
    ]
    .into_iter()
    .flatten()
    .map(clean_path)
    .find(|path| path.exists())
}

// ─── < Value After > ───────────

fn value_after<'a>(line: &'a str, separator: &str) -> Option<&'a str> {
    line.split_once(separator).map(|(_, value)| value.trim())
}

// ─── < Clean Path > ───────────

fn clean_path(path: &str) -> PathBuf {
    let path = path.trim().trim_matches('"').trim_matches('\'');

    if let Some(rest) = path.strip_prefix("~/")
        && let Some(home) = env::var_os("HOME")
    {
        return PathBuf::from(home).join(rest);
    }

    PathBuf::from(path)
}

// ─── < Hyprwall Cache File > ───────────

fn hyprwall_cache_file() -> Option<PathBuf> {
    if let Some(cache_home) = env::var_os("XDG_CACHE_HOME") {
        return Some(PathBuf::from(cache_home).join("hyprwall/current_wallpaper"));
    }

    env::var_os("HOME").map(|home| {
        PathBuf::from(home)
            .join(".cache")
            .join("hyprwall")
            .join("current_wallpaper")
    })
}
