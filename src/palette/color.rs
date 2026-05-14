// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// ─── < Implementations > ────────────────────────────────────────────────────

impl Rgb {
    pub fn to_hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    pub fn luminance(self) -> f32 {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    pub fn saturation(self) -> f32 {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);

        if max == 0.0 { 0.0 } else { (max - min) / max }
    }

    pub fn distance(self, other: Self) -> f32 {
        let dr = self.r as f32 - other.r as f32;
        let dg = self.g as f32 - other.g as f32;
        let db = self.b as f32 - other.b as f32;

        (dr * dr + dg * dg + db * db).sqrt()
    }

    pub fn darken(self, amount: f32) -> Self {
        let factor = 1.0 - amount.clamp(0.0, 1.0);

        Self {
            r: (self.r as f32 * factor) as u8,
            g: (self.g as f32 * factor) as u8,
            b: (self.b as f32 * factor) as u8,
        }
    }
}
