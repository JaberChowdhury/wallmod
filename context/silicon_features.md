# Silicon Rust Library Comprehensive Feature Map

The `silicon_fork` crate inside the `wallmod` project provides an advanced engine for rendering source code into beautiful, customizable images. This document comprehensively maps out **all** available features, modules, and rendering configurations exposed by the library.

## Core Formatting Engine (`ImageFormatterBuilder` / `ImageFormatter`)
The `ImageFormatter` handles parsing the highlighted code and generating the inner window layout.
- **Font Selection (`.font(Vec<(String, f32)>)`)**: Customizes the font family and size. Uses `zed-font-kit` internally to discover system fonts and `HarfBuzz` for text shaping and ligatures.
- **Line Offset (`.line_offset(u32)`)**: Sets the starting line index for the gutter (e.g., starting at line 10).
- **Line Numbers (`.line_number(bool)`)**: Toggles the rendering of line numbers in the left gutter.
- **Line Padding (`.line_pad(u32)`)**: Adds vertical spacing (in pixels) between individual lines of code for better readability.
- **Code Right Padding (`.code_pad_right(u32)`)**: Adds extra horizontal spacing on the right side of the editor window.
- **Window Controls (`.window_controls(bool)`)**: Renders macOS-style (red, yellow, green) window buttons at the top left of the image.
- **Window Title (`.window_title(Option<String>)`](.window_title(Option<String>))**: Renders a centered title string inside the title bar (only visible if window controls are enabled or enough padding exists).
- **Corner Rounding (`.round_corner(bool)`)**: Rounds the outer corners of the code editor window.
- **Tab Width (`.tab_width(u8)`)**: Customizes how many spaces a tab character is rendered as.
- **Line Highlighting (`.highlight_lines(Vec<u32>)`](.highlight_lines(Vec<u32>))**: Highlights specific lines by drawing a subtle background highlight block behind the code on those lines.
- **Shadow Adder Attachment (`.shadow_adder(ShadowAdder)`)**: Binds a configured `ShadowAdder` to handle the outer canvas rendering.

## Background & Shadow Engine (`ShadowAdder`)
The `ShadowAdder` handles everything outside the code editor window (canvas resizing, drop shadows, and backgrounds).
- **Background Fill (`.background(Background)`)**:
  - `Background::Solid(Rgba<u8>)`: Solid hex colors.
  - `Background::Image(RgbaImage)`: Arbitrary image buffers. The image is automatically resized (`FilterType::Triangle`) to fit the canvas, making it ideal for rendering linear/radial gradients or using wallpapers.
- **Shadow Color (`.shadow_color(Rgba<u8>)`](.shadow_color(Rgba<u8>))**: Configures the drop-shadow's tint (defaults to black with some transparency).
- **Blur Radius (`.blur_radius(f32)`)**: Configures the gaussian blur intensity (sigma) of the drop shadow.
- **Padding (`.pad_horiz(u32)`, `.pad_vert(u32)`)**: Defines the outer canvas size relative to the code editor window, expanding the background space.
- **Offset (`.offset_x(i32)`, `.offset_y(i32)`)**: Translates the drop shadow independently from the code window, creating directional lighting effects.

## Syntax & Themes (`syntect` / `HighlightingAssets`)
- Uses `syntect` for parsing TextMate (`.tmTheme`) XML themes and `sublime-syntax` files.
- `HighlightingAssets`: Manages a global cache of syntax files and themes. It can serialize and dump these assets into binary files using `bincode` and `flate2` for extremely fast startup times (`.dump_to_file()`, `.from_dump_file()`).
- Themes determine the inner background color of the code window and all syntax highlighting tokens.
- Custom themes can be dynamically constructed via `syntect::highlighting::ThemeSet` by parsing XML strings generated at runtime (allowing dynamic custom presets).

## Text Shaping & Rendering (`font.rs` & `hb_wrapper.rs`)
- **HarfBuzz Integration**: Wraps HarfBuzz C-bindings (`hb_buffer`, `hb_font`) to accurately shape text, process kerning, and apply complex ligatures (e.g., Fira Code ligatures).
- **Font Fallbacks**: Manages a `FontCollection` that falls back safely if a glyph is missing, utilizing `FontStyle` (Regular, Bold, Italic) properly.

## Utilities & Clipboard (`utils.rs` & `directories.rs`)
- **Cross-Platform Clipboard**: `dump_image_to_clipboard` exposes functionality to write raw `DynamicImage` buffers directly to the OS clipboard, implementing specific native code for X11/Wayland, macOS, and Windows.
- **Configuration Directories**: Uses the `directories` crate to resolve XDG Base Directory standard paths for caching assets (`~/.cache/silicon`) and configuration files (`~/.config/silicon`).
