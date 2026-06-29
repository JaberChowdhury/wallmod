//! Core Business Model for wallmod. Fully decoupled from GUI framework.

pub mod helpers;
pub mod state;

pub use state::*;

use crate::modules::histogram::HistogramData;

use lutgen::identity::{correct_pixel, detect_level};
use lutgen::interpolation::{GaussianRemapper, NearestNeighborRemapper, ShepardRemapper};
use lutgen::GenerateLut;
use rayon::prelude::*;
use std::path::{Path, PathBuf};

/// Multi-threaded image correction using rayon
fn par_correct_image(image: &mut image::RgbaImage, hald_clut: &image::RgbImage) {
    let level = detect_level(hald_clut);
    // RgbaImage pixels have 4 bytes (R, G, B, A)
    image.par_chunks_mut(4).for_each(|pixel| {
        let [r, g, b] = correct_pixel(&[pixel[0], pixel[1], pixel[2]], hald_clut, level);
        pixel[0] = r;
        pixel[1] = g;
        pixel[2] = b;
    });
}

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
    pub is_zoomed: bool,
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
    pub pixel_sort_enabled: bool,
    pub oklab_mapping: bool,
    pub active_tab: crate::app::state::AppTab,
    pub extracted_colors: Option<Vec<(String, f32)>>,

    // Palette Editor State
    pub selected_color_idx: Option<usize>,
    pub histogram_data: Option<HistogramData>,
    pub daemon_enabled: bool,
    pub day_time_hour: u32,
    pub night_time_hour: u32,
    pub day_theme: String,
    pub night_theme: String,
    pub photoshop_params: crate::modules::photoshop::PhotoshopParams,
    pub option_group_tab: usize,
    pub split_diff_ratio: f32,
    pub export_dir: Option<PathBuf>,
    pub theme_chain: Vec<crate::app::state::ThemeChainNode>,
    pub chaining_mode: bool,
    pub global_bit_depth: crate::app::state::BitDepthStyle,
    pub sys_ram_percent: f32,
    pub sys_cpu_threads: Vec<f32>,
    pub sys_last_cpu_ticks: Vec<(u64, u64)>,
    pub sys_last_update: std::time::Instant,
    pub frame_timestamps: std::collections::VecDeque<std::time::Instant>,
    pub current_fps: f32,
    pub show_floating_stats: bool,
    pub float_show_fps: bool,
    pub float_show_ram: bool,
    pub float_show_cpu: bool,
    pub auto_apply_nodes: bool,
}

impl Default for WallmodApp {
    fn default() -> Self {
        Self::new()
    }
}

impl WallmodApp {
    pub fn new() -> Self {
        let initial_theme = ThemeSource::Preset("Default".to_string());
        Self {
            base_image_path: None,
            base_image_dyn: None,
            preview_path: None,
            processed_dyn: None,
            image_width: 0,
            image_height: 0,
            image_filename: String::new(),
            current_theme: initial_theme.clone(),
            state: AppState::Idle,
            custom_palette_input: String::new(),
            selected_preset: Some("Default".to_string()),
            algorithm: RemapAlgorithm::Gaussian,
            preserve_luma: false,
            hald_level: 8,
            is_zoomed: false,
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
            pixel_sort_enabled: false,
            oklab_mapping: true,
            active_tab: crate::app::state::AppTab::Themer,
            extracted_colors: None,
            selected_color_idx: None,
            histogram_data: None,
            daemon_enabled: false,
            day_time_hour: 8,
            night_time_hour: 20,
            day_theme: "Catppuccin Mocha".to_string(),
            night_theme: "Tokyo Night".to_string(),
            photoshop_params: crate::modules::photoshop::PhotoshopParams::default(),
            option_group_tab: 0,
            split_diff_ratio: 0.5,
            export_dir: None,
            theme_chain: vec![crate::app::state::ThemeChainNode {
                id: 1,
                op: crate::app::state::PipelineOp::Theme(initial_theme.clone()),
                theme: initial_theme,
                enabled: true,
                bit_depth: crate::app::state::BitDepthStyle::Bit32,
            }],
            chaining_mode: false,
            global_bit_depth: crate::app::state::BitDepthStyle::Bit32,
            sys_ram_percent: 0.0,
            sys_cpu_threads: Vec::new(),
            sys_last_cpu_ticks: Vec::new(),
            sys_last_update: std::time::Instant::now() - std::time::Duration::from_secs(10),
            frame_timestamps: std::collections::VecDeque::new(),
            current_fps: 60.0,
            show_floating_stats: false,
            float_show_fps: true,
            float_show_ram: true,
            float_show_cpu: true,
            auto_apply_nodes: false,
        }
    }

