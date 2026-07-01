use silicon::assets::HighlightingAssets;
use silicon::formatter::ImageFormatterBuilder;
use silicon::utils::ShadowAdder;
use syntect::easy::HighlightLines;
use syntect::util::LinesWithEndings;

pub fn render_code_to_image(
    code: &str,
    lang: &str,
    theme_name: &str,
) -> Result<std::path::PathBuf, String> {
    let ha = HighlightingAssets::new();
    let ps = &ha.syntax_set;
    let ts = &ha.theme_set;

    let syntax = ps
        .find_syntax_by_name(lang)
        .or_else(|| ps.find_syntax_by_extension(lang))
        .or_else(|| ps.find_syntax_by_token(lang))
        .or_else(|| ps.syntaxes().first())
        .ok_or_else(|| "No syntax available".to_string())?;

    let theme = ts
        .themes
        .get(theme_name)
        .or_else(|| ts.themes.get("Dracula"))
        .or_else(|| ts.themes.values().next())
        .ok_or_else(|| "No themes available".to_string())?;

    let mut h = HighlightLines::new(syntax, theme);
    let highlight = LinesWithEndings::from(code)
        .map(|line| h.highlight_line(line, ps))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Highlight error: {}", e))?;

    // KEY: no .font(...) call -> uses FontCollection::default() -> embedded Hack bytes
    // This bypasses fontconfig/SystemSource entirely
    let mut formatter = ImageFormatterBuilder::<String>::new()
        .shadow_adder(ShadowAdder::default())
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
