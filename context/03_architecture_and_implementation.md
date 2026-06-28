# WallMod Studio — Architecture & Implementation Guide

## Codebase Modular Structure

```
wallmod/
├── Cargo.toml                # Dependencies: iced (0.14), lutgen (0.15), lutgen-palettes, wallpaper, rfd, tokio, image
├── context/                  # Project vision, goals, and documentation
├── src/
│   ├── main.rs               # Modular bootloader launching borderless Iced app
│   ├── app.rs                # Core Elm architecture: WallMod state models, asynchronous workers, update loop
│   ├── wallpaper.rs          # Cross-platform wallpaper application engine (gsettings, swww, swaybg, feh, qdbus)
│   └── ui/                   # Web-style Shadcn modular UI package
│       ├── mod.rs            # Master assembler combining header, sidebar controls, and workspace preview
│       ├── theme.rs          # Centralized Shadcn design tokens (zinc dark mode, colors, border radius, primitives)
│       ├── header.rs         # Window navigation top bar and functional CSD window control buttons
│       ├── sidebar.rs        # Controls pane assembling pick_list, custom palette builder, swatches, and batch actions
│       ├── swatches.rs       # Active shade visualization block rendering color blocks for current palette
│       └── workspace.rs      # Preview pane handling Idle, Loading progress bar, PreviewReady image, or Error cards
```

## Asynchronous Data Pipeline Flow

```
[User Clicks "Open Base Image"]
       │
       ▼
(iced::Command -> rfd::AsyncFileDialog)
       │
       ▼
[Image Decoded & Stats Calculated] ──► (Message::ImageOpened)
       │
       ▼
[User Selects Theme / Adjusts Tweaks]
       │
       ▼
(tokio::task::spawn_blocking -> lutgen GaussianRemapper)
       │  • Generates 8-bit HaldCLUT lookup tables
       │  • Remaps millions of pixels across 12 CPU cores
       ▼
[Processed Handle Returned] ──► (Message::ThemeProcessed)
       │
       ▼
[User Clicks "Set Wallpaper"]
       │
       ▼
(wallpaper crate / Linux Desktop Fallbacks) ──► Implemented instantly via gsettings / DBus / swww
```

## Compilation & Verification

To compile and run the application with maximum optimization:

```bash
# Build production binary
cargo build --release

# Run binary
./target/release/wallmod
```

The compiled binary is completely self-contained (~30 MB) with zero external runtime dependencies required.
