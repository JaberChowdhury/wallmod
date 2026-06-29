# WallMod Studio — Project Introduction

## Executive Summary

**WallMod Studio (Ricer Edition)** is a premier, industry-grade desktop application built in Rust. It merges the capabilities of three acclaimed open-source ricing repositories into a unified, lightning-fast studio:

1. **[lutgen-rs](https://github.com/ozwaldorf/lutgen-rs)**: Blazing fast HaldCLUT color grading and radial basis function (RBF) palette interpolation.
2. **[wallrust](https://github.com/prime-run/wallrust)** / **[wallpaper](https://crates.io/crates/wallpaper)**: Universal, cross-platform desktop wallpaper application engine supporting Linux window managers and desktop environments.
3. **[imagineer](https://github.com/foresterre/imagineer)**: High-speed image inspection, luminance calculation, and rich preview layouts.

## The Vision

Linux desktop ricing enthusiasts ("ricers") demand immediate, pixel-perfect visual feedback when tweaking desktop themes, terminal color matrices, and wallpapers. Traditional theming tools either rely on slow command-line scripts or bloated web-based Electron wrappers.

WallMod Studio solves this by providing a native GUI built on **[Iced 0.14](https://github.com/iced-rs/iced)** powered by asynchronous multi-threaded image processing via **Tokio**. It allows users to fuzzy-search themes, inspect 16-color ANSI terminal matrices, fine-tune mathematical grading parameters, and preview how generated palettes interact with simulated tiling window managers—all at a smooth 60 FPS.

<!--agy --conversation=bef77d4a-f56f-49ef-b152-36d4d1a188b2-->