    pub fn apply_theme(&mut self, theme: ThemeSource) {
        self.current_theme = theme.clone();
        if self.chaining_mode {
            let next_id = self.theme_chain.iter().map(|n| n.id).max().unwrap_or(0) + 1;
            self.theme_chain.push(crate::app::state::ThemeChainNode {
                id: next_id,
                op: crate::app::state::PipelineOp::Theme(theme.clone()),
                theme,
                enabled: true,
                bit_depth: self.global_bit_depth,
            });
        } else {
            self.theme_chain = vec![crate::app::state::ThemeChainNode {
                id: 1,
                op: crate::app::state::PipelineOp::Theme(theme.clone()),
                theme,
                enabled: true,
                bit_depth: self.global_bit_depth,
            }];
        }
    }

    pub fn update_preview(
        &mut self,
        dyn_img: image::DynamicImage,
        temp_path: PathBuf,
        histogram: Option<crate::modules::histogram::HistogramData>,
        wcag_contrast: f32,
    ) {
        self.image_width = dyn_img.width();
        self.image_height = dyn_img.height();
        self.wcag_contrast = wcag_contrast;
        self.processed_dyn = Some(dyn_img);
        self.preview_path = Some(temp_path.clone());
        self.state = AppState::PreviewReady(temp_path);
        self.histogram_data = histogram;
    }

    pub fn on_image_selected(&mut self, path: PathBuf, dyn_img: image::DynamicImage) {
        self.image_filename = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        self.image_width = dyn_img.width();
        self.image_height = dyn_img.height();
        self.base_image_path = Some(path.clone());
        self.preview_path = Some(path.clone());
        self.base_image_dyn = Some(dyn_img);
        self.seam_carve_target = self.image_width;
        self.blur_sigma = 0.0;
        self.photoshop_params = crate::modules::photoshop::PhotoshopParams::default();
        self.dither_enabled = false;
        self.extracted_colors = None;
        self.selected_album = None;
        self.album_images.clear();
        // Do not synchronously update preview here; trigger_async_processing will handle it!
    }

