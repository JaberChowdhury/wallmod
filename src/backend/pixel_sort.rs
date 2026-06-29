use image::{DynamicImage, Rgba};
use rayon::prelude::*;

/// Applies edge-detection based pixel segment sorting for glitch art and cyberpunk wallpaper transformations.
/// For each row, pixels are grouped into segments bounded by luminance edges, and sorted by brightness within segments.
pub fn apply_pixel_sort(img: &DynamicImage) -> DynamicImage {
    let mut rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();

    // Collect rows into a buffer for Rayon parallel processing
    let mut rows: Vec<Vec<Rgba<u8>>> =
        (0..height).map(|y| (0..width).map(|x| *rgba.get_pixel(x, y)).collect()).collect();

    rows.par_iter_mut().for_each(|row| {
        let len = row.len();
        let mut start = 0;
        while start < len {
            // Find start of segment (e.g. luma > 60)
            while start < len && luma(&row[start]) <= 60 {
                start += 1;
            }
            if start >= len {
                break;
            }
            let mut end = start;
            // Find end of segment (e.g. luma > 60 and luma < 220)
            while end < len && luma(&row[end]) > 60 && luma(&row[end]) < 220 {
                end += 1;
            }
            // Sort segment by luminance
            row[start..end].sort_by_key(luma);
            start = end + 1;
        }
    });

    for (y, row) in rows.iter().enumerate() {
        for (x, px) in row.iter().enumerate() {
            rgba.put_pixel(x as u32, y as u32, *px);
        }
    }

    DynamicImage::ImageRgba8(rgba)
}

#[inline]
fn luma(px: &Rgba<u8>) -> u32 {
    ((px[0] as u32 * 2126) + (px[1] as u32 * 7152) + (px[2] as u32 * 722)) / 10000
}
