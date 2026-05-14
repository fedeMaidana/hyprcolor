// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::{Context, Result};
use serde_json::{Map, Value};
use std::{env, fs, path::PathBuf};

use crate::{fs::atomic_write, palette::Palette};

// ─── < Constants > ──────────────────────────────────────────────────

const THEME_NAME: &str = "Hyprcolor";

// ─── < Structs > ──────────────────────────────────────────────────

struct TerminalColors<'a> {
    background: &'a str,
    foreground: &'a str,
    foreground_muted: &'a str,
    surface_variant: &'a str,
    accent: &'a str,
    accent_2: &'a str,
    accent_3: &'a str,
}

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
    let foreground = opaque(&palette.foreground);

    let accent = opaque(&palette.accent);
    let accent_1 = opaque(&palette.accent_1);
    let accent_2 = opaque(&palette.accent_2);
    let accent_3 = opaque(&palette.accent_3);

    let background = dim(&palette.background, "#000000", 0.35, "ee");
    let surface = dim(&palette.surface, "#000000", 0.25, "dd");
    let surface_variant = dim(&palette.surface_variant, "#000000", 0.20, "dd");

    let background_soft = dim(&palette.background, "#000000", 0.40, "cc");
    let background_muted = dim(&palette.background, "#000000", 0.45, "bb");

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

    insert(&mut style, "editor.line_number", "#5f6775ff");
    insert(&mut style, "editor.active_line_number", "#c6ceddff");

    insert(&mut style, "editor.active_line.background", "#ffffff06");
    insert(
        &mut style,
        "editor.highlighted_line.background",
        "#ffffff06",
    );

    insert(&mut style, "editor.indent_guide", "#ffffff10");
    insert(&mut style, "editor.indent_guide_active", "#ffffff1a");
    insert(&mut style, "editor.wrap_guide", "#ffffff10");
    insert(&mut style, "editor.active_wrap_guide", "#ffffff1a");

    insert(&mut style, "editor.invisible", "#ffffff22");

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

    insert_status_colors(
        &mut style,
        &background,
        &foreground,
        &accent,
        &accent_1,
        &accent_2,
        &accent_3,
    );

    let terminal_colors = TerminalColors {
        background: background.as_str(),
        foreground: foreground.as_str(),
        foreground_muted: foreground_muted.as_str(),
        surface_variant: surface_variant.as_str(),
        accent: accent.as_str(),
        accent_2: accent_2.as_str(),
        accent_3: accent_3.as_str(),
    };

    insert_terminal_colors(&mut style, terminal_colors);

    insert_syntax(
        &mut style,
        &background,
        &foreground,
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

fn insert_status_colors(
    style: &mut Map<String, Value>,
    background: &str,
    foreground: &str,
    accent: &str,
    accent_1: &str,
    accent_2: &str,
    accent_3: &str,
) {
    let success = role_color(accent_2, foreground, background, 0.12, 0.04, "ff");
    let warning = role_color(accent_1, foreground, background, 0.08, 0.22, "ff");
    let error = role_color(accent_3, foreground, background, 0.10, 0.06, "ff");
    let info = role_color(accent, foreground, background, 0.10, 0.03, "ff");

    let created = success.clone();
    let modified = role_color(accent_1, foreground, background, 0.04, 0.34, "ff");
    let deleted = role_color(accent_3, foreground, background, 0.04, 0.18, "ff");
    let conflict = role_color(accent_2, foreground, background, 0.04, 0.26, "ff");
    let renamed = role_color(accent, foreground, background, 0.12, 0.08, "ff");
    let ignored = with_alpha(foreground, "55");

    insert_status_role(style, "success", &success);
    insert_status_role(style, "warning", &warning);
    insert_status_role(style, "error", &error);
    insert_status_role(style, "info", &info);

    insert_status_role(style, "created", &created);
    insert_status_role(style, "modified", &modified);
    insert_status_role(style, "deleted", &deleted);
    insert_status_role(style, "conflict", &conflict);
    insert_status_role(style, "renamed", &renamed);
    insert_status_role(style, "ignored", &ignored);
}

// ─── < Insert Status Role > ───────

fn insert_status_role(style: &mut Map<String, Value>, key: &str, color: &str) {
    insert(style, key, color);
    insert(style, &format!("{key}.background"), with_alpha(color, "22"));
    insert(style, &format!("{key}.border"), with_alpha(color, "88"));
}

// ─── < Insert Terminal Colors > ───────

fn insert_terminal_colors(style: &mut Map<String, Value>, colors: TerminalColors<'_>) {
    insert(style, "terminal.foreground", colors.foreground);
    insert(style, "terminal.bright_foreground", colors.foreground);
    insert(style, "terminal.dim_foreground", colors.foreground_muted);

    insert(style, "terminal.ansi.black", colors.background);
    insert(style, "terminal.ansi.red", "#f38ba8ff");
    insert(style, "terminal.ansi.green", "#a6e3a1ff");
    insert(style, "terminal.ansi.yellow", "#f9e2afff");
    insert(style, "terminal.ansi.blue", colors.accent);
    insert(style, "terminal.ansi.magenta", colors.accent_3);
    insert(style, "terminal.ansi.cyan", colors.accent_2);
    insert(style, "terminal.ansi.white", colors.foreground);

    insert(style, "terminal.ansi.bright_black", colors.surface_variant);
    insert(style, "terminal.ansi.bright_red", "#f38ba8ff");
    insert(style, "terminal.ansi.bright_green", "#a6e3a1ff");
    insert(style, "terminal.ansi.bright_yellow", "#f9e2afff");
    insert(style, "terminal.ansi.bright_blue", colors.accent);
    insert(style, "terminal.ansi.bright_magenta", colors.accent_3);
    insert(style, "terminal.ansi.bright_cyan", colors.accent_2);
    insert(style, "terminal.ansi.bright_white", colors.foreground);
}

// ─── < Insert Syntax > ───────

fn insert_syntax(
    style: &mut Map<String, Value>,
    background: &str,
    foreground: &str,
    accent: &str,
    accent_1: &str,
    accent_2: &str,
    accent_3: &str,
) {
    let mut syntax = Map::new();

    let comment = "#6f7482ff";

    let primary = foreground.to_string();
    let variable = with_alpha(foreground, "dd");
    let muted = with_alpha(foreground, "88");
    let subtle = with_alpha(foreground, "66");

    let function = role_color(accent, foreground, background, 0.10, 0.00, "ff");
    let keyword = role_color(accent_3, foreground, background, 0.08, 0.03, "ff");

    let type_color = role_color(accent_2, foreground, background, 0.14, 0.05, "ee");
    let property = role_color(accent_1, foreground, background, 0.08, 0.14, "dd");
    let constant = role_color(accent_2, foreground, background, 0.08, 0.14, "ee");
    let number = role_color(accent_2, foreground, background, 0.04, 0.20, "ee");

    let string = role_color(accent_1, foreground, background, 0.18, 0.16, "ee");
    let special = role_color(accent_3, foreground, background, 0.16, 0.02, "ff");

    syntax.insert(
        "comment".to_string(),
        syntax_entry(comment, Some("italic"), None),
    );
    syntax.insert(
        "comment.doc".to_string(),
        syntax_entry(comment, Some("italic"), None),
    );

    syntax.insert("primary".to_string(), syntax_entry(&primary, None, None));
    syntax.insert("variable".to_string(), syntax_entry(&variable, None, None));
    syntax.insert("operator".to_string(), syntax_entry(&muted, None, None));
    syntax.insert("punctuation".to_string(), syntax_entry(&subtle, None, None));

    syntax.insert(
        "keyword".to_string(),
        syntax_entry(&keyword, None, Some(500)),
    );
    syntax.insert("preproc".to_string(), syntax_entry(&keyword, None, None));
    syntax.insert("tag".to_string(), syntax_entry(&keyword, None, None));

    syntax.insert(
        "function".to_string(),
        syntax_entry(&function, None, Some(500)),
    );
    syntax.insert(
        "title".to_string(),
        syntax_entry(&function, None, Some(600)),
    );

    syntax.insert("type".to_string(), syntax_entry(&type_color, None, None));
    syntax.insert(
        "constructor".to_string(),
        syntax_entry(&type_color, None, None),
    );

    syntax.insert("property".to_string(), syntax_entry(&property, None, None));
    syntax.insert("label".to_string(), syntax_entry(&property, None, None));

    syntax.insert("constant".to_string(), syntax_entry(&constant, None, None));
    syntax.insert("number".to_string(), syntax_entry(&number, None, None));

    syntax.insert("string".to_string(), syntax_entry(&string, None, None));
    syntax.insert(
        "text.literal".to_string(),
        syntax_entry(&string, None, None),
    );
    syntax.insert(
        "string.escape".to_string(),
        syntax_entry(&special, None, None),
    );

    syntax.insert(
        "variable.special".to_string(),
        syntax_entry(&special, None, None),
    );
    syntax.insert(
        "emphasis".to_string(),
        syntax_entry(&function, Some("italic"), None),
    );
    syntax.insert(
        "link_uri".to_string(),
        syntax_entry(&function, Some("normal"), None),
    );
    syntax.insert(
        "predictive".to_string(),
        syntax_entry(comment, Some("italic"), None),
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

// ─── < Role Color > ───────

fn role_color(
    color: &str,
    foreground: &str,
    background: &str,
    soften_amount: f32,
    dim_amount: f32,
    alpha: &str,
) -> String {
    let softened = soften(color, foreground, soften_amount, "ff");

    dim(&softened, background, dim_amount, alpha)
}

// ─── < Soften > ───────

fn soften(color: &str, foreground: &str, amount: f32, alpha: &str) -> String {
    mix_hex(color, foreground, amount, alpha)
}

// ─── < Dim > ───────

fn dim(color: &str, background: &str, amount: f32, alpha: &str) -> String {
    mix_hex(color, background, amount, alpha)
}

// ─── < Mix Hex > ───────

fn mix_hex(left: &str, right: &str, right_amount: f32, alpha: &str) -> String {
    let (left_r, left_g, left_b) = parse_rgb(left);
    let (right_r, right_g, right_b) = parse_rgb(right);

    let amount = right_amount.clamp(0.0, 1.0);

    let mix_channel = |left: u8, right: u8| -> u8 {
        let mixed = left as f32 + (right as f32 - left as f32) * amount;

        mixed.round().clamp(0.0, 255.0) as u8
    };

    rgba(
        mix_channel(left_r, right_r),
        mix_channel(left_g, right_g),
        mix_channel(left_b, right_b),
        alpha,
    )
}

// ─── < Parse RGB > ───────

fn parse_rgb(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');

    let rgb = match hex.len() {
        6 => hex,
        8 => &hex[..6],
        _ => "ffffff",
    };

    let red = u8::from_str_radix(&rgb[0..2], 16).unwrap_or(255);
    let green = u8::from_str_radix(&rgb[2..4], 16).unwrap_or(255);
    let blue = u8::from_str_radix(&rgb[4..6], 16).unwrap_or(255);

    (red, green, blue)
}

// ─── < RGBA > ───────

fn rgba(red: u8, green: u8, blue: u8, alpha: &str) -> String {
    format!("#{red:02x}{green:02x}{blue:02x}{alpha}")
}
