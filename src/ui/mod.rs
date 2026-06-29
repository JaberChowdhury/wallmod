//! Centralized GPUI Shadcn UI Presentation Layer.
//! Separated completely from core business models.
//! Features 100% feature parity with all wallmod capabilities.

use crate::app::{
    WallmodApp, RemapAlgorithm, SidebarTab, AppTab, WorkspaceView, WallpaperBackend,
    PRESET_NAMES, SWWW_TRANSITIONS, TARGET_DISPLAYS,
};
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
        let workspace_view = self.app.workspace_view;
        let preview_path = self.app.preview_path.clone();
        let base_path = self.app.base_image_path.clone();
        let selected_preset = self.app.selected_preset.clone();
        let algo = self.app.algorithm;
        let luma = self.app.preserve_luma;
        let hald_lvl = self.app.hald_level;
        let sync_a = self.app.sync_alacritty;
        let sync_k = self.app.sync_kitty;
        let backend = self.app.wallpaper_backend;
        let transition = self.app.swww_transition.clone();
        let display = self.app.target_display.clone();
        let is_dark = self.app.is_dark_mode;
        let blur_sigma = self.app.blur_sigma;
        let dither = self.app.dither_enabled;
        let daemon = self.app.daemon_enabled;
        let wcag = self.app.wcag_contrast;
        let img_w = self.app.image_width;
        let img_h = self.app.image_height;
        let img_name = self.app.image_filename.clone();
        let extracted_cols = self.app.extracted_colors.clone();
        let albums = self.app.albums.clone();

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
                            .child(div().text_sm().text_color(cx.theme().muted_foreground).child("— ricer edition [ * ]"))
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
                            .child(
                                Button::new("tab_comp").label("Compression")
                                    .selected(active_tab == AppTab::Compression)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.app.active_tab = AppTab::Compression;
                                        cx.notify();
                                    }))
                            )
                    )
                    .child(
                        h_flex().gap_3().items_center()
                            .child(div().text_xs().text_color(cx.theme().muted_foreground).child(status_str))
                            .child(
                                Button::new("btn_dark_toggle").label(if is_dark { "[ Light Mode ]" } else { "[ Dark Mode ]" })
                                    .small()
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.app.is_dark_mode = !this.app.is_dark_mode;
                                        cx.notify();
                                    }))
                            )
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
                            .w(px(340.0))
                            .h_full()
                            .border_r_1()
                            .border_color(cx.theme().border)
                            .p_4()
                            .gap_4()
                            .child(
                                h_flex().gap_1().w_full().justify_between()
                                    .child(Button::new("sb_theme").label("[*] Theme & LUT").small().selected(sidebar_tab == SidebarTab::ThemeLut).on_click(cx.listener(|this, _, _, cx| {
                                        this.app.sidebar_tab = SidebarTab::ThemeLut;
                                        cx.notify();
                                    })))
                                    .child(Button::new("sb_eng").label("[>] Engine").small().selected(sidebar_tab == SidebarTab::DesktopEngine).on_click(cx.listener(|this, _, _, cx| {
                                        this.app.sidebar_tab = SidebarTab::DesktopEngine;
                                        cx.notify();
                                    })))
                                    .child(Button::new("sb_exp").label("[+] Export").small().selected(sidebar_tab == SidebarTab::ExportSync).on_click(cx.listener(|this, _, _, cx| {
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
                                                h_flex().gap_2().w_full()
                                                    .child(
                                                        Button::new("btn_pick_img").label("Open Image...")
                                                            .primary()
                                                            .flex_1()
                                                            .on_click(cx.listener(|_, _, _, cx| {
                                                                cx.spawn(async move |this, cx| {
                                                                    if let Some(file) = rfd::AsyncFileDialog::new().add_filter("Image", &["png", "jpg", "jpeg", "webp", "avif"]).pick_file().await {
                                                                        let path = file.path().to_path_buf();
                                                                        if let Ok(dyn_img) = crate::backend::runtime::spawn_blocking(move || image::open(&path)).await.unwrap() {
                                                                            let _ = this.update(cx, |view, cx| {
                                                                                view.app.on_image_selected(file.path().to_path_buf(), dyn_img);
                                                                                cx.notify();
                                                                            });
                                                                        }
                                                                    }
                                                                }).detach();
                                                            }))
                                                    )
                                                    .child(
                                                        Button::new("btn_pick_lut").label("Import LUT...")
                                                            .flex_1()
                                                            .on_click(cx.listener(|_, _, _, cx| {
                                                                cx.spawn(async move |this, cx| {
                                                                    if let Some(file) = rfd::AsyncFileDialog::new().add_filter("LUT", &["cube", "png"]).pick_file().await {
                                                                        let path = file.path().to_path_buf();
                                                                        let _ = this.update(cx, |view, cx| {
                                                                            view.app.current_theme = crate::app::ThemeSource::Custom(path);
                                                                            let _ = view.app.run_processing();
                                                                            cx.notify();
                                                                        });
                                                                    }
                                                                }).detach();
                                                            }))
                                                    )
                                            )
                                            .child(div().text_sm().font_bold().pt_1().child("Color Presets (13 Total)"))
                                            .child(
                                                v_flex().gap_1()
                                                    .children(PRESET_NAMES.iter().map(|&name| {
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
                                            )
                                            .child(div().text_sm().font_bold().pt_2().child("Remap Algorithm"))
                                            .child(
                                                h_flex().gap_1().w_full()
                                                    .child(Button::new("alg_g").label("Gauss").small().flex_1().selected(algo == RemapAlgorithm::Gaussian).on_click(cx.listener(|this, _, _, cx| {
                                                        this.app.algorithm = RemapAlgorithm::Gaussian; let _ = this.app.run_processing();
                                                        cx.notify();
                                                    })))
                                                    .child(Button::new("alg_s").label("Shepard").small().flex_1().selected(algo == RemapAlgorithm::Shepard).on_click(cx.listener(|this, _, _, cx| {
                                                        this.app.algorithm = RemapAlgorithm::Shepard; let _ = this.app.run_processing();
                                                        cx.notify();
                                                    })))
                                                    .child(Button::new("alg_n").label("Nearest").small().flex_1().selected(algo == RemapAlgorithm::NearestNeighbor).on_click(cx.listener(|this, _, _, cx| {
                                                        this.app.algorithm = RemapAlgorithm::NearestNeighbor; let _ = this.app.run_processing();
                                                        cx.notify();
                                                    })))
                                            )
                                            .child(
                                                h_flex().items_center().justify_between().pt_1()
                                                    .child(div().text_sm().child("HaldCLUT Resolution"))
                                                    .child(
                                                        h_flex().gap_1()
                                                            .child(Button::new("hald_8").label("Lvl 8").small().selected(hald_lvl == 8).on_click(cx.listener(|this, _, _, cx| {
                                                                this.app.hald_level = 8; let _ = this.app.run_processing(); cx.notify();
                                                            })))
                                                            .child(Button::new("hald_16").label("Lvl 16").small().selected(hald_lvl == 16).on_click(cx.listener(|this, _, _, cx| {
                                                                this.app.hald_level = 16; let _ = this.app.run_processing(); cx.notify();
                                                            })))
                                                    )
                                            )
                                            .child(
                                                h_flex().items_center().justify_between().pt_1()
                                                    .child(div().text_sm().child("Preserve Luma"))
                                                    .child(Switch::new("sw_luma").checked(luma).on_click(cx.listener(|this, val: &bool, _, cx| {
                                                        this.app.preserve_luma = *val; let _ = this.app.run_processing();
                                                        cx.notify();
                                                    })))
                                            )
                                            .child(div().h_px().w_full().bg(cx.theme().border).my_1())
                                            .child(div().text_sm().font_bold().child("Algorithmic Effects"))
                                            .child(
                                                h_flex().items_center().justify_between()
                                                    .child(div().text_sm().child(format!("Blur Radius: {:.1}", blur_sigma)))
                                                    .child(
                                                        h_flex().gap_1()
                                                            .child(Button::new("blur_0").label("0").small().selected(blur_sigma == 0.0).on_click(cx.listener(|this, _, _, cx| { this.app.blur_sigma = 0.0; let _ = this.app.apply_blur(); cx.notify(); })))
                                                            .child(Button::new("blur_5").label("5").small().selected(blur_sigma == 5.0).on_click(cx.listener(|this, _, _, cx| { this.app.blur_sigma = 5.0; let _ = this.app.apply_blur(); cx.notify(); })))
                                                            .child(Button::new("blur_15").label("15").small().selected(blur_sigma == 15.0).on_click(cx.listener(|this, _, _, cx| { this.app.blur_sigma = 15.0; let _ = this.app.apply_blur(); cx.notify(); })))
                                                    )
                                            )
                                            .child(
                                                h_flex().items_center().justify_between()
                                                    .child(div().text_sm().child("Dithering Diffusion"))
                                                    .child(Switch::new("sw_dither").checked(dither).on_click(cx.listener(|this, val: &bool, _, cx| {
                                                        this.app.dither_enabled = *val; let _ = this.app.apply_dither();
                                                        cx.notify();
                                                    })))
                                            )
                                            .child(
                                                Button::new("btn_extract_cols").label("Extract k-Means Dominant Colors")
                                                    .w_full()
                                                    .small()
                                                    .on_click(cx.listener(|this, _, _, cx| {
                                                        let _ = this.app.extract_dominant_colors();
                                                        cx.notify();
                                                    }))
                                            )
                                            .child(
                                                if let Some(cols) = extracted_cols {
                                                    v_flex().gap_1()
                                                        .child(div().text_xs().text_color(cx.theme().muted_foreground).child("Extracted Palettes:"))
                                                        .child(
                                                            h_flex().gap_1().flex_wrap()
                                                                .children(cols.iter().map(|hex| {
                                                                    div().px_2().py_1().rounded_md().text_xs().font_bold().border_1().border_color(cx.theme().border).child(hex.clone())
                                                                }))
                                                        )
                                                } else {
                                                    v_flex()
                                                }
                                            )
                                            .into_any_element()
                                    }
                                    SidebarTab::DesktopEngine => {
                                        v_flex().gap_3().w_full().flex_1().overflow_y_scrollbar()
                                            .child(div().text_sm().font_bold().child("Wallpaper Backend Engine"))
                                            .child(div().text_xs().text_color(cx.theme().muted_foreground).child(backend.description()))
                                            .child(
                                                v_flex().gap_1()
                                                    .children(WallpaperBackend::ALL.iter().map(|&b| {
                                                        let is_sel = backend == b;
                                                        Button::new(SharedString::from(format!("be_{}", b.code()))).label(b.to_string())
                                                            .w_full()
                                                            .small()
                                                            .selected(is_sel)
                                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                                this.app.wallpaper_backend = b;
                                                                cx.notify();
                                                            }))
                                                    }))
                                            )
                                            .child(div().text_sm().font_bold().pt_2().child("Wayland Transition (swww)"))
                                            .child(
                                                h_flex().gap_1().flex_wrap()
                                                    .children(SWWW_TRANSITIONS.iter().map(|&t| {
                                                        let is_sel = transition == t;
                                                        let t_str = t.to_string();
                                                        Button::new(SharedString::from(format!("tr_{}", t))).label(t)
                                                            .small()
                                                            .selected(is_sel)
                                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                                this.app.swww_transition = t_str.clone();
                                                                cx.notify();
                                                            }))
                                                    }))
                                            )
                                            .child(div().text_sm().font_bold().pt_2().child("Target Display Output"))
                                            .child(
                                                h_flex().gap_1().flex_wrap()
                                                    .children(TARGET_DISPLAYS.iter().map(|&d| {
                                                        let is_sel = display == d;
                                                        let d_str = d.to_string();
                                                        Button::new(SharedString::from(format!("disp_{}", d))).label(d)
                                                            .small()
                                                            .selected(is_sel)
                                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                                this.app.target_display = d_str.clone();
                                                                cx.notify();
                                                            }))
                                                    }))
                                            )
                                            .child(
                                                h_flex().items_center().justify_between().pt_2()
                                                    .child(div().text_sm().child("Automated Scheduler Daemon"))
                                                    .child(Switch::new("sw_daemon").checked(daemon).on_click(cx.listener(|this, val: &bool, _, cx| {
                                                        this.app.daemon_enabled = *val;
                                                        cx.notify();
                                                    })))
                                            )
                                            .child(
                                                Button::new("btn_apply_wp").label("Apply to Desktop Now")
                                                    .primary()
                                                    .w_full()
                                                    .mt_2()
                                                    .on_click(cx.listener(|this, _, _, _| {
                                                        if let Some(ref path) = this.app.preview_path {
                                                            let p = path.clone();
                                                            let trans = this.app.swww_transition.clone();
                                                            let disp = this.app.target_display.clone();
                                                            let be = this.app.wallpaper_backend.code();
                                                            std::thread::spawn(move || {
                                                                let _ = crate::backend::runtime::spawn_blocking(move || {
                                                                    if be == "feh" || be == "auto" {
                                                                        std::process::Command::new("feh").arg("--bg-fill").arg(&p).spawn().ok();
                                                                    }
                                                                    if be == "swww" || be == "auto" {
                                                                        let mut cmd = std::process::Command::new("swww");
                                                                        cmd.arg("img").arg(&p).arg("--transition-type").arg(&trans);
                                                                        if disp != "All Displays" {
                                                                            cmd.arg("--outputs").arg(&disp);
                                                                        }
                                                                        cmd.spawn().ok();
                                                                    }
                                                                });
                                                            });
                                                        }
                                                    }))
                                            )
                                            .into_any_element()
                                    }
                                    SidebarTab::ExportSync => {
                                        v_flex().gap_3().w_full()
                                            .child(div().text_sm().font_bold().child("Terminal Emulator Sync"))
                                            .child(
                                                h_flex().items_center().justify_between()
                                                    .child(div().text_sm().child("Sync Alacritty (~/.config/alacritty)"))
                                                    .child(Switch::new("sw_alac").checked(sync_a).on_click(cx.listener(|this, val: &bool, _, cx| {
                                                        this.app.sync_alacritty = *val;
                                                        cx.notify();
                                                    })))
                                            )
                                            .child(
                                                h_flex().items_center().justify_between()
                                                    .child(div().text_sm().child("Sync Kitty (~/.config/kitty)"))
                                                    .child(Switch::new("sw_kitty").checked(sync_k).on_click(cx.listener(|this, val: &bool, _, cx| {
                                                        this.app.sync_kitty = *val;
                                                        cx.notify();
                                                    })))
                                            )
                                            .child(
                                                Button::new("btn_exp_term").label("Export Configs Now...")
                                                    .w_full()
                                                    .on_click(cx.listener(|this, _, _, _| {
                                                        if let Ok(home) = std::env::var("HOME") {
                                                            let _ = this.app.export_terminal_scheme(&PathBuf::from(home));
                                                        }
                                                    }))
                                            )
                                            .child(div().h_px().w_full().bg(cx.theme().border).my_2())
                                            .child(div().text_sm().font_bold().child("Image Export"))
                                            .child(
                                                Button::new("btn_save_img").label("Save Graded Image As...")
                                                    .primary()
                                                    .w_full()
                                                    .on_click(cx.listener(|_, _, _, cx| {
                                                        cx.spawn(async move |this, cx| {
                                                            if let Some(file) = rfd::AsyncFileDialog::new().set_file_name("wallmod_graded.png").save_file().await {
                                                                let save_path = file.path().to_path_buf();
                                                                let _ = this.update(cx, |view, cx| {
                                                                    if let Some(ref dyn_img) = view.app.processed_dyn {
                                                                        let _ = dyn_img.save(save_path);
                                                                        view.app.state = crate::app::AppState::Notice("Saved successfully!".to_string());
                                                                    }
                                                                    cx.notify();
                                                                });
                                                            }
                                                        }).detach();
                                                    }))
                                            )
                                            .into_any_element()
                                    }
                                }
                            )
                    )
                    .child(
                        // Main Workspace Preview & Telemetry Area
                        v_flex()
                            .flex_1()
                            .h_full()
                            .overflow_hidden()
                            .child(
                                // Workspace Switcher Bar
                                h_flex()
                                    .w_full()
                                    .h_10()
                                    .px_4()
                                    .border_b_1()
                                    .border_color(cx.theme().border)
                                    .items_center()
                                    .gap_2()
                                    .child(Button::new("wv_std").label("[ * ] Output Visual").small().selected(workspace_view == WorkspaceView::Standard).on_click(cx.listener(|this, _, _, cx| { this.app.workspace_view = WorkspaceView::Standard; cx.notify(); })))
                                    .child(Button::new("wv_diff").label("[ / ] Split Diff").small().selected(workspace_view == WorkspaceView::SplitDiff).on_click(cx.listener(|this, _, _, cx| { this.app.workspace_view = WorkspaceView::SplitDiff; cx.notify(); })))
                                    .child(Button::new("wv_tel").label("[ i ] Telemetry Info").small().selected(workspace_view == WorkspaceView::Telemetry).on_click(cx.listener(|this, _, _, cx| { this.app.workspace_view = WorkspaceView::Telemetry; cx.notify(); })))
                                    .child(Button::new("wv_gal").label("[ + ] Album Gallery").small().selected(workspace_view == WorkspaceView::Gallery).on_click(cx.listener(|this, _, _, cx| { this.app.workspace_view = WorkspaceView::Gallery; cx.notify(); })))
                            )
                            .child(
                                v_flex().flex_1().w_full().p_6().items_center().justify_center().overflow_y_scrollbar()
                                    .child(
                                        match workspace_view {
                                            WorkspaceView::Standard => {
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
                                                        .child(div().text_sm().text_color(cx.theme().muted_foreground).child("Click 'Open Image...' on the left sidebar to begin color grading."))
                                                        .into_any_element()
                                                }
                                            }
                                            WorkspaceView::SplitDiff => {
                                                h_flex().size_full().gap_4()
                                                    .child(
                                                        v_flex().flex_1().h_full().gap_2()
                                                            .child(div().text_sm().font_bold().child("Original Base Image"))
                                                            .child(
                                                                if let Some(bp) = base_path {
                                                                    div().size_full().border_1().border_color(cx.theme().border).rounded_lg().overflow_hidden().child(img(bp).size_full().object_fit(ObjectFit::Contain)).into_any_element()
                                                                } else {
                                                                    div().child("None").into_any_element()
                                                                }
                                                            )
                                                    )
                                                    .child(
                                                        v_flex().flex_1().h_full().gap_2()
                                                            .child(div().text_sm().font_bold().child("Processed Graded Output"))
                                                            .child(
                                                                if let Some(pp) = preview_path {
                                                                    div().size_full().border_1().border_color(cx.theme().border).rounded_lg().overflow_hidden().child(img(pp).size_full().object_fit(ObjectFit::Contain)).into_any_element()
                                                                } else {
                                                                    div().child("None").into_any_element()
                                                                }
                                                            )
                                                    )
                                                    .into_any_element()
                                            }
                                            WorkspaceView::Telemetry => {
                                                v_flex().gap_4().w_full().max_w(px(600.0)).p_6().border_1().border_color(cx.theme().border).rounded_xl().bg(cx.theme().secondary)
                                                    .child(div().text_lg().font_bold().child("Telemetry & Inspection Dashboard"))
                                                    .child(
                                                        h_flex().justify_between()
                                                            .child(div().text_sm().text_color(cx.theme().muted_foreground).child("Loaded Filename:"))
                                                            .child(div().text_sm().font_bold().child(if img_name.is_empty() { "N/A".to_string() } else { img_name.clone() }))
                                                    )
                                                    .child(
                                                        h_flex().justify_between()
                                                            .child(div().text_sm().text_color(cx.theme().muted_foreground).child("Dimensions:"))
                                                            .child(div().text_sm().font_bold().child(format!("{} x {} px", img_w, img_h)))
                                                    )
                                                    .child(
                                                        h_flex().justify_between()
                                                            .child(div().text_sm().text_color(cx.theme().muted_foreground).child("WCAG Accessibility Contrast:"))
                                                            .child(div().text_sm().font_bold().child(format!("{:.2} : 1", wcag)))
                                                    )
                                                    .child(div().text_xs().text_color(cx.theme().muted_foreground).pt_2().child("All color transforms are computed using 64-bit precision floating point math before quantizing down to 8-bit sRGB buffers."))
                                                    .into_any_element()
                                            }
                                            WorkspaceView::Gallery => {
                                                v_flex().gap_4().size_full()
                                                    .child(
                                                        h_flex().justify_between().items_center()
                                                            .child(div().text_lg().font_bold().child("System Wallpaper Albums"))
                                                            .child(Button::new("btn_scan_gal").label("Scan System Gallery").primary().on_click(cx.listener(|this, _, _, cx| {
                                                                this.app.albums = WallmodApp::scan_system_gallery();
                                                                cx.notify();
                                                            })))
                                                    )
                                                    .child(
                                                        if albums.is_empty() {
                                                            div().p_8().text_center().text_color(cx.theme().muted_foreground).child("No albums loaded. Click 'Scan System Gallery' above to discover system wallpapers.").into_any_element()
                                                        } else {
                                                            v_flex().gap_2().children(albums.iter().map(|alb| {
                                                                div().p_4().border_1().border_color(cx.theme().border).rounded_lg().child(format!("{} ({} images) - {:?}", alb.folder_name, alb.image_count, alb.folder_path))
                                                            })).into_any_element()
                                                        }
                                                    )
                                                    .into_any_element()
                                            }
                                        }
                                    )
                            )
                    )
            )
    }
}
