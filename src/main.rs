// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::Result;

// ─── < Modules > ────────────────────────────────────────────────────

mod app;
mod config;
mod export;
mod fs;
mod hooks;
mod logging;
mod palette;
mod wallpaper;
mod wayland;

// ── < Entry Point > ─────────────────────────────────────────────

fn main() -> Result<()> {
    logging::init();
    app::App::run()
}
