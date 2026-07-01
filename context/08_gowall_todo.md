# Gowall GUI Implementation TODO (Sidecar Architecture)

## Phase 1: Foundation & Scaffolding
- [x] Set up `GowallState` and global `Model<GowallState>` in `src/app/gowall_state.rs`.
- [x] Create base GPUI View `GowallWorkspace` in `src/ui/gowall_tab.rs`.
- [x] Add the Gowall tab to the main application navigation/tab bar.
- [x] Implement layout scaffolding (Left Sidebar for tools, Right Canvas for preview).
- [x] Add basic image loading (File Picker) and rendering to the Canvas.
- [x] Clone and clean up the `gowall` Go source code into `gowall_src/`.

## Phase 2: Go Sidecar Infrastructure
- [x] Create a `build.rs` script to compile the `gowall_src` Go code into a binary during Cargo build.
- [x] Implement an asynchronous `tokio::process::Command` wrapper in Rust to securely call the Go binary.
- [x] Add error handling to parse Go panic/stderr outputs and show GPUI toast notifications.

## Phase 3: Core Sidecar Integrations (Image Ops)
- [x] Bind Theme Conversion (`gowall color -t ...`) to the UI dropdowns.
- [x] Bind Quick Actions (Invert, Grayscale, Flip) to the UI buttons.
- [x] Bind Format Conversion and Compression.
- [x] Update GPUI Image Canvas automatically when the Go binary finishes processing.

## Phase 4: Advanced Sidecar Integrations (AI & Utilities)
- [x] Bind Background Removal (`gowall bg remove`) and hook up GPUI loading spinners.
- [x] Bind AI Upscaling (`gowall upscale`) and handle long-running process timeouts.
- [ ] Bind OCR (`gowall ocr`) and parse `stdout` into a GPUI selectable text area.
- [ ] Bind Daily Wallpapers fetching and render them in a grid view.
