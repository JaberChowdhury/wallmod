//! Top header navigation component.

use crate::app::SidebarTab;
use crate::ui::WallmodView;
use gpui::*;
use gpui_component::{
    button::*, h_flex, ActiveTheme, Icon, IconName, Sizable, Selectable, StyledExt,
};

/// Renders the primary application header bar with clean vector icons and no bracketed text.
pub fn render_header(view: &mut WallmodView, cx: &mut Context<WallmodView>) -> impl IntoElement {
    let sidebar_tab = view.app.sidebar_tab;
    let is_dark = view.app.is_dark_mode;

    let status_str = match &view.app.state {
        crate::app::AppState::Idle => "Ready".to_string(),
        crate::app::AppState::Loading(p, s) => format!("{} ({:.0}%)", s, p * 100.0),
        crate::app::AppState::PreviewReady(_) => "Preview Updated".to_string(),
        crate::app::AppState::Notice(s) => s.clone(),
        crate::app::AppState::Error(e) => format!("Error: {}", e),
    };

    h_flex()
        .w_full()
        .h_12()
        .px_4()
        .border_b_1()
        .border_color(cx.theme().border)
        .items_center()
        .justify_between()
        .child(
            h_flex().gap_2().items_center()
                .child(Icon::new(IconName::Palette).size_5().text_color(cx.theme().primary))
                .child(div().font_bold().text_lg().font_family("bootstrap-icons").child("wallmod"))
                .child(div().text_xs().text_color(cx.theme().muted_foreground).child("ricer edition"))
        )
        .child(
            h_flex().gap_1()
                .child(
                    Button::new("cat_cg")
                        .label("Color Grading")
                        .icon(IconName::Palette)
                        .small()
                        .selected(sidebar_tab == SidebarTab::ColorGrading)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.app.sidebar_tab = SidebarTab::ColorGrading;
                            cx.notify();
                        }))
                )
                .child(
                    Button::new("cat_ps")
                        .label("Adjust & Effects")
                        .icon(IconName::Settings)
                        .small()
                        .selected(sidebar_tab == SidebarTab::PhotoshopEffects)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.app.sidebar_tab = SidebarTab::PhotoshopEffects;
                            cx.notify();
                        }))
                )
                .child(
                    Button::new("cat_eng")
                        .label("Wallpaper Engine")
                        .icon(IconName::PanelLeft)
                        .small()
                        .selected(sidebar_tab == SidebarTab::DesktopEngine)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.app.sidebar_tab = SidebarTab::DesktopEngine;
                            cx.notify();
                        }))
                )
                .child(
                    Button::new("cat_exp")
                        .label("Export & Sync")
                        .icon(IconName::Replace)
                        .small()
                        .selected(sidebar_tab == SidebarTab::ExportSync)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.app.sidebar_tab = SidebarTab::ExportSync;
                            cx.notify();
                        }))
                )
                .child(
                    Button::new("cat_ai")
                        .label("AI & Tools")
                        .icon(IconName::Search)
                        .small()
                        .selected(sidebar_tab == SidebarTab::ToolsExt)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.app.sidebar_tab = SidebarTab::ToolsExt;
                            cx.notify();
                        }))
                )
        )
        .child(
            h_flex().gap_3().items_center()
                .child(div().text_xs().text_color(cx.theme().muted_foreground).child(status_str))
                .child(
                    Button::new("btn_dark_toggle")
                        .label(if is_dark { "Light Mode" } else { "Dark Mode" })
                        .small()
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.app.is_dark_mode = !this.app.is_dark_mode;
                            cx.notify();
                        }))
                )
        )
}
