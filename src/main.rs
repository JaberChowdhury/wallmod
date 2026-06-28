//! wallmod — Wallpaper Theme Changer (Ricer Edition)
//!
//! Engineered with modular Shadcn UI architecture where `src/ui/theme.rs`
//! acts as the centralized design system updating all components globally.

pub mod app;
pub mod backend;
pub mod ui;
pub mod wallpaper;

use app::WallmodApp;

/// Main bootloader launching borderless Iced application.
pub fn main() -> iced::Result {
    iced::application(WallmodApp::boot, WallmodApp::update, WallmodApp::view)
        .title("wallmod — ricer edition")
        .theme(WallmodApp::theme)
        .window(iced::window::Settings {
            decorations: false,
            ..Default::default()
        })
        .run()
}
