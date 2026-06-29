# Wallmod Development Context & To-Dos

## Recently Added & Implemented
- **Palette Editor Enhancements**: Added manual hex code typing, copy formats (Raw, JSON).
- **Node Graph UI & Pipeline**: Fixed flexbox bounds breaking for node UI, added all effect options into responsive containers.
- **Retro Bit-Depth Order**: Re-ordered pipeline so bit-depth quantization works properly before palettes are mapped, preventing theme destruction.
- **Header Revamp & Progress**: Restored the loading progress bar and animations to top panel.
- **Favorite Colors UI**: Added a dedicated 'Favorite Colors' tab accessible next to Settings.
- **Tailwind Shades Generator**: Color extractor now automatically creates 11-step (50-950) shade gradations for every dominant color extracted using k-means.
- **Linux Ricer Exporter**: Export output tab now correctly hooks up to `export_icon_theme` for system GTK icon generation.

## Future Plans & Remaining Work
- Complete the standalone Favorite Colors view logic to store and retrieve specific custom shade scales.
- Implement real-time parameter tweaking directly inside node visualizer blocks.
- Add advanced custom palette generation from color extraction.
