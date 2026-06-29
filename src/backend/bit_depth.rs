//! Bit-depth color quantization engine supporting 8-bit, 16-bit, and 32-bit styles.

use crate::app::state::BitDepthStyle;
use image::RgbaImage;
use rayon::prelude::*;

/// Applies bit-depth style quantization in parallel across image pixels.
pub fn apply_bit_depth(rgba: &mut RgbaImage, style: BitDepthStyle) {
    if style == BitDepthStyle::Bit32 {
        return;
    }

    rgba.par_pixels_mut().for_each(|px| {
        let r = px[0];
        let g = px[1];
        let b = px[2];

        match style {
            BitDepthStyle::Bit16 => {
                // RGB565 High Color quantization (5 bits Red, 6 bits Green, 5 bits Blue)
                px[0] = (r & 0xF8) | (r >> 5);
                px[1] = (g & 0xFC) | (g >> 6);
                px[2] = (b & 0xF8) | (b >> 5);
            },
            BitDepthStyle::Bit8 => {
                // 3-3-2 RGB Retro VGA posterized quantization (3 bits Red, 3 bits Green, 2 bits Blue)
                let rv = r & 0xE0;
                let gv = g & 0xE0;
                let bv = b & 0xC0;
                px[0] = rv | (rv >> 3) | (rv >> 6);
                px[1] = gv | (gv >> 3) | (gv >> 6);
                px[2] = bv | (bv >> 2) | (bv >> 4) | (bv >> 6);
            },
            BitDepthStyle::Bit32 => {},
        }
    });
}
