use image::DynamicImage;
use kmeans_colors::get_kmeans_hamerly;
use palette::{IntoColor, Lab, Oklab, Srgb};
use rayon::prelude::*;
use std::collections::HashMap;

/// Extracts the top `k` dominant colors from an image.
/// algo: 0 = KMeans (Oklab), 1 = KMeans (RGB), 2 = Histogram Peaks (RGB)
pub fn extract_dominant_colors(
    img: &DynamicImage,
    k: usize,
    algo: usize,
) -> Result<Vec<(String, f32)>, String> {
    let small_img = img.resize(256, 256, image::imageops::FilterType::Nearest);
    let rgba = small_img.to_rgba8();
    let total_pixels = (rgba.width() * rgba.height()) as f32;

    if algo == 2 {
        // Histogram Peak algorithm
        // Quantize colors to 5-bit per channel to reduce noise (32768 colors max)
        let pixels: Vec<u16> = rgba
            .as_raw()
            .par_chunks_exact(4)
            .map(|p| {
                let r = (p[0] >> 3) as u16;
                let g = (p[1] >> 3) as u16;
                let b = (p[2] >> 3) as u16;
                (r << 10) | (g << 5) | b
            })
            .collect();

        let mut counts = HashMap::new();
        for px in pixels {
            *counts.entry(px).or_insert(0) += 1;
        }

        let mut sorted: Vec<(u16, usize)> = counts.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));

        let colors = sorted
            .into_iter()
            .take(k)
            .map(|(px, count)| {
                let r = ((px >> 10) & 0x1F) as u8;
                let g = ((px >> 5) & 0x1F) as u8;
                let b = (px & 0x1F) as u8;
                // Expand 5-bit to 8-bit
                let r = (r << 3) | (r >> 2);
                let g = (g << 3) | (g >> 2);
                let b = (b << 3) | (b >> 2);
                let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
                let pct = count as f32 / total_pixels;
                (hex, pct)
            })
            .collect();
        return Ok(colors);
    }

    // K-Means approaches
    let pixels: Vec<Lab> = rgba
        .as_raw()
        .par_chunks_exact(4)
        .map(|p| {
            let srgb = Srgb::new(p[0] as f32 / 255.0, p[1] as f32 / 255.0, p[2] as f32 / 255.0);
            if algo == 1 {
                // KMeans RGB (mapped to Lab struct to satisfy traits)
                Lab::new(srgb.red * 100.0, srgb.green * 100.0, srgb.blue * 100.0)
            } else {
                // KMeans Oklab
                let oklab: Oklab = srgb.into_linear().into_color();
                Lab::new(oklab.l, oklab.a, oklab.b)
            }
        })
        .collect();

    let res = get_kmeans_hamerly(k, 20, 0.001, false, &pixels, 0);

    let mut counts = vec![0usize; k];
    for &idx in &res.indices {
        let i = idx as usize;
        if i < k {
            counts[i] += 1;
        }
    }

    let mut sorted_indices: Vec<usize> = (0..res.centroids.len()).collect();
    sorted_indices.sort_by(|a, b| counts[*b].cmp(&counts[*a]));

    let colors = sorted_indices
        .into_iter()
        .map(|idx| {
            let fake_lab = res.centroids[idx];
            let srgb = if algo == 1 {
                Srgb::new(fake_lab.l / 100.0, fake_lab.a / 100.0, fake_lab.b / 100.0)
            } else {
                let oklab = Oklab::new(fake_lab.l, fake_lab.a, fake_lab.b);
                oklab.into_color()
            };
            let r = (srgb.red * 255.0).clamp(0.0, 255.0) as u8;
            let g = (srgb.green * 255.0).clamp(0.0, 255.0) as u8;
            let b = (srgb.blue * 255.0).clamp(0.0, 255.0) as u8;
            let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
            let pct = counts[idx] as f32 / total_pixels;
            (hex, pct)
        })
        .collect();

    Ok(colors)
}
