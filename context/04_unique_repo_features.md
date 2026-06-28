# 🌐 Unified Feature Set — The Complete Deduplicated Union

Thinking of our project feature set like inserting arrays into a C++ `std::set`, we perform a mathematical union:

$$\mathcal{F}_{\text{total}} = \mathcal{F}_{\text{WallMod}} \cup \mathcal{F}_{\text{wallrust}} \cup \mathcal{F}_{\text{lutgen-rs}} \cup \mathcal{F}_{\text{imagineer}}$$

This deduplicates overlapping capabilities (such as basic image loading or palette application) and collects **every distinct feature** across all four sources into a single, comprehensive catalog. Below is the complete union of features, what they do, and **how we use/implement them** within our modular Shadcn UI architecture.

---

## 🗂️ Category A: Image Input & Source Management

### 1. Single Image Picker
- **What it is**: Asynchronous selection and decoding of individual raster files (`.png`, `.jpg`, `.jpeg`, `.webp`).
- **How to use / implement**: Handled via `rfd::AsyncFileDialog::new().pick_file()`. Decoded on Tokio blocking threads (`image::open`) to generate an Iced RGBA handle (`Handle::from_rgba`).

### 2. Batch Directory Scanner
- **What it is**: Bulk scanning of an entire folder to process multiple wallpaper images in one click.
- **How to use / implement**: Handled via `rfd::AsyncFileDialog::new().pick_folder()`. Iterates over directory entries, applies the active HaldCLUT or grading algorithm to every matching image, and saves results into `<folder>/wallmod_output/`.

### 3. Custom LUT Importer
- **What it is**: Loading standard 3D Look-Up Tables (`.cube`) or 2D HaldCLUT image identity matrices (`.png`).
- **How to use / implement**: If `.png`, loaded via `image::open()` and converted to RGB8 to act as an identity matrix for `lutgen::identity::correct_image()`.

---

## 🎨 Category B: Color Grading & Palette Engine (`lutgen-rs` Union)

### 4. Preset Palette Catalog
- **What it is**: Instant selection from curated, industry-standard color palettes (Catppuccin Mocha, Nord Arctic, Gruvbox Dark, Tokyo Night, Dracula, Rosé Pine, Solarized Dark, One Dark, Kanagawa).
- **How to use / implement**: Populated in a Shadcn `pick_list` dropdown. Mapped directly to `lutgen_palettes::Palette::get().to_vec()`.

### 5. Custom Hex Palette Builder
- **What it is**: Allowing ricers to construct personalized palettes on the fly by typing hex strings (e.g., `#89b4fa, #f38ba8, #a6e3a1`).
- **How to use / implement**: Parsed via `u8::from_str_radix` into RGB triplets. Passed dynamically into `GaussianRemapper::new()` to generate custom HaldCLUTs.

### 6. Multi-Algorithm Color Interpolation
- **What it is**: Providing distinct mathematical remapping techniques for different visual styles:
  - **Gaussian Remapping**: Smooth, photorealistic color blending.
  - **Shepard Remapping (RBF)**: Radial Basis Function interpolation for sharper color separation.
  - **Nearest Neighbor**: Hard color quantization for retro 8-bit or pixel-art aesthetics.
- **How to use / implement**: Add an algorithm selector dropdown to `sidebar.rs`. Pass the chosen algorithm enum to worker threads to initialize either `GaussianRemapper`, `ShepardRemapper`, or `NearestNeighborRemapper` in `lutgen`.

### 7. Luminance / Luma Preservation
- **What it is**: Locking the original image's brightness levels ($Y$ in YUV/Lab space) while altering only hue and saturation, preventing crushed shadows or blown-out highlights.
- **How to use / implement**: Implemented as a boolean toggle switch (`[x] Preserve Luma`) in `sidebar.rs`. Passed as the `preserve_luma: bool` argument into `GaussianRemapper::new(&colors, std, order, min_dist, preserve_luma)`.

