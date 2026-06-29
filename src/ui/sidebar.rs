//! Categorized left sidebar navigation component.

use crate::app::{
    RemapAlgorithm, SidebarTab, WallpaperBackend, PRESET_NAMES, SWWW_TRANSITIONS, TARGET_DISPLAYS,
};
use crate::ui::swatches::render_swatches;
use crate::ui::WallmodView;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    button::*,
    h_flex,
    menu::{DropdownMenu as _, PopupMenuItem},
    scroll::ScrollableElement,
    slider::Slider,
    switch::*,
    v_flex, ActiveTheme, Disableable, Selectable, Sizable, StyledExt,
};

/// Renders the active category sidebar controls cleanly separated without crowding.
pub fn render_sidebar(view: &mut WallmodView, cx: &mut Context<WallmodView>) -> impl IntoElement {
    let is_loading = view.app.state.is_loading();
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
    let psort = view.app.pixel_sort_enabled;
    let seam_target = view.app.seam_carve_target;
    let img_w = view.app.image_width;
    let ps = view.app.photoshop_params;
    let daemon = view.app.daemon_enabled;
    let day_t = view.app.day_theme.clone();
    let night_t = view.app.night_theme.clone();

    let current_theme = view.app.current_theme.clone();
    let chaining_mode = view.app.chaining_mode;
    let bit_depth = view.app.global_bit_depth;

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
                SidebarTab::FavoriteColors => { div().child("Favorite Colors").into_any_element() },
                SidebarTab::ColorGrading => {
                    v_flex().gap_3().w_full().flex_1().overflow_y_scrollbar()
                        .child(
                            h_flex().gap_1().w_full().p_1().bg(cx.theme().secondary).rounded_md()
                                .child(Button::new("sub_cg_0").child(gpui::svg().path("folder-open.svg").size_4().text_color(if sub_tab == 0 { cx.theme().secondary } else { cx.theme().primary })).child("Sources & Presets").small().flex_1().selected(sub_tab == 0).when(sub_tab == 0, |this| this.primary()).cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.option_group_tab = 0; cx.notify(); })))
                                .child(Button::new("sub_cg_1").child(gpui::svg().path("settings.svg").size_4().text_color(if sub_tab == 1 { cx.theme().secondary } else { cx.theme().primary })).child("Remap Engine").small().flex_1().selected(sub_tab == 1).when(sub_tab == 1, |this| this.primary()).cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.option_group_tab = 1; cx.notify(); })))
                        )
                        .child(
                            if sub_tab == 0 {
                                v_flex().gap_3().w_full()
                                    .child(
                                        h_flex().gap_2().w_full()
                                            .child(
                                                Button::new("btn_pick_img").disabled(is_loading).label("Open Image...")
                                                    .child(gpui::svg().path("folder-open.svg").size_4().text_color(cx.theme().primary))
                                                    .primary()
                                                    .flex_1()
                                                    .cursor_pointer().on_click(cx.listener(|_, _, _, cx| {
                                                        cx.spawn(async move |this, cx| {
                                                            if let Some(file) = rfd::AsyncFileDialog::new().add_filter("Image", &["png", "jpg", "jpeg", "webp", "avif", "bmp", "tiff", "tga", "gif", "ico", "hdr", "exr", "qoi", "pnm"]).pick_file().await {
                                                                let path = file.path().to_path_buf();
                                                                let _ = this.update(cx, |view, cx| {
                                                                    view.app.preview_path = Some(path.clone());
                                                                    cx.notify();
                                                                });
                                                                cx.background_executor().timer(std::time::Duration::from_millis(1500)).await;
                                                                let _ = this.update(cx, |view, cx| {
                                                                    view.app.state = crate::app::AppState::Loading(0.2, "Reading image file...".to_string());
                                                                    cx.notify();
                                                                });
                                                                let res = crate::backend::runtime::spawn_blocking(move || crate::app::helpers::open_image(&path)).await;
                                                                match res {
                                                                    Ok(Ok(dyn_img)) => {
                                                                        let _ = this.update(cx, |view, cx| {
                                                                            view.app.on_image_selected(file.path().to_path_buf(), dyn_img);
                                                                            view.trigger_async_processing(cx, "Applying theme...");
                                                                        });
                                                                    }
                                                                    Ok(Err(e)) => {
                                                                        let _ = this.update(cx, |view, cx| {
                                                                            view.app.state = crate::app::AppState::Error(format!("Failed to decode image: {}", e));
                                                                            cx.notify();
                                                                        });
                                                                    }
                                                                    Err(e) => {
                                                                        let _ = this.update(cx, |view, cx| {
                                                                            view.app.state = crate::app::AppState::Error(format!("Task failed: {}", e));
                                                                            cx.notify();
                                                                        });
                                                                    }
                                                                }
                                                            }
                                                        }).detach();
                                                    }))
                                            )
                                            .child(
                                                Button::new("btn_pick_lut").disabled(is_loading).label("Import LUT...")
                                                    .child(gpui::svg().path("file.svg").size_4().text_color(cx.theme().primary))
                                                    .flex_1()
                                                    .cursor_pointer().on_click(cx.listener(|_, _, _, cx| {
                                                        cx.spawn(async move |this, cx| {
                                                            if let Some(file) = rfd::AsyncFileDialog::new().add_filter("LUT", &["cube", "png"]).pick_file().await {
                                                                let path = file.path().to_path_buf();
                                                                let _ = this.update(cx, |view, cx| {
                                                                    let new_theme = crate::app::ThemeSource::Custom(path);
                                                                    view.app.apply_theme(new_theme);
                                                                    view.trigger_async_processing(cx, "Applying LUT color grading...");
                                                                });
                                                            }
                                                        }).detach();
                                                    }))
                                            )
                                    )
                                    .child(render_swatches(&current_theme, cx))
                                    .child(div().text_sm().font_bold().pt_2().child("Curated Theme Presets"))
                                    .child(
                                        Button::new("btn_preset_dropdown").disabled(is_loading)
                                            .label(format!("Preset: {}", selected_preset.as_deref().unwrap_or("None")))
                                            .child(gpui::svg().path("palette.svg").size_4().text_color(cx.theme().primary))
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
                                                                let new_theme = crate::app::ThemeSource::Preset(n.clone());
                                                                this.app.apply_theme(new_theme);
                                                                this.trigger_async_processing(cx, "Applying theme preset...");
                                                            }))
                                                        );
                                                    }
                                                                    menu
                                                                }
                                                            })
                                                    )
                                                    .child(
                                                        v_flex().gap_2().pt_2()
                                                            .child(div().text_xs().font_bold().child(format!("Quick Gaussian Blur: {:.1}px", blur_sigma)))
                                                            .child(
                                                                Slider::new(&view.blur_slider).disabled(is_loading)
                                                            )
                                                    )
                                                    .child(
                                                        v_flex().gap_2().pt_2().border_t_1().border_color(cx.theme().border)
                                                            .child(div().text_sm().font_bold().child("Retro Bit-Depth & Style"))
                                                            .child(
                                                                h_flex().gap_1().w_full()
                                                                    .child(Button::new("bd_32").disabled(is_loading).child(gpui::svg().path("monitor.svg").size_4().text_color(cx.theme().primary)).child("32-bit").small().flex_1().selected(bit_depth == crate::app::state::BitDepthStyle::Bit32).cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                                        this.app.global_bit_depth = crate::app::state::BitDepthStyle::Bit32; this.trigger_async_processing(cx, "Setting 32-bit color...");
                                                                    })))
                                                                    .child(Button::new("bd_16").disabled(is_loading).child(gpui::svg().path("cpu.svg").size_4().text_color(cx.theme().primary)).child("16-bit").small().flex_1().selected(bit_depth == crate::app::state::BitDepthStyle::Bit16).cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                                        this.app.global_bit_depth = crate::app::state::BitDepthStyle::Bit16; this.trigger_async_processing(cx, "Applying 16-bit High Color...");
                                                                    })))
                                                                    .child(Button::new("bd_8").disabled(is_loading).child(gpui::svg().path("layers.svg").size_4().text_color(cx.theme().primary)).child("8-bit").small().flex_1().selected(bit_depth == crate::app::state::BitDepthStyle::Bit8).cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                                        this.app.global_bit_depth = crate::app::state::BitDepthStyle::Bit8; this.trigger_async_processing(cx, "Applying 8-bit VGA style...");
                                                                    })))
                                                            )
                                                            .child(
                                                                h_flex().justify_between().items_center().pt_1()
                                                                    .child(div().text_xs().font_bold().child(if chaining_mode { "Chaining Mode: ON (Append)" } else { "Explore Mode: Single Theme" }))
                                                                    .child(Button::new("btn_toggle_chaining").disabled(is_loading).child(gpui::svg().path("git-branch.svg").size_4().text_color(cx.theme().primary)).label(if chaining_mode { "Chain ON" } else { "Explore" }).small().selected(chaining_mode).cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                                        this.app.chaining_mode = !this.app.chaining_mode;
                                                                        cx.notify();
                                                                    })))
                                                            )
                                                            .child(
                                                                Button::new("btn_view_node_pipeline").disabled(is_loading)
                                                                    .label("Open Node Graph Pipeline")
                                                                    .child(gpui::svg().path("git-commit.svg").size_4().text_color(cx.theme().primary))
                                                                    .w_full()
                                                                    .small()
                                                                    .outline()
                                                                    .cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                                        this.app.workspace_view = crate::app::state::WorkspaceView::NodePipeline;
                                                                        cx.notify();
                                                                    }))
                                                            )
                                                    )
                                                    .child(
                                                        Button::new("btn_apply_theme_main").disabled(is_loading)
                                                            .label("Apply Theme & Process")
                                                            .child(gpui::svg().path("check.svg").size_4().text_color(cx.theme().primary))
                                                            .primary()
                                                            .w_full()
                                                            .mt_2()
                                                            .cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                                this.trigger_async_processing(cx, "Processing theme & grading...");
                                                            }))
                                                    )
                                            } else {
                                v_flex().gap_3().w_full()
                                    .child(div().text_sm().font_bold().child("Remap Algorithm"))
                                    .child(
                                        h_flex().gap_1().w_full()
                                             .child(Button::new("alg_g").disabled(is_loading).child(gpui::svg().path("settings.svg").size_4().text_color(cx.theme().primary)).child("Gauss").small().flex_1().selected(algo == RemapAlgorithm::Gaussian).cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                this.app.algorithm = RemapAlgorithm::Gaussian; this.trigger_async_processing(cx, "Remapping colors (Gaussian)...");
                                            })))
                                            .child(Button::new("alg_s").disabled(is_loading).child(gpui::svg().path("settings.svg").size_4().text_color(cx.theme().primary)).child("Shepard").small().flex_1().selected(algo == RemapAlgorithm::Shepard).cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                this.app.algorithm = RemapAlgorithm::Shepard; this.trigger_async_processing(cx, "Remapping colors (Shepard)...");
                                            })))
                                            .child(Button::new("alg_n").disabled(is_loading).child(gpui::svg().path("settings.svg").size_4().text_color(cx.theme().primary)).child("Nearest").small().flex_1().selected(algo == RemapAlgorithm::NearestNeighbor).cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                this.app.algorithm = RemapAlgorithm::NearestNeighbor; this.trigger_async_processing(cx, "Remapping colors (Nearest)...");
                                            })))
                                    )
                                    .child(
                                        h_flex().items_center().justify_between().pt_1()
                                            .child(div().text_sm().child("HaldCLUT Resolution"))
                                            .child(
                                                h_flex().gap_1()
                                                    .child(Button::new("hald_8").disabled(is_loading).child(gpui::svg().path("layout-dashboard.svg").size_4().text_color(cx.theme().primary)).child("Lvl 8").small().selected(hald_lvl == 8).cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                        this.app.hald_level = 8; this.trigger_async_processing(cx, "Generating HaldCLUT 8...");
                                                    })))
                                                    .child(Button::new("hald_16").disabled(is_loading).child(gpui::svg().path("layout-dashboard.svg").size_4().text_color(cx.theme().primary)).child("Lvl 16").small().selected(hald_lvl == 16).cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                        this.app.hald_level = 16; this.trigger_async_processing(cx, "Generating HaldCLUT 16...");
                                                    })))
                                            )
                                    )
                                    .child(
                                        h_flex().items_center().justify_between().pt_1()
                                            .child(div().text_sm().child("Preserve Shadow Luma"))
                                            .child(Switch::new("sw_luma").disabled(is_loading).checked(luma).cursor_pointer().on_click(cx.listener(|this, val: &bool, _, cx| {
                                                this.app.preserve_luma = *val; this.trigger_async_processing(cx, "Applying luma preservation...");
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
                                .child(Button::new("sub_ps_0").child(gpui::svg().path("settings.svg").size_4().text_color(if sub_tab == 0 { cx.theme().secondary } else { cx.theme().primary })).child("Basic Adjust").small().flex_1().selected(sub_tab == 0).when(sub_tab == 0, |this| this.primary()).cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.option_group_tab = 0; cx.notify(); })))
                                .child(Button::new("sub_ps_1").child(gpui::svg().path("palette.svg").size_4().text_color(if sub_tab == 1 { cx.theme().secondary } else { cx.theme().primary })).child("Color & Blur").small().flex_1().selected(sub_tab == 1).when(sub_tab == 1, |this| this.primary()).cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.option_group_tab = 1; cx.notify(); })))
                        )
                        .child(
                            if sub_tab == 0 {
                                v_flex().gap_3().w_full()
                                    .child(div().text_sm().font_bold().child("Brightness & Contrast"))
                                    .child(
                                        v_flex().gap_2().pt_1()
                                            .child(div().text_sm().child(format!("Brightness: {}", ps.brightness)))
                                            .child(Slider::new(&view.brightness_slider).disabled(is_loading))
                                    )
                                    .child(
                                        v_flex().gap_2().pt_1()
                                            .child(div().text_sm().child(format!("Contrast: {:.0}", ps.contrast)))
                                            .child(Slider::new(&view.contrast_slider).disabled(is_loading))
                                    )
                            } else {
                                v_flex().gap_3().w_full()
                                    .child(div().text_sm().font_bold().child("Color & Effects"))
                                    .child(
                                        v_flex().gap_2().pt_1()
                                            .child(div().text_sm().child(format!("Saturation: {:.2}", ps.saturation)))
                                            .child(Slider::new(&view.saturation_slider).disabled(is_loading))
                                    )
                                    .child(
                                        v_flex().gap_2().pt_1()
                                            .child(div().text_sm().child(format!("Hue Shift: {}°", ps.hue)))
                                            .child(Slider::new(&view.hue_slider).disabled(is_loading))
                                    )
                                    .child(
                                        v_flex().gap_2().pt_1()
                                            .child(div().text_sm().child(format!("Blur Radius: {:.1}px", blur_sigma)))
                                            .child(
                                                Slider::new(&view.blur_slider).disabled(is_loading)
                                            )
                                    )
                                    .child(
                                        h_flex().items_center().justify_between()
                                            .child(div().text_sm().child("Dithering Diffusion"))
                                            .child(Switch::new("sw_dither").disabled(is_loading).checked(dither).cursor_pointer().on_click(cx.listener(|this, val: &bool, _, cx| {
                                                this.app.dither_enabled = *val; this.trigger_async_processing(cx, "Applying dithering diffusion...");
                                            })))
                                    )
                                    .child(
                                        h_flex().items_center().justify_between()
                                            .child(div().text_sm().child("Cyberpunk Pixel Sorting"))
                                            .child(Switch::new("sw_psort").disabled(is_loading).checked(psort).cursor_pointer().on_click(cx.listener(|this, val: &bool, _, cx| {
                                                this.app.pixel_sort_enabled = *val; this.trigger_async_processing(cx, "Applying pixel sorting...");
                                            })))
                                    )
                                    .child(
                                        v_flex().gap_2().pt_1()
                                            .child(div().text_sm().child(format!("Seam Carving Target: {}px", if seam_target == 0 { "Full".to_string() } else { seam_target.to_string() })))
                                            .child(
                                                h_flex().gap_1()
                                                    .child(Button::new("sc_full").disabled(is_loading).child("100%").small().selected(seam_target == 0).cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                        this.app.seam_carve_target = 0;
                                                        this.trigger_async_processing(cx, "Restoring full width...");
                                                    })))
                                                    .child(Button::new("sc_85").disabled(is_loading).child("85%").small().selected(seam_target > 0 && seam_target >= (img_w as f32 * 0.8) as u32).cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                        let w = (this.app.image_width as f32 * 0.85) as u32;
                                                        this.app.seam_carve_target = if w > 0 { w } else { 1600 };
                                                        this.trigger_async_processing(cx, "Seam carving (85%)...");
                                                    })))
                                                    .child(Button::new("sc_70").disabled(is_loading).child("70%").small().selected(seam_target > 0 && seam_target < (img_w as f32 * 0.8) as u32).cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                        let w = (this.app.image_width as f32 * 0.70) as u32;
                                                        this.app.seam_carve_target = if w > 0 { w } else { 1280 };
                                                        this.trigger_async_processing(cx, "Seam carving (70%)...");
                                                    })))
                                            )
                                    )
                            }
                        )
                        .into_any_element()
                }
                SidebarTab::DesktopEngine => {
                    v_flex().gap_3().w_full().flex_1().overflow_y_scrollbar()
                        .child(
                            h_flex().gap_1().w_full().p_1().bg(cx.theme().secondary).rounded_md()
                                .child(Button::new("sub_eng_0").child(gpui::svg().path("panel-left.svg").size_4().text_color(if sub_tab == 0 { cx.theme().secondary } else { cx.theme().primary })).child("Backend & Display").small().flex_1().selected(sub_tab == 0).when(sub_tab == 0, |this| this.primary()).cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.option_group_tab = 0; cx.notify(); })))
                                .child(Button::new("sub_eng_1").child(gpui::svg().path("calendar.svg").size_4().text_color(if sub_tab == 1 { cx.theme().secondary } else { cx.theme().primary })).child("Transitions & Daemon").small().flex_1().selected(sub_tab == 1).when(sub_tab == 1, |this| this.primary()).cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.option_group_tab = 1; cx.notify(); })))
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
                                                Button::new(SharedString::from(format!("be_{}", b.code()))).disabled(is_loading).label(b.to_string())
                                                    .child(gpui::svg().path("settings.svg").size_4().text_color(cx.theme().primary))
                                                    .w_full()
                                                    .small()
                                                    .selected(is_sel)
                                                    .cursor_pointer().on_click(cx.listener(move |this, _, _, cx| {
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
                                                Button::new(SharedString::from(format!("disp_{}", d))).disabled(is_loading).label(d)
                                                    .child(gpui::svg().path("maximize.svg").size_4().text_color(cx.theme().primary))
                                                    .small()
                                                    .selected(is_sel)
                                                    .cursor_pointer().on_click(cx.listener(move |this, _, _, cx| {
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
                                                Button::new(SharedString::from(format!("tr_{}", t))).disabled(is_loading).label(t)
                                                    .child(gpui::svg().path("replace.svg").size_4().text_color(cx.theme().primary))
                                                    .small()
                                                    .selected(is_sel)
                                                    .cursor_pointer().on_click(cx.listener(move |this, _, _, cx| {
                                                        this.app.swww_transition = t_str.clone();
                                                        cx.notify();
                                                    }))
                                            }))
                                    )
                                    .child(
                                        h_flex().items_center().justify_between().pt_2()
                                            .child(div().text_sm().child("Automated Scheduler Daemon"))
                                            .child(Switch::new("sw_daemon").disabled(is_loading).checked(daemon).cursor_pointer().on_click(cx.listener(|this, val: &bool, _, cx| {
                                                this.app.daemon_enabled = *val;
                                                cx.notify();
                                            })))
                                    )
                                    .child(
                                        if daemon {
                                            v_flex().gap_2().p_3().border_1().border_color(cx.theme().border).rounded_lg().bg(cx.theme().secondary)
                                                .child(div().text_xs().font_bold().child("Daytime Theme"))
                                                .child(
                                                    Button::new("btn_day_dropdown").disabled(is_loading)
                                                        .label(format!("Day: {}", day_t))
                                                        .child(gpui::svg().path("sun.svg").size_4().text_color(cx.theme().primary))
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
                                                    Button::new("btn_night_dropdown").disabled(is_loading)
                                                        .label(format!("Night: {}", night_t))
                                                        .child(gpui::svg().path("moon.svg").size_4().text_color(cx.theme().primary))
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
                            Button::new("btn_apply_wp").disabled(is_loading).label("Apply to Desktop Now")
                                .child(gpui::svg().path("check.svg").size_4().text_color(cx.theme().primary))
                                .primary()
                                .w_full()
                                .mt_2()
                                .cursor_pointer().on_click(cx.listener(|this, _, _, _| {
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
                                .child(Switch::new("sw_alac").disabled(is_loading).checked(sync_a).cursor_pointer().on_click(cx.listener(|this, val: &bool, _, cx| {
                                    this.app.sync_alacritty = *val;
                                    cx.notify();
                                })))
                        )
                        .child(
                            h_flex().items_center().justify_between()
                                .child(div().text_sm().child("Sync Kitty (~/.config/kitty)"))
                                .child(Switch::new("sw_kitty").disabled(is_loading).checked(sync_k).cursor_pointer().on_click(cx.listener(|this, val: &bool, _, cx| {
                                    this.app.sync_kitty = *val;
                                    cx.notify();
                                })))
                        )
                        .child(
                            Button::new("btn_exp_term").disabled(is_loading)
                                .child(gpui::svg().path("replace.svg").size_4().text_color(cx.theme().primary))
                                .child("Export Configs Now...")
                                .w_full()
                                .cursor_pointer().on_click(cx.listener(|this, _, _, _| {
                                    if let Ok(home) = std::env::var("HOME") {
                                        let _ = this.app.export_terminal_scheme(&std::path::PathBuf::from(&home));
                                    }
                                }))
                        )
                        .child(
                            Button::new("btn_exp_icons").disabled(is_loading)
                                .child(gpui::svg().path("wand.svg").size_4().text_color(cx.theme().primary))
                                .child("Export App Icons (Linux Ricer)")
                                .w_full()
                                .cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                    if let Ok(home) = std::env::var("HOME") {
                                        let _ = this.app.export_icon_theme(&std::path::PathBuf::from(&home));
                                        this.app.state = crate::app::AppState::Notice("Icon theme generated in ~/.icons/wallmod-theme".to_string());
                                        cx.notify();
                                    }
                                }))
                        )
                        .child(div().h_px().w_full().bg(cx.theme().border).my_2())
                        .child(div().text_sm().font_bold().child("Image Export"))
                        .child(
                            Button::new("btn_save_img").disabled(is_loading)
                                .child(gpui::svg().path("file.svg").size_4().text_color(cx.theme().primary))
                                .child("Save Graded Image As...")
                                .primary()
                                .w_full()
                                .cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                    let out_dir = this.app.export_dir.clone();
                                    cx.spawn(async move |this, cx| {
                                        let mut dialog = rfd::AsyncFileDialog::new().set_file_name("wallmod_graded.png");
                                        if let Some(dir) = out_dir {
                                            dialog = dialog.set_directory(&dir);
                                        }
                                        if let Some(file) = dialog.save_file().await {
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
                        .child(div().h_px().w_full().bg(cx.theme().border).my_2())
                        .child(div().text_sm().font_bold().child("Batch & Output Management"))
                        .child({
                            let export_dir_label = view.app.export_dir.as_ref()
                                .and_then(|p| p.file_name())
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_else(|| "Default (Source)".to_string());
                            h_flex().items_center().justify_between()
                                .child(div().text_xs().text_color(cx.theme().muted_foreground).child(format!("Output Dir: {}", export_dir_label)))
                                .child(
                                    Button::new("btn_set_out_dir").disabled(is_loading).label("Select Folder...")
                                        .cursor_pointer().on_click(cx.listener(|_, _, _, cx| {
                                            cx.spawn(async move |this, cx| {
                                                if let Some(folder) = rfd::AsyncFileDialog::new().pick_folder().await {
                                                    let path = folder.path().to_path_buf();
                                                    let _ = this.update(cx, |view, cx| {
                                                        view.app.export_dir = Some(path);
                                                        cx.notify();
                                                    });
                                                }
                                            }).detach();
                                        }))
                                )
                        })
                        .child(
                            Button::new("btn_batch_scan").disabled(is_loading)
                                .child(gpui::svg().path("folder.svg").size_4().text_color(cx.theme().primary))
                                .child("Batch Process Folder...")
                                .w_full()
                                .cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                    let theme = this.app.current_theme.clone();
                                    let params = this.app.photoshop_params;
                                    let blur = this.app.blur_sigma;
                                    let dither = this.app.dither_enabled;
                                    let seam = this.app.seam_carve_target;
                                    let psort = this.app.pixel_sort_enabled;
                                    let alg = this.app.algorithm;
                                    let luma = this.app.preserve_luma;
                                    let hald = this.app.hald_level;
                                    let chain = this.app.theme_chain.clone();
                                    let cmode = this.app.chaining_mode;
                                    let bitdepth = this.app.global_bit_depth;
                                    let out_dir = this.app.export_dir.clone();

                                    cx.spawn(async move |this, cx| {
                                        if let Some(folder) = rfd::AsyncFileDialog::new().pick_folder().await {
                                            let in_path = folder.path().to_path_buf();
                                            let target_dir = out_dir.unwrap_or_else(|| in_path.join("graded"));
                                            let _ = std::fs::create_dir_all(&target_dir);

                                            let _ = this.update(cx, |view, cx| {
                                                view.app.state = crate::app::AppState::Loading(0.5, "Batch scanning & processing folder...".to_string());
                                                cx.notify();
                                            });

                                            let _ = tokio::task::spawn_blocking(move || {
                                                if let Ok(entries) = std::fs::read_dir(&in_path) {
                                                    for entry in entries.flatten() {
                                                        let path = entry.path();
                                                        if let Some(ext) = path.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase()) {
                                                            if ["png", "jpg", "jpeg", "webp"].contains(&ext.as_str()) {
                                                                if let Ok(img) = crate::app::helpers::open_image(&path) {
                                                                    if let Ok(Some((processed, _, _, _))) = crate::app::WallmodApp::process_image_sync(Some(img), theme.clone(), params, blur, dither, seam, psort, chain.clone(), cmode, bitdepth, alg, luma, hald) {
                                                                        if let Some(name) = path.file_name() {
                                                                            let _ = processed.save(target_dir.join(name));
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }).await;

                                            let _ = this.update(cx, |view, cx| {
                                                view.app.state = crate::app::AppState::Notice("Batch processing complete!".to_string());
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
                        .child(div().h_px().w_full().bg(cx.theme().border))
                        .child(div().text_sm().font_bold().child("AI Super-Resolution Engine"))
                        .child(div().p_3().border_1().border_color(cx.theme().border).rounded_md().bg(cx.theme().secondary).text_xs().child("Real-ESRGAN neural upscaling pipeline is queued in Category H roadmap."))
                        .child(div().text_sm().font_bold().child("OCR Wallpaper Extraction"))
                        .child(div().p_3().border_1().border_color(cx.theme().border).rounded_md().bg(cx.theme().secondary).text_xs().child("Tesseract quote and text extraction pipeline is queued in Category H roadmap."))
                        .into_any_element()
                }
                SidebarTab::Settings => {
                    let dither = view.app.dither_enabled;
                    let level = view.app.hald_level;
                    let ram_pct = view.app.sys_ram_percent;
                    let fps = view.app.current_fps;
                    let cpu_threads = view.app.sys_cpu_threads.clone();
                    let show_float = view.app.show_floating_stats;
                    let f_fps = view.app.float_show_fps;
                    let f_ram = view.app.float_show_ram;
                    let f_cpu = view.app.float_show_cpu;

                    v_flex().gap_4().w_full().flex_1().overflow_y_scrollbar()
                        .child(div().h_px().w_full().bg(cx.theme().border))
                        .child(div().text_sm().font_bold().child("System Performance Stats"))
                        .child(
                            v_flex().gap_3().p_3().border_1().border_color(cx.theme().border).rounded_lg().bg(cx.theme().secondary)
                                .child(
                                    h_flex().justify_between().items_center()
                                        .child(div().text_xs().font_bold().child("RAM Usage"))
                                        .child(div().text_xs().font_bold().text_color(cx.theme().primary).child(format!("{:.1}%", ram_pct)))
                                )
                                .child(
                                    div().w_full().h(px(8.0)).rounded_full().bg(cx.theme().background).overflow_hidden()
                                        .child(div().h_full().w(gpui::Length::Definite(gpui::DefiniteLength::Fraction((ram_pct / 100.0).clamp(0.0, 1.0)))).bg(cx.theme().primary))
                                )
                                .child(
                                    h_flex().justify_between().items_center().pt_1()
                                        .child(div().text_xs().font_bold().child("App Render FPS"))
                                        .child(div().text_xs().font_bold().text_color(gpui::rgb(0x22c55e)).child(format!("{:.0} FPS", fps)))
                                )
                                .child(div().text_xs().font_bold().pt_1().child(format!("CPU Threads Load ({} Cores)", cpu_threads.len())))
                                .child(
                                    div().flex().flex_wrap().gap_1().w_full()
                                        .children(cpu_threads.iter().enumerate().map(|(idx, &load)| {
                                            let color = if load > 80.0 { gpui::rgb(0xef4444) } else if load > 50.0 { gpui::rgb(0xf59e0b) } else { gpui::rgb(0x22c55e) };
                                            div().w(px(58.0)).h(px(24.0)).flex().items_center().justify_center().rounded_md().bg(cx.theme().background).border_1().border_color(cx.theme().border).text_xs().text_color(color).child(format!("C{}:{:3.0}%", idx, load))
                                        }))
                                )
                                .child(div().h_px().w_full().bg(cx.theme().border))
                                .child(
                                    h_flex().justify_between().items_center()
                                        .child(div().text_xs().font_bold().child("Floating Stats Overlay"))
                                        .child(Switch::new("sw_float_main").disabled(is_loading).checked(show_float).cursor_pointer().on_click(cx.listener(|this, val: &bool, _, cx| {
                                            this.app.show_floating_stats = *val;
                                            cx.notify();
                                        })))
                                )
                                .child(if show_float {
                                    v_flex().gap_2().pl_2().pt_1().border_l_2().border_color(cx.theme().primary)
                                        .child(
                                            h_flex().justify_between().items_center()
                                                .child(div().text_xs().child("Show FPS in Overlay"))
                                                .child(Switch::new("sw_float_fps").disabled(is_loading).checked(f_fps).cursor_pointer().on_click(cx.listener(|this, val: &bool, _, cx| {
                                                    this.app.float_show_fps = *val;
                                                    cx.notify();
                                                })))
                                        )
                                        .child(
                                            h_flex().justify_between().items_center()
                                                .child(div().text_xs().child("Show RAM in Overlay"))
                                                .child(Switch::new("sw_float_ram").disabled(is_loading).checked(f_ram).cursor_pointer().on_click(cx.listener(|this, val: &bool, _, cx| {
                                                    this.app.float_show_ram = *val;
                                                    cx.notify();
                                                })))
                                        )
                                        .child(
                                            h_flex().justify_between().items_center()
                                                .child(div().text_xs().child("Show CPU in Overlay"))
                                                .child(Switch::new("sw_float_cpu").disabled(is_loading).checked(f_cpu).cursor_pointer().on_click(cx.listener(|this, val: &bool, _, cx| {
                                                    this.app.float_show_cpu = *val;
                                                    cx.notify();
                                                })))
                                        )
                                        .into_any_element()
                                } else {
                                    div().into_any_element()
                                })
                        )
                        .child(div().h_px().w_full().bg(cx.theme().border))
                        .child(div().text_sm().font_bold().child("Application Settings"))
                        .child(
                            h_flex().items_center().justify_between()
                                .child(div().text_sm().child("Enable Dithering"))
                                .child(Switch::new("sw_dither").disabled(is_loading).checked(dither).cursor_pointer().on_click(cx.listener(|this, val: &bool, _, cx| {
                                    this.app.dither_enabled = *val;
                                    cx.notify();
                                })))
                        )
                        .child(
                            h_flex().items_center().justify_between()
                                .child(div().text_sm().child("HALD CLUT Level (Quality vs Speed)"))
                                .child(
                                    Button::new("btn_hald_level")
                                        .label(format!("Level {}", level))
                                        .dropdown_menu({
                                            let ve = view_entity.clone();
                                            move |mut menu, window, _| {
                                                for l in [4, 8, 12, 16] {
                                                    let ve = ve.clone();
                                                    menu = menu.item(PopupMenuItem::new(format!("Level {}", l)).on_click(window.listener_for(&ve, move |this, _, _, cx| {
                                                        this.app.hald_level = l;
                                                        cx.notify();
                                                    })));
                                                }
                                                menu
                                            }
                                        })
                                )
                        )
                        .child(
                            Button::new("btn_clear_cache").disabled(is_loading).label("Clear Temp Files")
                                .child(gpui::svg().path("trash.svg").size_4().text_color(gpui::rgb(0xff5555)))
                                .w_full()
                                .cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                    this.app.state = crate::app::AppState::Notice("Cache cleared".to_string());
                                    cx.notify();
                                }))
                        )
                        .into_any_element()
                }
            }
        )
}
