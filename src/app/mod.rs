//! Core Business Model for wallmod. Fully decoupled from GUI framework.

pub mod helpers;
pub mod state;

pub use state::*;

use crate::modules::histogram::HistogramData;

use lutgen::identity::correct_image;
use lutgen::interpolation::{GaussianRemapper, NearestNeighborRemapper, ShepardRemapper};
use lutgen::GenerateLut;
use rayon::prelude::*;
use std::path::{Path, PathBuf};

/// Core Application Struct. Contains zero GUI framework dependencies.
pub struct WallmodApp {
    pub base_image_path: Option<PathBuf>,
    pub base_image_dyn: Option<image::DynamicImage>,
    pub preview_path: Option<PathBuf>,
    pub processed_dyn: Option<image::DynamicImage>,
    pub image_width: u32,
    pub image_height: u32,
    pub image_filename: String,
    pub current_theme: ThemeSource,
    pub state: AppState,
    pub custom_palette_input: String,
    pub selected_preset: Option<String>,
    pub algorithm: RemapAlgorithm,
    pub preserve_luma: bool,
    pub hald_level: u8,
    pub workspace_view: WorkspaceView,
    pub wcag_contrast: f32,
    pub swww_transition: String,
    pub target_display: String,
    pub sync_alacritty: bool,
    pub sync_kitty: bool,
    pub albums: Vec<Album>,
    pub selected_album: Option<PathBuf>,
    pub album_images: Vec<PathBuf>,
    pub scanning_gallery: bool,
    pub sidebar_tab: SidebarTab,
    pub wallpaper_backend: WallpaperBackend,
    pub is_dark_mode: bool,
    pub blur_sigma: f32,
    pub seam_carve_target: u32,
    pub dither_enabled: bool,
    pub active_tab: crate::app::state::AppTab,
    pub extracted_colors: Option<Vec<String>>,
    pub histogram_data: Option<HistogramData>,
    pub daemon_enabled: bool,
    pub day_time_hour: u32,
    pub night_time_hour: u32,
    pub day_theme: String,
    pub night_theme: String,
    pub photoshop_params: crate::modules::photoshop::PhotoshopParams,
    pub option_group_tab: usize,
    pub split_diff_ratio: f32,
}

impl Default for WallmodApp {
    fn default() -> Self {
        Self::new()
    }
}

impl WallmodApp {
    pub fn new() -> Self {
        Self {
            base_image_path: None,
            base_image_dyn: None,
            preview_path: None,
            processed_dyn: None,
            image_width: 0,
            image_height: 0,
            image_filename: String::new(),
            current_theme: ThemeSource::Preset("Catppuccin Mocha".to_string()),
            state: AppState::Idle,
            custom_palette_input: String::new(),
            selected_preset: Some("Catppuccin Mocha".to_string()),
            algorithm: RemapAlgorithm::Gaussian,
            preserve_luma: false,
            hald_level: 8,
            workspace_view: WorkspaceView::Standard,
            wcag_contrast: 0.0,
            swww_transition: "wipe".to_string(),
            target_display: "All Displays".to_string(),
            sync_alacritty: true,
            sync_kitty: true,
            albums: Vec::new(),
            selected_album: None,
            album_images: Vec::new(),
            scanning_gallery: false,
            sidebar_tab: SidebarTab::ColorGrading,
            wallpaper_backend: WallpaperBackend::Auto,
            is_dark_mode: true,
            blur_sigma: 0.0,
            seam_carve_target: 0,
            dither_enabled: false,
            active_tab: crate::app::state::AppTab::Themer,
            extracted_colors: None,
            histogram_data: None,
            daemon_enabled: false,
            day_time_hour: 8,
            night_time_hour: 20,
            day_theme: "Catppuccin Mocha".to_string(),
            night_theme: "Tokyo Night".to_string(),
            photoshop_params: crate::modules::photoshop::PhotoshopParams::default(),
            option_group_tab: 0,
            split_diff_ratio: 0.5,
        }
    }

    pub fn update_preview(&mut self, dyn_img: image::DynamicImage) {
        let temp_path = std::env::temp_dir().join("wallmod_preview.png");
        let _ = dyn_img.save(&temp_path);
        self.image_width = dyn_img.width();
        self.image_height = dyn_img.height();
        self.wcag_contrast = helpers::compute_wcag_contrast(&dyn_img);
        self.processed_dyn = Some(dyn_img.clone());
        self.preview_path = Some(temp_path.clone());
        self.state = AppState::PreviewReady(temp_path);
        self.compute_histograms();
    }

    pub fn on_image_selected(&mut self, path: PathBuf, dyn_img: image::DynamicImage) {
        self.image_filename = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        self.image_width = dyn_img.width();
        self.image_height = dyn_img.height();
        self.base_image_path = Some(path.clone());
        self.base_image_dyn = Some(dyn_img.clone());
        self.seam_carve_target = self.image_width;
        self.blur_sigma = 0.0;
        self.update_preview(dyn_img);
        let _ = self.run_processing();
    }

