//! wallmod — Wallpaper Theme Changer (Ricer Edition)
//!
//! Engineered with modular GPUI + Shadcn architecture where `gpui-component`
//! acts as the centralized design system updating all components globally.

pub mod app;
pub mod backend;
pub mod modules;
pub mod ui;
pub mod wallpaper;

use gpui::*;
use gpui_component::Root;
use ui::WallmodView;

use std::borrow::Cow;

/// Main bootloader launching GPUI desktop application.
fn main() {
    gpui_platform::application().run(move |cx| {
        gpui_component::init(cx);

        let font_bytes = include_bytes!("../fonts/bootstrap-icons.ttf");
        let _ = cx.text_system().add_fonts(vec![Cow::Borrowed(font_bytes)]);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(1200.), px(800.)), cx)),
            titlebar: Some(TitlebarOptions {
                title: Some(SharedString::from("wallmod — ricer edition")),
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.spawn(async move |cx| {
            cx.open_window(window_options, |window, cx| {
                let view = cx.new(|cx| WallmodView::new(cx));
                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}
