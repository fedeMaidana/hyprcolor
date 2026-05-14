// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::{Context, Result};
use serde_json::{Map, Value};
use std::{env, fs, path::PathBuf};

use crate::{fs::atomic_write, palette::Palette};

// ─── < Constants > ──────────────────────────────────────────────────

const THEME_NAME: &str = "Hyprcolor";

// ─── < Public Functions > ───────────────────────────────────────────

pub fn export(palette: &Palette) -> Result<()> {
    let theme_dir =
        zed_themes_dir().context("no se pudo resolver el directorio de themes de Zed")?;

    fs::create_dir_all(&theme_dir).with_context(|| {
        format!(
            "no se pudo crear el directorio de themes de Zed: {}",
            theme_dir.display()
        )
    })?;

    let theme = build_theme(palette);
    let content = serde_json::to_string_pretty(&theme)?;

    atomic_write(theme_dir.join("hyprcolor.json"), format!("{content}\n"))
}

// ─── < Private Functions > ──────────────────────────────────────────

// ─── < Zed Themes Dir > ───────

fn zed_themes_dir() -> Option<PathBuf> {
    if let Some(config_home) = env::var_os("XDG_CONFIG_HOME") {
        return Some(PathBuf::from(config_home).join("zed/themes"));
    }

    env::var_os("HOME").map(|home| PathBuf::from(home).join(".config/zed/themes"))
}

// ─── < Build Theme > ───────

fn build_theme(palette: &Palette) -> Value {
    let background = "#000000ff".to_string();
    let foreground = opaque(&palette.foreground);

    let accent = opaque(&palette.accent);
    let accent_1 = opaque(&palette.accent_1);
    let accent_2 = opaque(&palette.accent_2);
    let accent_3 = opaque(&palette.accent_3);

    let surface = "#050505ff".to_string();
    let surface_variant = "#151515ff".to_string();

    let background_soft = "#000000ff".to_string();
    let background_muted = "#000000ff".to_string();

    let foreground_muted = with_alpha(&palette.foreground, "99");
    let foreground_disabled = with_alpha(&palette.foreground, "55");

    let accent_soft = with_alpha(&palette.accent, "33");
    let accent_hover = with_alpha(&palette.accent, "55");

    let transparent = "#00000000".to_string();

    let mut style = Map::new();

    insert_array(
        &mut style,
        "accents",
        vec![
            accent.clone(),
            accent_1.clone(),
            accent_2.clone(),
            accent_3.clone(),
        ],
    );

    insert(&mut style, "background.appearance", "blurred");

    insert(&mut style, "background", background.clone());
    insert(&mut style, "surface.background", background_soft.clone());
    insert(&mut style, "elevated_surface.background", surface.clone());
    insert(&mut style, "panel.background", background_soft.clone());
    insert(&mut style, "editor.background", background_muted.clone());
    insert(&mut style, "terminal.background", background_muted.clone());

    insert(&mut style, "title_bar.background", background_soft.clone());
    insert(
        &mut style,
        "title_bar.inactive_background",
        background_muted.clone(),
    );
    insert(&mut style, "toolbar.background", background_soft.clone());
    insert(&mut style, "tab_bar.background", background_soft.clone());
    insert(&mut style, "tab.active_background", surface.clone());
    insert(
        &mut style,
        "tab.inactive_background",
        background_muted.clone(),
    );
    insert(&mut style, "status_bar.background", background_soft.clone());

    insert(&mut style, "border", background.clone());
    insert(&mut style, "border.variant", background.clone());
    insert(&mut style, "border.focused", background.clone());
    insert(&mut style, "border.selected", background.clone());
    insert(&mut style, "border.transparent", transparent.clone());
    insert(&mut style, "border.disabled", background.clone());

    insert(&mut style, "panel.focused_border", background.clone());
    insert(&mut style, "pane.focused_border", background.clone());
    insert(&mut style, "pane_group.border", background.clone());

    insert(&mut style, "text", foreground.clone());
    insert(&mut style, "text.accent", accent.clone());
    insert(&mut style, "text.muted", foreground_muted.clone());
    insert(&mut style, "text.placeholder", foreground_muted.clone());
    insert(&mut style, "text.disabled", foreground_disabled.clone());

    insert(&mut style, "icon", foreground.clone());
    insert(&mut style, "icon.accent", accent.clone());
    insert(&mut style, "icon.muted", foreground_muted.clone());
    insert(&mut style, "icon.placeholder", foreground_muted.clone());
    insert(&mut style, "icon.disabled", foreground_disabled.clone());

    insert(&mut style, "element.background", surface.clone());
    insert(&mut style, "element.hover", accent_soft.clone());
    insert(&mut style, "element.active", accent_hover.clone());
    insert(&mut style, "element.selected", accent_soft.clone());
    insert(&mut style, "element.disabled", background_muted.clone());

    insert(&mut style, "ghost_element.background", transparent.clone());
    insert(&mut style, "ghost_element.hover", accent_soft.clone());
    insert(&mut style, "ghost_element.active", accent_hover.clone());
    insert(&mut style, "ghost_element.selected", accent_soft.clone());
    insert(&mut style, "ghost_element.disabled", transparent.clone());

    insert(&mut style, "editor.foreground", foreground.clone());
    insert(&mut style, "editor.gutter.background", transparent.clone());
    insert(&mut style, "editor.line_number", foreground_muted.clone());
    insert(&mut style, "editor.active_line_number", accent.clone());
    insert(&mut style, "editor.active_line.background", surface.clone());
    insert(
        &mut style,
        "editor.highlighted_line.background",
        surface.clone(),
    );
    insert(&mut style, "editor.indent_guide", background.clone());
    insert(&mut style, "editor.indent_guide_active", background.clone());
    insert(&mut style, "editor.wrap_guide", background.clone());
    insert(&mut style, "editor.active_wrap_guide", background.clone());
    insert(&mut style, "editor.invisible", foreground_disabled.clone());

    insert(
        &mut style,
        "scrollbar.thumb.background",
        accent_soft.clone(),
    );
    insert(
        &mut style,
        "scrollbar.thumb.hover_background",
        accent_hover.clone(),
    );
    insert(&mut style, "scrollbar.thumb.border", transparent.clone());
    insert(
        &mut style,
        "scrollbar.track.background",
        transparent.clone(),
    );
    insert(&mut style, "scrollbar.track.border", transparent.clone());

    insert(&mut style, "search.match_background", accent_soft.clone());
    insert(&mut style, "drop_target.background", accent_soft.clone());
    insert(&mut style, "link_text.hover", accent.clone());

    insert_status_colors(&mut style, &accent_2);
    insert_terminal_colors(
        &mut style,
        &background,
        &foreground,
        &foreground_muted,
        &surface_variant,
        &accent,
        &accent_2,
        &accent_3,
    );
    insert_syntax(
        &mut style,
        &foreground,
        &foreground_muted,
        &accent,
        &accent_1,
        &accent_2,
        &accent_3,
    );

    let mut theme = Map::new();
    theme.insert("name".to_string(), Value::String(THEME_NAME.to_string()));
    theme.insert("appearance".to_string(), Value::String("dark".to_string()));
    theme.insert("style".to_string(), Value::Object(style));

    let mut root = Map::new();
    root.insert(
        "$schema".to_string(),
        Value::String("https://zed.dev/schema/themes/v0.2.0.json".to_string()),
    );
    root.insert("name".to_string(), Value::String(THEME_NAME.to_string()));
    root.insert(
        "author".to_string(),
        Value::String("Federico Maidana".to_string()),
    );
    root.insert(
        "themes".to_string(),
        Value::Array(vec![Value::Object(theme)]),
    );

    Value::Object(root)
}

