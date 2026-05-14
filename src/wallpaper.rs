// ─── < Modules > ─────────────────────────────────────────────────

mod current;
mod fingerprint;

// ─── < Public API > ─────────────────────────────────────────────────

pub use current::current_wallpaper_path;
pub use fingerprint::WallpaperFingerprint;