    pub fn run_processing(&mut self) -> Result<(), String> {
        let Some(ref dyn_img) = self.base_image_dyn else {
            return Ok(());
        };
        let mut rgba = dyn_img.to_rgba8();
        let shades = self.current_theme.get_shades();

        match &self.current_theme {
            ThemeSource::Custom(path) => {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase()) {
                    if ext == "png" {
                        if let Ok(lut_img) = image::open(path) {
                            let (lw, lh) = (lut_img.width(), lut_img.height());
                            if lw == lh && [8, 12, 14, 16].iter().any(|&l| l * l * l == lw) {
                                let rgb_lut = lut_img.to_rgb8();
                                correct_image(&mut rgba, &rgb_lut);
                                let processed_dyn = image::DynamicImage::ImageRgba8(rgba);
                                self.update_preview(processed_dyn);
                                return Ok(());
                            }
                        }
                    }
                }
                if shades.is_empty() {
                    return Err(format!("Could not extract colors from LUT file {:?}", path));
                }
            }
            _ => {}
        }

        if !shades.is_empty() {
            match self.algorithm {
                RemapAlgorithm::Gaussian => {
                    let remapper = GaussianRemapper::new(&shades, 96.0, 0, 1.0, self.preserve_luma);
                    let hald_clut = remapper.par_generate_lut(self.hald_level);
                    correct_image(&mut rgba, &hald_clut);
                }
                RemapAlgorithm::Shepard => {
                    let remapper = ShepardRemapper::new(&shades, 16.0, 0, 1.0, self.preserve_luma);
                    let hald_clut = remapper.par_generate_lut(self.hald_level);
                    correct_image(&mut rgba, &hald_clut);
                }
                RemapAlgorithm::NearestNeighbor => {
                    let remapper = NearestNeighborRemapper::new(&shades, 1.0, self.preserve_luma);
                    let hald_clut = remapper.par_generate_lut(self.hald_level);
                    correct_image(&mut rgba, &hald_clut);
                }
            }
        }

