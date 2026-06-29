//! Centralized GPUI UI Presentation Layer.
//! Refactored into clean modular components following standard software engineering practices.

pub mod header;
pub mod sidebar;
pub mod swatches;
pub mod histogram;
pub mod workspace;

use crate::app::WallmodApp;
use gpui::*;
use gpui_component::{v_flex, h_flex, ActiveTheme, spinner::Spinner, Sizable, StyledExt};

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
        let is_loading = matches!(self.app.state, crate::app::AppState::Loading(_, _));
        let loading_msg = if let crate::app::AppState::Loading(_, ref s) = self.app.state { s.clone() } else { "Processing Image...".to_string() };

        v_flex()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(header::render_header(self, cx))
            .child(
                if is_loading {
                    h_flex()
                        .w_full().h_9().px_4().gap_3().items_center().justify_center()
                        .bg(cx.theme().primary).text_color(cx.theme().primary_foreground)
                        .child(Spinner::new().small())
                        .child(div().text_sm().font_bold().child(format!("⚡ {} (Animated Loading Dot Active)", loading_msg)))
                        .into_any_element()
                } else {
                    div().into_any_element()
                }
            )
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
