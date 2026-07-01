# Gowall GUI Implementation TODO

## Phase 1: Foundation & Scaffolding
- [ ] Set up `GowallState` and global `Model<GowallState>` in `src/app/gowall_state.rs`.
- [ ] Create base GPUI View `GowallWorkspace` in `src/ui/gowall_tab.rs`.
- [ ] Add the Gowall tab to the main application navigation/tab bar.
- [ ] Implement layout scaffolding (Left Sidebar for tools, Right Canvas for preview).
- [ ] Add basic image loading (File Picker) and rendering to the Canvas.

## Phase 2: Core Image Adjustments (Synchronous / Basic Async)
- [ ] Implement "Quick Actions" row: Invert, Grayscale, Flip Horizontal, Flip Vertical.
- [ ] Add Image Format Conversion (Save As: png, webp, jpeg) with quality slider.
- [ ] Build the Custom Theme Recolor tool (reuse existing `image_themer` or `lutgen` logic).
- [ ] Add Pixel Art Generator (downscale/upscale with Nearest filter).
- [ ] Add Specific Color Replacement (Color picker + tolerance slider).

## Phase 3: Advanced Integrations & External Libraries
- [ ] Add SVG/Icon Theming using `usvg`/`resvg`.
- [ ] Implement OCR Text Extraction (integrate `rusty-tesseract`).
- [ ] Build GIF Maker (drag-and-drop multiple frames, output GIF).
- [ ] Integrate Daily Wallpapers (fetch from API via `reqwest`, display in grid).

## Phase 4: Heavy AI Tasks (ONNX)
- [ ] Add Background Removal (`rembg` or U-2-Net ONNX model).
- [ ] Add AI Image Upscaling (Real-ESRGAN ONNX model).
- [ ] Connect heavy AI tasks to `cx.background_executor()` to prevent UI blocking.
- [ ] Add visual loading states (spinners/progress bars) during async execution.