    pub fn process_image_sync(
        base_image_dyn: Option<image::DynamicImage>,
        current_theme: ThemeSource,
        photoshop_params: crate::modules::photoshop::PhotoshopParams,
        blur_sigma: f32,
        dither_enabled: bool,
        seam_carve_target: u32,
        pixel_sort_enabled: bool,
        theme_chain: Vec<crate::app::state::ThemeChainNode>,
        chaining_mode: bool,
        global_bit_depth: crate::app::state::BitDepthStyle,
        algorithm: RemapAlgorithm,
        preserve_luma: bool,
        hald_level: u8,
    ) -> Result<
        Option<(
            image::DynamicImage,
            PathBuf,
            Option<crate::modules::histogram::HistogramData>,
            f32,
        )>,
        String,
    > {
        let Some(dyn_img) = base_image_dyn else {
            return Ok(None);
        };
        let mut rgba = dyn_img.to_rgba8();
        let shades = current_theme.get_shades();

        match &current_theme {
            ThemeSource::Custom(path) => {
                if let Some(ext) =
                    path.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase())
                {
                    if ext == "png" {
                        if let Ok(lut_img) = crate::app::helpers::open_image(path) {
                            let (lw, lh) = (lut_img.width(), lut_img.height());
                            if lw == lh && [8, 12, 14, 16].iter().any(|&l| l * l * l == lw) {
                                let rgb_lut = lut_img.to_rgb8();
                                par_correct_image(&mut rgba, &rgb_lut);
                                let mut processed_dyn = image::DynamicImage::ImageRgba8(rgba);
                                if !photoshop_params.is_neutral() {
                                    processed_dyn = crate::modules::photoshop::apply_photoshop_sync(
                                        processed_dyn,
                                        photoshop_params,
                                    );
                                }
                                if blur_sigma > 0.0 {
                                    processed_dyn = processed_dyn.blur(blur_sigma);
                                }
                                if dither_enabled {
                                    let palette_colors = current_theme.get_shades();
                                    if !palette_colors.is_empty() {
                                        processed_dyn =
                                            crate::backend::dither::apply_floyd_steinberg(
                                                &processed_dyn,
                                                &palette_colors,
                                            );
                                    }
                                }
                                let mut rgba_bit = processed_dyn.to_rgba8();
                                crate::backend::bit_depth::apply_bit_depth(
                                    &mut rgba_bit,
                                    global_bit_depth,
                                );
                                processed_dyn = image::DynamicImage::ImageRgba8(rgba_bit);
                                if seam_carve_target > 0
                                    && seam_carve_target < processed_dyn.width()
                                {
                                    processed_dyn = crate::backend::seam_carve::carve_width(
                                        &processed_dyn,
                                        seam_carve_target,
                                        |_, _| {},
                                    );
                                }
                                if pixel_sort_enabled {
                                    processed_dyn = crate::backend::pixel_sort::apply_pixel_sort(
                                        &processed_dyn,
                                    );
                                }
                                let histogram =
                                    crate::modules::histogram::compute_histogram(&processed_dyn)
                                        .ok();
                                let wcag_contrast =
                                    crate::app::helpers::compute_wcag_contrast(&processed_dyn);
                                static PREVIEW_COUNTER: std::sync::atomic::AtomicUsize =
                                    std::sync::atomic::AtomicUsize::new(1);
                                let count = PREVIEW_COUNTER
                                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                                let temp_path = std::env::temp_dir()
                                    .join(format!("wallmod_preview_{}.jpg", count));
                                let _ = processed_dyn.save(&temp_path);
                                return Ok(Some((
                                    processed_dyn,
                                    temp_path,
                                    histogram,
                                    wcag_contrast,
                                )));
                            }
                        }
                    }
                }
                if shades.is_empty() {
                    return Err(format!("Could not extract colors from LUT file {:?}", path));
                }
            },
            _ => {},
        }

