//! Desktop wallpaper management engine inspired by wallrust.
//! Supports Wayland transitions (`swww`), multi-monitor display targeting, and multi-DE fallback.

use std::path::PathBuf;
use std::process::Command;

/// Asynchronously sets the given image file path as the active system desktop wallpaper
/// with customized Wayland transition animation and display output targeting.
pub async fn set_wallpaper_async(path: PathBuf, transition: String, display: String, backend: String) -> Result<PathBuf, String> {
    tokio::task::spawn_blocking(move || {
        let path_str = path.to_string_lossy().to_string();

        let run_swww = || {
            let mut swww_args = vec![
                "img",
                &path_str,
                "--transition-type",
                &transition,
                "--transition-fps",
                "60",
                "--transition-duration",
                "2.0",
            ];
            if display != "All Displays" {
                swww_args.push("--outputs");
                swww_args.push(&display);
            }
            Command::new("swww").args(&swww_args).status().map_or(false, |s| s.success())
        };

        let run_swaybg = || {
            let target = if display == "All Displays" { "*" } else { &display };
            Command::new("swaymsg")
                .args(["output", target, "bg", &path_str, "fill"])
                .status()
                .map_or(false, |s| s.success())
        };

        let run_feh = || {
            Command::new("feh")
                .args(["--bg-fill", &path_str])
                .status()
                .map_or(false, |s| s.success())
        };

        let run_gsettings = || {
            if Command::new("gsettings")
                .args(["set", "org.gnome.desktop.background", "picture-uri", &format!("file://{}", path_str)])
                .status()
                .map_or(false, |s| s.success())
            {
                let _ = Command::new("gsettings")
                    .args(["set", "org.gnome.desktop.background", "picture-uri-dark", &format!("file://{}", path_str)])
                    .status();
                true
            } else {
                false
            }
        };

        match backend.as_str() {
            "swww" => {
                if run_swww() {
                    return Ok(path);
                }
                return Err("Failed to execute swww daemon. Is swww-daemon running?".to_string());
            }
            "swaybg" => {
                if run_swaybg() {
                    return Ok(path);
                }
                return Err("Failed to set background via swaymsg / swaybg.".to_string());
            }
            "feh" => {
                if run_feh() {
                    return Ok(path);
                }
                return Err("Failed to execute feh --bg-fill.".to_string());
            }
            "gsettings" => {
                if run_gsettings() {
                    return Ok(path);
                }
                return Err("Failed to set gsettings desktop background.".to_string());
            }
            _ => {} // Auto fallback below
        }

        // Auto fallback chain
        if run_swww() {
            return Ok(path);
        }
        if run_swaybg() {
            return Ok(path);
        }
        if wallpaper::set_from_path(&path_str).is_ok() {
            let _ = wallpaper::set_mode(wallpaper::Mode::Crop);
            return Ok(path);
        }
        if run_gsettings() {
            return Ok(path);
        }

        let kde_script = format!(
            "string:var allDesktops = desktops();for (i=0;i<allDesktops.length;i++) {{d = allDesktops[i];d.wallpaperPlugin = \"org.kde.image\";d.currentConfigGroup = Array(\"Wallpaper\", \"org.kde.image\", \"General\");d.writeConfig(\"Image\", \"file://{}\");}}",
            path_str
        );
        if Command::new("qdbus")
            .args(["org.kde.plasmashell", "/PlasmaShell", "org.kde.PlasmaShell.evaluateScript", &kde_script])
            .status()
            .map_or(false, |s| s.success())
        {
            return Ok(path.clone());
        }
        if run_feh() {
            return Ok(path);
        }

        Err("Failed to set wallpaper across all supported backends. Ensure a wallpaper daemon is installed.".to_string())
    })
    .await
    .map_err(|e| format!("Worker thread error: {}", e))?
}

/// Applies a Gaussian blur on a background thread to prevent UI blocking.
/// `sigma` controls the blur radius (e.g., 5.0 for a moderate blur).
pub async fn process_blur(img: image::DynamicImage, sigma: f32) -> Result<(Vec<u8>, u32, u32, image::DynamicImage), String> {
    tokio::task::spawn_blocking(move || {
        let blurred_buffer = image::imageops::blur(&img, sigma);
        let blurred_dyn = image::DynamicImage::ImageRgba8(blurred_buffer);
        let rgba = blurred_dyn.to_rgba8();
        let (w, h) = rgba.dimensions();
        Ok((rgba.into_raw(), w, h, blurred_dyn))
    })
    .await
    .map_err(|e| format!("Background blur task failed: {}", e))?
}
