# Wallmod — Ricer Edition 🎨⚡

Wallmod is a comprehensive, blazing-fast desktop wallpaper management, image processing, and color-grading suite built in Rust. Built on top of **[GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui)** (the high-performance GPU-accelerated UI framework from Zed) and `gpui-component`, Wallmod delivers a silky-smooth, modern desktop experience for Linux power users, ricing enthusiasts, and developers.

By uniting the capabilities of `wallrust`, `lutgen-rs`, and `imagineer` into a single GPU-accelerated GUI, Wallmod allows you to mathematically transform any image or wallpaper to fit your exact desktop color scheme.

---

## Highlights & Features ✨

### 🎛️ Color Grading & Theme Engine
- **13+ Curated Color Palettes**: Apply popular color themes (Catppuccin Mocha, Nord, Gruvbox Dark, Tokyo Night, Synthwave, Cyberpunk, Solarized Dark, Rose Pine, One Dark, Monokai, Dracula, Material Deep Ocean, Gruvbox Material) directly to your wallpapers using 3D HaldCLUT mapping.
- **Custom LUT Importer**: Import standard `.cube` or `.png` HaldCLUT files to apply complex cinematic color grades.
- **Luminance Preservation**: Interpolate colors using Gaussian, Shepard RBF, or Nearest Neighbor algorithms while preserving the original shadow/highlight structural integrity.
- **$k$-means Dominant Color Extraction**: Extract color palettes *from* images in the mathematically uniform Oklab color space.

### 🎨 Photoshop-Style Color Adjustments
- **Real-time Adjustments**: Fine-tune your graded or original wallpapers with dedicated controls for:
  - **Brightness**: Adjust highlight/shadow intensity (`-20`, `0`, `+20`).
  - **Contrast**: Expand or compress dynamic range (`-20`, `0`, `+20`).
  - **Saturation**: Desaturate to grayscale or boost color vibrancy (`Desat`, `Norm`, `Vivid`).
  - **Hue Rotation**: Shift color angles across the spectrum (`0°`, `90°`, `180°`).

### 🖼️ Advanced Image Processing & Effects
- **Content-Aware Scaling (Seam Carving)**: Resize images without stretching subjects using Sobel gradient energy calculation and dynamic programming path removal.
- **Algorithmic Dithering**: Apply Floyd-Steinberg error diffusion for retro aesthetics and banding reduction.
- **Non-blocking Blur**: High-quality asynchronous Gaussian blurs processed via multi-threaded Rayon worker pools.

### 📊 Telemetry & Inspection Dashboard
- **Interactive Diffs**: Split-screen before/after visual inspection comparing the base image against graded output.
- **WCAG Accessibility Auditing**: Live contrast ratio calculation to ensure desktop widgets remain readable against your background.
- **Album Gallery Scanner**: Rayon-powered background folder discovery across system directories.

### 🚀 Desktop & System Integration
- **Universal Backend Dispatcher**: Instantly apply wallpapers using native backends (`swww`, `swaybg`, `feh`, `gsettings`, or Auto-detect).
- **Wayland Hardware Acceleration**: Control `swww` transition animations (Wipe, Wave, Grow, Outer, Random) and target specific monitors (`DP-1`, `HDMI-A-1`, `eDP-1`).
- **Time-of-Day Automated Scheduler**: A background daemon that shifts your wallpaper between day/night themes based on your system clock.
- **Terminal Synchronization**: One-click export and syncing of generated color palettes to Alacritty (`~/.config/alacritty`) and Kitty (`~/.config/kitty`).

---

## Getting Started 🛠️

### Prerequisites
- Rust toolchain (`cargo >= 1.75`)
- Linux desktop environment (Wayland or X11)
- Optional CLI dependencies for live desktop application: `swww`, `swaybg`, or `feh`.

### Installation & Running
```bash
git clone https://github.com/JaberChowdhury/wallmod.git
cd wallmod
cargo run --release
```

---

## Architecture & Design Philosophy 🏗️
Wallmod enforces a strict separation between **UI Presentation** and **Core Logic**:
1. **Presentation Layer (`src/ui`)**: Built exclusively with `gpui` and Shadcn-inspired `gpui-component` widgets. Contains zero business logic or CPU-bound loops.
2. **Core Logic (`src/app` & `src/backend`)**: Pure Rust state models and pipeline controllers. Heavy workloads (like KD-Tree color mapping, HaldCLUT generation, and Photoshop pixel loops) are heavily parallelized using `rayon` and offloaded to a global `tokio` background thread pool via `crate::backend::runtime::spawn_blocking`. This guarantees a stutter-free, zero-blocking UI runtime.

---

*Built with ❤️ in Rust & GPUI.*
