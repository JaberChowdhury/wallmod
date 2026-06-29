//! Photoshop-style color adjustment module (Brightness, Contrast, Saturation, Hue).

use image::DynamicImage;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhotoshopParams {
    pub brightness: i32, // -100 to 100
    pub contrast: f32,   // -100.0 to 100.0
    pub saturation: f32, // -1.0 to 1.0
    pub hue: i32,        // 0 to 360
}

impl std::fmt::Display for PhotoshopParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "B:{}, C:{:.1}, S:{:.1}, H:{}",
            self.brightness, self.contrast, self.saturation, self.hue
        )
    }
}

impl Default for PhotoshopParams {
    fn default() -> Self {
        Self {
            brightness: 0,
            contrast: 0.0,
            saturation: 0.0,
            hue: 0,
        }
    }
}

impl PhotoshopParams {
    pub fn is_neutral(&self) -> bool {
        self.brightness == 0 && self.contrast == 0.0 && self.saturation == 0.0 && self.hue == 0
    }
}

/// Applies Photoshop adjustments synchronously.
pub fn apply_photoshop_sync(img: DynamicImage, params: PhotoshopParams) -> DynamicImage {
    let mut current = img;

    if params.brightness != 0 {
        current = current.brighten(params.brightness);
    }

    if params.contrast != 0.0 {
        current = current.adjust_contrast(params.contrast);
    }

    if params.hue != 0 && params.hue % 360 != 0 {
        current = current.huerotate(params.hue);
    }

    if params.saturation != 0.0 {
        let mut rgba = current.to_rgba8();
        let k = 1.0 + params.saturation;
        rgba.pixels_mut().par_bridge().for_each(|pixel| {
            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;
            let l = 0.299 * r + 0.587 * g + 0.114 * b;

            let nr = (l + (r - l) * k).clamp(0.0, 255.0) as u8;
            let ng = (l + (g - l) * k).clamp(0.0, 255.0) as u8;
            let nb = (l + (b - l) * k).clamp(0.0, 255.0) as u8;

            pixel[0] = nr;
            pixel[1] = ng;
            pixel[2] = nb;
        });
        current = DynamicImage::ImageRgba8(rgba);
    }

    current
}

/// Applies Photoshop adjustments asynchronously on a background thread.
pub async fn apply_photoshop_async(
    img: DynamicImage,
    params: PhotoshopParams,
) -> Result<DynamicImage, String> {
    crate::backend::runtime::spawn_blocking(move || apply_photoshop_sync(img, params))
        .await
        .map_err(|e| format!("Join error: {}", e))
}
