# WallMod Studio: Gowall GUI Integration Plan (GPUI)

This document outlines a highly detailed architectural and implementation plan for integrating the complete A-to-Z feature set of [Achno/gowall](https://github.com/Achno/gowall) into **WallMod Studio**. 

Instead of treating these as disparate tools, we will build a dedicated, fully-featured **"Gowall Workspace Tab"** within our app, entirely powered by **Zed's GPUI framework**. This tab will act as a complete graphical interface for `gowall`, allowing users to visually execute every CLI command without ever touching the terminal.

---

## 1. Architectural Strategy (GPUI)

Since we are using **GPUI**, our architecture must conform to its `Model`, `View`, and reactive update (`cx.notify()`) paradigms.

### 1.1 State Management & Async Execution
- **`GowallModel` (Data Layer):** A globally accessible GPUI `Model<GowallState>` that holds the currently loaded image buffer, selected target formats, extracted text (from OCR), and configuration settings.
- **Background Executor:** Heavy image manipulation (Upscaling, OCR, Rembg) must never block the main thread. We will utilize GPUI's built-in `cx.background_executor().spawn()` to offload these tasks, returning results via `cx.update()`.
- **View Hierarchy (`gpui::View`):** The Gowall tab will be a master `View` that implements `gpui::Render`. It will be split into a **Sidebar (Tools & Settings)** and a **Canvas (Preview & Results)**.

---

## 2. Comprehensive Feature Implementation Breakdown

Below is the A-to-Z implementation plan for every feature in `gowall`, translated into GPUI logic.

### 2.1 Convert Wallpaper & Icon Themes (Recolor)
- **UI:** A dropdown in the sidebar to select presets (Catppuccin, Nord, etc.) or a custom color picker component. For icons, an SVG uploader.
- **Backend:** 
  - For raster images: Dispatch to our existing color mapping logic (`lutgen-rs` / $k$-means), but wrapped in `cx.background_executor()`.
  - For SVG/Icons: Use the `usvg` or `resvg` crates to parse the XML tree, replace `fill="..."` and `stroke="..."` attributes with the theme's hex codes, and render the SVG back to a GPUI `Image` element.

### 2.2 Image Compression & Format Conversion
- **UI:** A "Save As" section with a dropdown for format (`png`, `jpeg`, `webp`, `avif`) and a slider (`gpui` custom slider component) for quality (1-100%).
- **Backend:** 
  - Read the active buffer from `GowallModel`.
  - Use `image::codecs` (e.g., `WebPEncoder`, `JpegEncoder`).
  - Output estimated file size dynamically on slider change by writing to an in-memory `Cursor<Vec<u8>>` on a background task before finalizing the save.

### 2.3 Optical Character Recognition (OCR)
- **UI:** An "Extract Text" button. When clicked, a loading spinner (GPUI SVG rotation animation) appears. Once done, a `gpui` text area populates with the extracted text, alongside a "Copy to Clipboard" button (`cx.write_to_clipboard()`).
- **Backend:** 
  - Integrate `rusty-tesseract` (calling local tesseract binary) or a lightweight ONNX model via `ort`.
  - Capture the user's cropped bounding box (via mouse drag events on the GPUI Canvas) and send that specific region to the OCR engine.

### 2.4 AI Image Upscaling & Background Removal
- **UI:** Two prominent action buttons: "✨ Upscale Image" and "✂️ Remove Background".
- **Backend:**
  - Incorporate `ort` (ONNX Runtime).
  - **Upscaling:** Load a Real-ESRGAN `.onnx` model. Chunk the image into tensors, pass through the neural network, and stitch back.
  - **Background Removal:** Load U-2-Net or `rembg` `.onnx` model. Generate the alpha mask, multiply it with the original image buffer, and update the GPUI `View` to show the transparent PNG.

### 2.5 Image to Pixel Art
- **UI:** A slider for "Block Size" or "Downscale Factor".
- **Backend:** 
  - Downscale the image to `width / factor` using `imageops::FilterType::Nearest`.
  - Upscale back to the original size using the same `Nearest` filter. Update the GPUI preview buffer.

### 2.6 Specific Color Replacement
- **UI:** A "Source Color" eyedropper tool (tracking mouse clicks over the GPUI Image view to grab the pixel's RGB) and a "Target Color" picker. A "Tolerance" slider.
- **Backend:** Iterate over the image buffer. Calculate Euclidean distance or Oklab delta. If `distance < tolerance`, swap with target.

### 2.7 GIF Creation from Images
- **UI:** A drag-and-drop zone (`gpui` supports drag/drop events) to import multiple image frames. Settings for FPS/Delay and Loop Count.
- **Backend:** Use the `image::codecs::gif::GifEncoder`. Assemble the frames on a background task and save to disk.

### 2.8 Essential Effects (Invert, Grayscale, Brightness, Draw)
- **UI:** A grid of toggle buttons (like standard photo editors). For drawing, inputs for border thickness and color.
- **Backend:** 
  - Use standard `imageops` functions: `invert`, `grayscale`, `flip_horizontal`, `flip_vertical`.
  - Brightness/Contrast: Simple math over RGB channels.
  - Draw: Overlay colored rectangles onto the `ImageBuffer` for grids/borders.

### 2.9 Daily Wallpapers
- **UI:** A "Discover" tab inside the Gowall view. A grid layout (`gpui` Flexbox/Grid) displaying remote thumbnails.
- **Backend:** Use `reqwest` to hit the same APIs `gowall` uses. Fetch the images to memory, decode them, and render them in GPUI `Image` components.

---

## 3. GPUI Layout Blueprint for the Gowall Tab

To seamlessly integrate this without disrupting WallMod Studio's core ricer flow, the Gowall GUI will have its own tab. 

### Structure
```rust
struct GowallWorkspace {
    state: Model<GowallState>,
    // other view state...
}

impl Render for GowallWorkspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .child(self.render_sidebar(cx)) // Left: Tools (Upscale, OCR, Pixel Art)
            .child(self.render_canvas(cx))  // Right: Live Image Preview & Text Results
    }
}
```

### The Sidebar (`render_sidebar`)
- **Accordions/Sections:** Group tools logically:
  - **Colors:** Recolor Theme, Replace Color, Invert, Grayscale, Extract Palette.
  - **AI & Text:** OCR, Background Removal, AI Upscale.
  - **Format & Size:** Pixel Art, Compress, Resize, Format Shift.
  - **Batch & GIF:** GIF Maker.

### The Canvas (`render_canvas`)
- A large, centered GPUI `Image` container.
- Pan and zoom support (using GPUI mouse wheel and drag event handlers).
- A floating toolbar at the top for Undo/Redo (requires our `GowallModel` to keep a history stack of `ImageBuffer` states).

---

## 4. Crate Dependencies Required

Add these to `Cargo.toml` to support the full A-to-Z Gowall capabilities in GPUI:

```toml
# Web Requests for Daily Wallpapers
reqwest = { version = "0.12", features = ["json"] }

# Machine Learning / AI (Upscaling & Rembg)
ort = "2.0" # ONNX Runtime

# Vector Graphics (Icon Theming)
usvg = "0.41"
resvg = "0.41"

# OCR support
rusty-tesseract = "1.1" 
```

## 5. Next Steps
1. **Bootstrap the GPUI Tab:** Create `src/ui/gowall_tab.rs` and establish the `GowallModel` state.
2. **Implement File I/O:** Setup the GPUI canvas to receive dropped images and render them.
3. **Port Basic Effects:** Start with synchronous `imageops` (Invert, Grayscale) to test the `cx.notify()` loop.
4. **Tackle AI/Async:** Implement OCR and Background Removal using the `background_executor()`.
