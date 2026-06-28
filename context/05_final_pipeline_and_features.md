# Wallmod: Final Architecture, Pipeline & Features Documentation

## 1. Executive Summary & Design Philosophy
**Wallmod** is an idiomatic, high-performance Linux desktop application built with Rust and the **iced (v0.14)** GUI library. Designed specifically for Linux ricers and customization enthusiasts, Wallmod provides an end-to-end wallpaper and color theme customization pipeline.

Key Design Constraints & Solutions Achieved:
- **Polished ASCII Icons:** Replaced all emojis with sleek bracketed ASCII icons (`[ * ]`, `[ > ]`, `[ i ]`, `[ + ]`) ensuring a clean terminal-hacker aesthetic.
- **Top Bar Tab Navigation:** Relocated workspace view switching out of the sidebar dropdown into an intuitive Top Bar placed directly over the image workspace.
- **Universal Image Support:** Expanded format parsing across file pickers and batch processors to support `png`, `jpg`, `jpeg`, `webp`, `bmp`, `tiff`, `tga`, `gif`, `ico`, `hdr`, `exr`, `qoi`, and `avif`.

---

## 2. Architecture & Modular Breakdown (Shadcn-like Design System)
Inspired by web development practices with **shadcn/ui**, Wallmod isolates all visual design tokens and shared UI helpers inside a single design system file: `src/ui/theme.rs`. Modifying color palettes, border radii, or typography in `theme.rs` instantly propagates across all application components.

### Modular Source Structure:
- `src/main.rs`: Application initialization, font loading, and window setup.
- `src/app/mod.rs`: The Elm-architecture controller implementing state transitions, asynchronous commands, and message handling.
- `src/app/state.rs`: Defines core state structures (`WallmodApp`, `AppState`, `WorkspaceView`, `Album`).
- `src/ui/mod.rs`: Root UI layout container combining the Header, Sidebar, and Workspace panes.
- `src/ui/theme.rs`: Design tokens, button styles, card containers, and color definitions.
- `src/ui/header.rs`: Custom window title bar with window controls (`[ _ ]`, `[ [] ]`, `[ X ]`).
- `src/ui/sidebar.rs`: Left control panel organizing controls into categories (LUT selection, palettes, export settings).
- `src/ui/workspace.rs`: Right preview pane handling standard visual previewing, split diffing, dashboard info, and the system album gallery.
- `src/ui/swatches.rs`: Palette color swatch visualizers.

---

## 3. Core Feature Pipelines

### A. Multi-Threaded System Gallery Scanner
To facilitate seamless image discovery without manual file navigation, Wallmod includes a built-in gallery app:
1. **Directory Discovery:** Scans common image directories (`~/Pictures`, `~/Downloads`, `~/Wallpapers`, `/usr/share/backgrounds`).
2. **Parallel Processing:** Uses **Rayon** (`par_iter`) and asynchronous `tokio::task::spawn_blocking` to index directories and count valid image files across multiple CPU threads without blocking the GUI thread.
3. **Album Navigation:** Presents discovered folders as interactive Album cards. Clicking an album loads its image contents into a responsive grid.

### B. Memoized LUT Pipeline & Universal Config Exporter
Processing large 4K/8K images through Look-Up Tables (`lutgen`) is computationally intensive.
- **Memoization Layer:** Pre-computes and caches generated images based on path and palette selection, reducing switching latency to near zero.
- **Universal Config Exporter:** Syncs generated RGB hex color palettes directly into terminal emulator config files (`~/.config/alacritty`, `~/.config/kitty`).

### C. Live Progress & WCAG Telemetry
- Displays a progressive percentage bar (`0%` -> `100%`) during background image transforms.
- Calculates live WCAG contrast ratios between selected primary foreground and background colors to ensure accessibility compliance.