        let processed_dyn = if chaining_mode && !theme_chain.is_empty() {
            let mut current_dyn = image::DynamicImage::ImageRgba8(rgba);
            for node in &theme_chain {
                if !node.enabled {
                    continue;
                }
                match &node.op {
                    crate::app::state::PipelineOp::Theme(theme) => {
                        let node_shades = theme.get_shades();
                        if !node_shades.is_empty() {
                            let mut buf = current_dyn.to_rgba8();
                            match algorithm {
                                RemapAlgorithm::Gaussian => {
                                    let remapper = GaussianRemapper::new(
                                        &node_shades,
                                        96.0,
                                        0,
                                        1.0,
                                        preserve_luma,
                                    );
                                    let hald_clut = remapper.par_generate_lut(hald_level);
                                    par_correct_image(&mut buf, &hald_clut);
                                },
                                RemapAlgorithm::Shepard => {
                                    let remapper = ShepardRemapper::new(
                                        &node_shades,
                                        16.0,
                                        0,
                                        1.0,
                                        preserve_luma,
                                    );
                                    let hald_clut = remapper.par_generate_lut(hald_level);
                                    par_correct_image(&mut buf, &hald_clut);
                                },
                                RemapAlgorithm::NearestNeighbor => {
                                    let remapper = NearestNeighborRemapper::new(
                                        &node_shades,
                                        1.0,
                                        preserve_luma,
                                    );
                                    let hald_clut = remapper.par_generate_lut(hald_level);
                                    par_correct_image(&mut buf, &hald_clut);
                                },
                            }
                            current_dyn = image::DynamicImage::ImageRgba8(buf);
                        }
                    },
                    crate::app::state::PipelineOp::Blur(sigma) => {
                        if *sigma > 0.0 {
                            let buf = current_dyn.to_rgba8();
                            let blurred = crate::modules::blur::parallel_blur(&buf, *sigma);
                            current_dyn = image::DynamicImage::ImageRgba8(blurred);
                        }
                    },
                    crate::app::state::PipelineOp::Photoshop(p) => {
                        if !p.is_neutral() {
                            current_dyn = crate::modules::photoshop::apply_photoshop_sync(
                                current_dyn,
                                p.clone(),
                            );
                        }
                    },
                    crate::app::state::PipelineOp::Dither => {
                        let shades = current_theme.get_shades();
                        if !shades.is_empty() {
                            current_dyn = crate::backend::dither::apply_floyd_steinberg(
                                &current_dyn,
                                &shades,
                            );
                        }
                    },
                    crate::app::state::PipelineOp::PixelSort => {
                        current_dyn = crate::backend::pixel_sort::apply_pixel_sort(&current_dyn);
                    },
                }
                if node.bit_depth != crate::app::state::BitDepthStyle::Bit32 {
                    let mut buf = current_dyn.to_rgba8();
                    crate::backend::bit_depth::apply_bit_depth(&mut buf, node.bit_depth);
                    current_dyn = image::DynamicImage::ImageRgba8(buf);
                }
            }
            if global_bit_depth != crate::app::state::BitDepthStyle::Bit32 {
                let mut buf = current_dyn.to_rgba8();
                crate::backend::bit_depth::apply_bit_depth(&mut buf, global_bit_depth);
                current_dyn = image::DynamicImage::ImageRgba8(buf);
            }
            if seam_carve_target > 0 && seam_carve_target < current_dyn.width() {
                current_dyn = crate::backend::seam_carve::carve_width(
                    &current_dyn,
                    seam_carve_target,
                    |_, _| {},
                );
            }
            current_dyn
        } else {
            if !shades.is_empty() {
                match algorithm {
                    RemapAlgorithm::Gaussian => {
                        let remapper = GaussianRemapper::new(&shades, 96.0, 0, 1.0, preserve_luma);
                        let hald_clut = remapper.par_generate_lut(hald_level);
                        par_correct_image(&mut rgba, &hald_clut);
                    },
                    RemapAlgorithm::Shepard => {
                        let remapper = ShepardRemapper::new(&shades, 16.0, 0, 1.0, preserve_luma);
                        let hald_clut = remapper.par_generate_lut(hald_level);
                        par_correct_image(&mut rgba, &hald_clut);
                    },
                    RemapAlgorithm::NearestNeighbor => {
                        let remapper = NearestNeighborRemapper::new(&shades, 1.0, preserve_luma);
                        let hald_clut = remapper.par_generate_lut(hald_level);
                        par_correct_image(&mut rgba, &hald_clut);
                    },
                }
            }
            crate::backend::bit_depth::apply_bit_depth(&mut rgba, global_bit_depth);

            let mut processed_dyn = image::DynamicImage::ImageRgba8(rgba);
            if !photoshop_params.is_neutral() {
                processed_dyn = crate::modules::photoshop::apply_photoshop_sync(
                    processed_dyn,
                    photoshop_params,
                );
            }
            if blur_sigma > 0.0 {
                let rgba = processed_dyn.to_rgba8();
                let blurred_rgba = crate::modules::blur::parallel_blur(&rgba, blur_sigma);
                processed_dyn = image::DynamicImage::ImageRgba8(blurred_rgba);
            }
            if dither_enabled {
                let palette_colors = current_theme.get_shades();
                if !palette_colors.is_empty() {
                    processed_dyn = crate::backend::dither::apply_floyd_steinberg(
                        &processed_dyn,
                        &palette_colors,
                    );
                }
            }
            if seam_carve_target > 0 && seam_carve_target < processed_dyn.width() {
                processed_dyn = crate::backend::seam_carve::carve_width(
                    &processed_dyn,
                    seam_carve_target,
                    |_, _| {},
                );
            }
            if pixel_sort_enabled {
                processed_dyn = crate::backend::pixel_sort::apply_pixel_sort(&processed_dyn);
            }
            processed_dyn
        };
        let histogram = crate::modules::histogram::compute_histogram(&processed_dyn).ok();
        let wcag_contrast = crate::app::helpers::compute_wcag_contrast(&processed_dyn);
        static PREVIEW_COUNTER: std::sync::atomic::AtomicUsize =
            std::sync::atomic::AtomicUsize::new(1);
        let count = PREVIEW_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let temp_path = std::env::temp_dir().join(format!("wallmod_preview_{}.jpg", count));
        let _ = processed_dyn.save(&temp_path);
        Ok(Some((processed_dyn, temp_path, histogram, wcag_contrast)))
    }

    pub fn run_processing(&mut self) -> Result<(), String> {
        let result = Self::process_image_sync(
            self.base_image_dyn.clone(),
            self.current_theme.clone(),
            self.photoshop_params,
            self.blur_sigma,
            self.dither_enabled,
            self.seam_carve_target,
            self.pixel_sort_enabled,
            self.theme_chain.clone(),
            self.chaining_mode,
            self.global_bit_depth,
            self.algorithm,
            self.preserve_luma,
            self.hald_level,
        )?;
        if let Some((processed_dyn, temp_path, histogram, wcag_contrast)) = result {
            self.update_preview(processed_dyn, temp_path, histogram, wcag_contrast);
        }
        Ok(())
    }

    pub fn apply_blur(&mut self) -> Result<(), String> {
        self.run_processing()
    }

    pub fn apply_seam_carving(&mut self, target_width: u32) -> Result<(), String> {
        let Some(dyn_img) = self.processed_dyn.as_ref().or(self.base_image_dyn.as_ref()) else {
            return Ok(());
        };
        let carved = crate::backend::seam_carve::carve_width(dyn_img, target_width, |_, _| {});
        let histogram = crate::modules::histogram::compute_histogram(&carved).ok();
        let wcag_contrast = crate::app::helpers::compute_wcag_contrast(&carved);
        static PREVIEW_COUNTER: std::sync::atomic::AtomicUsize =
            std::sync::atomic::AtomicUsize::new(1);
        let count = PREVIEW_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let temp_path = std::env::temp_dir().join(format!("wallmod_preview_{}.jpg", count));
        let _ = carved.save(&temp_path);
        self.update_preview(carved, temp_path, histogram, wcag_contrast);
        Ok(())
    }

    pub fn apply_dither(&mut self) -> Result<(), String> {
        self.run_processing()
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
        let colors: Vec<[u8; 3]> = self
            .custom_palette_input
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
        let new_theme = ThemeSource::CustomPalette("Custom Palette".to_string(), colors);
        self.apply_theme(new_theme);
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
            if depth > 3 {
                return;
            }
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
                let Ok(entries) = std::fs::read_dir(&dir) else {
                    return None;
                };
                let mut img_files = Vec::new();
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) =
                            path.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase())
                        {
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
                    let folder_name =
                        dir.file_name().unwrap_or_default().to_string_lossy().to_string();
                    Some(Album {
                        folder_name: if folder_name.is_empty() {
                            dir.to_string_lossy().to_string()
                        } else {
                            folder_name
                        },
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
                    if let Some(ext) =
                        p.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase())
                    {
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
            alacritty
                .push_str(&format!("{} = \"#{:02x}{:02x}{:02x}\"\n", name, rgb[0], rgb[1], rgb[2]));
        }
        std::fs::write(dir.join("alacritty_theme.toml"), &alacritty)
            .map_err(|e| format!("Write error: {}", e))?;
        if self.sync_alacritty {
            let alac_dir = dir.join(".config").join("alacritty");
            let _ = std::fs::create_dir_all(&alac_dir);
            let _ = std::fs::write(alac_dir.join("alacritty.toml"), &alacritty);
        }

        let mut kitty = String::from("background #09090b\nforeground #fafafa\n");
        for (i, _name) in names.iter().enumerate() {
            let rgb = shades.get(i % shades.len()).unwrap_or(&[128, 128, 128]);
            kitty.push_str(&format!("color{} #{:02x}{:02x}{:02x}\n", i, rgb[0], rgb[1], rgb[2]));
        }
        std::fs::write(dir.join("kitty_theme.conf"), &kitty)
            .map_err(|e| format!("Write error: {}", e))?;
        if self.sync_kitty {
            let kitty_dir = dir.join(".config").join("kitty");
            let _ = std::fs::create_dir_all(&kitty_dir);
            let _ = std::fs::write(kitty_dir.join("kitty.conf"), &kitty);
        }
        Ok(())
    }

    pub fn check_daemon_tick(&mut self) -> bool {
        use chrono::Timelike;
        let now = chrono::Local::now();
        let hour = now.time().hour();
        let is_day = hour >= self.day_time_hour && hour < self.night_time_hour;
        let expected_theme = if is_day {
            &self.day_theme
        } else {
            &self.night_theme
        };

        if self.current_theme.display_name() != *expected_theme && self.base_image_path.is_some() {
            let new_theme = ThemeSource::Preset(expected_theme.to_string());
            self.apply_theme(new_theme);
            let _ = self.run_processing();
            return true;
        }
        false
    }

    pub fn update_system_stats(&mut self) {
        let now = std::time::Instant::now();
        if now.duration_since(self.sys_last_update).as_millis() < 400 {
            return;
        }
        self.sys_last_update = now;

        // RAM Usage
        if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
            let mut total = 0.0;
            let mut avail = 0.0;
            for line in content.lines() {
                if line.starts_with("MemTotal:") {
                    total = line
                        .split_whitespace()
                        .nth(1)
                        .and_then(|v| v.parse::<f64>().ok())
                        .unwrap_or(0.0);
                } else if line.starts_with("MemAvailable:") {
                    avail = line
                        .split_whitespace()
                        .nth(1)
                        .and_then(|v| v.parse::<f64>().ok())
                        .unwrap_or(0.0);
                }
            }
            if total > 0.0 {
                self.sys_ram_percent = (((total - avail) / total) * 100.0) as f32;
            }
        }

        // CPU Thread Usage
        if let Ok(content) = std::fs::read_to_string("/proc/stat") {
            let mut new_ticks = Vec::new();
            let mut cpu_usages = Vec::new();
            for line in content.lines() {
                if line.starts_with("cpu") && !line.starts_with("cpu ") {
                    let parts: Vec<u64> = line
                        .split_whitespace()
                        .skip(1)
                        .filter_map(|p| p.parse::<u64>().ok())
                        .collect();
                    if parts.len() >= 4 {
                        let user = parts[0];
                        let nice = parts[1];
                        let system = parts[2];
                        let idle = parts[3];
                        let iowait = if parts.len() > 4 {
                            parts[4]
                        } else {
                            0
                        };
                        let irq = if parts.len() > 5 {
                            parts[5]
                        } else {
                            0
                        };
                        let softirq = if parts.len() > 6 {
                            parts[6]
                        } else {
                            0
                        };
                        let steal = if parts.len() > 7 {
                            parts[7]
                        } else {
                            0
                        };
                        let total = user + nice + system + idle + iowait + irq + softirq + steal;
                        let idle_total = idle + iowait;
                        let idx = new_ticks.len();
                        new_ticks.push((idle_total, total));

                        if let Some(&(last_idle, last_total)) = self.sys_last_cpu_ticks.get(idx) {
                            let diff_total = total.saturating_sub(last_total);
                            let diff_idle = idle_total.saturating_sub(last_idle);
                            if diff_total > 0 {
                                let usage = (1.0 - (diff_idle as f64 / diff_total as f64)) * 100.0;
                                cpu_usages.push(usage.clamp(0.0, 100.0) as f32);
                            } else {
                                cpu_usages.push(*self.sys_cpu_threads.get(idx).unwrap_or(&0.0));
                            }
                        } else {
                            cpu_usages.push(0.0);
                        }
                    }
                }
            }
            if !new_ticks.is_empty() {
                self.sys_last_cpu_ticks = new_ticks;
                self.sys_cpu_threads = cpu_usages;
            }
        }
    }

    pub fn record_frame(&mut self) {
        let now = std::time::Instant::now();
        self.frame_timestamps.push_back(now);
        while let Some(&front) = self.frame_timestamps.front() {
            if now.duration_since(front).as_secs_f32() > 1.0 {
                self.frame_timestamps.pop_front();
            } else {
                break;
            }
        }
        let len = self.frame_timestamps.len();
        if let Some(&first) = self.frame_timestamps.front() {
            let elapsed = now.duration_since(first).as_secs_f32();
            if elapsed > 0.1 && len > 1 {
                self.current_fps = ((len - 1) as f32 / elapsed).round();
            } else {
                self.current_fps = len as f32;
            }
        }
    }
}
