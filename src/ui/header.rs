//! Top header navigation component.

use crate::app::SidebarTab;
use crate::ui::WallmodView;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    button::*, h_flex, spinner::Spinner, ActiveTheme, Icon, IconName, Sizable, Selectable, StyledExt,
};

/// Renders the primary application header bar with clean vector icons and no bracketed text.
pub fn render_header(view: &mut WallmodView, cx: &mut Context<WallmodView>) -> impl IntoElement {
    let sidebar_tab = view.app.sidebar_tab;
    let is_dark = view.app.is_dark_mode;

    let is_loading = matches!(view.app.state, crate::app::AppState::Loading(_, _));
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
        .on_mouse_down(MouseButton::Left, |_, window, _| {
            window.start_window_move();
        })
        .child(
            h_flex().gap_2().items_center()
                .child(Icon::new(IconName::Palette).size_5().text_color(cx.theme().primary))
                .child(div().font_bold().text_lg().font_family("bootstrap-icons").child("wallmod"))
                .child(div().text_xs().text_color(cx.theme().muted_foreground).child("ricer edition"))
        )
        .child(
            h_flex().gap_1()
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .child(
                    Button::new("cat_cg")
                        .label("Color Grading")
                        .icon(IconName::Palette)
                        .small()
                        .selected(sidebar_tab == SidebarTab::ColorGrading)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.app.sidebar_tab = SidebarTab::ColorGrading;
                            this.app.option_group_tab = 0;
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
                            this.app.option_group_tab = 0;
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
                            this.app.option_group_tab = 0;
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
                            this.app.option_group_tab = 0;
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
                            this.app.option_group_tab = 0;
                            cx.notify();
                        }))
                )
        )
        .child(
            h_flex().gap_3().items_center()
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .child(
                    h_flex().gap_2().items_center()
                        .when(is_loading, |this| this.child(Spinner::new().small().color(cx.theme().primary)))
                        .child(div().text_xs().text_color(cx.theme().muted_foreground).child(status_str))
                )
                .child(
                    Button::new("btn_dark_toggle")
                        .label(if is_dark { "Light" } else { "Dark" })
                        .icon(if is_dark { IconName::Sun } else { IconName::Moon })
                        .small()
                        .ghost()
                        .on_click(cx.listener(|this, _, window, cx| {
                            let mode = if this.app.is_dark_mode { gpui_component::ThemeMode::Light } else { gpui_component::ThemeMode::Dark };
                            this.app.is_dark_mode = mode.is_dark();
                            gpui_component::Theme::change(mode, Some(window), cx);
                            cx.notify();
                        }))
                )
                .child(div().w_px().h_4().bg(cx.theme().border))
                .child(
                    Button::new("btn_win_min")
                        .icon(IconName::Minus)
                        .small()
                        .ghost()
                        .on_click(|_, window, _| {
                            window.minimize_window();
                        })
                )
                .child(
                    Button::new("btn_win_max")
                        .icon(IconName::Maximize)
                        .small()
                        .ghost()
                        .on_click(|_, window, _| {
                            window.zoom_window();
                        })
                )
                .child(
                    Button::new("btn_win_close")
                        .icon(IconName::Close)
                        .small()
                        .ghost()
                        .on_click(|_, window, _| {
                            window.remove_window();
                        })
                )
        )
}
