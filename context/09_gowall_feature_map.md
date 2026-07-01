# Gowall Feature Map

Based on the cloned `gowall` source code, this document maps all the available commands, internal operations, and features that need to be exposed in our GPUI frontend.

## 1. Color & Theming Operations (`cmd/color.go`, `cmd/extract.go`)
- **Theme Conversion:** Recolor an image to match presets (Catppuccin, Nord, etc.) or custom themes.
- **Icon Theming:** Apply themes directly to `.svg` and `.ico` files.
- **Color Extraction:** Extract dominant color palettes (like pywal) using k-means/median-cut.
- **Replace Color:** Target a specific color in an image and replace it.

## 2. Image Transformations (`cmd/effects.go`, `cmd/invert.go`, `cmd/pixelate.go`)
- **Basic Effects:** 
  - Grayscale
  - Flip (Vertical/Horizontal)
  - Mirror
  - Brightness / Contrast adjustments
- **Invert:** Mathematically invert all color channels.
- **Pixelate:** Transform high-res images into blocky pixel art.

## 3. Structural Editing (`cmd/resize.go`, `cmd/draw.go`, `cmd/stack.go`)
- **Resize:** Scale images up or down based on pixel dimensions.
- **Draw:** Add grids or borders around an image.
- **Stack:** Combine/stack multiple images together horizontally or vertically.

## 4. Format & Output Operations (`cmd/convert.go`, `cmd/compress.go`, `cmd/gif.go`)
- **Convert:** Change image extensions (e.g., `.webp` to `.png`).
- **Compress:** Reduce file size for `.png`, `.jpg`, `.webp`.
- **GIF Maker:** Take an input sequence of images and compile them into a `.gif` with configurable delay and loops.

## 5. Advanced AI & Processing (`cmd/ocr.go`, `cmd/upscale.go`, `cmd/bg.go`)
- **OCR (Optical Character Recognition):** Extract text from images/PDFs using various providers (Tesseract, LLM APIs).
- **AI Upscaling:** Increase resolution while preserving quality using ONNX models.
- **Background Removal:** Use AI models (U-2-Net / Bria-RMBG via ONNX) to strip backgrounds from subjects.
- **Daily Wallpapers:** Fetch community-curated wallpapers dynamically.