// ─── < Insert > ───────

fn insert(style: &mut Map<String, Value>, key: &str, value: impl Into<String>) {
    style.insert(key.to_string(), Value::String(value.into()));
}

// ─── < Insert Array > ───────

fn insert_array(style: &mut Map<String, Value>, key: &str, values: Vec<String>) {
    style.insert(
        key.to_string(),
        Value::Array(values.into_iter().map(Value::String).collect()),
    );
}

// ─── < Insert Status Colors > ───────

fn insert_status_colors(style: &mut Map<String, Value>, info: &str) {
    insert(style, "success", "#a6e3a1ff");
    insert(style, "success.background", "#a6e3a133");
    insert(style, "success.border", "#a6e3a1aa");

    insert(style, "warning", "#f9e2afff");
    insert(style, "warning.background", "#f9e2af33");
    insert(style, "warning.border", "#f9e2afaa");

    insert(style, "error", "#f38ba8ff");
    insert(style, "error.background", "#f38ba833");
    insert(style, "error.border", "#f38ba8aa");

    insert(style, "info", info);
    insert(style, "info.background", with_alpha(info, "33"));
    insert(style, "info.border", with_alpha(info, "aa"));
}

// ─── < Insert Terminal Colors > ───────

fn insert_terminal_colors(
    style: &mut Map<String, Value>,
    background: &str,
    foreground: &str,
    foreground_muted: &str,
    surface_variant: &str,
    accent: &str,
    accent_2: &str,
    accent_3: &str,
) {
    insert(style, "terminal.foreground", foreground);
    insert(style, "terminal.bright_foreground", foreground);
    insert(style, "terminal.dim_foreground", foreground_muted);

    insert(style, "terminal.ansi.black", background);
    insert(style, "terminal.ansi.red", "#f38ba8ff");
    insert(style, "terminal.ansi.green", "#a6e3a1ff");
    insert(style, "terminal.ansi.yellow", "#f9e2afff");
    insert(style, "terminal.ansi.blue", accent);
    insert(style, "terminal.ansi.magenta", accent_3);
    insert(style, "terminal.ansi.cyan", accent_2);
    insert(style, "terminal.ansi.white", foreground);

    insert(style, "terminal.ansi.bright_black", surface_variant);
    insert(style, "terminal.ansi.bright_red", "#f38ba8ff");
    insert(style, "terminal.ansi.bright_green", "#a6e3a1ff");
    insert(style, "terminal.ansi.bright_yellow", "#f9e2afff");
    insert(style, "terminal.ansi.bright_blue", accent);
    insert(style, "terminal.ansi.bright_magenta", accent_3);
    insert(style, "terminal.ansi.bright_cyan", accent_2);
    insert(style, "terminal.ansi.bright_white", foreground);
}

