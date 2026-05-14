// ─── < Modules > ──────────────────────────────────────────

mod color;
mod extractor;
mod model;

// ─── < Public API > ─────────────────────────────────────────────────

pub use extractor::extract_palette;
pub use model::Palette;
