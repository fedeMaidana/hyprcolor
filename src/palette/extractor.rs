// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::{Context, Result};

use image::imageops::FilterType;

use std::{cmp::Ordering, collections::HashMap, path::Path};

use super::{color::Rgb, model::Palette};

// ─── < Constants > ──────────────────────────────────────────────────

const SWATCH_COUNT: usize = 8;

// ─── < Structs > ────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
struct Hsl {
    h: f32,
    s: f32,
    l: f32,
}

// ─── < Implementations > ────────────────────────────────────────────────

impl Hsl {
    fn from_rgb(color: Rgb) -> Self {
        let r = color.r as f32 / 255.0;
        let g = color.g as f32 / 255.0;
        let b = color.b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let l = (max + min) / 2.0;

        if delta == 0.0 {
            return Self { h: 0.0, s: 0.0, l };
        }

        let s = delta / (1.0 - (2.0 * l - 1.0).abs());

        let h = if max == r {
            ((g - b) / delta).rem_euclid(6.0) / 6.0
        } else if max == g {
            (((b - r) / delta) + 2.0) / 6.0
        } else {
            (((r - g) / delta) + 4.0) / 6.0
        };

        Self { h: wrap_unit(h), s, l }
    }

    fn to_rgb(self) -> Rgb {
        let h = wrap_unit(self.h);
        let s = self.s.clamp(0.0, 1.0);
        let l = self.l.clamp(0.0, 1.0);

        if s == 0.0 {
            let value = channel_from_unit(l);
            return Rgb {
                r: value,
                g: value,
                b: value,
            };
        }

        let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };

        let p = 2.0 * l - q;

        Rgb {
            r: channel_from_unit(hue_to_rgb(p, q, h + 1.0 / 3.0)),
            g: channel_from_unit(hue_to_rgb(p, q, h)),
            b: channel_from_unit(hue_to_rgb(p, q, h - 1.0 / 3.0)),
        }
    }
}

// ─── < Public Functions > ───────────────────────────────────────────

// ─── < Extract Palette > ──────

pub fn extract_palette(path: &Path) -> Result<Palette> {
    let image = image::open(path)
        .with_context(|| format!("no se pudo abrir imagen {}", path.display()))?
        .resize(180, 180, FilterType::Triangle)
        .to_rgb8();

    let mut histogram: HashMap<(u8, u8, u8), u32> = HashMap::new();

    for pixel in image.pixels() {
        let [r, g, b] = pixel.0;

        let key = (r / 12, g / 12, b / 12);
        *histogram.entry(key).or_insert(0) += 1;
    }

    let mut colors = histogram
        .into_iter()
        .map(|((r, g, b), count)| {
            (
                Rgb {
                    r: (r * 12).saturating_add(6),
                    g: (g * 12).saturating_add(6),
                    b: (b * 12).saturating_add(6),
                },
                count,
            )
        })
        .collect::<Vec<_>>();

    colors.sort_by_key(|(_, count)| std::cmp::Reverse(*count));

    let dominant = colors
        .first()
        .map(|(color, _)| *color)
        .unwrap_or(Rgb { r: 20, g: 20, b: 20 });

    let accents = pick_accents(&colors);
    let swatches = pick_visual_swatches(&colors, &accents);
    let ui_accent = pick_ui_accent(&swatches).unwrap_or(accents[0]);

    let background = dominant.darken(0.65);
    let surface = dominant.darken(0.45);
    let surface_variant = dominant.darken(0.25);

    let foreground = if background.luminance() > 0.5 {
        Rgb { r: 20, g: 20, b: 20 }
    } else {
        Rgb { r: 238, g: 238, b: 238 }
    };

    Ok(Palette {
        wallpaper: path.to_path_buf(),

        background: background.to_hex(),
        foreground: foreground.to_hex(),
        surface: surface.to_hex(),
        surface_variant: surface_variant.to_hex(),

        accent: ui_accent.to_hex(),
        accent_1: accents[0].to_hex(),
        accent_2: accents[1].to_hex(),
        accent_3: accents[2].to_hex(),

        ui_accent: ui_accent.to_hex(),
        swatches: swatches.into_iter().map(Rgb::to_hex).collect(),
    })
}

// ─── < Private Functions > ───────────────────────────────────────────

// ─── < Pick Accent > ──────

fn pick_accents(colors: &[(Rgb, u32)]) -> [Rgb; 3] {
    let mut picked = Vec::new();

    for (color, count) in colors {
        let saturation = color.saturation();
        let luminance = color.luminance();

        if saturation < 0.20 {
            continue;
        }

        if !(0.18..=0.86).contains(&luminance) {
            continue;
        }

        let score = *count as f32 * (1.0 + saturation);

        if score <= 0.0 {
            continue;
        }

        if picked
            .iter()
            .all(|picked_color: &Rgb| picked_color.distance(*color) > 42.0)
        {
            picked.push(*color);
        }

        if picked.len() == 3 {
            break;
        }
    }

    while picked.len() < 3 {
        picked.push(
            colors
                .iter()
                .map(|(color, _)| *color)
                .find(|color| color.saturation() >= 0.08)
                .unwrap_or(Rgb { r: 136, g: 136, b: 136 }),
        );
    }

    [picked[0], picked[1], picked[2]]
}

// ─── < Pick Visual Swatches > ──────

