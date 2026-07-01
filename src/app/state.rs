//! Core state models and enumerations for wallmod.

use std::path::PathBuf;

/// Available preset palette names from lutgen-palettes.
pub const PRESET_NAMES: &[&str] = &[
    "Arc Dark",
    "Atom Dark",
    "Ayu",
    "Ayu Dark",
    "Ayu Light",
    "Ayu Mirage",
    "Catppuccin",
    "Catppuccin Frappe",
    "Catppuccin Latte",
    "Cyberpunk",
    "Dracula",
    "Everforest",
    "GitHub Light",
    "Gruvbox",
    "Kanagawa",
    "Material",
    "Melange Dark",
    "Melange Light",
    "Monokai",
    "Night Owl",
    "Nord",
    "Oceanic Next",
    "Onedark",
    "PaleNight",
    "Rose Pine",
    "Shades of Purple",
    "Solarized",
    "Srcery",
    "Sunset Aurant",
    "Sunset Saffron",
    "Sunset Tangerine",
    "Synthwave 84",
    "Tokyo Dark",
    "Tokyo Moon",
    "Tokyo Storm",
];

/// Available Wayland transition animation types (`swww`).
pub const SWWW_TRANSITIONS: &[&str] = &["wipe", "wave", "grow", "outer", "random"];

