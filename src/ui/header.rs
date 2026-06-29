//! Top header navigation component.

use crate::app::SidebarTab;
use crate::ui::WallmodView;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    button::*, h_flex, spinner::Spinner, v_flex, ActiveTheme, Disableable, Selectable, Sizable,
    StyledExt,
};

/// Renders the primary application header bar with clean vector icons and no bracketed text.
pub fn render_header(view: &mut WallmodView, cx: &mut Context<WallmodView>) -> impl IntoElement {
    let sidebar_tab = view.app.sidebar_tab;
    let is_dark = view.app.is_dark_mode;

    let is_loading = matches!(view.app.state, crate::app::AppState::Loading(_, _));
    let status_str = match &view.app.state {
        crate::app::AppState::Idle => "Ready".to_string(),
        crate::app::AppState::Loading(_, s) => s.clone(),
        crate::app::AppState::PreviewReady(_) => "Preview Updated".to_string(),
        crate::app::AppState::Notice(s) => s.clone(),
        crate::app::AppState::Error(e) => format!("Error: {}", e),
    };

    v_flex()
        .w_full()
        .child(
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
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(gpui::svg().path("logo.svg").size_5().text_color(cx.theme().primary))
                        .child(div().font_bold().text_lg().child("wallmod"))
                        .child(
                            div()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child("ricer edition"),
                        ),
                )
                .child(
                    h_flex()
                        .gap_3()
                        .items_center()
                        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                        .child(
                            h_flex()
                                .gap_2()
                                .items_center()
                                .when(is_loading, |this| {
                                    this.child(Spinner::new().small().color(cx.theme().primary))
                                })
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(cx.theme().muted_foreground)
                                        .child(status_str),
                                )
                                .when(is_loading, |this| {
                                    let p = if let crate::app::AppState::Loading(p, _) =
                                        &view.app.state
                                    {
                                        *p
                                    } else {
                                        0.0
                                    };
                                    this.child(
                                        div().w(px(100.0)).flex().items_center().child(
                                            gpui_component::progress::Progress::new("hdr_prog")
                                                .value(p * 100.0)
                                                .loading(true),
                                        ),
                                    )
                                }),
                        )
                        .child(
                            Button::new("btn_dark_toggle")
                                .disabled(is_loading)
                                .child(if is_dark {
                                    gpui::svg()
                                         .path("sun.svg")
                                        .size_5()
                                        .text_color(cx.theme().primary)
                                } else {
                                    gpui::svg()
                                        .path("moon.svg")
                                        .size_5()
                                        .text_color(cx.theme().primary)
                                })
                                // .label(if is_dark {
                                //     "Light"
                                // } else {
                                //     "Dark"
                                // })
                                .p_1()
                                .rounded_md()
                                .cursor_pointer()
                                // .hover(|s| s.bg(cx.theme().secondary))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    let mode = if this.app.is_dark_mode {
                                        gpui_component::ThemeMode::Light
                                    } else {
                                        gpui_component::ThemeMode::Dark
                                    };
                                    this.app.is_dark_mode = mode.is_dark();
                                    gpui_component::Theme::change(mode, Some(window), cx);
                                    cx.notify();
                                })),
                        )
                        .child(div().w_px().h_4().bg(cx.theme().border))
                        .child(
                            div()
                                .id("btn_win_min")
                                .p_1()
                                .rounded_md()
                                .cursor_pointer()
                                .hover(|s| s.bg(cx.theme().secondary))
                                .on_mouse_down(MouseButton::Left, |_, window, _| {
                                    window.minimize_window();
                                })
                                .child(
                                    gpui::svg()
                                        .path("minus.svg")
                                        .size_5()
                                        .text_color(cx.theme().primary),
                                ), // .child(
                                   //     Icon::new(IconName::WindowMinimize)
                                   //         .size_4()
                                   //         .text_color(gpui::black()),
                                   // ),
                        )
                        .child(
                            div()
                                .id("btn_win_max")
                                .p_1()
                                .rounded_md()
                                .cursor_pointer()
                                .hover(|s| s.bg(cx.theme().secondary))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _, window, cx| {
                                        if this.app.is_zoomed {
                                            window.zoom_window();
                                            this.app.is_zoomed = false;
                                        } else {
                                            window.zoom_window();
                                            this.app.is_zoomed = true;
                                        }
                                        cx.notify();
                                    }),
                                )
                                .child(
                                    gpui::svg()
                                        .path(if view.app.is_zoomed {
                                            "window-restore.svg"
                                        } else {
                                            "frame.svg"
                                        })
                                        .size_5()
                                        .text_color(cx.theme().primary),
                                ), // .child(
                                   //     Icon::new(IconName::WindowMaximize)
                                   //         .size_4()
                                   //         .text_color(gpui::black()),
                                   // ),
                        )
                        .child(
                            div()
                                .id("btn_win_close")
                                .p_1()
                                .rounded_md()
                                .cursor_pointer()
                                .hover(|s| s.bg(cx.theme().secondary))
                                .on_mouse_down(MouseButton::Left, |_, window, _| {
                                    window.remove_window();
                                })
                                .child(
                                    gpui::svg()
                                        .path("close.svg")
                                        .size_5()
                                        .text_color(cx.theme().primary),
                                ), // .child(
                                   //     Icon::new(IconName::WindowClose)
                                   //         .size_4()
                                   //         .text_color(gpui::black()),
                                   // ),
                        ),
                ),
        )
        .child(
            h_flex()
                .w_full()
                .h_10()
                .px_4()
                .border_b_1()
                .border_color(cx.theme().border)
                .items_center()
                .gap_1()
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .child(
                    Button::new("cat_cg")
                        .child(
                            gpui::svg().path("palette.svg").size_4().text_color(
                                if sidebar_tab == SidebarTab::ColorGrading {
                                    cx.theme().secondary
                                } else {
                                    cx.theme().primary
                                }
                            ),
                        )
                        .child("Color Grading")
                        .small()
                        .cursor_pointer()
                        .selected(sidebar_tab == SidebarTab::ColorGrading)
                        .when(sidebar_tab == SidebarTab::ColorGrading, |this| this.primary())
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.app.sidebar_tab = SidebarTab::ColorGrading;
                            this.app.option_group_tab = 0;
                            cx.notify();
                        })),
                )
                .child(
                    Button::new("cat_ps")
                        .child(
                            gpui::svg()
                                .path("settings.svg")
                                .size_4()
                                .text_color(
                                    if sidebar_tab == SidebarTab::PhotoshopEffects {
                                        cx.theme().secondary
                                    } else {
                                        cx.theme().primary
                                    }
                                ),
                        )
                        .child("Adjust & Effects")
                        .small()
                        .cursor_pointer()
                        .selected(sidebar_tab == SidebarTab::PhotoshopEffects)
                        .when(sidebar_tab == SidebarTab::PhotoshopEffects, |this| this.primary())
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.app.sidebar_tab = SidebarTab::PhotoshopEffects;
                            this.app.option_group_tab = 0;
                            cx.notify();
                        })),
                )
                .child(
                    Button::new("cat_eng")
                        .child(
                            gpui::svg()
                                .path("panel-left.svg")
                                .size_4()
                                .text_color(
                                    if sidebar_tab == SidebarTab::DesktopEngine {
                                        cx.theme().secondary
                                    } else {
                                        cx.theme().primary
                                    }
                                ),
                        )
                        .child("Wallpaper Engine")
                        .small()
                        .cursor_pointer()
                        .selected(sidebar_tab == SidebarTab::DesktopEngine)
                        .when(sidebar_tab == SidebarTab::DesktopEngine, |this| this.primary())
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.app.sidebar_tab = SidebarTab::DesktopEngine;
                            this.app.option_group_tab = 0;
                            cx.notify();
                        })),
                )
                .child(
                    Button::new("cat_exp")
                        .child(
                            gpui::svg().path("replace.svg").size_4().text_color(
                                if sidebar_tab == SidebarTab::ExportSync {
                                    cx.theme().secondary
                                } else {
                                    cx.theme().primary
                                }
                            ),
                        )
                        .child("Export & Sync")
                        .small()
                        .cursor_pointer()
                        .selected(sidebar_tab == SidebarTab::ExportSync)
                        .when(sidebar_tab == SidebarTab::ExportSync, |this| this.primary())
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.app.sidebar_tab = SidebarTab::ExportSync;
                            this.app.option_group_tab = 0;
                            cx.notify();
                        })),
                )
                .child(
                    Button::new("cat_ai")
                        .child(
                            gpui::svg().path("search.svg").size_4().text_color(
                                if sidebar_tab == SidebarTab::ToolsExt {
                                    cx.theme().secondary
                                } else {
                                    cx.theme().primary
                                }
                            ),
                        )
                        .child("AI & Tools")
                        .small()
                        .cursor_pointer()
                        .selected(sidebar_tab == SidebarTab::ToolsExt)
                        .when(sidebar_tab == SidebarTab::ToolsExt, |this| this.primary())
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.app.sidebar_tab = SidebarTab::ToolsExt;
                            this.app.option_group_tab = 0;
                            cx.notify();
                        })),
                )
                .child(
                    Button::new("cat_settings")
                        .child(
                            gpui::svg()
                                .path("settings.svg")
                                .size_4()
                                .text_color(
                                    if sidebar_tab == SidebarTab::Settings {
                                        cx.theme().secondary
                                    } else {
                                        cx.theme().primary
                                    }
                                ),
                        )
                        .child("Settings")
                        .small()
                        .cursor_pointer()
                        .selected(sidebar_tab == SidebarTab::Settings)
                        .when(sidebar_tab == SidebarTab::Settings, |this| this.primary())
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.app.sidebar_tab = SidebarTab::Settings;
                            this.app.option_group_tab = 0;
                            cx.notify();
                        })),
                )
                .child(
                    Button::new("cat_favs")
                        .child(
                            gpui::svg().path("heart.svg").size_4().text_color(
                                if sidebar_tab == SidebarTab::FavoriteColors {
                                    cx.theme().secondary
                                } else {
                                    cx.theme().primary
                                }
                            ),
                        )
                        .child("Favorite Colors")
                        .small()
                        .cursor_pointer()
                        .selected(sidebar_tab == SidebarTab::FavoriteColors)
                        .when(sidebar_tab == SidebarTab::FavoriteColors, |this| this.primary())
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.app.sidebar_tab = SidebarTab::FavoriteColors;
                            this.app.option_group_tab = 0;
                            cx.notify();
                        })),
                )
                .child(
                    Button::new("btn_toggle_float_stats")
                        .child(gpui::svg().path("duck.svg").size_4().text_color(
                            if view.app.show_floating_stats {
                                cx.theme().secondary
                            } else {
                                cx.theme().primary
                            },
                        ))
                        .child("Floating Stats")
                        .small()
                        .selected(view.app.show_floating_stats)
                        .when(view.app.show_floating_stats, |this| this.primary())
                        .cursor_pointer()
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.app.show_floating_stats = !this.app.show_floating_stats;
                            cx.notify();
                        })),
                ),
        )
}
