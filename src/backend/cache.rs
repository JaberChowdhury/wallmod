//! Memoized caching layer saving pre-computed color-graded images to disk.

use crate::app::state::RemapAlgorithm;
use crate::backend::error::AppError;
use lutgen::identity::correct_image;
use lutgen::interpolation::{GaussianRemapper, NearestNeighborRemapper, ShepardRemapper};
use lutgen::GenerateLut;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

pub struct CacheManager;

impl CacheManager {
    /// Returns the user's cache directory: ~/.cache/wallmod/
    pub fn cache_dir() -> Result<PathBuf, AppError> {
        let home = std::env::var("HOME").map_err(|_| AppError::Io("Could not resolve $HOME variable".to_string()))?;
        let dir = Path::new(&home).join(".cache").join("wallmod");
        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
        }
        Ok(dir)
    }

    /// Computes a deterministic 64-bit hash for memoization.
    pub fn compute_hash(
        img_path: &Path,
        theme_name: &str,
        shades: &[[u8; 3]],
        algo: RemapAlgorithm,
        level: u8,
        preserve_luma: bool,
    ) -> String {
        let mut hasher = DefaultHasher::new();
        img_path.hash(&mut hasher);
        theme_name.hash(&mut hasher);
        shades.hash(&mut hasher);
        (algo as u8).hash(&mut hasher);
        level.hash(&mut hasher);
        preserve_luma.hash(&mut hasher);
        format!("{:016x}.png", hasher.finish())
    }

    /// Asynchronously retrieves from cache or computes the color grading and caches it.
    /// Returns (PathBuf to cached image, bool indicating cache hit).
    pub async fn get_or_compute(
        img_path: PathBuf,
        theme_name: String,
        shades: Vec<[u8; 3]>,
        algo: RemapAlgorithm,
        level: u8,
        preserve_luma: bool,
    ) -> Result<(PathBuf, bool), AppError> {
        let cache_dir = Self::cache_dir()?;
        let hash_filename = Self::compute_hash(&img_path, &theme_name, &shades, algo, level, preserve_luma);
        let cached_path = cache_dir.join(&hash_filename);

        // Check if pre-computed image exists on disk
        if tokio::fs::try_exists(&cached_path).await.unwrap_or(false) {
            return Ok((cached_path, true));
        }

        // Offload heavy CPU processing to a blocking thread pool
        let out_path = cached_path.clone();
        tokio::task::spawn_blocking(move || -> Result<(), AppError> {
            let dyn_img = image::open(&img_path)?;
            let mut rgba = dyn_img.to_rgba8();

            if !shades.is_empty() {
                match algo {
                    RemapAlgorithm::Gaussian => {
                        let remapper = GaussianRemapper::new(&shades, 96.0, 0, 1.0, preserve_luma);
                        let hald_clut = remapper.par_generate_lut(level);
                        correct_image(&mut rgba, &hald_clut);
                    }
                    RemapAlgorithm::Shepard => {
                        let remapper = ShepardRemapper::new(&shades, 16.0, 0, 1.0, preserve_luma);
                        let hald_clut = remapper.par_generate_lut(level);
                        correct_image(&mut rgba, &hald_clut);
                    }
                    RemapAlgorithm::NearestNeighbor => {
                        let remapper = NearestNeighborRemapper::new(&shades, 1.0, preserve_luma);
                        let hald_clut = remapper.par_generate_lut(level);
                        correct_image(&mut rgba, &hald_clut);
                    }
                }
            }

            rgba.save(&out_path)?;
            Ok(())
        })
        .await
        .map_err(|e| AppError::Processing(format!("Worker thread join failed: {}", e)))??;

        Ok((cached_path, false))
    }
}
