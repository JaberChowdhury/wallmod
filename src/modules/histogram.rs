use image::{DynamicImage, GenericImageView};

#[derive(Debug, Clone)]
pub struct HistogramData {
    pub r: [u32; 256],
    pub g: [u32; 256],
    pub b: [u32; 256],
    pub luma: [u32; 256],
    pub max_count: u32,
}

pub fn compute_histogram(img: &DynamicImage) -> Result<HistogramData, String> {
    let mut r_bins = [0u32; 256];
    let mut g_bins = [0u32; 256];
    let mut b_bins = [0u32; 256];
    let mut l_bins = [0u32; 256];
    let mut max_c = 0;

    let (width, height) = img.dimensions();
    if width == 0 || height == 0 {
        return Err("Empty image".to_string());
    }

    // Downscale if image is huge to avoid O(N) blocking for too long, but for accuracy we can just sample or process all
    // Since spawn_blocking runs in background, processing 4K (8M pixels) is very fast in Rust (few ms).
    let rgba = img.to_rgba8();
    for pixel in rgba.pixels() {
        let r = pixel[0] as usize;
        let g = pixel[1] as usize;
        let b = pixel[2] as usize;

        // standard luma formula (Rec. 709)
        let luma =
            (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32).clamp(0.0, 255.0) as usize;

        r_bins[r] += 1;
        g_bins[g] += 1;
        b_bins[b] += 1;
        l_bins[luma] += 1;

        max_c = max_c
            .max(r_bins[r])
            .max(g_bins[g])
            .max(b_bins[b])
            .max(l_bins[luma]);
    }

    Ok(HistogramData {
        r: r_bins,
        g: g_bins,
        b: b_bins,
        luma: l_bins,
        max_count: max_c,
    })
}
