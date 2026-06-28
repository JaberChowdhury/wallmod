# WallMod Studio — Core Goals & Requirements

## Core Architectural Mandates
1. **Industry-Grade Engineering**: Avoid monolithic single-file implementations. Cleanly decouple application state, asynchronous worker tasks, UI components, and desktop integration into specialized modules.
2. **Ultra-Fast Performance**: All heavy image transformations (HaldCLUT generation, Gaussian RBF interpolation, pixel remapping) must execute asynchronously on background threads without locking or stuttering the GUI main thread.
3. **Pure Rust Stack**: Leverage native OS dialogues via `rfd` and native windowing via `winit`/`iced` without requiring GTK or Qt runtime dependencies.

## Design & UI/UX Requirements (Ricer Edition)

### 1. Keyboard-First Command Palette
- Center the interface around a prominent top search bar (`text_input`) reminiscent of Raycast, KRunner, or terminal search tools.
- Typing dynamically fuzzy-filters the list of color palettes and studio commands in real time.

### 2. Split-Pane Workspace
- A responsive flexbox layout separating navigation (`Row`/`Column` proportions 2:8).
- Left pane: Collapsible vertical sidebar with styled navigation tabs and filtered theme cards.
- Right pane: Expansive reading/editing workspace dedicated to inspection and previewing.

### 3. Custom Client-Side Decorations (CSD)
- Configure borderless windows (`decorations: false`).
- Render custom branding, logo, command bar, action buttons, and mock window control buttons (close/minimize/maximize colored dots) embedded directly into the top navigation bar.

### 4. Ricer-Centric Workspace Views
- **🖥️ Live-Emulation Canvas**: A simulated minimal Linux desktop featuring a mock Polybar/Waybar status bar, an active window border colored dynamically by the primary theme accent, and a mock terminal window running `fastfetch`/`neofetch` with live color blocks.
- **📟 Split ANSI Matrix**: A structured 16-color grid (8 Base colors 0..7, 8 Bright colors 8..15). Clicking any color block inspects and displays its exact Hex (`#RRGGBB`) and RGB coordinates.
- **⚙️ Backend Tweak Dashboard**: Mathematical sliders allowing users to adjust Gaussian RBF shape spread factors, luminance preservation weights, and contrast enhancement multipliers.
- **🖼️ Side-by-Side Diff**: Classic before/after comparison with execution timing and luminance analytics.
