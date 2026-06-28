# Wallmod 🎨

Wallmod is a comprehensive, blazing-fast asynchronous desktop application built in Rust (with `iced` and `tokio`). It is designed to be the ultimate wallpaper management, image processing, and color-grading suite for Linux power users, ricing enthusiasts, and developers.

By uniting the capabilities of `wallrust`, `lutgen-rs`, and `imagineer` into a single, cohesive GUI, Wallmod allows you to mathematically transform any image to fit your exact desktop color scheme.

## Features ✨

### 🎛️ Color Grading & Theme Engine
- **Universal Palette Support**: Apply popular color themes (Catppuccin, Nord, Gruvbox, Tokyo Night, Synthwave, Cyberpunk) directly to your wallpapers using 3D HaldCLUT mapping.
- **Custom Palettes**: Dynamically generate or input custom hex codes to build your own themes.
- **Luminance Preservation**: Interpolate colors using Gaussian, Shepard RBF, or Nearest Neighbor algorithms while preserving the original shadow/highlight structural integrity.
- **$k$-means Dominant Color Extraction**: Extract color palettes *from* images in the mathematically uniform Oklab color space.

### 🖼️ Advanced Image Processing
- **Content-Aware Scaling**: Resize images without stretching the subjects using Sobel gradient energy calculation (Seam Carving).
- **Algorithmic Dithering**: Apply Floyd-Steinberg and Bayer diffusion for retro, banded aesthetics.
- **Non-blocking Blur**: High-quality asynchronous Gaussian blurs processed via multi-threaded worker pools.

### 📊 Telemetry & Analytics
- **Live Histograms**: Real-time waveform rendering of RGB and Luma channels.
- **Interactive Diffs**: Split-screen before/after visual inspection overlay.
- **WCAG Auditing**: Live readability / contrast ratio auditing against white/black UI elements.

### 🚀 Desktop & System Integration
- **Universal Backend Dispatcher**: Instantly apply wallpapers to your desktop using native backends (`swww`, `swaybg`, `feh`, `gsettings`, `qdbus`, or Auto-detect).
- **Wayland Hardware Acceleration**: Control `swww` transition animations (Wipe, Wave, Grow) and multi-monitor display targeting straight from the UI.
- **Time-of-Day Daemon**: A headless background task that smoothly shifts your wallpaper between day/night themes based on your system clock.
- **Terminal Synchronization**: One-click export and syncing of generated color palettes to Alacritty and Kitty config files.

## Getting Started 🛠️

### Prerequisites
- Cargo & Rust toolchain (`cargo >= 1.70`)
- A Linux environment (Wayland or X11)
- Optional Backends: `swww`, `swaybg`, or `feh` for wallpaper application.

### Installation & Running
```bash
git clone https://github.com/yourusername/wallmod.git
cd wallmod
cargo run --release
```

## Architecture 🏗️
Wallmod is driven by the **Elm Architecture** (via `iced`), ensuring robust, predictable state management. 
Heavy workloads (like KD-Tree color mapping or image encoding) are heavily parallelized using `rayon` and offloaded to `tokio::task::spawn_blocking` to guarantee a stutter-free 60FPS graphical interface.

---
*Built with ❤️ in Rust.*
