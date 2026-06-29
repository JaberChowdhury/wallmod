//! Core state models and enumerations for wallmod.

use std::path::PathBuf;

/// Available preset palette names from lutgen-palettes.
pub const PRESET_NAMES: &[&str] = &[
    "Catppuccin Mocha",
    "Catppuccin Latte",
    "Gruvbox Dark",
    "Nord Arctic",
    "Tokyo Night",
    "Dracula",
    "Rose Pine",
    "Rose Pine Moon",
    "Solarized Dark",
    "One Dark",
    "Kanagawa",
    "Everforest Dark",
    "Ayu Dark",
    "Monokai Pro",
    "Night Owl",
    "Synthwave",
    "Cyberpunk",
    "Vintage Sepia",
    "Retro 4-Color",
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
    pub const ALL: &[RemapAlgorithm] =
        &[RemapAlgorithm::Gaussian, RemapAlgorithm::Shepard, RemapAlgorithm::NearestNeighbor];
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
    pub const ALL: &[AppTab] =
        &[AppTab::Themer, AppTab::Upscaler, AppTab::Ocr, AppTab::Compression];
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
    Telemetry,
    ExtractColor,
    PaletteEditor,
    Albums,
}

impl WorkspaceView {
    pub const ALL: &[WorkspaceView] = &[
        WorkspaceView::Standard,
        WorkspaceView::SplitDiff,
        WorkspaceView::Telemetry,
        WorkspaceView::ExtractColor,
        WorkspaceView::PaletteEditor,
        WorkspaceView::Albums,
    ];
}

impl std::fmt::Display for WorkspaceView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkspaceView::Standard => write!(f, "Output Visual"),
            WorkspaceView::SplitDiff => write!(f, "Split Diff"),
            WorkspaceView::Telemetry => write!(f, "Dashboard Info"),
            WorkspaceView::ExtractColor => write!(f, "Extract Color"),
            WorkspaceView::PaletteEditor => write!(f, "Edit Palette"),
            WorkspaceView::Albums => write!(f, "Album Gallery"),
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
            ThemeSource::Custom(path) => {
                path.file_name().unwrap_or_default().to_string_lossy().to_string()
            },
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
    PhotoshopEffects,
    DesktopEngine,
    ExportSync,
    ToolsExt,
    Settings,
}

impl SidebarTab {
    pub const ALL: &[SidebarTab] = &[
        SidebarTab::ColorGrading,
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
            SidebarTab::PhotoshopEffects => write!(f, "Adjust & Effects"),
            SidebarTab::DesktopEngine => write!(f, "Wallpaper Engine"),
            SidebarTab::ExportSync => write!(f, "Export & Sync"),
            SidebarTab::ToolsExt => write!(f, "AI & Tools"),
            SidebarTab::Settings => write!(f, "Settings"),
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