        let mut processed_dyn = image::DynamicImage::ImageRgba8(rgba);
        if !self.photoshop_params.is_neutral() {
            processed_dyn = crate::modules::photoshop::apply_photoshop_sync(processed_dyn, self.photoshop_params);
        }
        self.update_preview(processed_dyn);
        Ok(())
    }

    pub fn apply_blur(&mut self) -> Result<(), String> {
        let Some(dyn_img) = self.processed_dyn.as_ref().or(self.base_image_dyn.as_ref()) else {
            return Ok(());
        };
        let blurred = dyn_img.blur(self.blur_sigma);
        self.update_preview(blurred);
        Ok(())
    }

    pub fn apply_seam_carving(&mut self, target_width: u32) -> Result<(), String> {
        let Some(dyn_img) = self.processed_dyn.as_ref().or(self.base_image_dyn.as_ref()) else {
            return Ok(());
        };
        let carved = crate::backend::seam_carve::carve_width(dyn_img, target_width, |_, _| {});
        self.update_preview(carved);
        Ok(())
    }

    pub fn apply_dither(&mut self) -> Result<(), String> {
        let Some(dyn_img) = self.processed_dyn.as_ref().or(self.base_image_dyn.as_ref()) else {
            return Ok(());
        };
        let palette_colors = self.current_theme.get_shades();
        let dithered = crate::backend::dither::apply_floyd_steinberg(dyn_img, &palette_colors);
        self.update_preview(dithered);
        Ok(())
    }

    pub fn extract_dominant_colors(&mut self) -> Result<(), String> {
        let Some(dyn_img) = &self.base_image_dyn else {
            return Ok(());
        };
        let colors = crate::modules::extractor::extract_dominant_colors(dyn_img, 8)?;
        self.extracted_colors = Some(colors);
        Ok(())
    }

    pub fn compute_histograms(&mut self) {
        if let Some(ref dyn_img) = self.processed_dyn {
            if let Ok(data) = crate::modules::histogram::compute_histogram(dyn_img) {
                self.histogram_data = Some(data);
            }
        }
    }

    pub fn apply_custom_palette(&mut self) -> Result<(), String> {
        let colors: Vec<[u8; 3]> = self.custom_palette_input
            .split(',')
            .filter_map(|s| {
                let s = s.trim().trim_start_matches('#');
                if s.len() == 6 {
                    let r = u8::from_str_radix(&s[0..2], 16).ok()?;
                    let g = u8::from_str_radix(&s[2..4], 16).ok()?;
                    let b = u8::from_str_radix(&s[4..6], 16).ok()?;
                    Some([r, g, b])
                } else {
                    None
                }
            })
            .collect();
        if colors.is_empty() {
            return Err("No valid hex colors provided.".to_string());
        }
        self.current_theme = ThemeSource::CustomPalette("Custom Palette".to_string(), colors);
        self.selected_preset = None;
        self.run_processing()
    }

    pub fn scan_system_gallery() -> Vec<Album> {
        let mut search_paths = Vec::new();
        if let Ok(home) = std::env::var("HOME") {
            let home_path = PathBuf::from(&home);
            for sub in ["Pictures", "Downloads", "Wallpapers", ".local/share/backgrounds"] {
                let p = home_path.join(sub);
                if p.exists() {
                    search_paths.push(p);
                }
            }
        }
        let usr_bg = PathBuf::from("/usr/share/backgrounds");
        if usr_bg.exists() {
            search_paths.push(usr_bg);
        }

        let exts = ["png", "jpg", "jpeg", "webp", "bmp", "tiff", "tga", "gif", "avif"];
        let mut dirs_to_scan = Vec::new();

        fn collect_dirs(dir: &Path, dirs: &mut Vec<PathBuf>, depth: u32) {
            if depth > 3 { return; }
            dirs.push(dir.to_path_buf());
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    if let Ok(meta) = entry.metadata() {
                        if meta.is_dir() {
                            let name = entry.file_name().to_string_lossy().to_string();
                            if !name.starts_with('.') {
                                collect_dirs(&entry.path(), dirs, depth + 1);
                            }
                        }
                    }
                }
            }
        }

        for p in &search_paths {
            collect_dirs(p, &mut dirs_to_scan, 0);
        }
        dirs_to_scan.sort();
        dirs_to_scan.dedup();

        dirs_to_scan
            .into_par_iter()
            .filter_map(|dir| {
                let Ok(entries) = std::fs::read_dir(&dir) else { return None; };
                let mut img_files = Vec::new();
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase()) {
                            if exts.contains(&ext.as_str()) {
                                img_files.push(path);
                            }
                        }
                    }
                }
                if img_files.is_empty() {
                    None
                } else {
                    img_files.sort();
                    let folder_name = dir.file_name().unwrap_or_default().to_string_lossy().to_string();
                    Some(Album {
                        folder_name: if folder_name.is_empty() { dir.to_string_lossy().to_string() } else { folder_name },
                        folder_path: dir,
                        cover_image: img_files.first().cloned(),
                        image_count: img_files.len(),
                    })
                }
            })
            .collect()
    }

    pub fn scan_album_images(album_path: &Path) -> Vec<PathBuf> {
        let exts = ["png", "jpg", "jpeg", "webp", "bmp", "tiff", "tga", "gif", "avif"];
        let mut imgs = Vec::new();
        if let Ok(entries) = std::fs::read_dir(album_path) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_file() {
                    if let Some(ext) = p.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase()) {
                        if exts.contains(&ext.as_str()) {
                            imgs.push(p);
                        }
                    }
                }
            }
        }
        imgs.sort();
        imgs.into_iter().take(48).collect()
    }

    pub fn export_terminal_scheme(&self, dir: &Path) -> Result<(), String> {
        let shades = self.current_theme.get_shades();
        let mut alacritty = String::from("[colors.primary]\nbackground = \"#09090b\"\nforeground = \"#fafafa\"\n\n[colors.normal]\n");
        let names = ["black", "red", "green", "yellow", "blue", "magenta", "cyan", "white"];
        for (i, name) in names.iter().enumerate() {
            let rgb = shades.get(i % shades.len()).unwrap_or(&[128, 128, 128]);
            alacritty.push_str(&format!("{} = \"#{:02x}{:02x}{:02x}\"\n", name, rgb[0], rgb[1], rgb[2]));
        }
        std::fs::write(dir.join("alacritty_theme.toml"), alacritty).map_err(|e| format!("Write error: {}", e))?;

        let mut kitty = String::from("background #09090b\nforeground #fafafa\n");
        for (i, _name) in names.iter().enumerate() {
            let rgb = shades.get(i % shades.len()).unwrap_or(&[128, 128, 128]);
            kitty.push_str(&format!("color{} #{:02x}{:02x}{:02x}\n", i, rgb[0], rgb[1], rgb[2]));
        }
        std::fs::write(dir.join("kitty_theme.conf"), kitty).map_err(|e| format!("Write error: {}", e))?;
        Ok(())
    }

    pub fn check_daemon_tick(&mut self) -> bool {
        use chrono::Timelike;
        let now = chrono::Local::now();
        let hour = now.time().hour();
        let is_day = hour >= self.day_time_hour && hour < self.night_time_hour;
        let expected_theme = if is_day { &self.day_theme } else { &self.night_theme };

        if self.current_theme.display_name() != *expected_theme && self.base_image_path.is_some() {
            self.current_theme = ThemeSource::Preset(expected_theme.to_string());
            let _ = self.run_processing();
            return true;
        }
        false
    }
}
