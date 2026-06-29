//! Centralized GPUI UI Presentation Layer.
//! Refactored into clean modular components following standard software engineering practices.

pub mod header;
pub mod sidebar;
pub mod swatches;
pub mod histogram;
pub mod workspace;

use crate::app::WallmodApp;
use gpui::*;
use gpui_component::{v_flex, h_flex, ActiveTheme};

pub struct WallmodView {
    pub app: WallmodApp,
}

impl WallmodView {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self { app: WallmodApp::new() }
    }
}

impl Render for WallmodView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(header::render_header(self, cx))
            .child(
                h_flex()
                    .flex_1()
                    .w_full()
                    .overflow_hidden()
                    .child(sidebar::render_sidebar(self, cx))
                    .child(workspace::render_workspace(self, cx))
            )
    }
}
