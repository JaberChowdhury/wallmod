# Silicon Rust Library Feature Map

The `silicon_fork` crate inside the `wallmod` project provides an advanced engine for rendering source code into beautiful, customizable images. This document maps out all available features and rendering configurations that can be exposed.

## Core Formatting Engine (`ImageFormatterBuilder`)
The core renderer supports:
- **Font Selection** (`.font(Vec<(String, f32)>)`): Bypasses fontconfig if omitted (using embedded Hack). Can load custom fonts (e.g. Fira Code, JetBrains Mono) with specific font sizes if fontconfig is available.
- **Line Offset / Numbers** (`.line_offset(u32)`, `.line_number(bool)`): Optionally renders line numbers on the left gutter, starting at a specific line index.
- **Line Padding** (`.line_pad(u32)`): Adds vertical spacing between individual lines of code for better readability.
- **Window Controls / macOS style** (`.window_controls(bool)`, `.window_title(Option<String>)`): Can render macOS-style red/yellow/green window buttons at the top left of the image, and optionally a centered window title.
- **Corner Rounding** (`.round_corner(bool)`): Rounds the corners of the code editor window (hardcoded to 15px radius in the fork).
- **Tab Width** (`.tab_width(u8)`): Customizes how many spaces a tab character is rendered as.

## Background & Shadow Engine (`ShadowAdder`)
The `ShadowAdder` handles everything outside the code editor window itself:
- **Background Fill** (`.background(Background)`):
  - `Background::Solid(Rgba<u8>)`: Solid hex colors.
  - `Background::Image(RgbaImage)`: Any arbitrary image buffer (allows us to generate linear or radial gradients, or even inject other wallpapers as the background behind the code).
- **Shadow Generation** (`.shadow_color(Rgba<u8>)`, `.blur_radius(f32)`): Creates a drop-shadow effect behind the code window. Both the color and the intensity (sigma/radius) of the blur can be configured.
- **Padding** (`.pad_horiz(u32)`, `.pad_vert(u32)`): Defines the extra background space around the code editor window (the canvas size relative to the code).
- **Offset** (`.offset_x(i32)`, `.offset_y(i32)`): Can translate the code window within the padding space (e.g., positioning it asymmetrically).

## Syntax & Themes (`syntect`)
- Uses `syntect` underneath for parsing TextMate (`.tmTheme`) themes and `sublime-syntax` files.
- Themes determine the inner background color of the code window, as well as syntax highlighting colors.
- Custom themes can be dynamically constructed via `syntect::highlighting::ThemeSet` by parsing XML strings generated at runtime, which allows us to create custom preset color schemes for the code render.
