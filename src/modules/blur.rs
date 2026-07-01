use image::{Rgba, RgbaImage};
use rayon::prelude::*;

pub fn parallel_blur(img: &RgbaImage, sigma: f32) -> RgbaImage {
    if sigma <= 0.0 {
        return img.clone();
    }

    let radius = (sigma * 3.0).ceil() as i32;
    let mut kernel = Vec::with_capacity((radius * 2 + 1) as usize);
    let mut sum = 0.0;
    for i in -radius..=radius {
        let weight = (-((i * i) as f32) / (2.0 * sigma * sigma)).exp();
        kernel.push(weight);
        sum += weight;
    }
    for w in &mut kernel {
        *w /= sum;
    }

    let (width, height) = img.dimensions();
    let width = width as usize;
    let height = height as usize;

    let in_buf = img.pixels().cloned().collect::<Vec<_>>();
    let mut out_x = vec![Rgba([0, 0, 0, 0]); width * height];

    // Horizontal pass
    out_x
        .par_chunks_mut(width)
        .enumerate()
        .for_each(|(y, row_out)| {
            for (x, out_px) in row_out.iter_mut().enumerate() {
                let mut r = 0.0;
                let mut g = 0.0;
                let mut b = 0.0;
                let mut a = 0.0;
                for i in -radius..=radius {
                    let px = (x as i32 + i).clamp(0, width as i32 - 1) as usize;
                    let pixel = in_buf[y * width + px];
                    let w = kernel[(i + radius) as usize];
                    r += pixel[0] as f32 * w;
                    g += pixel[1] as f32 * w;
                    b += pixel[2] as f32 * w;
                    a += pixel[3] as f32 * w;
                }
                *out_px = Rgba([r as u8, g as u8, b as u8, a as u8]);
            }
        });

    let mut final_buf = vec![Rgba([0, 0, 0, 0]); width * height];

    // Vertical pass
    final_buf
        .par_chunks_mut(width)
        .enumerate()
        .for_each(|(y, row_out)| {
            for (x, out_px) in row_out.iter_mut().enumerate() {
                let mut r = 0.0;
                let mut g = 0.0;
                let mut b = 0.0;
                let mut a = 0.0;
                for i in -radius..=radius {
                    let py = (y as i32 + i).clamp(0, height as i32 - 1) as usize;
                    let pixel = out_x[py * width + x];
                    let w = kernel[(i + radius) as usize];
                    r += pixel[0] as f32 * w;
                    g += pixel[1] as f32 * w;
                    b += pixel[2] as f32 * w;
                    a += pixel[3] as f32 * w;
                }
                *out_px = Rgba([r as u8, g as u8, b as u8, a as u8]);
            }
        });

    let mut out_y = RgbaImage::new(width as u32, height as u32);
    for (idx, p) in final_buf.into_iter().enumerate() {
        let x = (idx % width) as u32;
        let y = (idx / width) as u32;
        out_y.put_pixel(x, y, p);
    }
    out_y
}