/// Available display target options.
pub const TARGET_DISPLAYS: &[&str] = &["All Displays", "DP-1", "HDMI-A-1", "eDP-1"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemapAlgorithm {
    Gaussian,
    Shepard,
    NearestNeighbor,
}

impl RemapAlgorithm {
    pub const ALL: &[RemapAlgorithm] = &[
        RemapAlgorithm::Gaussian,
        RemapAlgorithm::Shepard,
        RemapAlgorithm::NearestNeighbor,
    ];
}

impl std::fmt::Display for RemapAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RemapAlgorithm::Gaussian => write!(f, "Gaussian (Smooth)"),
            RemapAlgorithm::Shepard => write!(f, "Shepard RBF (Sharp)"),
            RemapAlgorithm::NearestNeighbor => write!(f, "Nearest Neighbor (8-bit)"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppTab {
    Themer,
    Upscaler,
    Ocr,
    Compression,
}

impl AppTab {
    pub const ALL: &[AppTab] = &[
        AppTab::Themer,
        AppTab::Upscaler,
        AppTab::Ocr,
        AppTab::Compression,
    ];
}

impl std::fmt::Display for AppTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppTab::Themer => write!(f, "Themer"),
            AppTab::Upscaler => write!(f, "Upscaler"),
            AppTab::Ocr => write!(f, "OCR"),
            AppTab::Compression => write!(f, "Compression"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceView {
    Standard,
    SplitDiff,
    NodePipeline,
    Telemetry,
    ExtractColor,
    PaletteEditor,
    Albums,
    Gowall,
}

impl WorkspaceView {
    pub const ALL: &[WorkspaceView] = &[
        WorkspaceView::Standard,
        WorkspaceView::SplitDiff,
        WorkspaceView::NodePipeline,
        WorkspaceView::Telemetry,
        WorkspaceView::ExtractColor,
        WorkspaceView::PaletteEditor,
        WorkspaceView::Albums,
        WorkspaceView::Gowall,
    ];
}

impl std::fmt::Display for WorkspaceView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkspaceView::Standard => write!(f, "Output Visual"),
            WorkspaceView::SplitDiff => write!(f, "Split Diff"),
            WorkspaceView::NodePipeline => write!(f, "Node Pipeline"),
            WorkspaceView::Telemetry => write!(f, "Dashboard Info"),
            WorkspaceView::ExtractColor => write!(f, "Extract Color"),
            WorkspaceView::PaletteEditor => write!(f, "Edit Palette"),
            WorkspaceView::Albums => write!(f, "Album Gallery"),
            WorkspaceView::Gowall => write!(f, "Gowall GUI"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Album {
    pub folder_name: String,
    pub folder_path: PathBuf,
    pub cover_image: Option<PathBuf>,
    pub image_count: usize,
}

/// Theme Source Model.
#[derive(Debug, Clone, PartialEq)]
pub enum ThemeSource {
    Preset(String),
    Custom(PathBuf),
    CustomPalette(String, Vec<[u8; 3]>),
}

impl ThemeSource {
    pub fn display_name(&self) -> String {
        match self {
            ThemeSource::Preset(name) => name.clone(),
            ThemeSource::Custom(path) => path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            ThemeSource::CustomPalette(name, _) => format!("Custom: {}", name),
        }
    }

    pub fn as_custom_palette(&self) -> Option<(String, Vec<[u8; 3]>)> {
        match self {
            ThemeSource::CustomPalette(name, colors) => Some((name.clone(), colors.clone())),
            _ => None,
        }
    }

    pub fn get_shades(&self) -> Vec<[u8; 3]> {
        if let ThemeSource::Preset(name) = self {
            if name.eq_ignore_ascii_case("default") || name.eq_ignore_ascii_case("none") {
                return Vec::new();
            }
        }
        let mut shades = match self {
            ThemeSource::Preset(name) => crate::app::helpers::get_preset_shades(name),
            ThemeSource::Custom(path) => crate::app::helpers::extract_lut_shades(path),
            ThemeSource::CustomPalette(_, colors) => colors.clone(),
        };
        shades.sort_unstable();
        shades.dedup();
        if shades.is_empty() {
            shades = vec![[0, 0, 0], [128, 128, 128], [255, 255, 255]];
        } else if shades.len() == 1 {
            let c = shades[0];
            shades.push([
                c[0].saturating_add(64),
                c[1].saturating_add(64),
                c[2].saturating_add(64),
            ]);
        }
        shades
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitDepthStyle {
    Bit32,
    Bit16,
    Bit8,
}

impl BitDepthStyle {
    pub const ALL: &[BitDepthStyle] = &[
        BitDepthStyle::Bit32,
        BitDepthStyle::Bit16,
        BitDepthStyle::Bit8,
    ];
    pub fn display_name(&self) -> &'static str {
        match self {
            BitDepthStyle::Bit32 => "32-bit (True Color)",
            BitDepthStyle::Bit16 => "16-bit (High Color)",
            BitDepthStyle::Bit8 => "8-bit (VGA Posterized)",
        }
    }
    pub fn to_code(&self) -> &'static str {
        match self {
            Self::Bit32 => "Bit32",
            Self::Bit16 => "Bit16",
            Self::Bit8 => "Bit8",
        }
    }
    pub fn from_code(code: &str) -> Self {
        match code {
            "Bit16" => Self::Bit16,
            "Bit8" => Self::Bit8,
            _ => Self::Bit32,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PipelineOp {
    Theme(ThemeSource, f32),
    Blur(f32),
    Photoshop(crate::modules::photoshop::PhotoshopParams),
    Dither,
    PixelSort,
    Gowall(crate::app::gowall_state::GowallTool, String),
    Shader(String, [f32; 4]),
}

impl PipelineOp {
    pub fn display_name(&self) -> String {
        match self {
            Self::Theme(t, op) => {
                format!(
                    "Theme Grade: {} (Opacity: {:.0}%)",
                    t.display_name(),
                    op * 100.0
                )
            },
            Self::Blur(s) => format!("Blur Effect (σ={:.1})", s),
            Self::Photoshop(p) => format!(
                "Color Adjust (B:{}, C:{:.0}%, S:{:.1})",
                p.brightness, p.contrast, p.saturation
            ),
            Self::Dither => "Floyd-Steinberg Dither".to_string(),
            Self::PixelSort => "Luminance Pixel Sort".to_string(),
            Self::Gowall(tool, param) => format!("Gowall {:?} ({})", tool, param),
            Self::Shader(name, p) => format!(
                "WGSL Shader: {} (Params: {:.2}, {:.2}, {:.2}, {:.2})",
                name, p[0], p[1], p[2], p[3]
            ),
        }
    }

    pub fn to_code(&self) -> String {
        match self {
            Self::Theme(ts, op) => match ts {
                ThemeSource::Preset(name) => format!("theme:Preset:{}:{}", name, op),
                ThemeSource::Custom(path) => {
                    format!("theme:Custom:{}:{}", path.to_string_lossy(), op)
                },
                ThemeSource::CustomPalette(name, _) => format!("theme:Preset:{}:{}", name, op),
            },
            Self::Blur(sigma) => format!("blur:{}", sigma),
            Self::Photoshop(p) => {
                format!(
                    "photoshop:{}:{}:{}:{}",
                    p.brightness, p.contrast, p.saturation, p.hue
                )
            },
            Self::Dither => "dither".to_string(),
            Self::PixelSort => "pixelsort".to_string(),
            Self::Gowall(tool, param) => format!("gowall:{:?}:{}", tool, param),
            Self::Shader(name, p) => format!("shader:{}:{}:{}:{}:{}", name, p[0], p[1], p[2], p[3]),
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        let parts: Vec<&str> = code.splitn(3, ':').collect();
        if parts.is_empty() {
            return None;
        }
        match parts[0] {
            "theme" => {
                let p: Vec<&str> = code.splitn(4, ':').collect();
                if p.len() >= 3 {
                    let op = if p.len() == 4 {
                        p[3].parse().unwrap_or(1.0)
                    } else {
                        1.0
                    };
                    if p[1] == "Preset" {
                        return Some(Self::Theme(ThemeSource::Preset(p[2].to_string()), op));
                    } else if p[1] == "Custom" {
                        return Some(Self::Theme(ThemeSource::Custom(PathBuf::from(p[2])), op));
                    }
                }
                None
            },
            "blur" => {
                let sigma = code.split(':').nth(1)?.parse::<f32>().ok()?;
                Some(Self::Blur(sigma))
            },
            "photoshop" => {
                let p: Vec<&str> = code.split(':').collect();
                if p.len() >= 5 {
                    Some(Self::Photoshop(
                        crate::modules::photoshop::PhotoshopParams {
                            brightness: p[1].parse().unwrap_or(0),
                            contrast: p[2].parse().unwrap_or(0.0),
                            saturation: p[3].parse().unwrap_or(0.0),
                            hue: p[4].parse().unwrap_or(0),
                        },
                    ))
                } else {
                    None
                }
            },
            "dither" => Some(Self::Dither),
            "pixelsort" => Some(Self::PixelSort),
            "gowall" => {
                let p: Vec<&str> = code.splitn(3, ':').collect();
                if p.len() >= 3 {
                    // Quick parse of GowallTool from debug string
                    let tool = match p[1] {
                        "Recolor" => crate::app::gowall_state::GowallTool::Recolor,
                        "Effects" => crate::app::gowall_state::GowallTool::Effects,
                        "Compress" => crate::app::gowall_state::GowallTool::Compress,
                        "Ocr" => crate::app::gowall_state::GowallTool::Ocr,
                        "Upscale" => crate::app::gowall_state::GowallTool::Upscale,
                        "PixelArt" => crate::app::gowall_state::GowallTool::PixelArt,
                        "ReplaceColor" => crate::app::gowall_state::GowallTool::ReplaceColor,
                        "Extract" => crate::app::gowall_state::GowallTool::Extract,
                        "Resize" => crate::app::gowall_state::GowallTool::Resize,
                        _ => crate::app::gowall_state::GowallTool::Effects,
                    };
                    Some(Self::Gowall(tool, p[2].to_string()))
                } else {
                    None
                }
            },
            "shader" => {
                let p: Vec<&str> = code.split(':').collect();
                if p.len() >= 2 {
                    let mut params = [1.0, 1.0, 1.0, 1.0];
                    if p.len() >= 3 {
                        params[0] = p[2].parse().unwrap_or(1.0);
                    }
                    if p.len() >= 4 {
                        params[1] = p[3].parse().unwrap_or(1.0);
                    }
                    if p.len() >= 5 {
                        params[2] = p[4].parse().unwrap_or(1.0);
                    }
                    if p.len() >= 6 {
                        params[3] = p[5].parse().unwrap_or(1.0);
                    }
                    Some(Self::Shader(p[1].to_string(), params))
                } else {
                    None
                }
            },
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThemeChainNode {
    pub id: usize,
    pub op: PipelineOp,
    pub theme: ThemeSource,
    pub enabled: bool,
    pub bit_depth: BitDepthStyle,
}

pub fn export_pipeline_to_string(chain: &[ThemeChainNode]) -> String {
    let mut lines = Vec::new();
    lines.push("[\n".to_string());
    for (i, node) in chain.iter().enumerate() {
        let comma = if i + 1 < chain.len() { "," } else { "" };
        lines.push(format!("  {{\n    \"id\": {},\n    \"op\": \"{}\",\n    \"enabled\": {},\n    \"bit_depth\": \"{}\"\n  }}{}\n",
            node.id, node.op.to_code().replace('"', "\\\""), node.enabled, node.bit_depth.to_code(), comma));
    }
    lines.push("]\n".to_string());
    lines.join("")
}

pub fn import_pipeline_from_string(content: &str) -> Vec<ThemeChainNode> {
    let mut chain = Vec::new();
    let mut current_id = 1;
    let mut current_op = None;
    let mut current_enabled = true;
    let mut current_bd = BitDepthStyle::Bit32;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("\"id\":") {
            if let Some(val) = trimmed.split(':').nth(1) {
                current_id = val.trim().trim_matches(',').parse().unwrap_or(current_id);
            }
        } else if trimmed.starts_with("\"op\":") {
            if let Some(val) = trimmed.split(':').nth(1) {
                let code = val
                    .trim()
                    .trim_matches(',')
                    .trim_matches('"')
                    .replace("\\\"", "\"");
                current_op = PipelineOp::from_code(&code);
            }
        } else if trimmed.starts_with("\"enabled\":") {
            if let Some(val) = trimmed.split(':').nth(1) {
                current_enabled = val.trim().trim_matches(',') == "true";
            }
        } else if trimmed.starts_with("\"bit_depth\":") {
            if let Some(val) = trimmed.split(':').nth(1) {
                let code = val.trim().trim_matches(',').trim_matches('"');
                current_bd = BitDepthStyle::from_code(code);
            }
        } else if trimmed.starts_with('}') {
            if let Some(op) = current_op.take() {
                let theme = match &op {
                    PipelineOp::Theme(t, _) => t.clone(),
                    _ => ThemeSource::Preset("Default".to_string()),
                };
                chain.push(ThemeChainNode {
                    id: current_id,
                    op,
                    theme,
                    enabled: current_enabled,
                    bit_depth: current_bd,
                });
                current_id += 1;
            }
        }
    }
    chain
}

/// Application State Model.
#[derive(Debug, Clone)]
pub enum AppState {
    Idle,
    Loading(f32, String),
    PreviewReady(PathBuf),
    Notice(String),
    Error(String),
}

impl AppState {
    pub fn is_loading(&self) -> bool {
        matches!(self, AppState::Loading(_, _))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarTab {
    ColorGrading,
    FavoriteColors,
    PhotoshopEffects,
    DesktopEngine,
    ExportSync,
    ToolsExt,
    Settings,
    CodeRender,
}

impl SidebarTab {
    pub const ALL: &[SidebarTab] = &[
        SidebarTab::ColorGrading,
        SidebarTab::FavoriteColors,
        SidebarTab::PhotoshopEffects,
        SidebarTab::DesktopEngine,
        SidebarTab::ExportSync,
        SidebarTab::ToolsExt,
        SidebarTab::Settings,
    ];
}

impl std::fmt::Display for SidebarTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SidebarTab::ColorGrading => write!(f, "Color Grading"),
            SidebarTab::FavoriteColors => write!(f, "Favorite Colors"),
            SidebarTab::PhotoshopEffects => write!(f, "Adjust & Effects"),
            SidebarTab::DesktopEngine => write!(f, "Wallpaper Engine"),
            SidebarTab::ExportSync => write!(f, "Export & Sync"),
            SidebarTab::ToolsExt => write!(f, "AI & Tools"),
            SidebarTab::Settings => write!(f, "Settings"),
            SidebarTab::CodeRender => write!(f, "Code Render"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallpaperBackend {
    Auto,
    Swww,
    Swaybg,
    Feh,
    Gsettings,
}

impl WallpaperBackend {
    pub const ALL: &[WallpaperBackend] = &[
        WallpaperBackend::Auto,
        WallpaperBackend::Swww,
        WallpaperBackend::Swaybg,
        WallpaperBackend::Feh,
        WallpaperBackend::Gsettings,
    ];

    pub fn description(&self) -> &'static str {
        match self {
            WallpaperBackend::Auto => "Auto-detect environment ($XDG_CURRENT_DESKTOP) and execute optimal fallback setter.",
            WallpaperBackend::Swww => "Hardware-accelerated Wayland wallpaper daemon supporting 60 FPS GPU transition animations.",
            WallpaperBackend::Swaybg => "Lightweight Wayland static background setter for Sway and Hyprland with zero RAM overhead.",
            WallpaperBackend::Feh => "Universal X11 background setter compatible with i3, bspwm, awesome, and standard Xorg WMs.",
            WallpaperBackend::Gsettings => "Native dconf/gsettings integration for GNOME, Ubuntu, Zorin, and Cinnamon desktops.",
        }
    }

    pub fn code(&self) -> String {
        match self {
            WallpaperBackend::Auto => "auto".to_string(),
            WallpaperBackend::Swww => "swww".to_string(),
            WallpaperBackend::Swaybg => "swaybg".to_string(),
            WallpaperBackend::Feh => "feh".to_string(),
            WallpaperBackend::Gsettings => "gsettings".to_string(),
        }
    }
}

impl std::fmt::Display for WallpaperBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WallpaperBackend::Auto => write!(f, "Auto Fallback (Universal)"),
            WallpaperBackend::Swww => write!(f, "swww (Wayland Animated)"),
            WallpaperBackend::Swaybg => write!(f, "swaybg (Wayland Lightweight)"),
            WallpaperBackend::Feh => write!(f, "feh (X11 Universal)"),
            WallpaperBackend::Gsettings => write!(f, "gsettings (GNOME/Cinnamon)"),
        }
    }
}
