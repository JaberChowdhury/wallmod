//! Main Application module assembling refactored state, helpers, and backend controller.

pub mod helpers;
pub mod state;

pub use state::*;

use crate::ui;
use iced::widget::image as iced_image;
use iced::{Task, Theme};
use lutgen::identity::correct_image;
use lutgen::interpolation::{GaussianRemapper, NearestNeighborRemapper, ShepardRemapper};
use lutgen::GenerateLut;
use std::path::PathBuf;

/// Required Workflows & Messages.
#[derive(Debug, Clone)]
pub enum Message {
    SelectImage,
    ImageSelected(Result<(PathBuf, image::DynamicImage, iced_image::Handle), String>),
    SelectCustomTheme,
    CustomThemeSelected(Result<PathBuf, String>),
    ThemePresetSelected(String),
    ApplyTheme,
    StartProcessingWorker,
    ImageProcessed(Result<(Vec<u8>, u32, u32, image::DynamicImage, f32), String>),
    SetWallpaper,
    WallpaperSet(Result<PathBuf, String>),
    CustomPaletteInputChanged(String),
    ApplyCustomPalette,
    SelectBatchFolder,
    BatchFolderSelected(Result<PathBuf, String>),
    BatchProcessed(Result<usize, String>),
    AlgorithmChanged(RemapAlgorithm),
    TogglePreserveLuma(bool),
    HaldLevelChanged(u8),
    ExtractPaletteFromImage,
    WorkspaceViewChanged(WorkspaceView),
    SplitOffsetChanged(f32),
    SwwwTransitionChanged(String),
    TargetDisplayChanged(String),
    SaveImageToFolder,
    ImageSaved(Result<PathBuf, String>),
    ExportTerminalScheme,
    TerminalSchemeExported(Result<PathBuf, String>),
    ToggleSyncAlacritty(bool),
    ToggleSyncKitty(bool),
    PipelineFinished(Result<(PathBuf, bool, Vec<String>), String>),
    DismissNotice,
    ScanSystemGallery,
    GalleryScanned(Result<Vec<Album>, String>),
    SelectAlbum(Option<PathBuf>),
    AlbumImagesScanned(Result<(PathBuf, Vec<(PathBuf, iced::widget::image::Handle)>), String>),
    SelectGalleryImage(PathBuf),
    SidebarTabChanged(SidebarTab),
    WallpaperBackendChanged(WallpaperBackend),
    ToggleAppTheme,
    BlurSigmaChanged(f32),
    ApplyBlur,
    BlurCompleted(Result<(Vec<u8>, u32, u32, image::DynamicImage), String>),
    WindowClose,
    WindowMinimize,
    WindowMaximize,
    SeamCarveTargetChanged(u32),
    ApplySeamCarving,
    SeamCarvingProgress(u32, u32),
    SeamCarvingCompleted(Result<image::DynamicImage, String>),
    ToggleDither,
    ApplyDither,
    DitherCompleted(Result<image::DynamicImage, String>),
    AppTabChanged(crate::app::state::AppTab),
    ExtractDominantColors,
    DominantColorsExtracted(Result<Vec<String>, String>),
    ComputeHistograms,
    HistogramsComputed(Result<crate::modules::histogram::HistogramData, String>),
    ToggleDaemon(bool),
    SetDayHour(u32),
    SetNightHour(u32),
    SetDayTheme(String),
    SetNightTheme(String),
    DaemonTick,
}

/// Core Elm Architecture Application Struct.
pub struct WallmodApp {
    base_image_path: Option<PathBuf>,
    base_image_dyn: Option<image::DynamicImage>,
    base_image_handle: Option<iced_image::Handle>,
    processed_dyn: Option<image::DynamicImage>,
    image_width: u32,
    image_height: u32,
    image_filename: String,
    current_theme: ThemeSource,
    state: AppState,
    custom_palette_input: String,
    selected_preset: Option<String>,
    algorithm: RemapAlgorithm,
    preserve_luma: bool,
    hald_level: u8,
    workspace_view: WorkspaceView,
    wcag_contrast: f32,
    swww_transition: String,
    target_display: String,
    sync_alacritty: bool,
    sync_kitty: bool,
    albums: Vec<Album>,
    selected_album: Option<PathBuf>,
    album_images: Vec<(PathBuf, iced::widget::image::Handle)>,
    scanning_gallery: bool,
    sidebar_tab: SidebarTab,
    wallpaper_backend: WallpaperBackend,
    is_dark_mode: bool,
    preview_handle: Option<iced_image::Handle>,
    blur_sigma: f32,
    seam_carve_target: u32,
    dither_enabled: bool,
    active_tab: crate::app::state::AppTab,
    extracted_colors: Option<Vec<String>>,
    histogram_data: Option<crate::modules::histogram::HistogramData>,
    daemon_enabled: bool,
    day_time_hour: u32,
    night_time_hour: u32,
    day_theme: String,
    night_theme: String,
}

