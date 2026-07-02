use silicon::assets::HighlightingAssets;
use silicon::formatter::ImageFormatterBuilder;
use silicon::utils::ShadowAdder;
use syntect::easy::HighlightLines;
use syntect::util::LinesWithEndings;

pub fn render_code_to_image(
    code: &str,
    lang: &str,
    theme_name: &str,
    bg_color: &str,
    pad_horiz: u32,
    pad_vert: u32,
    corner_radius: u32,
    font_name: &str,
    favorites: &Vec<String>,
) -> Result<std::path::PathBuf, String> {
    let ha = HighlightingAssets::new();
    let ps = &ha.syntax_set;

    let syntax = ps
        .find_syntax_by_name(lang)
        .or_else(|| ps.find_syntax_by_extension(lang))
        .or_else(|| ps.find_syntax_by_token(lang))
        .or_else(|| ps.syntaxes().first())
        .ok_or_else(|| "No syntax available".to_string())?;

    let custom_theme;
    let theme = if theme_name == "Custom Preset" {
        let bg = favorites
            .get(0)
            .cloned()
            .unwrap_or_else(|| "#1e1e1e".to_string());
        let fg = favorites
            .get(1)
            .cloned()
            .unwrap_or_else(|| "#ffffff".to_string());
        let kw = favorites
            .get(2)
            .cloned()
            .unwrap_or_else(|| "#ff79c6".to_string());
        let func = favorites
            .get(3)
            .cloned()
            .unwrap_or_else(|| "#50fa7b".to_string());
        let str_col = favorites
            .get(4)
            .cloned()
            .unwrap_or_else(|| "#f1fa8c".to_string());

        let xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple Computer//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>name</key><string>Custom Preset</string>
    <key>settings</key>
    <array>
        <dict>
            <key>settings</key>
            <dict>
                <key>background</key><string>{bg}</string>
                <key>foreground</key><string>{fg}</string>
            </dict>
        </dict>
        <dict>
            <key>scope</key><string>keyword, storage</string>
            <key>settings</key><dict><key>foreground</key><string>{kw}</string></dict>
        </dict>
        <dict>
            <key>scope</key><string>entity.name.function</string>
            <key>settings</key><dict><key>foreground</key><string>{func}</string></dict>
        </dict>
        <dict>
            <key>scope</key><string>string</string>
            <key>settings</key><dict><key>foreground</key><string>{str_col}</string></dict>
        </dict>
    </array>
</dict>
</plist>"#
        );
        custom_theme = syntect::highlighting::ThemeSet::load_from_reader(
            &mut std::io::Cursor::new(xml.as_bytes()),
        )
        .unwrap();
        &custom_theme
    } else {
        let ts = &ha.theme_set;
        ts.themes
            .get(theme_name)
            .or_else(|| ts.themes.get("Dracula"))
            .or_else(|| ts.themes.values().next())
            .ok_or_else(|| "No themes available".to_string())?
    };

    let mut h = HighlightLines::new(syntax, theme);
    let highlight = LinesWithEndings::from(code)
        .map(|line| h.highlight_line(line, ps))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Highlight error: {}", e))?;

    use silicon::utils::{Background, ToRgba};

    let background = if bg_color.starts_with("gradient:") {
        let parts: Vec<&str> = bg_color
            .trim_start_matches("gradient:")
            .split(',')
            .collect();
        let c1 = parts
            .get(0)
            .unwrap_or(&"#ff0000")
            .to_rgba()
            .unwrap_or_else(|_| "#abb8c3".to_rgba().unwrap());
        let c2 = parts
            .get(1)
            .unwrap_or(&"#0000ff")
            .to_rgba()
            .unwrap_or_else(|_| "#abb8c3".to_rgba().unwrap());

        let mut grad_img = silicon::image::RgbaImage::new(256, 1);
        for x in 0..256 {
            let t = x as f32 / 255.0;
            let r = (c1[0] as f32 * (1.0 - t) + c2[0] as f32 * t) as u8;
            let g = (c1[1] as f32 * (1.0 - t) + c2[1] as f32 * t) as u8;
            let b = (c1[2] as f32 * (1.0 - t) + c2[2] as f32 * t) as u8;
            let a = (c1[3] as f32 * (1.0 - t) + c2[3] as f32 * t) as u8;
            grad_img.put_pixel(x, 0, silicon::image::Rgba([r, g, b, a]));
        }
        Background::Image(grad_img)
    } else {
        Background::Solid(
            bg_color
                .to_rgba()
                .unwrap_or_else(|_| "#abb8c3".to_rgba().unwrap()),
        )
    };

    // NOTE: For fonts, Silicon's default builder handles embedded Hack if no font is passed.
    // Passing a custom font relies on FontConfig being available on the system.
    let mut font_vec = Vec::new();
    if !font_name.is_empty() && font_name.to_lowercase() != "hack" {
        font_vec.push((font_name.to_string(), 26.0));
    }

    let shadow = ShadowAdder::default()
        .background(background)
        .pad_horiz(pad_horiz)
        .pad_vert(pad_vert);

    let mut builder = ImageFormatterBuilder::<String>::new().shadow_adder(shadow);

    if corner_radius == 0 {
        builder = builder.round_corner(false);
    } else {
        builder = builder.round_corner(true);
        // Note: Silicon fork hardcodes corner radius to 15 unless we fork ImageFormatter further.
    }

    if !font_vec.is_empty() {
        builder = builder.font(font_vec);
    }

    let mut formatter = builder
        .build()
        .map_err(|e| format!("Formatter build error: {}", e))?;

    let image = formatter.format(&highlight, theme);

    let out_path = std::env::temp_dir().join(format!(
        "wallmod_render_{}.png",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    ));

    image
        .save(&out_path)
        .map_err(|e| format!("Save error: {}", e))?;
    Ok(out_path)
}
