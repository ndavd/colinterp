#![allow(dead_code)]

use colored::*;

#[derive(Debug)]
pub struct HexColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl std::fmt::Display for HexColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let simple_luminance = self.r as f32 * 0.2 + self.g as f32 * 0.7 + self.b as f32 * 0.1;
        let fg = if simple_luminance >= 128.0 {
            Color::Black
        } else {
            Color::White
        };
        write!(
            f,
            "{}",
            self.get_hex()
                .to_uppercase()
                .bold()
                .on_color(self.to_true_color())
                .color(fg)
        )
    }
}

impl HexColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_str(hex_str: &str) -> Result<Self, &str> {
        let err_msg = "Invalid color";
        let mut hex_char_vec = hex_str.chars().collect::<Vec<_>>();

        if hex_str.len() != 7 || hex_char_vec[0] != '#' {
            return Err(err_msg);
        }

        let hex_vals = hex_char_vec.split_off(1);
        let mut hex_chunks = hex_vals.chunks(2);

        let (r, g, b) = match (
            u8::from_str_radix(&hex_chunks.next().unwrap().iter().collect::<String>(), 16),
            u8::from_str_radix(&hex_chunks.next().unwrap().iter().collect::<String>(), 16),
            u8::from_str_radix(&hex_chunks.next().unwrap().iter().collect::<String>(), 16),
        ) {
            (Ok(r), Ok(g), Ok(b)) => (r, g, b),
            _ => return Err(err_msg),
        };

        Ok(Self { r, g, b })
    }

    pub fn get_hex(&self) -> String {
        format!("#{:02x?}{:02x?}{:02x?}", self.r, self.g, self.b)
    }

    pub fn to_true_color(&self) -> Color {
        Color::TrueColor {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }

    pub fn get_palette(color_a: Self, color_b: Self, n: usize) -> Vec<Self> {
        let mut palette: Vec<Self> = vec![];

        let jump_r = (color_b.r as i16 - color_a.r as i16) as f32 / (n as f32 - 1.0);
        let jump_g = (color_b.g as i16 - color_a.g as i16) as f32 / (n as f32 - 1.0);
        let jump_b = (color_b.b as i16 - color_a.b as i16) as f32 / (n as f32 - 1.0);

        let mut curr_r = color_a.r as f32;
        let mut curr_g = color_a.g as f32;
        let mut curr_b = color_a.b as f32;

        for _ in 0..n {
            let r = curr_r.round() as u8;
            let g = curr_g.round() as u8;
            let b = curr_b.round() as u8;

            palette.push(HexColor { r, g, b });

            curr_r += jump_r;
            curr_g += jump_g;
            curr_b += jump_b;
        }

        palette
    }
}
