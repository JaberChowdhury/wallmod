use image::DynamicImage;
use kmeans_colors::get_kmeans_hamerly;
use palette::{IntoColor, Lab, Oklab, Srgb};

/// Extracts the top `k` dominant colors from an image using K-Means clustering in the Oklab color space.
pub fn extract_dominant_colors(img: &DynamicImage, k: usize) -> Result<Vec<String>, String> {
    let rgba = img.to_rgba8();
    // Convert to Oklab mathematically, then cast to Lab to satisfy kmeans_colors trait bounds
    let pixels: Vec<Lab> = rgba
        .pixels()
        .map(|p| {
            let srgb = Srgb::new(
                p[0] as f32 / 255.0,
                p[1] as f32 / 255.0,
                p[2] as f32 / 255.0,
            );
            let oklab: Oklab = srgb.into_linear().into_color();
            Lab::new(oklab.l, oklab.a, oklab.b)
        })
        .collect();

    // Max iterations: 20, converge threshold: 0.001
    let res = get_kmeans_hamerly(k, 20, 0.001, false, &pixels, 0);
    
    // Sort by frequency (descending) manually by tracking index occurrences
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
            let oklab = Oklab::new(fake_lab.l, fake_lab.a, fake_lab.b);
            let srgb: Srgb = oklab.into_color();
            let r = (srgb.red * 255.0).clamp(0.0, 255.0) as u8;
            let g = (srgb.green * 255.0).clamp(0.0, 255.0) as u8;
            let b = (srgb.blue * 255.0).clamp(0.0, 255.0) as u8;
            format!("#{:02X}{:02X}{:02X}", r, g, b)
        })
        .collect();

    Ok(colors)
}
