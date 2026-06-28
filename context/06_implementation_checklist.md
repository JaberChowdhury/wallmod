# Wallmod Feature Implementation Checklist

This checklist tracks the status of all unified features collected across our source repositories (`WallMod`, `wallrust`, `lutgen-rs`, `imagineer`) and requested user customizations.

---

## Category A: Image Input & Source Management
- [x] **Single Image Picker**: Asynchronous selection and decoding of raster files (`png`, `jpg`, `jpeg`, `webp`, `avif`, etc.) via `rfd`.
- [x] **Batch Directory Scanner**: Multi-format bulk scanning of entire folders to process wallpapers in one action.
- [x] **Custom LUT Importer**: Loading Look-Up Tables (`.cube`, `.png` HaldCLUT identity matrices).

## Category B: Color Grading & Palette Engine (`lutgen-rs`)
- [x] **Preset Palette Catalog**: Instant selection from curated palettes (Catppuccin, Nord, Gruvbox, Tokyo Night, etc.).
- [x] **Custom Hex Palette Builder**: Real-time hex string input (`#89b4fa, #f38ba8`) converted to RGB triplets. Note: Core iced library does not provide a native visual color picker, but the text box enables precise theme syncing.
- [x] **Multi-Algorithm Color Interpolation**: Switchable Gaussian, Shepard RBF, and Nearest Neighbor remapping algorithms.
- [x] **Luminance / Luma Preservation**: Boolean toggle preventing crushed shadows or blown highlights during color shift.
- [x] **HaldCLUT Level Resolution Control**: UI slider switching between Level 8 ($512\times512$) and Level 16 ($4096\times4096$) matrix resolutions.
- [x] **High-Quality Background Gaussian Blur**: Asynchronous `tokio::task::spawn_blocking` blur processing ($O(n \cdot r^2)$) using `imageops::blur` with adjustable slider intensity ($\sigma \in [0.0, 25.0]$) without UI freezing.
- [ ] **Dominant Color Extraction ($k$-means)**: Reverse ricing extraction of top colors from an image (Planned).

## Category C: Telemetry, Inspection & Analytics (`imagineer`)
- [x] **Top Bar Tab Navigation**: Relocated view switching from sidebar dropdown into top-level tabs above workspace preview.
- [x] **Interactive Split-Screen Diff Slider**: Mouse-controlled before/after visual inspection overlay.
- [x] **Deep Image Metadata Inspector**: Readouts of dimensions, aspect ratio, filename, and processing status.
- [x] **WCAG Accessibility Contrast Auditing**: Live computation of legibility contrast ratios against white/black labels.
- [x] **Live Processing Preview**: Continuous preview rendering while asynchronous theme calculation runs in background.
- [ ] **Channel Histograms (RGB + Luma)**: Graphical density waveform charts (Planned).

## Category D: Desktop Environment & Wallpaper Engine (`wallrust`)
- [x] **Universal Backend Dispatcher & Tool Selector**: Explicit choice between backend engines (`swww`, `swaybg`, `feh`, `gsettings`, `qdbus`, or `Auto`) with informative tool descriptions.
- [x] **Hardware-Accelerated Wayland Transitions (`swww`)**: Wipe, Wave, Grow, and Outer animation controls.
- [x] **Multi-Monitor Display Targeting**: Selection of target displays (`All Displays`, `DP-1`, `HDMI-A-1`).
- [ ] **Time-of-Day Automated Scheduler**: Solar-based background theme shifting daemon (Planned).

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
- [x] **Bootstrap UI Icon System**: Complete eradication of text-brackets and emojis for a consistent vector icon system.

## Category G: Next Generation Advanced Algorithmic Engine (Upcoming Goals)
- [x] **Content-Aware Scaling (Seam Carving)**: Sobel gradient energy calculation & Dynamic Programming path removal to shrink wallpapers while preserving subjects without stretching.
- [ ] **Algorithmic Dithering**: Floyd-Steinberg and Bayer quantization error diffusion for retro aesthetics and banding reduction.
- [ ] **Perceptual Color Space Mapping (Oklab/Oklch)**: Shifting color grading math from RGB/LAB to Oklab for mathematically uniform hue shifts without luminance artifacts.
- [ ] **Pixel Sorting Effects**: Edge-detection based pixel segment sorting for glitch art and cyberpunk wallpaper transformations.
