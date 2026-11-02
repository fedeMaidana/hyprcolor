// ─── < Imports > ────────────────────────────────────────────────────

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

// ─── < Public Functions > ────────────────────────────────────────────────────

pub fn current_wallpaper_path() -> Option<PathBuf> {
    detect_current_wallpaper(command_output, hyprwall_cache_file().as_deref())
}

// ─── < Private Functions > ────────────────────────────────────────────────────

fn detect_current_wallpaper(
    mut query: impl FnMut(&str, &[&str]) -> Option<String>,
    cache_file: Option<&Path>,
) -> Option<PathBuf> {
    // Ask the running backends first: the picker cache may describe an older
    // wallpaper when the background was changed outside hyprwall.
    for (program, args) in [
        ("awww", &["query"][..]),
        ("swww", &["query"][..]),
        ("hyprctl", &["hyprpaper", "listactive"][..]),
    ] {
        if let Some(output) = query(program, args)
            && let Some(path) = first_existing_path_from_text(&output)
        {
            return Some(path);
        }
    }

    current_from_hyprwall_cache(cache_file?)
}

// ─── < Command Output > ───────────

fn command_output(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;

    if !output.status.success() {
        return None;
    }

    Some(String::from_utf8_lossy(&output.stdout).into_owned())
}

// ─── < Current From Hyprwall Cache > ───────────

fn current_from_hyprwall_cache(cache_file: &Path) -> Option<PathBuf> {
    let path = fs::read_to_string(cache_file).ok()?;
    let path = clean_path(&path);

    path.is_file().then_some(path)
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
    .find(|path| path.is_file())
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Wallpapers {
        dir: PathBuf,
        active: PathBuf,
        cached: PathBuf,
        cache_file: PathBuf,
    }

    impl Wallpapers {
        fn new() -> Self {
            static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
            let dir = env::temp_dir().join(format!(
                "hyprcolor-wallpapers-{}-{}",
                std::process::id(),
                NEXT_ID.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&dir).unwrap();
            let active = dir.join("active wallpaper.jpeg");
            let cached = dir.join("old wallpaper.jpeg");
            let cache_file = dir.join("current_wallpaper");
            fs::write(&active, []).unwrap();
            fs::write(&cached, []).unwrap();
            fs::write(&cache_file, format!("{}\n", cached.display())).unwrap();
            Self {
                dir,
                active,
                cached,
                cache_file,
            }
        }
    }

    impl Drop for Wallpapers {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }

    #[test]
    fn live_backends_override_a_stale_picker_cache() {
        let files = Wallpapers::new();
        for backend in ["awww", "swww", "hyprctl"] {
            let actual = detect_current_wallpaper(
                |program, args| {
                    if program != backend {
                        return None;
                    }
                    if program == "hyprctl" {
                        assert_eq!(args, ["hyprpaper", "listactive"]);
                        Some(format!("eDP-1 = {}\n", files.active.display()))
                    } else {
                        assert_eq!(args, ["query"]);
                        Some(format!(
                            ": eDP-1: 1280x800, scale: 2, currently displaying: image: {}\n",
                            files.active.display()
                        ))
                    }
                },
                Some(&files.cache_file),
            );
            assert_eq!(actual, Some(files.active.clone()), "backend: {backend}");
        }
    }

    #[test]
    fn cache_is_used_when_no_backend_reports_a_valid_image() {
        let files = Wallpapers::new();
        for output in [None, Some("eDP-1: image: /missing/wallpaper.png".to_string())] {
            let actual = detect_current_wallpaper(|_, _| output.clone(), Some(&files.cache_file));
            assert_eq!(actual, Some(files.cached.clone()));
        }
    }

    #[test]
    fn active_awww_short_circuits_other_sources() {
        let files = Wallpapers::new();
        let actual = detect_current_wallpaper(
            |program, _| {
                assert_eq!(program, "awww", "a live source must take precedence");
                Some(format!("eDP-1: image: {}", files.active.display()))
            },
            Some(&files.cache_file),
        );
        assert_eq!(actual, Some(files.active.clone()));
    }

    #[test]
    fn skips_missing_images_and_directories_in_backend_output() {
        let files = Wallpapers::new();
        let output = format!(
            "eDP-1: image: /missing/wallpaper.png\nDP-1: image: {}\nDP-2: image: \"{}\"\n",
            files.dir.display(),
            files.active.display(),
        );
        assert_eq!(first_existing_path_from_text(&output), Some(files.active.clone()));
    }

    #[test]
    fn invalid_cache_does_not_resolve_to_a_wallpaper() {
        let files = Wallpapers::new();
        for content in [
            "".to_string(),
            "/missing/wallpaper.png".to_string(),
            files.dir.display().to_string(),
        ] {
            fs::write(&files.cache_file, content).unwrap();
            assert_eq!(detect_current_wallpaper(|_, _| None, Some(&files.cache_file)), None);
        }
        assert_eq!(detect_current_wallpaper(|_, _| None, None), None);
    }
}
