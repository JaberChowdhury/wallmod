//! Centralized GPUI Shadcn UI Presentation Layer.
//! Separated completely from core business models.

use crate::app::{WallmodApp, RemapAlgorithm, SidebarTab, AppTab, PRESET_NAMES};
use gpui::*;
use gpui_component::{
    button::*, switch::*, scroll::ScrollableElement,
    v_flex, h_flex, ActiveTheme, Icon, IconName, Sizable, Selectable, StyledExt,
};
use std::path::PathBuf;

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
        let active_tab = self.app.active_tab;
        let sidebar_tab = self.app.sidebar_tab;
        let preview_path = self.app.preview_path.clone();
        let selected_preset = self.app.selected_preset.clone();
        let algo = self.app.algorithm;
        let luma = self.app.preserve_luma;
        let sync_a = self.app.sync_alacritty;
        let sync_k = self.app.sync_kitty;
        let status_str = match &self.app.state {
            crate::app::AppState::Idle => "Ready".to_string(),
            crate::app::AppState::Loading(p, s) => format!("{} ({:.0}%)", s, p * 100.0),
            crate::app::AppState::PreviewReady(_) => "Preview Updated".to_string(),
            crate::app::AppState::Notice(s) => s.clone(),
            crate::app::AppState::Error(e) => format!("Error: {}", e),
        };

        v_flex()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(
                // Header / Top Bar
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
                            .child(div().font_bold().text_lg().child("wallmod"))
                            .child(div().text_sm().text_color(cx.theme().muted_foreground).child("— ricer edition"))
                    )
                    .child(
                        h_flex().gap_1()
                            .child(
                                Button::new("tab_themer").label("Themer")
                                    .selected(active_tab == AppTab::Themer)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.app.active_tab = AppTab::Themer;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("tab_upscaler").label("Upscaler")
                                    .selected(active_tab == AppTab::Upscaler)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.app.active_tab = AppTab::Upscaler;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("tab_ocr").label("OCR Engine")
                                    .selected(active_tab == AppTab::Ocr)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.app.active_tab = AppTab::Ocr;
                                        cx.notify();
                                    }))
                            )
                    )
                    .child(
                        h_flex().gap_2().items_center()
                            .child(div().text_xs().text_color(cx.theme().muted_foreground).child(status_str))
                    )
            )
            .child(
                // Body Content Splitter
                h_flex()
                    .flex_1()
                    .w_full()
                    .overflow_hidden()
                    .child(
                        // Left Sidebar
                        v_flex()
                            .w(px(320.0))
                            .h_full()
                            .border_r_1()
                            .border_color(cx.theme().border)
                            .p_4()
                            .gap_4()
                            .child(
                                h_flex().gap_1().w_full().justify_between()
                                    .child(Button::new("sb_theme").label("Palettes").small().selected(sidebar_tab == SidebarTab::ThemeLut).on_click(cx.listener(|this, _, _, cx| {
                                        this.app.sidebar_tab = SidebarTab::ThemeLut;
                                        cx.notify();
                                    })))
                                    .child(Button::new("sb_eng").label("Engine").small().selected(sidebar_tab == SidebarTab::DesktopEngine).on_click(cx.listener(|this, _, _, cx| {
                                        this.app.sidebar_tab = SidebarTab::DesktopEngine;
                                        cx.notify();
                                    })))
                                    .child(Button::new("sb_exp").label("Export").small().selected(sidebar_tab == SidebarTab::ExportSync).on_click(cx.listener(|this, _, _, cx| {
                                        this.app.sidebar_tab = SidebarTab::ExportSync;
                                        cx.notify();
                                    })))
                            )
                            .child(div().h_px().w_full().bg(cx.theme().border))
                            .child(
                                match sidebar_tab {
                                    SidebarTab::ThemeLut => {
                                        v_flex().gap_3().w_full().flex_1().overflow_y_scrollbar()
                                            .child(
                                                Button::new("btn_pick_img").label("Open Wallpaper Image...")
                                                    .primary()
                                                    .w_full()
                                                    .on_click(cx.listener(|_, _, _, cx| {
                                                        cx.spawn(async move |this, cx| {
                                                            if let Some(file) = rfd::AsyncFileDialog::new().add_filter("Image", &["png", "jpg", "jpeg", "webp"]).pick_file().await {
                                                                let path = file.path().to_path_buf();
                                                                if let Ok(dyn_img) = tokio::task::spawn_blocking(move || image::open(&path)).await.unwrap() {
                                                                    let _ = this.update(cx, |view, cx| {
                                                                        view.app.on_image_selected(file.path().to_path_buf(), dyn_img);
                                                                        cx.notify();
                                                                    });
                                                                }
                                                            }
                                                        }).detach();
                                                    }))
                                            )
                                            .child(div().text_sm().font_bold().child("Color Presets"))
                                            .children(PRESET_NAMES.iter().take(6).map(|&name| {
                                                let is_sel = selected_preset.as_deref() == Some(name);
                                                let n = name.to_string();
                                                Button::new(SharedString::from(format!("p_{}", name))).label(name)
                                                    .w_full()
                                                    .small()
                                                    .selected(is_sel)
                                                    .on_click(cx.listener(move |this, _, _, cx| {
                                                        this.app.selected_preset = Some(n.clone());
                                                        this.app.current_theme = crate::app::ThemeSource::Preset(n.clone());
                                                        let _ = this.app.run_processing();
                                                        cx.notify();
                                                    }))
                                            }))
                                            .child(div().text_sm().font_bold().pt_2().child("Remap Algorithm"))
                                            .child(
                                                h_flex().gap_1().w_full()
                                                    .child(Button::new("alg_g").label("Gauss").small().selected(algo == RemapAlgorithm::Gaussian).on_click(cx.listener(|this, _, _, cx| {
                                                        this.app.algorithm = RemapAlgorithm::Gaussian; let _ = this.app.run_processing();
                                                        cx.notify();
                                                    })))
                                                    .child(Button::new("alg_s").label("Shepard").small().selected(algo == RemapAlgorithm::Shepard).on_click(cx.listener(|this, _, _, cx| {
                                                        this.app.algorithm = RemapAlgorithm::Shepard; let _ = this.app.run_processing();
                                                        cx.notify();
                                                    })))
                                                    .child(Button::new("alg_n").label("Nearest").small().selected(algo == RemapAlgorithm::NearestNeighbor).on_click(cx.listener(|this, _, _, cx| {
                                                        this.app.algorithm = RemapAlgorithm::NearestNeighbor; let _ = this.app.run_processing();
                                                        cx.notify();
                                                    })))
                                            )
                                            .child(
                                                h_flex().items_center().justify_between().pt_2()
                                                    .child(div().text_sm().child("Preserve Luma"))
                                                    .child(Switch::new("sw_luma").checked(luma).on_click(cx.listener(|this, val: &bool, _, cx| {
                                                        this.app.preserve_luma = *val; let _ = this.app.run_processing();
                                                        cx.notify();
                                                    })))
                                            )
                                            .into_any_element()
                                    }
                                    SidebarTab::DesktopEngine => {
                                        v_flex().gap_3().w_full()
                                            .child(div().text_sm().font_bold().child("Wallpaper Backend"))
                                            .child(div().text_xs().text_color(cx.theme().muted_foreground).child("Universal SWWW / SwayBG / FEH Engine"))
                                            .child(
                                                Button::new("btn_apply_wp").label("Apply to Desktop Now")
                                                    .primary()
                                                    .w_full()
                                                    .on_click(cx.listener(|this, _, _, _| {
                                                        if let Some(ref path) = this.app.preview_path {
                                                            let p = path.clone();
                                                            std::process::Command::new("feh").arg("--bg-fill").arg(&p).spawn().ok();
                                                            std::process::Command::new("swww").arg("img").arg(&p).spawn().ok();
                                                        }
                                                    }))
                                            )
                                            .into_any_element()
                                    }
                                    SidebarTab::ExportSync => {
                                        v_flex().gap_3().w_full()
                                            .child(div().text_sm().font_bold().child("Terminal Sync"))
                                            .child(
                                                h_flex().items_center().justify_between()
                                                    .child(div().text_sm().child("Sync Alacritty"))
                                                    .child(Switch::new("sw_alac").checked(sync_a).on_click(cx.listener(|this, val: &bool, _, cx| {
                                                        this.app.sync_alacritty = *val;
                                                        cx.notify();
                                                    })))
                                            )
                                            .child(
                                                h_flex().items_center().justify_between()
                                                    .child(div().text_sm().child("Sync Kitty"))
                                                    .child(Switch::new("sw_kitty").checked(sync_k).on_click(cx.listener(|this, val: &bool, _, cx| {
                                                        this.app.sync_kitty = *val;
                                                        cx.notify();
                                                    })))
                                            )
                                            .child(
                                                Button::new("btn_exp_term").label("Export Configs to Folder...")
                                                    .w_full()
                                                    .on_click(cx.listener(|this, _, _, _| {
                                                        if let Ok(home) = std::env::var("HOME") {
                                                            let _ = this.app.export_terminal_scheme(&PathBuf::from(home));
                                                        }
                                                    }))
                                            )
                                            .into_any_element()
                                    }
                                }
                            )
                    )
                    .child(
                        // Main Workspace Preview Area
                        v_flex()
                            .flex_1()
                            .h_full()
                            .p_6()
                            .items_center()
                            .justify_center()
                            .child(
                                if let Some(path) = preview_path {
                                    div()
                                        .size_full()
                                        .border_1()
                                        .border_color(cx.theme().border)
                                        .rounded_lg()
                                        .overflow_hidden()
                                        .child(img(path).size_full().object_fit(ObjectFit::Contain))
                                        .into_any_element()
                                } else {
                                    v_flex().gap_4().items_center().justify_center().p_12().border_1().border_color(cx.theme().border).rounded_xl().bg(cx.theme().secondary)
                                        .child(Icon::new(IconName::Folder).size_12().text_color(cx.theme().muted_foreground))
                                        .child(div().text_lg().font_bold().child("No Image Loaded"))
                                        .child(div().text_sm().text_color(cx.theme().muted_foreground).child("Select a wallpaper from the left sidebar to begin color grading."))
                                        .into_any_element()
                                }
                            )
                    )
            )
    }
}