### 8. Dominant Color Extraction (Reverse Ricing)
- **What it is**: Analyzing a loaded wallpaper using $k$-means clustering to automatically discover its top 6–16 dominant colors.
- **How to use / implement**: Button `[ ⚡ ] Extract Palette from Image`. Runs color quantization on background threads to produce a new custom palette that automatically populates the Active Color Shades and Custom Builder input.

### 9. HaldCLUT Level Resolution Control
- **What it is**: Controlling the internal Look-Up Table matrix resolution.
- **How to use / implement**: Toggle between Level 8 ($512\times512$ matrix, lightning-fast for interactive preview sliders) and Level 16 ($4096\times4096$ matrix, maximum color fidelity for final wallpaper export).

---

## 🔍 Category C: Telemetry, Inspection & Analytics (`imagineer` Union)

### 10. Interactive Split-Screen Diff Slider
- **What it is**: A side-by-side visual inspection view where dragging a vertical line compares the unmodified original pixels on the left against the color-graded theme pixels on the right.
- **How to use / implement**: Built inside `workspace.rs`. Tracks mouse horizontal offset ($X$) to dynamically clip and overlay the original image handle over the processed image handle.

### 11. Channel Histograms (RGB + Luminance)
- **What it is**: Graphical density waveform distributions showing the pixel intensity frequency ($0..255$) across Red, Green, Blue, and Luminance channels.
- **How to use / implement**: Sub-sample pixels during processing (`compute_stats`) to build 256-bin arrays. Rendered in Iced using custom canvas paths or vertical bar charts.

### 12. Deep Image Metadata Inspector
- **What it is**: Readouts of image resolution, aspect ratio, color profile, bit depth, file size, and dynamic range.
- **How to use / implement**: Rendered as a Shadcn info card below the workspace preview.

### 13. WCAG Accessibility Contrast Auditing
- **What it is**: Real-time evaluation of wallpaper luminance contrast against white and black text labels.
- **How to use / implement**: Computes average contrast ratio according to WCAG 2.1 formulas. Displays a badge (`Pass AA`, `Pass AAA`, or `Warning: Low Legibility`) to inform the ricer if desktop widgets/text will be easily readable.

---

## 🖥️ Category D: Desktop Environment & Wallpaper Engine (`wallrust` Union)

### 14. Universal Backend Dispatcher
- **What it is**: Cross-platform wallpaper application supporting all major Linux desktop environments and window managers.
- **How to use / implement**: Auto-detects session via `$XDG_CURRENT_DESKTOP` or `$DESKTOP_SESSION` and executes the optimal command (`swww`, `swaybg`, `feh`, `gsettings`, `qdbus`).

### 15. Hardware-Accelerated Wayland Transitions (`swww`)
- **What it is**: High-FPS visual transition animations when setting wallpapers on Wayland compositors (Hyprland, Sway).
- **How to use / implement**: Add transition selector (`Wipe`, `Wave`, `Grow`, `Outer`, `Random`) and duration slider to sidebar controls. Dispatches `swww img /tmp/wallmod.png --transition-type <type> --transition-duration <sec>`.

### 16. Multi-Monitor Display Targeting
- **What it is**: Setting different color-graded themes to separate connected displays independently.
- **How to use / implement**: Query display outputs via `swww query` or `xrandr`. Provide a display selector dropdown (`All Displays`, `DP-1`, `HDMI-A-1`) before dispatching wallpaper commands.

### 17. Time-of-Day Automated Scheduler
- **What it is**: Automated background daemon scheduling that shifts desktop themes based on local solar hours.
- **How to use / implement**: Option to enable background interval loop (`tokio::time::interval`) applying light palettes (e.g., Solarized Light) at noon and dark palettes (Tokyo Night) after sunset.

---

## 📦 Category E: Export & System Integration

### 18. Terminal Scheme Exporter
- **What it is**: Exporting the active color palette directly to terminal configuration syntax.
- **How to use / implement**: Clicking `[ ↗ ] Export Terminal Theme` generates formatted config files for Alacritty (`.toml`), Kitty (`.conf`), Waybar (`style.css`), or Neovim (`.lua`) inside `wallmod_output/`.
