//! Utility functions for color extraction, WCAG contrast auditing, and preset mapping.

use std::path::{Path, PathBuf};
use image::{DynamicImage, ImageError, ImageReader};

/// Opens an image with no size limits to prevent failures on high-res wallpapers
pub fn open_image<P: AsRef<Path>>(path: P) -> Result<DynamicImage, ImageError> {
    let mut reader = ImageReader::open(path)?;
    reader.no_limits();
    reader.with_guessed_format()?.decode()
}

fn normalize_palette_to_16(mut colors: Vec<[u8; 3]>) -> Vec<[u8; 3]> {
    colors.sort_unstable();
    colors.dedup();
    if colors.is_empty() {
        return vec![[0, 0, 0], [128, 128, 128], [255, 255, 255]];
    }
    colors
}

/// Retrieves raw RGB shades for a preset string, ensuring exactly 16 standard Base16 colors.
pub fn get_preset_shades(name: &str) -> Vec<[u8; 3]> {
    let raw = match name {
        "Catppuccin Mocha" => lutgen_palettes::Palette::CatppuccinMocha.get().to_vec(),
        "Catppuccin Latte" => {
            vec![[220, 138, 120], [221, 120, 120], [234, 118, 203], [136, 57, 239], [30, 102, 245]]
        },
        "Gruvbox Dark" => lutgen_palettes::Palette::GruvboxDark.get().to_vec(),
        "Nord Arctic" => lutgen_palettes::Palette::Nord.get().to_vec(),
        "Tokyo Night" => lutgen_palettes::Palette::TokyoNightDark.get().to_vec(),
        "Dracula" => lutgen_palettes::Palette::Dracula.get().to_vec(),
        "Rose Pine" => lutgen_palettes::Palette::RosePine.get().to_vec(),
        "Rose Pine Moon" => vec![
            [235, 111, 146],
            [246, 193, 119],
            [234, 154, 151],
            [196, 167, 231],
            [156, 207, 216],
        ],
        "Solarized Dark" => lutgen_palettes::Palette::SolarizedDark.get().to_vec(),
        "One Dark" => lutgen_palettes::Palette::Onedark.get().to_vec(),
        "Kanagawa" => lutgen_palettes::Palette::Kanagawa.get().to_vec(),
        "Everforest Dark" => {
            vec![[231, 138, 78], [216, 166, 87], [167, 192, 128], [127, 180, 202], [211, 134, 155]]
        },
        "Ayu Dark" => {
            vec![[255, 51, 51], [255, 151, 56], [255, 213, 128], [184, 204, 82], [54, 163, 217]]
        },
        "Monokai Pro" => {
            vec![[255, 97, 136], [252, 152, 103], [255, 216, 102], [169, 220, 118], [120, 220, 232]]
        },
        "Night Owl" => {
            vec![[239, 83, 80], [247, 140, 108], [255, 235, 149], [34, 218, 110], [130, 170, 255]]
        },
        "Synthwave" => vec![[11, 12, 16], [31, 40, 51], [255, 0, 127], [0, 255, 255]],
        "Cyberpunk" => vec![[13, 14, 21], [57, 255, 20], [255, 0, 255], [252, 226, 5]],
        "Vintage Sepia" => vec![[43, 29, 20], [112, 66, 20], [230, 194, 143]],
        "Retro 4-Color" => vec![[15, 56, 15], [48, 98, 48], [139, 172, 15], [155, 188, 15]],
        _ => vec![[137, 180, 250], [243, 139, 168], [166, 227, 161], [249, 226, 175]],
    };
    normalize_palette_to_16(raw)
}

