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

use std::fs;
use std::path::PathBuf;
use anyhow::Result;

struct AppAssets {
    base: PathBuf,
}

impl AssetSource for AppAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        fs::read(self.base.join(path))
            .map(|data| Some(Cow::Owned(data)))
            .map_err(|err| err.into())
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.file_name().into_string().ok())
                            .map(SharedString::from)
                    })
                    .collect()
            })
            .map_err(|err| err.into())
    }
}

/// Main bootloader launching GPUI desktop application.
fn main() {
    let assets = AppAssets {
        base: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets"),
    };

    gpui_platform::application()
        .with_assets(assets)
        .run(move |cx| {
            gpui_component::init(cx);

            let font_bytes = include_bytes!("../fonts/bootstrap-icons.ttf");
            let _ = cx.text_system().add_fonts(vec![Cow::Borrowed(font_bytes)]);

            let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(1200.), px(800.)), cx)),
            titlebar: None,
            window_decorations: Some(WindowDecorations::Client),
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
