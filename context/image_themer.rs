use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

pub struct ImageThemer {
    // Boilerplate provided by user
    pub original_image: Option<DynamicImage>,
    pub current_theme: String,
}

impl ImageThemer {
    pub fn apply_theme(img: &DynamicImage, theme_name: &str) -> DynamicImage {
        let (width, height) = img.dimensions();
        let mut output = ImageBuffer::new(width, height);

        for (x, y, pixel) in img.pixels() {
            let Rgba([r, g, b, a]) = pixel;

            // Extract standard luma for mapping
            let luma = (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32) as u8;

            let new_pixel = match theme_name {
                "Synthwave" => {
                    // Midnight Blue: #0B0C10 (11, 12, 16)
                    // Deep Purple: #1F2833 (31, 40, 51)
                    // Neon Magenta: #FF007F (255, 0, 127)
                    // Cyan: #00FFFF (0, 255, 255)
                    if luma < 64 {
                        Rgba([11, 12, 16, a])
                    } else if luma < 128 {
                        Rgba([31, 40, 51, a])
                    } else if luma < 192 {
                        Rgba([255, 0, 127, a])
                    } else {
                        Rgba([0, 255, 255, a])
                    }
                },
                "Cyberpunk" => {
                    // Dark Matrix: #0D0E15 (13, 14, 21)
                    // Acid Green: #39FF14 (57, 255, 20)
                    // Electric Pink: #FF00FF (255, 0, 255)
                    // Warning Yellow: #FCE205 (252, 226, 5)
                    if luma < 64 {
                        Rgba([13, 14, 21, a])
                    } else if luma < 128 {
                        Rgba([57, 255, 20, a])
                    } else if luma < 192 {
                        Rgba([255, 0, 255, a])
                    } else {
                        Rgba([252, 226, 5, a])
                    }
                },
                "Vintage Sepia" | "Sepia" => {
                    // Darkest Brown: #2B1D14 (43, 29, 20)
                    // Mid-tone Brown: #704214 (112, 66, 20)
                    // Paper: #E6C28F (230, 194, 143)

                    // Simple tinting based on luma interpolation
                    let f_luma = luma as f32 / 255.0;
                    if f_luma < 0.33 {
                        Rgba([43, 29, 20, a])
                    } else if f_luma < 0.66 {
                        Rgba([112, 66, 20, a])
                    } else {
                        Rgba([230, 194, 143, a])
                    }
                },
                "Nord" => {
                    // Polar Night: #2E3440 (46, 52, 64)
                    // Snow Storm: #D8DEE9 (216, 222, 233)
                    // Frost: #88C0D0 (136, 192, 208)
                    if luma < 85 {
                        Rgba([46, 52, 64, a])
                    } else if luma < 170 {
                        Rgba([136, 192, 208, a])
                    } else {
                        Rgba([216, 222, 233, a])
                    }
                },
                "Retro 4-Color" | "8-bit" => {
                    // Darkest: #0F380F (15, 56, 15)
                    // Dark: #306230 (48, 98, 48)
                    // Light: #8BAC0F (139, 172, 15)
                    // Lightest: #9BBC0F (155, 188, 15)
                    if luma < 64 {
                        Rgba([15, 56, 15, a])
                    } else if luma < 128 {
                        Rgba([48, 98, 48, a])
                    } else if luma < 192 {
                        Rgba([139, 172, 15, a])
                    } else {
                        Rgba([155, 188, 15, a])
                    }
                },
                _ => {
                    // Default Grayscale fallback
                    Rgba([luma, luma, luma, a])
                },
            };

            output.put_pixel(x, y, new_pixel);
        }

        DynamicImage::ImageRgba8(output)
    }
}
