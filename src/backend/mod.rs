//! Backend service layer orchestrating caching, processing, wallpaper setting, and config export.

pub mod cache;
pub mod dither;
pub mod error;
pub mod exporter;
pub mod runtime;
pub mod seam_carve;

use crate::app::state::RemapAlgorithm;
use cache::CacheManager;
use error::AppError;
use exporter::{AlacrittyExporter, KittyExporter, ThemeExporter};
use std::path::PathBuf;

pub struct PipelineController;

impl PipelineController {
    /// Orchestrates memoized processing, wallpaper setting, and concurrent terminal config export.
    pub async fn execute_pipeline(
        img_path: PathBuf,
        theme_name: String,
        shades: Vec<[u8; 3]>,
        algo: RemapAlgorithm,
        level: u8,
        preserve_luma: bool,
        transition: String,
        display: String,
        backend: String,
        sync_alacritty: bool,
        sync_kitty: bool,
    ) -> Result<(PathBuf, bool, Vec<String>), AppError> {
        // 1. Get from memoization cache or compute
        let (cached_path, cache_hit) = CacheManager::get_or_compute(
            img_path,
            theme_name,
            shades.clone(),
            algo,
            level,
            preserve_luma,
        )
        .await?;

        // 2. Set desktop wallpaper asynchronously
        let wp_path = cached_path.clone();
        crate::wallpaper::set_wallpaper_async(wp_path, transition, display, backend)
            .await
            .map_err(AppError::Config)?;

        // 3. Iterate through active ThemeExporters to sync terminal apps concurrently
        let mut exporters: Vec<Box<dyn ThemeExporter>> = Vec::new();
        if sync_alacritty {
            exporters.push(Box::new(AlacrittyExporter));
        }
        if sync_kitty {
            exporters.push(Box::new(KittyExporter));
        }

        let mut synced = Vec::new();
        for exporter in exporters {
            match exporter.export(&shades).await {
                Ok(_) => synced.push(exporter.name().to_string()),
                Err(e) => eprintln!("Failed to export {}: {}", exporter.name(), e),
            }
        }

        Ok((cached_path, cache_hit, synced))
    }
}
