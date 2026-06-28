use image::Rgba;

/// Applies Floyd-Steinberg Dithering to an image using a limited color palette.
/// For each pixel, it finds the closest color in the provided palette, sets the pixel,
/// and diffuses the quantization error to neighboring pixels.
pub fn apply_floyd_steinberg(img: &image::DynamicImage, palette: &[[u8; 3]]) -> image::DynamicImage {
    let mut rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();

    // We need to operate with floating point or signed integers for error diffusion
    // To prevent clipping during diffusion, we copy the image to a buffer of f32
    let mut buffer: Vec<f32> = vec![0.0; (width * height * 3) as usize];
    
    for y in 0..height {
        for x in 0..width {
            let px = rgba.get_pixel(x, y);
            let idx = ((y * width + x) * 3) as usize;
            buffer[idx] = px[0] as f32;
            buffer[idx + 1] = px[1] as f32;
            buffer[idx + 2] = px[2] as f32;
        }
    }

    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 3) as usize;
            let old_r = buffer[idx].clamp(0.0, 255.0);
            let old_g = buffer[idx + 1].clamp(0.0, 255.0);
            let old_b = buffer[idx + 2].clamp(0.0, 255.0);

            // Find closest palette color
            let mut min_dist = f32::MAX;
            let mut new_r = 0.0;
            let mut new_g = 0.0;
            let mut new_b = 0.0;
            
            for color in palette {
                let dr = old_r - color[0] as f32;
                let dg = old_g - color[1] as f32;
                let db = old_b - color[2] as f32;
                let dist = dr * dr + dg * dg + db * db;
                if dist < min_dist {
                    min_dist = dist;
                    new_r = color[0] as f32;
                    new_g = color[1] as f32;
                    new_b = color[2] as f32;
                }
            }

            rgba.put_pixel(x, y, Rgba([new_r as u8, new_g as u8, new_b as u8, 255]));

            let err_r = old_r - new_r;
            let err_g = old_g - new_g;
            let err_b = old_b - new_b;

            // Distribute error
            // x + 1, y (7/16)
            if x + 1 < width {
                let n_idx = ((y * width + (x + 1)) * 3) as usize;
                buffer[n_idx] += err_r * 7.0 / 16.0;
                buffer[n_idx + 1] += err_g * 7.0 / 16.0;
                buffer[n_idx + 2] += err_b * 7.0 / 16.0;
            }
            // x - 1, y + 1 (3/16)
            if x > 0 && y + 1 < height {
                let n_idx = (((y + 1) * width + (x - 1)) * 3) as usize;
                buffer[n_idx] += err_r * 3.0 / 16.0;
                buffer[n_idx + 1] += err_g * 3.0 / 16.0;
                buffer[n_idx + 2] += err_b * 3.0 / 16.0;
            }
            // x, y + 1 (5/16)
            if y + 1 < height {
                let n_idx = (((y + 1) * width + x) * 3) as usize;
                buffer[n_idx] += err_r * 5.0 / 16.0;
                buffer[n_idx + 1] += err_g * 5.0 / 16.0;
                buffer[n_idx + 2] += err_b * 5.0 / 16.0;
            }
            // x + 1, y + 1 (1/16)
            if x + 1 < width && y + 1 < height {
                let n_idx = (((y + 1) * width + (x + 1)) * 3) as usize;
                buffer[n_idx] += err_r * 1.0 / 16.0;
                buffer[n_idx + 1] += err_g * 1.0 / 16.0;
                buffer[n_idx + 2] += err_b * 1.0 / 16.0;
            }
        }
    }

    image::DynamicImage::ImageRgba8(rgba)
}