// ─── < Insert Syntax > ───────

fn insert_syntax(
    style: &mut Map<String, Value>,
    foreground: &str,
    foreground_muted: &str,
    accent: &str,
    accent_1: &str,
    accent_2: &str,
    accent_3: &str,
) {
    let mut syntax = Map::new();

    syntax.insert(
        "comment".to_string(),
        syntax_entry(foreground_muted, Some("italic"), None),
    );
    syntax.insert(
        "comment.doc".to_string(),
        syntax_entry(foreground_muted, Some("italic"), None),
    );
    syntax.insert("constant".to_string(), syntax_entry(accent_2, None, None));
    syntax.insert(
        "constructor".to_string(),
        syntax_entry(accent_2, None, None),
    );
    syntax.insert(
        "emphasis".to_string(),
        syntax_entry(accent, Some("italic"), None),
    );
    syntax.insert("function".to_string(), syntax_entry(accent, None, None));
    syntax.insert("keyword".to_string(), syntax_entry(accent_3, None, None));
    syntax.insert("label".to_string(), syntax_entry(accent_2, None, None));
    syntax.insert(
        "link_uri".to_string(),
        syntax_entry(accent, Some("normal"), None),
    );
    syntax.insert("number".to_string(), syntax_entry(accent_2, None, None));
    syntax.insert("operator".to_string(), syntax_entry(foreground, None, None));
    syntax.insert(
        "predictive".to_string(),
        syntax_entry(foreground_muted, Some("italic"), None),
    );
    syntax.insert("preproc".to_string(), syntax_entry(accent_3, None, None));
    syntax.insert("primary".to_string(), syntax_entry(foreground, None, None));
    syntax.insert("property".to_string(), syntax_entry(accent_1, None, None));
    syntax.insert(
        "punctuation".to_string(),
        syntax_entry(foreground_muted, None, None),
    );
    syntax.insert("string".to_string(), syntax_entry("#a6e3a1ff", None, None));
    syntax.insert(
        "string.escape".to_string(),
        syntax_entry(accent_2, None, None),
    );
    syntax.insert("tag".to_string(), syntax_entry(accent_3, None, None));
    syntax.insert(
        "text.literal".to_string(),
        syntax_entry("#a6e3a1ff", None, None),
    );
    syntax.insert("title".to_string(), syntax_entry(accent, None, Some(600)));
    syntax.insert("type".to_string(), syntax_entry(accent_2, None, None));
    syntax.insert("variable".to_string(), syntax_entry(foreground, None, None));
    syntax.insert(
        "variable.special".to_string(),
        syntax_entry(accent_1, None, None),
    );

    style.insert("syntax".to_string(), Value::Object(syntax));
}

// ─── < Syntax Entry > ───────

fn syntax_entry(color: &str, font_style: Option<&str>, font_weight: Option<u64>) -> Value {
    let mut entry = Map::new();

    entry.insert("color".to_string(), Value::String(color.to_string()));

    if let Some(font_style) = font_style {
        entry.insert(
            "font_style".to_string(),
            Value::String(font_style.to_string()),
        );
    }

    if let Some(font_weight) = font_weight {
        entry.insert("font_weight".to_string(), Value::Number(font_weight.into()));
    }

    Value::Object(entry)
}

// ─── < Opaque > ───────

fn opaque(hex: &str) -> String {
    with_alpha(hex, "ff")
}

// ─── < With Alpha > ───────

fn with_alpha(hex: &str, alpha: &str) -> String {
    let hex = hex.trim_start_matches('#');

    let rgb = match hex.len() {
        6 => hex,
        8 => &hex[..6],
        _ => "ffffff",
    };

    format!("#{rgb}{alpha}")
}
