use image::RgbaImage;

/// Computes the energy of a pixel using a simple gradient method.
/// Uses the R, G, B channels to calculate differences.
fn energy(img: &RgbaImage, x: u32, y: u32, width: u32, height: u32) -> u32 {
    let left = if x == 0 {
        width - 1
    } else {
        x - 1
    };
    let right = if x + 1 == width {
        0
    } else {
        x + 1
    };
    let up = if y == 0 {
        height - 1
    } else {
        y - 1
    };
    let down = if y + 1 == height {
        0
    } else {
        y + 1
    };

    let pl = img.get_pixel(left, y);
    let pr = img.get_pixel(right, y);
    let pu = img.get_pixel(x, up);
    let pd = img.get_pixel(x, down);

    let dx_r = pl[0] as i32 - pr[0] as i32;
    let dx_g = pl[1] as i32 - pr[1] as i32;
    let dx_b = pl[2] as i32 - pr[2] as i32;
    let dx2 = dx_r * dx_r + dx_g * dx_g + dx_b * dx_b;

    let dy_r = pu[0] as i32 - pd[0] as i32;
    let dy_g = pu[1] as i32 - pd[1] as i32;
    let dy_b = pu[2] as i32 - pd[2] as i32;
    let dy2 = dy_r * dy_r + dy_g * dy_g + dy_b * dy_b;

    (dx2 + dy2) as u32
}

/// Finds the vertical seam with the minimum energy.
fn find_vertical_seam(img: &RgbaImage, width: u32, height: u32) -> Vec<u32> {
    let mut energies = vec![0u32; (width * height) as usize];
    let mut dp = vec![0u32; (width * height) as usize];
    let mut paths = vec![0i8; (width * height) as usize]; // -1 for left, 0 for straight, 1 for right

    // Calculate initial energies and DP for first row
    for y in 0..height {
        for x in 0..width {
            let e = energy(img, x, y, width, height);
            energies[(y * width + x) as usize] = e;
            if y == 0 {
                dp[x as usize] = e;
            }
        }
    }

    // DP down the image
    for y in 1..height {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            let up_idx = ((y - 1) * width + x) as usize;

            let mut min_val = dp[up_idx];
            let mut dir = 0i8;

            if x > 0 {
                let left_val = dp[up_idx - 1];
                if left_val < min_val {
                    min_val = left_val;
                    dir = -1;
                }
            }

            if x + 1 < width {
                let right_val = dp[up_idx + 1];
                if right_val < min_val {
                    min_val = right_val;
                    dir = 1;
                }
            }

            dp[idx] = energies[idx] + min_val;
            paths[idx] = dir;
        }
    }

    // Find min in bottom row
    let mut min_x = 0;
    let mut min_val = u32::MAX;
    let bottom_row = (height - 1) * width;
    for x in 0..width {
        let val = dp[(bottom_row + x) as usize];
        if val < min_val {
            min_val = val;
            min_x = x;
        }
    }

    // Traceback
    let mut seam = vec![0u32; height as usize];
    let mut current_x = min_x;
    for y in (0..height).rev() {
        seam[y as usize] = current_x;
        let dir = paths[(y * width + current_x) as usize];
        if dir == -1 {
            current_x -= 1;
        } else if dir == 1 {
            current_x += 1;
        }
    }

    seam
}

/// Removes a vertical seam from the image.
fn remove_vertical_seam(img: &mut RgbaImage, seam: &[u32], width: u32, height: u32) {
    for y in 0..height {
        let sx = seam[y as usize];
        for x in sx..(width - 1) {
            let px = img.get_pixel(x + 1, y).clone();
            img.put_pixel(x, y, px);
        }
    }
}

/// Perform seam carving to reduce width by `pixels_to_remove`.
/// Progress callback takes (current, total)
pub fn carve_width<F>(
    img: &image::DynamicImage,
    target_width: u32,
    mut progress_cb: F,
) -> image::DynamicImage
where
    F: FnMut(u32, u32),
{
    let mut rgba = img.to_rgba8();
    let mut current_width = rgba.width();
    let height = rgba.height();

    if target_width >= current_width {
        return img.clone();
    }

    let pixels_to_remove = current_width - target_width;

    for i in 0..pixels_to_remove {
        let seam = find_vertical_seam(&rgba, current_width, height);
        remove_vertical_seam(&mut rgba, &seam, current_width, height);
        current_width -= 1;

        if i % 10 == 0 || i == pixels_to_remove - 1 {
            progress_cb(i + 1, pixels_to_remove);
        }
    }

    // Crop the image to the new width
    let final_img = image::imageops::crop_imm(&rgba, 0, 0, current_width, height).to_image();
    image::DynamicImage::ImageRgba8(final_img)
}