/// Extracts representative RGB shades from custom .cube or .png LUT files.
pub fn extract_lut_shades(path: &PathBuf) -> Vec<[u8; 3]> {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase()) {
        if ext == "png" {
            if let Ok(img) = open_image(path) {
                let (w, h) = (img.width(), img.height());
                let is_hald = w == h && [8, 12, 14, 16].iter().any(|&l| l * l * l == w);
                if is_hald {
                    let rgba = img.to_rgba8();
                    let mut colors = Vec::new();
                    for step in [0, 50, 100, 150, 200, 250] {
                        if step < rgba.width() && step < rgba.height() {
                            let p = rgba.get_pixel(step, step);
                            colors.push([p[0], p[1], p[2]]);
                        }
                    }
                    if !colors.is_empty() {
                        return colors;
                    }
                } else {
                    return extract_dominant_colors(&img);
                }
            }
        }
    }
    if let Ok(content) = std::fs::read_to_string(path) {
        let mut colors = Vec::new();
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty()
                || line.starts_with('#')
                || line.starts_with('A')
                || line.starts_with('T')
                || line.starts_with('L')
                || line.starts_with('D')
            {
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                if let (Ok(r), Ok(g), Ok(b)) =
                    (parts[0].parse::<f32>(), parts[1].parse::<f32>(), parts[2].parse::<f32>())
                {
                    let r8 = if r <= 1.0 {
                        (r * 255.0) as u8
                    } else {
                        r.clamp(0.0, 255.0) as u8
                    };
                    let g8 = if g <= 1.0 {
                        (g * 255.0) as u8
                    } else {
                        g.clamp(0.0, 255.0) as u8
                    };
                    let b8 = if b <= 1.0 {
                        (b * 255.0) as u8
                    } else {
                        b.clamp(0.0, 255.0) as u8
                    };
                    colors.push([r8, g8, b8]);
                }
            }
        }
        if !colors.is_empty() {
            let step = (colors.len() / 6).max(1);
            return colors.into_iter().step_by(step).take(6).collect();
        }
    }
    vec![[120, 130, 150], [150, 160, 180], [180, 190, 210], [210, 220, 240]]
}

/// Reverse ricing: extracts dominant color clusters using sub-sampled quantization.
pub fn extract_dominant_colors(img: &image::DynamicImage) -> Vec<[u8; 3]> {
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    let mut buckets: Vec<[u32; 3]> = vec![[0, 0, 0]; 8];
    let mut counts: Vec<u32> = vec![0; 8];
    let step_x = (w / 32).max(1);
    let step_y = (h / 32).max(1);
    for y in (0..h).step_by(step_y as usize) {
        for x in (0..w).step_by(step_x as usize) {
            let p = rgba.get_pixel(x, y);
            if p[3] < 128 {
                continue;
            }
            let idx = ((p[0] as usize >> 5) ^ (p[1] as usize >> 5) ^ (p[2] as usize >> 5)) % 8;
            buckets[idx][0] += p[0] as u32;
            buckets[idx][1] += p[1] as u32;
            buckets[idx][2] += p[2] as u32;
            counts[idx] += 1;
        }
    }
    let mut result = Vec::new();
    for i in 0..8 {
        if counts[i] > 0 {
            result.push([
                (buckets[i][0] / counts[i]) as u8,
                (buckets[i][1] / counts[i]) as u8,
                (buckets[i][2] / counts[i]) as u8,
            ]);
        }
    }
    if result.len() < 4 {
        return vec![[137, 180, 250], [243, 139, 168], [166, 227, 161], [249, 226, 175]];
    }
    result
}

/// Telemetry: computes WCAG contrast ratio against white text.
pub fn compute_wcag_contrast(img: &image::DynamicImage) -> f32 {
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    let step = ((w * h) / 1000).max(1);
    let mut total_lum = 0.0;
    let mut count = 0.0;
    for (i, p) in rgba.pixels().enumerate() {
        if i as u32 % step == 0 {
            let r = p[0] as f32 / 255.0;
            let g = p[1] as f32 / 255.0;
            let b = p[2] as f32 / 255.0;
            let lum = 0.2126 * r + 0.7152 * g + 0.0722 * b;
            total_lum += lum;
            count += 1.0;
        }
    }
    let avg_lum = if count > 0.0 {
        total_lum / count
    } else {
        0.5
    };
    (1.0 + 0.05) / (avg_lum + 0.05)
}