impl WallmodApp {
    pub fn boot() -> (Self, Task<Message>) {
        let initial_preset = "Catppuccin Mocha".to_string();
        (
            Self {
                base_image_path: None,
                base_image_dyn: None,
                base_image_handle: None,
                processed_dyn: None,
                image_width: 0,
                image_height: 0,
                image_filename: String::new(),
                current_theme: ThemeSource::Preset(initial_preset.clone()),
                state: AppState::Idle,
                custom_palette_input: "#89b4fa, #f38ba8, #a6e3a1, #f9e2af".to_string(),
                selected_preset: Some(initial_preset),
                algorithm: RemapAlgorithm::Gaussian,
                preserve_luma: false,
                hald_level: 8,
                workspace_view: WorkspaceView::Standard,
                wcag_contrast: 0.0,
                swww_transition: "grow".to_string(),
                target_display: "All Displays".to_string(),
                sync_alacritty: true,
                sync_kitty: true,
                albums: Vec::new(),
                selected_album: None,
                album_images: Vec::new(),
                scanning_gallery: false,
                sidebar_tab: SidebarTab::ThemeLut,
                wallpaper_backend: WallpaperBackend::Auto,
                is_dark_mode: true,
                preview_handle: None,
                blur_sigma: 0.0,
                seam_carve_target: 0,
                dither_enabled: false,
                active_tab: crate::app::state::AppTab::Themer,
                extracted_colors: None,
                histogram_data: None,
                daemon_enabled: false,
                day_time_hour: 6,
                night_time_hour: 18,
                day_theme: "Catppuccin Mocha".to_string(),
                night_theme: "Tokyo Night".to_string(),
            },
            Task::none(),
        )
    }

    pub fn blur_sigma(&self) -> f32 { self.blur_sigma }
    pub fn seam_carve_target(&self) -> u32 { self.seam_carve_target }
    pub fn dither_enabled(&self) -> bool { self.dither_enabled }
    pub fn active_tab(&self) -> crate::app::state::AppTab { self.active_tab }
    pub fn extracted_colors(&self) -> Option<&Vec<String>> { self.extracted_colors.as_ref() }
    pub fn histogram_data(&self) -> Option<&crate::modules::histogram::HistogramData> { self.histogram_data.as_ref() }
    pub fn daemon_enabled(&self) -> bool { self.daemon_enabled }
    pub fn day_time_hour(&self) -> u32 { self.day_time_hour }
    pub fn night_time_hour(&self) -> u32 { self.night_time_hour }
    pub fn day_theme(&self) -> &str { &self.day_theme }
    pub fn night_theme(&self) -> &str { &self.night_theme }

