#[derive(Debug, Clone)]
pub struct GowallState {
    pub extracted_text: String,
    pub is_processing: bool,
    pub current_tool: GowallTool,
    pub selected_theme: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GowallTool {
    Recolor,
    Effects,
    Compress,
    Ocr,
    Upscale,
    PixelArt,
    ReplaceColor,
    Extract,
    Resize,
}

impl GowallState {
    pub fn new() -> Self {
        Self {
            extracted_text: String::new(),
            is_processing: false,
            current_tool: GowallTool::Recolor,
            selected_theme: "catppuccin-mocha".to_string(),
        }
    }
}
