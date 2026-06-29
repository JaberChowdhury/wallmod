//! Categorized left sidebar navigation component.

use crate::app::{
    RemapAlgorithm, SidebarTab, WallpaperBackend,
    PRESET_NAMES, SWWW_TRANSITIONS, TARGET_DISPLAYS,
};
use crate::ui::WallmodView;
use crate::ui::swatches::render_swatches;
use gpui::*;
use gpui_component::{
    button::*, switch::*, scroll::ScrollableElement, menu::{DropdownMenu as _, PopupMenuItem},
    v_flex, h_flex, ActiveTheme, IconName, Sizable, Selectable, StyledExt,
};
use std::path::PathBuf;

/// Renders the active category sidebar controls cleanly separated without crowding.
pub fn render_sidebar(view: &mut WallmodView, cx: &mut Context<WallmodView>) -> impl IntoElement {
    let sidebar_tab = view.app.sidebar_tab;
    let sub_tab = view.app.option_group_tab;
    let selected_preset = view.app.selected_preset.clone();
    let algo = view.app.algorithm;
    let luma = view.app.preserve_luma;
    let hald_lvl = view.app.hald_level;
    let sync_a = view.app.sync_alacritty;
    let sync_k = view.app.sync_kitty;
    let backend = view.app.wallpaper_backend;
    let transition = view.app.swww_transition.clone();
    let display = view.app.target_display.clone();
    let blur_sigma = view.app.blur_sigma;
    let dither = view.app.dither_enabled;
    let ps = view.app.photoshop_params;
    let daemon = view.app.daemon_enabled;
    let day_t = view.app.day_theme.clone();
    let night_t = view.app.night_theme.clone();
    let extracted_cols = view.app.extracted_colors.clone();
    let current_theme = view.app.current_theme.clone();

    let view_entity = cx.entity().clone();

    v_flex()
        .w(px(350.0))
        .h_full()
        .border_r_1()
        .border_color(cx.theme().border)
        .p_4()
        .gap_4()
        .child(
            div().text_base().font_bold().text_color(cx.theme().primary).child(sidebar_tab.to_string())
        )
        .child(div().h_px().w_full().bg(cx.theme().border))
        .child(
            match sidebar_tab {
                SidebarTab::ColorGrading => {
                    v_flex().gap_3().w_full().flex_1().overflow_y_scrollbar()
                        .child(
                            h_flex().gap_1().w_full().p_1().bg(cx.theme().secondary).rounded_md()
                                .child(Button::new("sub_cg_0").label("Sources & Presets").small().flex_1().selected(sub_tab == 0).on_click(cx.listener(|this, _, _, cx| { this.app.option_group_tab = 0; cx.notify(); })))
                                .child(Button::new("sub_cg_1").label("Remap Engine").small().flex_1().selected(sub_tab == 1).on_click(cx.listener(|this, _, _, cx| { this.app.option_group_tab = 1; cx.notify(); })))
                        )
                        .child(
                            if sub_tab == 0 {
                                v_flex().gap_3().w_full()
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
                                    .child(render_swatches(&current_theme, cx))
                                    .child(div().text_sm().font_bold().pt_2().child("Curated Theme Presets"))
                                    .child(
                                        Button::new("btn_preset_dropdown")
                                            .label(format!("Preset: {}", selected_preset.as_deref().unwrap_or("None")))
                                            .icon(IconName::Palette)
                                            .w_full()
                                            .outline()
                                            .dropdown_menu({
                                                let ve = view_entity.clone();
                                                move |mut menu, window, _| {
                                                    for &name in PRESET_NAMES.iter() {
                                                        let n = name.to_string();
                                                        let ve = ve.clone();
                                                        menu = menu.item(
                                                            PopupMenuItem::new(name).on_click(window.listener_for(&ve, move |this, _, _, cx| {
                                                                this.app.selected_preset = Some(n.clone());
                                                                this.app.current_theme = crate::app::ThemeSource::Preset(n.clone());
                                                                let _ = this.app.run_processing();
                                                                cx.notify();
                                                            }))
                                                        );
                                                    }
                                                    menu
                                                }
                                            })
                                    )
                            } else {
                                v_flex().gap_3().w_full()
                                    .child(div().text_sm().font_bold().child("Remap Algorithm"))
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
                                            .child(div().text_sm().child("Preserve Shadow Luma"))
                                            .child(Switch::new("sw_luma").checked(luma).on_click(cx.listener(|this, val: &bool, _, cx| {
                                                this.app.preserve_luma = *val; let _ = this.app.run_processing();
                                                cx.notify();
                                            })))
                                    )
                            }
                        )
                        .into_any_element()
                }
                SidebarTab::PhotoshopEffects => {
                    v_flex().gap_3().w_full().flex_1().overflow_y_scrollbar()
                        .child(
                            h_flex().gap_1().w_full().p_1().bg(cx.theme().secondary).rounded_md()
                                .child(Button::new("sub_ps_0").label("Basic Adjust").small().flex_1().selected(sub_tab == 0).on_click(cx.listener(|this, _, _, cx| { this.app.option_group_tab = 0; cx.notify(); })))
                                .child(Button::new("sub_ps_1").label("Color & Blur").small().flex_1().selected(sub_tab == 1).on_click(cx.listener(|this, _, _, cx| { this.app.option_group_tab = 1; cx.notify(); })))
                        )
                        .child(
                            if sub_tab == 0 {
                                v_flex().gap_3().w_full()
                                    .child(div().text_sm().font_bold().child("Brightness & Contrast"))
                                    .child(
                                        h_flex().items_center().justify_between()
                                            .child(div().text_sm().child(format!("Brightness: {}", ps.brightness)))
                                            .child(
                                                h_flex().gap_1()
                                                    .child(Button::new("ps_b_m20").label("-20").small().selected(ps.brightness == -20).on_click(cx.listener(|this, _, _, cx| { this.app.photoshop_params.brightness = -20; let _ = this.app.run_processing(); cx.notify(); })))
                                                    .child(Button::new("ps_b_0").label("0").small().selected(ps.brightness == 0).on_click(cx.listener(|this, _, _, cx| { this.app.photoshop_params.brightness = 0; let _ = this.app.run_processing(); cx.notify(); })))
                                                    .child(Button::new("ps_b_p20").label("+20").small().selected(ps.brightness == 20).on_click(cx.listener(|this, _, _, cx| { this.app.photoshop_params.brightness = 20; let _ = this.app.run_processing(); cx.notify(); })))
                                            )
                                    )
                                    .child(
                                        h_flex().items_center().justify_between()
                                            .child(div().text_sm().child(format!("Contrast: {:.0}", ps.contrast)))
                                            .child(
                                                h_flex().gap_1()
                                                    .child(Button::new("ps_c_m20").label("-20").small().selected(ps.contrast == -20.0).on_click(cx.listener(|this, _, _, cx| { this.app.photoshop_params.contrast = -20.0; let _ = this.app.run_processing(); cx.notify(); })))
                                                    .child(Button::new("ps_c_0").label("0").small().selected(ps.contrast == 0.0).on_click(cx.listener(|this, _, _, cx| { this.app.photoshop_params.contrast = 0.0; let _ = this.app.run_processing(); cx.notify(); })))
                                                    .child(Button::new("ps_c_p20").label("+20").small().selected(ps.contrast == 20.0).on_click(cx.listener(|this, _, _, cx| { this.app.photoshop_params.contrast = 20.0; let _ = this.app.run_processing(); cx.notify(); })))
                                            )
                                    )
                            } else {
                                v_flex().gap_3().w_full()
                                    .child(div().text_sm().font_bold().child("Color & Effects"))
                                    .child(
                                        h_flex().items_center().justify_between()
                                            .child(div().text_sm().child(format!("Saturation: {:.1}", ps.saturation)))
                                            .child(
                                                h_flex().gap_1()
                                                    .child(Button::new("ps_s_m1").label("Desat").small().selected(ps.saturation == -1.0).on_click(cx.listener(|this, _, _, cx| { this.app.photoshop_params.saturation = -1.0; let _ = this.app.run_processing(); cx.notify(); })))
                                                    .child(Button::new("ps_s_0").label("Norm").small().selected(ps.saturation == 0.0).on_click(cx.listener(|this, _, _, cx| { this.app.photoshop_params.saturation = 0.0; let _ = this.app.run_processing(); cx.notify(); })))
                                                    .child(Button::new("ps_s_p05").label("Vivid").small().selected(ps.saturation == 0.5).on_click(cx.listener(|this, _, _, cx| { this.app.photoshop_params.saturation = 0.5; let _ = this.app.run_processing(); cx.notify(); })))
                                            )
                                    )
                                    .child(
                                        h_flex().items_center().justify_between()
                                            .child(div().text_sm().child(format!("Hue Shift: {}°", ps.hue)))
                                            .child(
                                                h_flex().gap_1()
                                                    .child(Button::new("ps_h_0").label("0°").small().selected(ps.hue == 0).on_click(cx.listener(|this, _, _, cx| { this.app.photoshop_params.hue = 0; let _ = this.app.run_processing(); cx.notify(); })))
                                                    .child(Button::new("ps_h_90").label("90°").small().selected(ps.hue == 90).on_click(cx.listener(|this, _, _, cx| { this.app.photoshop_params.hue = 90; let _ = this.app.run_processing(); cx.notify(); })))
                                                    .child(Button::new("ps_h_180").label("180°").small().selected(ps.hue == 180).on_click(cx.listener(|this, _, _, cx| { this.app.photoshop_params.hue = 180; let _ = this.app.run_processing(); cx.notify(); })))
                                            )
                                    )
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
                            }
                        )
                        .into_any_element()
                }
                SidebarTab::DesktopEngine => {
                    v_flex().gap_3().w_full().flex_1().overflow_y_scrollbar()
                        .child(
                            h_flex().gap_1().w_full().p_1().bg(cx.theme().secondary).rounded_md()
                                .child(Button::new("sub_eng_0").label("Backend & Display").small().flex_1().selected(sub_tab == 0).on_click(cx.listener(|this, _, _, cx| { this.app.option_group_tab = 0; cx.notify(); })))
                                .child(Button::new("sub_eng_1").label("Transitions & Daemon").small().flex_1().selected(sub_tab == 1).on_click(cx.listener(|this, _, _, cx| { this.app.option_group_tab = 1; cx.notify(); })))
                        )
                        .child(
                            if sub_tab == 0 {
                                v_flex().gap_3().w_full()
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
                            } else {
                                v_flex().gap_3().w_full()
                                    .child(div().text_sm().font_bold().child("Wayland Transition (swww)"))
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
                                    .child(
                                        h_flex().items_center().justify_between().pt_2()
                                            .child(div().text_sm().child("Automated Scheduler Daemon"))
                                            .child(Switch::new("sw_daemon").checked(daemon).on_click(cx.listener(|this, val: &bool, _, cx| {
                                                this.app.daemon_enabled = *val;
                                                cx.notify();
                                            })))
                                    )
                                    .child(
                                        if daemon {
                                            v_flex().gap_2().p_3().border_1().border_color(cx.theme().border).rounded_lg().bg(cx.theme().secondary)
                                                .child(div().text_xs().font_bold().child("Daytime Theme"))
                                                .child(
                                                    Button::new("btn_day_dropdown")
                                                        .label(format!("Day: {}", day_t))
                                                        .w_full().small().outline()
                                                        .dropdown_menu({
                                                            let ve = view_entity.clone();
                                                            move |mut menu, window, _| {
                                                                for &name in PRESET_NAMES.iter() {
                                                                    let n = name.to_string();
                                                                    let ve = ve.clone();
                                                                    menu = menu.item(PopupMenuItem::new(name).on_click(window.listener_for(&ve, move |this, _, _, cx| {
                                                                        this.app.day_theme = n.clone(); cx.notify();
                                                                    })));
                                                                }
                                                                menu
                                                            }
                                                        })
                                                )
                                                .child(div().text_xs().font_bold().pt_1().child("Nighttime Theme"))
                                                .child(
                                                    Button::new("btn_night_dropdown")
                                                        .label(format!("Night: {}", night_t))
                                                        .w_full().small().outline()
                                                        .dropdown_menu({
                                                            let ve = view_entity.clone();
                                                            move |mut menu, window, _| {
                                                                for &name in PRESET_NAMES.iter() {
                                                                    let n = name.to_string();
                                                                    let ve = ve.clone();
                                                                    menu = menu.item(PopupMenuItem::new(name).on_click(window.listener_for(&ve, move |this, _, _, cx| {
                                                                        this.app.night_theme = n.clone(); cx.notify();
                                                                    })));
                                                                }
                                                                menu
                                                            }
                                                        })
                                                )
                                        } else {
                                            v_flex()
                                        }
                                    )
                            }
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
                SidebarTab::ToolsExt => {
                    v_flex().gap_4().w_full().flex_1().overflow_y_scrollbar()
                        .child(div().text_sm().font_bold().child("Dominant Color Extraction"))
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
                                    .child(div().text_xs().text_color(cx.theme().muted_foreground).child("Extracted Oklab Palettes:"))
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
                        .child(div().h_px().w_full().bg(cx.theme().border))
                        .child(div().text_sm().font_bold().child("AI Super-Resolution Engine"))
                        .child(div().p_3().border_1().border_color(cx.theme().border).rounded_md().bg(cx.theme().secondary).text_xs().child("Real-ESRGAN neural upscaling pipeline is queued in Category H roadmap."))
                        .child(div().text_sm().font_bold().child("OCR Wallpaper Extraction"))
                        .child(div().p_3().border_1().border_color(cx.theme().border).rounded_md().bg(cx.theme().secondary).text_xs().child("Tesseract quote and text extraction pipeline is queued in Category H roadmap."))
                        .into_any_element()
                }
            }
        )
}
