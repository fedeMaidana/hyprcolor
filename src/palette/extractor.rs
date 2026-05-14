// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::{Context, Result};

use image::imageops::FilterType;

use std::{collections::HashMap, path::Path};

use super::{color::Rgb, model::Palette};

// ─── < Public Functions > ────────────────────────────────────────────────────

pub fn extract_palette(path: &Path) -> Result<Palette> {
    let image = image::open(path)
        .with_context(|| format!("no se pudo abrir imagen {}", path.display()))?
        .resize(160, 160, FilterType::Triangle)
        .to_rgb8();

    let mut histogram: HashMap<(u8, u8, u8), u32> = HashMap::new();

    for pixel in image.pixels() {
        let [r, g, b] = pixel.0;

        let key = (r / 16, g / 16, b / 16);
        *histogram.entry(key).or_insert(0) += 1;
    }

    let mut colors = histogram
        .into_iter()
        .map(|((r, g, b), count)| {
            (
                Rgb {
                    r: r * 16 + 8,
                    g: g * 16 + 8,
                    b: b * 16 + 8,
                },
                count,
            )
        })
        .collect::<Vec<_>>();

    colors.sort_by_key(|(_, count)| std::cmp::Reverse(*count));

    let dominant = colors.first().map(|(color, _)| *color).unwrap_or(Rgb {
        r: 20,
        g: 20,
        b: 20,
    });

    let accents = pick_accents(&colors);

    let background = dominant.darken(0.65);
    let surface = dominant.darken(0.45);
    let surface_variant = dominant.darken(0.25);

    let foreground = if background.luminance() > 0.5 {
        Rgb {
            r: 20,
            g: 20,
            b: 20,
        }
    } else {
        Rgb {
            r: 238,
            g: 238,
            b: 238,
        }
    };

    Ok(Palette {
        wallpaper: path.to_path_buf(),
        background: background.to_hex(),
        foreground: foreground.to_hex(),
        accent: accents[0].to_hex(),
        accent_1: accents[0].to_hex(),
        accent_2: accents[1].to_hex(),
        accent_3: accents[2].to_hex(),
        surface: surface.to_hex(),
        surface_variant: surface_variant.to_hex(),
    })
}

// ─── < Private Functions > ────────────────────────────────────────────────────

fn pick_accents(colors: &[(Rgb, u32)]) -> [Rgb; 3] {
    let mut picked = Vec::new();

    for (color, count) in colors {
        let saturation = color.saturation();
        let luminance = color.luminance();

        if saturation < 0.20 {
            continue;
        }

        if !(0.20..=0.85).contains(&luminance) {
            continue;
        }

        let score = *count as f32 * (1.0 + saturation);

        if score <= 0.0 {
            continue;
        }

        if picked
            .iter()
            .all(|picked_color: &Rgb| picked_color.distance(*color) > 45.0)
        {
            picked.push(*color);
        }

        if picked.len() == 3 {
            break;
        }
    }

    while picked.len() < 3 {
        picked.push(Rgb {
            r: 136,
            g: 136,
            b: 136,
        });
    }

    [picked[0], picked[1], picked[2]]
}