    pub fn theme(&self) -> Theme {
        if self.is_dark_mode {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    pub fn is_dark_mode(&self) -> bool {
        self.is_dark_mode
    }

    pub fn preview_handle(&self) -> Option<&iced_image::Handle> {
        self.preview_handle.as_ref()
    }

    pub fn has_image(&self) -> bool {
        self.base_image_dyn.is_some()
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub fn selected_preset(&self) -> Option<&str> {
        self.selected_preset.as_deref()
    }

    pub fn current_theme(&self) -> &ThemeSource {
        &self.current_theme
    }

    pub fn custom_palette_input(&self) -> &str {
        &self.custom_palette_input
    }

    pub fn image_width(&self) -> u32 {
        self.image_width
    }

    pub fn image_height(&self) -> u32 {
        self.image_height
    }

    pub fn image_filename(&self) -> &str {
        &self.image_filename
    }

    pub fn algorithm(&self) -> RemapAlgorithm {
        self.algorithm
    }

    pub fn preserve_luma(&self) -> bool {
        self.preserve_luma
    }

    pub fn hald_level(&self) -> u8 {
        self.hald_level
    }

    pub fn workspace_view(&self) -> WorkspaceView {
        self.workspace_view
    }

    pub fn sidebar_tab(&self) -> SidebarTab {
        self.sidebar_tab
    }

    pub fn wallpaper_backend(&self) -> WallpaperBackend {
        self.wallpaper_backend
    }

    pub fn wcag_contrast(&self) -> f32 {
        self.wcag_contrast
    }

    pub fn swww_transition(&self) -> &str {
        &self.swww_transition
    }

    pub fn target_display(&self) -> &str {
        &self.target_display
    }

    pub fn base_image_handle(&self) -> Option<&iced_image::Handle> {
        self.base_image_handle.as_ref()
    }

    pub fn sync_alacritty(&self) -> bool {
        self.sync_alacritty
    }

    pub fn sync_kitty(&self) -> bool {
        self.sync_kitty
    }

    pub fn albums(&self) -> &[Album] {
        &self.albums
    }

    pub fn selected_album(&self) -> Option<&PathBuf> {
        self.selected_album.as_ref()
    }

    pub fn album_images(&self) -> &[(PathBuf, iced_image::Handle)] {
        &self.album_images
    }

    pub fn scanning_gallery(&self) -> bool {
        self.scanning_gallery
    }

    fn trigger_processing(&mut self) -> Task<Message> {
        if self.base_image_dyn.is_none() {
            return Task::none();
        }
        let theme_name = self.current_theme.display_name();
        self.state = AppState::Loading(0.25, format!("[ # ] Extracting palette for {}...", theme_name));

        Task::perform(
            async {
                tokio::time::sleep(std::time::Duration::from_millis(40)).await;
            },
            |_| Message::StartProcessingWorker,
        )
    }

    fn run_processing_worker(&mut self) -> Task<Message> {
        let Some(ref dyn_img) = self.base_image_dyn else {
            return Task::none();
        };
        let img_clone = dyn_img.clone();
        let theme_clone = self.current_theme.clone();
        let algo = self.algorithm;
        let luma = self.preserve_luma;
        let level = self.hald_level;

        self.state = AppState::Loading(0.65, format!("[ * ] Applying 3D HaldCLUT mapping for {}...", theme_clone.display_name()));

        Task::perform(
            async move {
                tokio::task::spawn_blocking(move || {
                    let mut rgba = img_clone.to_rgba8();
                    let shades = theme_clone.get_shades();

                    match theme_clone {
                        ThemeSource::Custom(path) => {
                            if let Some(ext) = path.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase()) {
                                if ext == "png" {
                                    if let Ok(lut_img) = image::open(&path) {
                                        let (lw, lh) = (lut_img.width(), lut_img.height());
                                        if lw == lh && [8, 12, 14, 16].iter().any(|&l| l * l * l == lw) {
                                            let rgb_lut = lut_img.to_rgb8();
                                            correct_image(&mut rgba, &rgb_lut);
                                            let (w, h) = rgba.dimensions();
                                            let processed_dyn = image::DynamicImage::ImageRgba8(rgba.clone());
                                            let contrast = helpers::compute_wcag_contrast(&processed_dyn);
                                            return Ok((rgba.into_raw(), w, h, processed_dyn, contrast));
                                        }
                                    }
                                }
                            }
                            if !shades.is_empty() {
                                match algo {
                                    RemapAlgorithm::Gaussian => {
                                        let remapper = GaussianRemapper::new(&shades, 96.0, 0, 1.0, luma);
                                        let hald_clut = remapper.par_generate_lut(level);
                                        correct_image(&mut rgba, &hald_clut);
                                    }
                                    RemapAlgorithm::Shepard => {
                                        let remapper = ShepardRemapper::new(&shades, 16.0, 0, 1.0, luma);
                                        let hald_clut = remapper.par_generate_lut(level);
                                        correct_image(&mut rgba, &hald_clut);
                                    }
                                    RemapAlgorithm::NearestNeighbor => {
                                        let remapper = NearestNeighborRemapper::new(&shades, 1.0, luma);
                                        let hald_clut = remapper.par_generate_lut(level);
                                        correct_image(&mut rgba, &hald_clut);
                                    }
                                }
                            } else {
                                return Err(format!("Could not extract colors from LUT file {:?}", path));
                            }
                        }
                        _ => {
                            match algo {
                                RemapAlgorithm::Gaussian => {
                                    let remapper = GaussianRemapper::new(&shades, 96.0, 0, 1.0, luma);
                                    let hald_clut = remapper.par_generate_lut(level);
                                    correct_image(&mut rgba, &hald_clut);
                                }
                                RemapAlgorithm::Shepard => {
                                    let remapper = ShepardRemapper::new(&shades, 16.0, 0, 1.0, luma);
                                    let hald_clut = remapper.par_generate_lut(level);
                                    correct_image(&mut rgba, &hald_clut);
                                }
                                RemapAlgorithm::NearestNeighbor => {
                                    let remapper = NearestNeighborRemapper::new(&shades, 1.0, luma);
                                    let hald_clut = remapper.par_generate_lut(level);
                                    correct_image(&mut rgba, &hald_clut);
                                }
                            }
                        }
                    }

                    let (w, h) = rgba.dimensions();
                    let processed_dyn = image::DynamicImage::ImageRgba8(rgba.clone());
                    let contrast = helpers::compute_wcag_contrast(&processed_dyn);
                    Ok((rgba.into_raw(), w, h, processed_dyn, contrast))
                })
                .await
                .map_err(|e| format!("Worker thread panicked: {}", e))?
            },
            Message::ImageProcessed,
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SelectImage => {
                self.state = AppState::Loading(0.2, "Opening file dialog...".to_string());
                Task::perform(
                    async move {
                        let handle = rfd::AsyncFileDialog::new()
                            .add_filter("Image Files", &["png", "jpg", "jpeg", "webp", "bmp", "tiff", "tga", "gif", "ico", "hdr", "exr", "qoi", "avif"])
                            .pick_file()
                            .await;

                        match handle {
                            Some(file) => {
                                let path = file.path().to_path_buf();
                                tokio::task::spawn_blocking(move || {
                                    let dyn_img = image::open(&path)
                                        .map_err(|e| format!("Failed to decode image: {}", e))?;
                                    let rgba = dyn_img.to_rgba8();
                                    let (w, h) = rgba.dimensions();
                                    let img_handle = iced_image::Handle::from_rgba(w, h, rgba.into_raw());
                                    Ok((path, dyn_img, img_handle))
                                })
                                .await
                                .map_err(|e| format!("Worker join error: {}", e))?
                            }
                            None => Err("File selection canceled.".to_string()),
                        }
                    },
                    Message::ImageSelected,
                )
            }
            Message::ImageSelected(result) => {
                match result {
                    Ok((path, dyn_img, handle)) => {
                        self.image_filename = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                        self.image_width = dyn_img.width();
                        self.image_height = dyn_img.height();
                        self.base_image_path = Some(path);
                        self.base_image_dyn = Some(dyn_img.clone());
                        self.base_image_handle = Some(handle.clone());
                        self.preview_handle = Some(handle.clone());
                        self.processed_dyn = Some(dyn_img);
                        self.seam_carve_target = self.image_width;
                        
                        // Reset image grading state
                        self.blur_sigma = 0.0;
                        self.algorithm = RemapAlgorithm::Gaussian;
                        self.preserve_luma = false;
                        self.hald_level = 8;
                        let default_preset = "Catppuccin Mocha".to_string();
                        self.selected_preset = Some(default_preset.clone());
                        self.current_theme = ThemeSource::Preset(default_preset);

                        self.state = AppState::PreviewReady(handle);
                        self.workspace_view = WorkspaceView::Standard;
                        return self.trigger_processing();
                    }
                    Err(err) => {
                        if err != "File selection canceled." {
                            self.state = AppState::Error(err);
                        } else {
                            if let Some(ref h) = self.preview_handle {
                                self.state = AppState::PreviewReady(h.clone());
                            } else {
                                self.state = AppState::Idle;
                            }
                        }
                    }
                }
                Task::none()
            }
            Message::SelectCustomTheme => {
                Task::perform(
                    async move {
                        let handle = rfd::AsyncFileDialog::new()
                            .add_filter("LUT Files", &["png", "cube"])
                            .pick_file()
                            .await;
                        match handle {
                            Some(file) => Ok(file.path().to_path_buf()),
                            None => Err("LUT selection canceled.".to_string()),
                        }
                    },
                    Message::CustomThemeSelected,
                )
            }
            Message::CustomThemeSelected(result) => {
                match result {
                    Ok(path) => {
                        self.current_theme = ThemeSource::Custom(path);
                        self.selected_preset = None;
                        if self.base_image_dyn.is_some() {
                            return self.trigger_processing();
                        }
                    }
                    Err(err) => {
                        if err != "LUT selection canceled." {
                            self.state = AppState::Error(err);
                        }
                    }
                }
                Task::none()
            }
            Message::ThemePresetSelected(preset_name) => {
                self.selected_preset = Some(preset_name.clone());
                self.current_theme = ThemeSource::Preset(preset_name);
                if self.base_image_dyn.is_some() {
                    return self.trigger_processing();
                }
                Task::none()
            }
            Message::ApplyTheme => self.trigger_processing(),
            Message::StartProcessingWorker => self.run_processing_worker(),
            Message::ImageProcessed(result) => {
                match result {
                    Ok((buf, w, h, processed_dyn, contrast)) => {
                        self.processed_dyn = Some(processed_dyn.clone());
                        self.wcag_contrast = contrast;
                        let handle = iced_image::Handle::from_rgba(w, h, buf);
                        self.preview_handle = Some(handle.clone());
                        self.state = AppState::PreviewReady(handle);
                        return Task::perform(
                            async move {
                                tokio::task::spawn_blocking(move || {
                                    crate::modules::histogram::compute_histogram(&processed_dyn)
                                })
                                .await
                                .unwrap_or(Err("Histogram task panicked".to_string()))
                            },
                            Message::HistogramsComputed,
                        );
                    }
                    Err(err) => {
                        self.state = AppState::Error(err);
                    }
                }
                Task::none()
            }
            Message::ComputeHistograms => Task::none(),
            Message::HistogramsComputed(result) => {
                if let Ok(data) = result {
                    self.histogram_data = Some(data);
                }
                Task::none()
            }
            Message::SetWallpaper => {
                let Some(ref img_path) = self.base_image_path else {
                    return Task::none();
                };
                let path_clone = img_path.clone();
                let theme_name = self.current_theme.display_name();
                let shades = self.current_theme.get_shades();
                let algo = self.algorithm;
                let level = self.hald_level;
                let luma = self.preserve_luma;
                let trans = self.swww_transition.clone();
                let disp = self.target_display.clone();
                let backend = self.wallpaper_backend.code();
                let sync_a = self.sync_alacritty;
                let sync_k = self.sync_kitty;

                self.state = AppState::Loading(0.8, "Running Memoized Pipeline & Config Exporter...".to_string());
                Task::perform(
                    async move {
                        crate::backend::PipelineController::execute_pipeline(
                            path_clone, theme_name, shades, algo, level, luma, trans, disp, backend, sync_a, sync_k
                        )
                        .await
                        .map_err(|e| e.to_string())
                    },
                    Message::PipelineFinished,
                )
            }
            Message::WallpaperSet(result) => {
                match result {
                    Ok(path) => {
                        if let Some(ref dyn_img) = self.processed_dyn {
                            let rgba = dyn_img.to_rgba8();
                            let (w, h) = rgba.dimensions();
                            let handle = iced_image::Handle::from_rgba(w, h, rgba.into_raw());
                            self.state = AppState::PreviewReady(handle);
                        } else {
                            self.state = AppState::Idle;
                        }
                        println!("Successfully applied wallpaper from {:?}", path);
                    }
                    Err(err) => {
                        self.state = AppState::Error(err);
                    }
                }
                Task::none()
            }
            Message::CustomPaletteInputChanged(input) => {
                self.custom_palette_input = input;
                Task::none()
            }
            Message::ApplyCustomPalette => {
                let mut colors = Vec::new();
                for part in self.custom_palette_input.split(',') {
                    let s = part.trim().trim_start_matches('#');
                    if s.len() == 6 {
                        if let (Ok(r), Ok(g), Ok(b)) = (
                            u8::from_str_radix(&s[0..2], 16),
                            u8::from_str_radix(&s[2..4], 16),
                            u8::from_str_radix(&s[4..6], 16),
                        ) {
                            colors.push([r, g, b]);
                        }
                    }
                }
                if colors.len() >= 2 {
                    self.current_theme = ThemeSource::CustomPalette("Custom Riced".to_string(), colors);
                    self.selected_preset = None;
                    if self.base_image_dyn.is_some() {
                        return self.trigger_processing();
                    }
                } else {
                    self.state = AppState::Error("Please enter at least 2 valid 6-character hex codes (e.g. #89b4fa, #f38ba8).".to_string());
                }
                Task::none()
            }
            Message::SelectBatchFolder => {
                self.state = AppState::Loading(0.1, "Selecting directory for batch processing...".to_string());
                Task::perform(
                    async move {
                        let handle = rfd::AsyncFileDialog::new().pick_folder().await;
                        match handle {
                            Some(folder) => Ok(folder.path().to_path_buf()),
                            None => Err("Batch folder selection canceled.".to_string()),
                        }
                    },
                    Message::BatchFolderSelected,
                )
            }
            Message::BatchFolderSelected(result) => {
                match result {
                    Ok(folder) => {
                        let theme_clone = self.current_theme.clone();
                        let algo = self.algorithm;
                        let luma = self.preserve_luma;
                        let level = self.hald_level;

                        self.state = AppState::Loading(0.5, format!("Batch processing images in {:?}...", folder.file_name().unwrap_or_default()));
                        Task::perform(
                            async move {
                                tokio::task::spawn_blocking(move || {
                                    let output_dir = folder.join("wallmod_output");
                                    std::fs::create_dir_all(&output_dir)
                                        .map_err(|e| format!("Failed to create output directory: {}", e))?;

                                    let entries = std::fs::read_dir(&folder)
                                        .map_err(|e| format!("Failed to read directory: {}", e))?;

                                    let mut count = 0;
                                    let shades = theme_clone.get_shades();

                                    for entry in entries.flatten() {
                                        let path = entry.path();
                                        if path.is_file() {
                                            if let Some(ext) = path.extension().and_then(|s| s.to_str()).map(|s| s.to_lowercase()) {
                                                if ["png", "jpg", "jpeg", "webp", "bmp", "tiff", "tga", "gif", "ico", "hdr", "exr", "qoi", "avif"].contains(&ext.as_str()) {
                                                    if let Ok(dyn_img) = image::open(&path) {
                                                        let mut rgba = dyn_img.to_rgba8();
                                                        match algo {
                                                            RemapAlgorithm::Gaussian => {
                                                                let r = GaussianRemapper::new(&shades, 96.0, 0, 1.0, luma);
                                                                let lut = r.par_generate_lut(level);
                                                                correct_image(&mut rgba, &lut);
                                                            }
                                                            RemapAlgorithm::Shepard => {
                                                                let r = ShepardRemapper::new(&shades, 16.0, 0, 1.0, luma);
                                                                let lut = r.par_generate_lut(level);
                                                                correct_image(&mut rgba, &lut);
                                                            }
                                                            RemapAlgorithm::NearestNeighbor => {
                                                                let r = NearestNeighborRemapper::new(&shades, 1.0, luma);
                                                                let lut = r.par_generate_lut(level);
                                                                correct_image(&mut rgba, &lut);
                                                            }
                                                        }
                                                        let out_path = output_dir.join(path.file_name().unwrap());
                                                        if rgba.save(&out_path).is_ok() {
                                                            count += 1;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Ok(count)
                                })
                                .await
                                .map_err(|e| format!("Worker thread error: {}", e))?
                            },
                            Message::BatchProcessed,
                        )
                    }
                    Err(err) => {
                        if err != "Batch folder selection canceled." {
                            self.state = AppState::Error(err);
                        } else {
                            self.state = AppState::Idle;
                        }
                        Task::none()
                    }
                }
            }
            Message::BatchProcessed(result) => {
                match result {
                    Ok(count) => {
                        self.state = AppState::Notice(format!("Batch complete: Successfully processed {} images into subfolder wallmod_output/", count));
                    }
                    Err(err) => {
                        self.state = AppState::Error(err);
                    }
                }
                Task::none()
            }
            Message::AlgorithmChanged(algo) => {
                self.algorithm = algo;
                if self.base_image_dyn.is_some() {
                    return self.trigger_processing();
                }
                Task::none()
            }
            Message::TogglePreserveLuma(luma) => {
                self.preserve_luma = luma;
                if self.base_image_dyn.is_some() {
                    return self.trigger_processing();
                }
                Task::none()
            }
            Message::HaldLevelChanged(level) => {
                self.hald_level = level;
                if self.base_image_dyn.is_some() {
                    return self.trigger_processing();
                }
                Task::none()
            }
            Message::ExtractPaletteFromImage => {
                if let Some(ref dyn_img) = self.base_image_dyn {
                    let colors = helpers::extract_dominant_colors(dyn_img);
                    let mut hex_strs = Vec::new();
                    for c in &colors {
                        hex_strs.push(format!("#{x:02x}{y:02x}{z:02x}", x=c[0], y=c[1], z=c[2]));
                    }
                    self.custom_palette_input = hex_strs.join(", ");
                    self.current_theme = ThemeSource::CustomPalette("Extracted from Image".to_string(), colors);
                    self.selected_preset = None;
                    return self.trigger_processing();
                }
                Task::none()
            }
            Message::WorkspaceViewChanged(view) => {
                self.workspace_view = view;
                if view == WorkspaceView::Gallery && self.albums.is_empty() && !self.scanning_gallery {
                    return self.update(Message::ScanSystemGallery);
                }
                Task::none()
            }
            Message::ScanSystemGallery => {
                if self.scanning_gallery {
                    return Task::none();
                }
                self.scanning_gallery = true;
                Task::perform(
                    async move {
                        tokio::task::spawn_blocking(move || {
                            use rayon::prelude::*;
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

                            fn collect_dirs(dir: &std::path::Path, dirs: &mut Vec<PathBuf>, depth: u32) {
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

                            let albums: Vec<Album> = dirs_to_scan
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
                                .collect();

                            Ok(albums)
                        })
                        .await
                        .map_err(|e| format!("Scan failed: {}", e))?
                    },
                    Message::GalleryScanned
                )
            }
            Message::GalleryScanned(res) => {
                self.scanning_gallery = false;
                if let Ok(albums) = res {
                    self.albums = albums;
                }
                Task::none()
            }
            Message::SelectAlbum(opt_path) => {
                self.selected_album = opt_path.clone();
                if let Some(path) = opt_path {
                    self.state = AppState::Loading(0.3, format!("[ # ] Generating bento thumbnails for album..."));
                    Task::perform(
                        async move {
                            tokio::task::spawn_blocking(move || {
                                use rayon::prelude::*;
                                let exts = ["png", "jpg", "jpeg", "webp", "bmp", "tiff", "tga", "gif", "avif"];
                                let mut imgs = Vec::new();
                                if let Ok(entries) = std::fs::read_dir(&path) {
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
                                
                                let thumbs: Vec<(PathBuf, iced::widget::image::Handle)> = imgs.into_par_iter().take(48).filter_map(|p| {
                                    if let Ok(dyn_img) = image::open(&p) {
                                        let thumb = dyn_img.thumbnail(300, 300);
                                        let rgba = thumb.to_rgba8();
                                        let (w, h) = rgba.dimensions();
                                        let handle = iced::widget::image::Handle::from_rgba(w, h, rgba.into_raw());
                                        Some((p, handle))
                                    } else {
                                        None
                                    }
                                }).collect();
                                
                                Ok((path, thumbs))
                            })
                            .await
                            .map_err(|e| format!("Failed to read album: {}", e))?
                        },
                        Message::AlbumImagesScanned
                    )
                } else {
                    self.album_images.clear();
                    Task::none()
                }
            }
            Message::ExtractDominantColors => {
                if let Some(dyn_img) = &self.base_image_dyn {
                    let img_clone = dyn_img.clone();
                    self.state = AppState::Loading(0.5, format!("[ * ] Extracting dominant Oklab colors..."));
                    Task::perform(
                        async move {
                            tokio::task::spawn_blocking(move || {
                                crate::modules::extractor::extract_dominant_colors(&img_clone, 8)
                            })
                            .await
                            .unwrap_or(Err("Task panicked".to_string()))
                        },
                        Message::DominantColorsExtracted,
                    )
                } else {
                    Task::none()
                }
            }
            Message::DominantColorsExtracted(result) => {
                match result {
                    Ok(colors) => {
                        self.extracted_colors = Some(colors);
                        self.state = AppState::Idle;
                    }
                    Err(e) => {
                        self.state = AppState::Error(e);
                    }
                }
                Task::none()
            }
            Message::AlbumImagesScanned(res) => {
                if let Ok((path, imgs)) = res {
                    if self.selected_album.as_ref() == Some(&path) {
                        self.album_images = imgs;
                        if let Some(ref handle) = self.preview_handle {
                            self.state = AppState::PreviewReady(handle.clone());
                        } else {
                            self.state = AppState::Idle;
                        }
                    }
                }
                Task::none()
            }
            Message::SelectGalleryImage(path) => {
                self.state = AppState::Loading(0.1, format!("[ # ] Loading gallery image {}...", path.file_name().unwrap_or_default().to_string_lossy()));
                Task::perform(
                    async move {
                        tokio::task::spawn_blocking(move || {
                            let dyn_img = image::open(&path).map_err(|e| format!("Failed to parse image: {}", e))?;
                            let rgba = dyn_img.to_rgba8();
                            let (w, h) = rgba.dimensions();
                            let handle = iced_image::Handle::from_rgba(w, h, rgba.into_raw());
                            Ok((path, dyn_img, handle))
                        })
                        .await
                        .map_err(|e| format!("Worker error: {}", e))?
                    },
                    Message::ImageSelected
                )
            }
            Message::SplitOffsetChanged(_) => Task::none(),
            Message::SeamCarveTargetChanged(target) => {
                self.seam_carve_target = target;
                Task::none()
            }
            Message::ApplySeamCarving => {
                let Some(dyn_img) = self.processed_dyn.as_ref().or(self.base_image_dyn.as_ref()) else {
                    return Task::none();
                };
                let img_clone = dyn_img.clone();
                let target_w = self.seam_carve_target;
                
                self.state = AppState::Loading(0.0, format!("[ # ] Initializing Seam Carving Algorithm..."));
                
                // Since iced 0.14 doesn't support streaming progress easily in Task::perform, 
                // we'll run it and emit just the completion event for now. Progress would need a stream.
                Task::perform(
                    async move {
                        tokio::task::spawn_blocking(move || {
                            crate::backend::seam_carve::carve_width(&img_clone, target_w, |_,_| {})
                        })
                        .await
                        .map_err(|e| format!("Seam carving panicked: {}", e))
                    },
                    Message::SeamCarvingCompleted
                )
            }
            Message::SeamCarvingProgress(current, total) => {
                self.state = AppState::Loading(current as f32 / total as f32, format!("[ # ] Carving seam {}/{}", current, total));
                Task::none()
            }
            Message::SeamCarvingCompleted(res) => {
                match res {
                    Ok(dyn_img) => {
                        let rgba = dyn_img.to_rgba8();
                        let (w, h) = rgba.dimensions();
                        let handle = iced_image::Handle::from_rgba(w, h, rgba.into_raw());
                        
                        self.image_width = w;
                        self.image_height = h;
                        self.processed_dyn = Some(dyn_img);
                        self.preview_handle = Some(handle.clone());
                        self.state = AppState::PreviewReady(handle);
                    }
                    Err(e) => {
                        self.state = AppState::Error(e);
                    }
                }
                Task::none()
            }
            Message::ToggleDither => {
                self.dither_enabled = !self.dither_enabled;
                Task::none()
            }
            Message::ApplyDither => {
                let Some(dyn_img) = self.processed_dyn.as_ref().or(self.base_image_dyn.as_ref()) else {
                    return Task::none();
                };
                let img_clone = dyn_img.clone();
                let palette_colors = self.current_theme().get_shades();
                self.state = AppState::Loading(0.5, format!("[ * ] Applying Floyd-Steinberg Dithering..."));
                Task::perform(
                    async move {
                        tokio::task::spawn_blocking(move || {
                            crate::backend::dither::apply_floyd_steinberg(&img_clone, &palette_colors)
                        })
                        .await
                        .map_err(|e| format!("Dither panicked: {}", e))
                    },
                    Message::DitherCompleted
                )
            }
            Message::DitherCompleted(res) => {
                match res {
                    Ok(dyn_img) => {
                        let rgba = dyn_img.to_rgba8();
                        let (w, h) = rgba.dimensions();
                        let handle = iced_image::Handle::from_rgba(w, h, rgba.into_raw());
                        self.processed_dyn = Some(dyn_img);
                        self.preview_handle = Some(handle.clone());
                        self.state = AppState::PreviewReady(handle);
                    }
                    Err(e) => {
                        self.state = AppState::Error(e);
                    }
                }
                Task::none()
            }
            Message::SwwwTransitionChanged(trans) => {
                self.swww_transition = trans;
                Task::none()
            }
            Message::AppTabChanged(tab) => {
                self.active_tab = tab;
                Task::none()
            }
            Message::TargetDisplayChanged(disp) => {
                self.target_display = disp;
                Task::none()
            }
            Message::SidebarTabChanged(tab) => {
                self.sidebar_tab = tab;
                Task::none()
            }
            Message::WallpaperBackendChanged(be) => {
                self.wallpaper_backend = be;
                Task::none()
            }
            Message::ToggleAppTheme => {
                self.is_dark_mode = !self.is_dark_mode;
                Task::none()
            }
            Message::BlurSigmaChanged(sigma) => {
                self.blur_sigma = sigma;
                Task::none()
            }
            Message::ApplyBlur => {
                let Some(dyn_img) = self.processed_dyn.as_ref().or(self.base_image_dyn.as_ref()) else {
                    return Task::none();
                };
                let img_clone = dyn_img.clone();
                let sigma = self.blur_sigma;
                self.state = AppState::Loading(0.5, format!("[ * ] Applying Gaussian blur (sigma {:.1})...", sigma));
                Task::perform(
                    crate::wallpaper::process_blur(img_clone, sigma),
                    Message::BlurCompleted,
                )
            }
            Message::BlurCompleted(result) => {
                match result {
                    Ok((buf, w, h, blurred_dyn)) => {
                        self.processed_dyn = Some(blurred_dyn);
                        let handle = iced_image::Handle::from_rgba(w, h, buf);
                        self.preview_handle = Some(handle.clone());
                        self.state = AppState::PreviewReady(handle);
                    }
                    Err(err) => {
                        self.state = AppState::Error(err);
                    }
                }
                Task::none()
            }
            Message::SaveImageToFolder => {
                let Some(ref dyn_img) = self.processed_dyn else {
                    return Task::none();
                };
                let img_clone = dyn_img.clone();
                let default_name = if !self.image_filename.is_empty() {
                    format!("riced_{}", self.image_filename)
                } else {
                    "riced_wallpaper.png".to_string()
                };
                self.state = AppState::Loading(0.3, "Selecting destination folder/file...".to_string());
                Task::perform(
                    async move {
                        let handle = rfd::AsyncFileDialog::new().set_file_name(&default_name).save_file().await;
                        match handle {
                            Some(file) => {
                                let path = file.path().to_path_buf();
                                tokio::task::spawn_blocking(move || {
                                    img_clone.save(&path).map_err(|e| format!("Failed to save image: {}", e))?;
                                    Ok(path)
                                }).await.map_err(|e| format!("Worker error: {}", e))?
                            }
                            None => Err("Save operation canceled.".to_string()),
                        }
                    },
                    Message::ImageSaved,
                )
            }
            Message::ImageSaved(result) => {
                match result {
                    Ok(path) => {
                        self.state = AppState::Notice(format!("Successfully saved riced wallpaper to {:?}", path));
                    }
                    Err(err) => {
                        if err != "Save operation canceled." {
                            self.state = AppState::Error(err);
                        } else {
                            let _ = self.update(Message::DismissNotice);
                        }
                    }
                }
                Task::none()
            }
            Message::ExportTerminalScheme => {
                let shades = self.current_theme.get_shades();
                self.state = AppState::Loading(0.3, "Selecting export folder...".to_string());
                Task::perform(
                    async move {
                        let handle = rfd::AsyncFileDialog::new().pick_folder().await;
                        match handle {
                            Some(folder) => {
                                let dir = folder.path().to_path_buf();
                                tokio::task::spawn_blocking(move || {
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

                                    Ok(dir)
                                }).await.map_err(|e| format!("Worker error: {}", e))?
                            }
                            None => Err("Export canceled.".to_string()),
                        }
                    },
                    Message::TerminalSchemeExported,
                )
            }
            Message::TerminalSchemeExported(result) => {
                match result {
                    Ok(dir) => {
                        self.state = AppState::Notice(format!("Successfully exported Alacritty and Kitty color schemes to {:?}", dir));
                    }
                    Err(err) => {
                        if err != "Export canceled." {
                            self.state = AppState::Error(err);
                        } else {
                            let _ = self.update(Message::DismissNotice);
                        }
                    }
                }
                Task::none()
            }
            Message::ToggleSyncAlacritty(val) => {
                self.sync_alacritty = val;
                Task::none()
            }
            Message::ToggleSyncKitty(val) => {
                self.sync_kitty = val;
                Task::none()
            }
            Message::ToggleDaemon(enabled) => {
                self.daemon_enabled = enabled;
                if enabled {
                    // Instantly trigger a tick check when enabled
                    return Task::perform(async { () }, |_| Message::DaemonTick);
                }
                Task::none()
            }
            Message::SetDayHour(hour) => {
                self.day_time_hour = hour;
                Task::none()
            }
            Message::SetNightHour(hour) => {
                self.night_time_hour = hour;
                Task::none()
            }
            Message::SetDayTheme(theme) => {
                self.day_theme = theme;
                Task::none()
            }
            Message::SetNightTheme(theme) => {
                self.night_theme = theme;
                Task::none()
            }
            Message::DaemonTick => {
                use chrono::Timelike;
                let now = chrono::Local::now();
                let hour = now.time().hour();
                
                let is_day = hour >= self.day_time_hour && hour < self.night_time_hour;
                let expected_theme = if is_day { &self.day_theme } else { &self.night_theme };
                
                if self.current_theme.display_name() != *expected_theme && self.base_image_path.is_some() {
                    self.current_theme = crate::app::helpers::ThemeSource::Preset(expected_theme.to_string());
                    
                    // Directly fire SetWallpaper which handles processing and applying via the universal backend
                    return Task::perform(async { () }, |_| Message::SetWallpaper);
                }
                
                Task::none()
            }
            Message::PipelineFinished(result) => {
                match result {
                    Ok((path, hit, synced)) => {
                        if let Ok(dyn_img) = image::open(&path) {
                            let rgba = dyn_img.to_rgba8();
                            let (w, h) = rgba.dimensions();
                            let contrast = helpers::compute_wcag_contrast(&dyn_img);
                            self.processed_dyn = Some(dyn_img);
                            self.wcag_contrast = contrast;
                            let handle = iced_image::Handle::from_rgba(w, h, rgba.into_raw());
                            self.state = AppState::PreviewReady(handle);
                        } else {
                            self.state = AppState::Idle;
                        }
                        let hit_str = if hit { "[ + ] Instant Cache Hit" } else { "[ * ] Freshly Computed" };
                        let sync_str = if synced.is_empty() { "No terminal export synced".to_string() } else { format!("Synced: {}", synced.join(", ")) };
                        self.state = AppState::Notice(format!("Wallpaper Applied ({})! {}", hit_str, sync_str));
                    }
                    Err(e) => {
                        self.state = AppState::Error(e);
                    }
                }
                Task::none()
            }
            Message::DismissNotice => {
                if let Some(ref dyn_img) = self.processed_dyn {
                    let rgba = dyn_img.to_rgba8();
                    let (w, h) = rgba.dimensions();
                    let handle = iced_image::Handle::from_rgba(w, h, rgba.into_raw());
                    self.state = AppState::PreviewReady(handle);
                } else {
                    self.state = AppState::Idle;
                }
                Task::none()
            }
            Message::WindowClose => iced::window::oldest().and_then(iced::window::close),
            Message::WindowMinimize => iced::window::oldest().and_then(|id| iced::window::minimize(id, true)),
            Message::WindowMaximize => iced::window::oldest().and_then(iced::window::toggle_maximize),
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        ui::view(self)
    }
}
