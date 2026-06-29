# Wallmod Feature Implementation Checklist

This checklist tracks the status of all unified features collected across our source repositories (`WallMod`, `wallrust`, `lutgen-rs`, `imagineer`) and requested user customizations.

---

## Category A: Image Input & Source Management

- [x] **Single Image Picker**: Asynchronous selection and decoding of raster files (`png`, `jpg`, `jpeg`, `webp`, `avif`, etc.) via `rfd`.
- [x] **Batch Directory Scanner**: Multi-format bulk scanning of entire folders to process wallpapers in one action.
- [x] **Output directory selection**: let user select where to export the image
- [x] **Extract all color from an image in a different tab**: in a different tab add option to extract all colors from the image and visually render the colors for better understanding

## Category B: Color Grading & Palette Engine (`lutgen-rs`)

- [x] **Preset Palette Catalog**: Instant selection from curated palettes (Catppuccin, Nord, Gruvbox Dark, Tokyo Night, Synthwave, Cyberpunk, Vintage Sepia, Retro 4-Color, etc.) normalized to exactly 16 standard shades.
- [x] **Custom Hex Palette Builder**: Real-time visual palette editor with precise RGB sliders, live gradient preview, and multi-color manipulation.
- [x] **Multi-Algorithm Color Interpolation**: Switchable Gaussian, Shepard RBF, and Nearest Neighbor remapping algorithms.
- [x] **Luminance / Luma Preservation**: Boolean toggle preventing crushed shadows or blown highlights during color shift.
- [x] **HaldCLUT Level Resolution Control**: UI slider switching between Level 8 ($512\times512$) and Level 16 ($4096\times4096$) matrix resolutions.
- [x] **High-Quality Background Gaussian Blur**: Asynchronous `tokio::task::spawn_blocking` blur processing ($O(n \cdot r^2)$) using `imageops::blur` with adjustable slider intensity ($\sigma \in [0.0, 25.0]$) without UI freezing.
- [x] **Dominant Color Extraction ($k$-means)**: Reverse ricing extraction of top colors from an image using Oklab.

## Category C: Telemetry, Inspection & Analytics (`imagineer`)

- [x] **Top Bar Tab Navigation**: Relocated view switching from sidebar dropdown into top-level tabs above workspace preview.
- [x] **Interactive Split-Screen Diff Slider**: Mouse-controlled before/after visual inspection overlay with interactive percentage buttons (`10%`, `30%`, `50%`, `70%`, `90%`) with resizable panel that can be resize by cursor.
- [x] **Animated Loading Dot Overlay(loading animation)**: additionally with existing animation Stylized top-bar pulsating spinner indicator active during asynchronous image decoding and heavy color grading pipelines.
- [x] **Deep Image Metadata Inspector**: Readouts of dimensions, aspect ratio, filename, and processing status.
- [x] **WCAG Accessibility Contrast Auditing**: Live computation of legibility contrast ratios against white/black labels.
- [x] **Live Processing Preview**: Continuous preview rendering while asynchronous theme calculation runs in background.
- [x] **Channel Histograms (RGB + Luma)**: Graphical density waveform charts displayed in the telemetry dashboard.

## Category D: Desktop Environment & Wallpaper Engine (`wallrust`)

- [x] **Universal Backend Dispatcher & Tool Selector**: Explicit choice between backend engines (`swww`, `swaybg`, `feh`, `gsettings`, `qdbus`, or `Auto`) with informative tool descriptions.
- [x] **Hardware-Accelerated Wayland Transitions (`swww`)**: Wipe, Wave, Grow, and Outer animation controls.
- [x] **Multi-Monitor Display Targeting**: Selection of target displays (`All Displays`, `DP-1`, `HDMI-A-1`).
- [x] **Time-of-Day Automated Scheduler**: Fixed-time background theme shifting daemon with chrono integration.

## Category E: Export & System Integration

- [x] **Terminal Scheme Exporter**: Automated config syncing for Alacritty (`~/.config/alacritty`) and Kitty (`~/.config/kitty`).
- [x] **Save Processed Image**: Direct saving of color-graded images to target directories.
- [x] **Memoized Caching Layer**: Pre-computation checking via hash-based caching (`CacheManager`).

## Category F: Gallery & Visual Organization

- [x] **Multi-Threaded System Gallery Scanner**: Rayon-powered background folder discovery across system directories.
- [x] **Bento Grid Thumbnails**: Visual image thumbnail cards displayed inside responsive grid layouts, generated in parallel via Rayon.
- [x] **Left Panel Category Tabs**: Clean tabbed interface separating Theme & LUT, Desktop Engine, and Export controls.
- [x] **App Theme Toggle**: Live switching between clean Light and Dark CSD application themes.
- [x] **Visual Error Handling Card**: Interactive diagnostic view displaying detailed errors and troubleshooting steps.
- [x] **Bootstrap UI Icon System**: Complete eradication of text-brackets and emojis for a vector icon system, attaching proper Lucide/Zed vector icons (`IconName`) to 100% of buttons, subtabs, adjustment sliders, and dropdown options across all workspace categories.

## Category G: Next Generation Advanced Algorithmic Engine (Upcoming Goals)

- [x] **Content-Aware Scaling (Seam Carving)**: Sobel gradient energy calculation & Dynamic Programming path removal to shrink wallpapers while preserving subjects without stretching.
- [x] **Algorithmic Dithering**: Floyd-Steinberg and Bayer quantization error diffusion for retro aesthetics and banding reduction.
- [x] **Perceptual Color Space Mapping (Oklab/Oklch)**: Shifting color grading math from RGB/LAB to Oklab for mathematically uniform hue shifts without luminance artifacts.
- [x] **Photoshop Color Adjustments**: Saturation, Brightness, Contrast, and Hue Rotation sliders/buttons integrated into real-time grading pipeline.
- [x] **Pixel Sorting Effects**: Edge-detection based pixel segment sorting for glitch art and cyberpunk wallpaper transformations.

## Category H: Future Roadmap & Next-Gen Capabilities

- [] **AI Image Upscaling**: Real-time neural upscaling (Real-ESRGAN/ONNX runtime integration) for low-res wallpapers. (not now)
- [] **Video & Live Wallpaper Engine**: Support for animated webm/mp4 wallpapers via mpvpaper or swww video integration.(not now)
- [] **OCR Wallpaper Text Extraction**: Extract quote or code snippets from desktop wallpapers using Tesseract OCR.(not now)
- [] **Custom Shader Pipeline**: Support user-provided WebGPU / GLSL fragment shaders for live background post-processing.( with 10+ preset )
- [] **8bit ,16bit, 32bit etc**: Support all of these style