fn pick_visual_swatches(colors: &[(Rgb, u32)], accents: &[Rgb; 3]) -> Vec<Rgb> {
    let mut picked = Vec::new();

    for accent in accents {
        push_unique_swatch(&mut picked, normalize_swatch(*accent), 28.0);
    }

    let mut rich_candidates = colors
        .iter()
        .copied()
        .filter(|(color, _)| {
            let saturation = color.saturation();
            let luminance = color.luminance();

            saturation >= 0.18 && (0.16..=0.88).contains(&luminance)
        })
        .collect::<Vec<_>>();

    rich_candidates.sort_by(|(a_color, a_count), (b_color, b_count)| {
        swatch_score(*b_color, *b_count)
            .partial_cmp(&swatch_score(*a_color, *a_count))
            .unwrap_or(Ordering::Equal)
    });

    for (color, _) in rich_candidates {
        push_unique_swatch(&mut picked, normalize_swatch(color), 28.0);

        if picked.len() == SWATCH_COUNT {
            return picked;
        }
    }

    let mut relaxed_candidates = colors
        .iter()
        .copied()
        .filter(|(color, _)| {
            let luminance = color.luminance();
            (0.10..=0.92).contains(&luminance)
        })
        .collect::<Vec<_>>();

    relaxed_candidates.sort_by(|(a_color, a_count), (b_color, b_count)| {
        relaxed_swatch_score(*b_color, *b_count)
            .partial_cmp(&relaxed_swatch_score(*a_color, *a_count))
            .unwrap_or(Ordering::Equal)
    });

    for (color, _) in relaxed_candidates {
        push_unique_swatch(&mut picked, normalize_swatch(color), 24.0);

        if picked.len() == SWATCH_COUNT {
            return picked;
        }
    }

    complete_with_derived_swatches(&mut picked);

    picked.truncate(SWATCH_COUNT);
    picked
}

// ─── < Push Unique Swatch > ──────

fn push_unique_swatch(swatches: &mut Vec<Rgb>, color: Rgb, min_distance: f32) {
    if swatches.iter().any(|picked| picked.distance(color) < min_distance) {
        return;
    }

    swatches.push(color);
}

// ─── < Swatch Score > ──────

fn swatch_score(color: Rgb, count: u32) -> f32 {
    let saturation = color.saturation();
    let luminance = color.luminance();

    count as f32 * (0.80 + saturation * 3.20) - (luminance - 0.56).abs() * 750.0
}

// ─── < Relaxed Swatch Score > ──────

fn relaxed_swatch_score(color: Rgb, count: u32) -> f32 {
    let saturation = color.saturation();
    let luminance = color.luminance();

    count as f32 * (0.45 + saturation * 2.00) - (luminance - 0.58).abs() * 500.0
}

// ─── < Normalize Swatch > ──────

fn normalize_swatch(color: Rgb) -> Rgb {
    let mut hsl = Hsl::from_rgb(color);

    hsl.s = hsl.s.max(0.42);

    hsl.l = hsl.l.clamp(0.38, 0.72);

    hsl.to_rgb()
}

// ─── < Complete With Derived Swatches > ──────

fn complete_with_derived_swatches(swatches: &mut Vec<Rgb>) {
    let seeds = if swatches.is_empty() {
        vec![Rgb { r: 120, g: 170, b: 255 }]
    } else {
        swatches.clone()
    };

    let variants = [
        (0.00, 0.52, 0.50),
        (0.00, 0.62, 0.62),
        (0.08, 0.56, 0.54),
        (-0.08, 0.56, 0.54),
        (0.16, 0.58, 0.58),
        (-0.16, 0.58, 0.58),
        (0.24, 0.60, 0.60),
        (-0.24, 0.60, 0.60),
    ];

    for seed in seeds.iter().cycle() {
        let base = Hsl::from_rgb(*seed);

        for (h_shift, saturation, lightness) in variants {
            let color = Hsl {
                h: wrap_unit(base.h + h_shift),
                s: saturation,
                l: lightness,
            }
            .to_rgb();

            push_unique_swatch(swatches, color, 24.0);

            if swatches.len() == SWATCH_COUNT {
                return;
            }
        }

        if swatches.len() == SWATCH_COUNT {
            return;
        }
    }
}

// ─── < Pick UI Accent > ──────

fn pick_ui_accent(swatches: &[Rgb]) -> Option<Rgb> {
    swatches
        .iter()
        .copied()
        .max_by(|a, b| {
            ui_color_score(*a)
                .partial_cmp(&ui_color_score(*b))
                .unwrap_or(Ordering::Equal)
        })
        .map(normalize_ui_color)
}

// ─── < UI Color Score > ──────

fn ui_color_score(color: Rgb) -> f32 {
    let saturation = color.saturation();
    let luminance = color.luminance();

    saturation * 3.0 - (luminance - 0.58).abs()
}

// ─── < Normalize UI Color > ──────

fn normalize_ui_color(color: Rgb) -> Rgb {
    let mut hsl = Hsl::from_rgb(color);

    hsl.s = hsl.s.max(0.48);
    hsl.l = hsl.l.clamp(0.42, 0.68);

    hsl.to_rgb()
}

// ─── < Hue To RGB > ──────

fn hue_to_rgb(p: f32, q: f32, t: f32) -> f32 {
    let t = wrap_unit(t);

    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }

    if t < 1.0 / 2.0 {
        return q;
    }

    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }

    p
}

// ─── < Channel From Unit > ──────

fn channel_from_unit(value: f32) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}

// ─── < Wrap Unit > ──────

fn wrap_unit(value: f32) -> f32 {
    value.rem_euclid(1.0)
}
