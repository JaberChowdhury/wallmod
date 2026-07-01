use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct GowallState {
    pub loaded_image_path: Option<PathBuf>,
    pub extracted_text: String,
    pub is_processing: bool,
    pub current_tool: GowallTool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GowallTool {
    Recolor,
    Compress,
    Ocr,
    Upscale,
    PixelArt,
    ReplaceColor,
}

impl GowallState {
    pub fn new() -> Self {
        Self {
            loaded_image_path: None,
            extracted_text: String::new(),
            is_processing: false,
            current_tool: GowallTool::Recolor,
        }
    }
}
