//! Main visual workspace and preview component.

use crate::app::{WallmodApp, WorkspaceView};
use crate::ui::histogram::render_histogram;
use crate::ui::swatches::render_swatches;
use crate::ui::WallmodView;
use gpui::*;
use gpui_component::{
    button::*,
    h_flex,
    menu::{DropdownMenu as _, PopupMenuItem},
    scroll::ScrollableElement,
    v_flex, ActiveTheme, Disableable, Selectable, Sizable, StyledExt,
};

/// Renders the central workspace preview, split diff overlay, dashboard info, or album gallery.
pub fn render_workspace(
    view: &mut WallmodView,
    window: &mut Window,
    cx: &mut Context<WallmodView>,
) -> impl IntoElement {
    let is_loading = view.app.state.is_loading();
    let loading_msg = if let crate::app::AppState::Loading(_, ref s) = view.app.state {
        s.clone()
    } else {
        "Processing...".to_string()
    };
    let error_msg = if let crate::app::AppState::Error(ref e) = view.app.state {
        Some(e.clone())
    } else {
        None
    };
    let workspace_view = view.app.workspace_view;
    let preview_path = view.app.preview_path.clone();
    let base_path = view.app.base_image_path.clone();
    let img_name = view.app.image_filename.clone();
    let img_w = view.app.image_width;
    let img_h = view.app.image_height;
    let wcag = view.app.wcag_contrast;
    let albums = view.app.albums.clone();
    let current_theme = view.app.current_theme.clone();
    let hist_data = view.app.histogram_data.clone();
    let split_ratio = view.app.split_diff_ratio;
    let selected_album = view.app.selected_album.clone();
    let album_images = view.app.album_images.clone();

    v_flex()
        .relative()
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
                .child(
                    Button::new("wv_std").child(gpui::svg().path("eye.svg").size_4().text_color(cx.theme().primary)).child("Output Visual").small()
                        .selected(workspace_view == WorkspaceView::Standard)
                        .cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.workspace_view = WorkspaceView::Standard; cx.notify(); }))
                )
                .child(
                    Button::new("wv_diff").child(gpui::svg().path("frame.svg").size_4().text_color(cx.theme().primary)).child("Split Diff").small()
                        .selected(workspace_view == WorkspaceView::SplitDiff)
                        .cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.workspace_view = WorkspaceView::SplitDiff; cx.notify(); }))
                )
                .child(
                    Button::new("wv_tel").child(gpui::svg().path("layout-dashboard.svg").size_4().text_color(cx.theme().primary)).child("Dashboard Info").small()
                        .selected(workspace_view == WorkspaceView::Telemetry)
                        .cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.workspace_view = WorkspaceView::Telemetry; cx.notify(); }))
                )
                .child(
                    Button::new("wv_extract").child(gpui::svg().path("palette.svg").size_4().text_color(cx.theme().primary)).child("Extract Color").small()
                        .selected(view.app.workspace_view == WorkspaceView::ExtractColor)
                        .cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.workspace_view = WorkspaceView::ExtractColor; cx.notify(); }))
                )
                .child(
                    Button::new("wv_gal").child(gpui::svg().path("folder.svg").size_4().text_color(cx.theme().primary)).child("Album Gallery").small()
                        .selected(workspace_view == WorkspaceView::Albums)
                        .cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.workspace_view = WorkspaceView::Albums; cx.notify(); }))
                )
        )
        .child(
            v_flex().flex_1().w_full().p_6().items_center().justify_center().overflow_y_scrollbar()
                .child(
                    match workspace_view {
                        WorkspaceView::Standard => {
                            if let Some(path) = preview_path {
                                div()
                                    .size_full()
                                    .relative()
                                    .border_1()
                                    .border_color(cx.theme().border)
                                    .rounded_lg()
                                    .overflow_hidden()
                                    .child(img(path).size_full().object_fit(ObjectFit::Contain))
                                    .into_any_element()
                            } else {
                                v_flex().gap_4().items_center().justify_center().p_12().border_1().border_color(cx.theme().border).rounded_xl().bg(cx.theme().secondary)
                                    .child(gpui::svg().path("inbox.svg").size_12().text_color(cx.theme().muted_foreground))
                                    .child(div().text_lg().font_bold().child("No Image Loaded"))
                                    .child(div().text_sm().text_color(cx.theme().muted_foreground).child("Click 'Open Image...' in Color Grading tab to begin color grading."))
                                    .into_any_element()
                            }
                        }
                        WorkspaceView::SplitDiff => {
                            v_flex().size_full().gap_3()
                                .child(
                                    h_flex().gap_2().items_center().justify_center().w_full()
                                        .child(div().text_xs().font_bold().child("Split Comparison:"))
                                        .child(Button::new("split_10").disabled(is_loading).child(gpui::svg().path("eye.svg").size_4().text_color(cx.theme().primary)).child("10% Orig").small().selected((split_ratio - 0.1).abs() < 0.01).cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.split_diff_ratio = 0.1; cx.notify(); })))
                                        .child(Button::new("split_30").disabled(is_loading).child(gpui::svg().path("eye.svg").size_4().text_color(cx.theme().primary)).child("30% Orig").small().selected((split_ratio - 0.3).abs() < 0.01).cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.split_diff_ratio = 0.3; cx.notify(); })))
                                        .child(Button::new("split_50").disabled(is_loading).child(gpui::svg().path("eye.svg").size_4().text_color(cx.theme().primary)).child("50% / 50%").small().selected((split_ratio - 0.5).abs() < 0.01).cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.split_diff_ratio = 0.5; cx.notify(); })))
                                        .child(Button::new("split_70").disabled(is_loading).child(gpui::svg().path("eye.svg").size_4().text_color(cx.theme().primary)).child("70% Orig").small().selected((split_ratio - 0.7).abs() < 0.01).cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.split_diff_ratio = 0.7; cx.notify(); })))
                                        .child(Button::new("split_90").disabled(is_loading).child(gpui::svg().path("eye.svg").size_4().text_color(cx.theme().primary)).child("90% Orig").small().selected((split_ratio - 0.9).abs() < 0.01).cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.split_diff_ratio = 0.9; cx.notify(); })))
                                )
                                .child(
                                    h_flex().flex_1().w_full().gap_3().overflow_hidden()
                                        .child(
                                            v_flex().w(relative(split_ratio)).h_full().gap_1()
                                                .child(div().text_xs().font_bold().child("Original Base Image"))
                                                .child(
                                                    if let Some(bp) = base_path {
                                                        div().size_full().border_1().border_color(cx.theme().border).rounded_lg().overflow_hidden().child(img(bp).size_full().object_fit(ObjectFit::Contain)).into_any_element()
                                                    } else {
                                                        div().child("None").into_any_element()
                                                    }
                                                )
                                        )
                                        .child(div().w_1().h_full().bg(cx.theme().primary))
                                        .child(
                                            v_flex().w(relative(1.0 - split_ratio)).h_full().gap_1()
                                                .child(div().text_xs().font_bold().child("Processed Graded Output"))
                                                .child(
                                                    if let Some(pp) = preview_path {
                                                        div().size_full().border_1().border_color(cx.theme().border).rounded_lg().overflow_hidden().child(img(pp).size_full().object_fit(ObjectFit::Contain)).into_any_element()
                                                    } else {
                                                        div().child("None").into_any_element()
                                                    }
                                                )
                                        )
                                )
                                .into_any_element()
                        }
                        WorkspaceView::Telemetry => {
                            v_flex().gap_4().w_full().max_w(px(650.0)).p_6().border_1().border_color(cx.theme().border).rounded_xl().bg(cx.theme().secondary)
                                .child(div().text_lg().font_bold().child("Telemetry & Inspection Dashboard"))
                                .child(render_swatches(&current_theme, cx))
                                .child(render_histogram(hist_data.as_ref(), cx))
                                .child(
                                    h_flex().justify_between().pt_2()
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
                        WorkspaceView::ExtractColor => {
                            let extracted_cols = view.app.extracted_colors.clone();
                            v_flex().gap_4().size_full().p_6()
                                .child(
                                    h_flex().justify_between().items_center()
                                        .child(div().text_xl().font_bold().child("Dominant Color Analytics"))
                                        .child(
                                            h_flex().gap_2()
                                                .child(
                                                    if extracted_cols.is_some() {
                                                        Button::new("btn_apply_extracted").disabled(is_loading).child(gpui::svg().path("wand.svg").size_4().text_color(cx.theme().primary)).child("Use as Preset")
                                                            .primary()
                                                            .cursor_pointer()
                                                            .on_click(cx.listener(|this, _, _, cx| {
                                                                if let Some(extracted) = &this.app.extracted_colors {
                                                                    let colors: Vec<[u8; 3]> = extracted.iter().filter_map(|(hex, _)| {
                                                                        let s = hex.trim_start_matches('#');
                                                                        if s.len() == 6 {
                                                                            let r = u8::from_str_radix(&s[0..2], 16).ok()?;
                                                                            let g = u8::from_str_radix(&s[2..4], 16).ok()?;
                                                                            let b = u8::from_str_radix(&s[4..6], 16).ok()?;
                                                                            Some([r, g, b])
                                                                        } else {
                                                                            None
                                                                        }
                                                                    }).collect();
                                                                    let new_theme = crate::app::state::ThemeSource::CustomPalette("Extracted".to_string(), colors);
                                                                    this.app.apply_theme(new_theme);
                                                                    this.app.selected_preset = None;
                                                                    this.app.workspace_view = crate::app::state::WorkspaceView::PaletteEditor;
                                                                    this.app.selected_color_idx = None;
                                                                    this.trigger_async_processing(cx, "Applying extracted palette...");
                                                                }
                                                            }))
                                                            .into_any_element()
                                                    } else {
                                                        div().into_any_element()
                                                    }
                                                )
                                                .child(
                                                    Button::new("btn_extract_cols").disabled(is_loading).child(gpui::svg().path("palette.svg").size_4().text_color(cx.theme().primary)).child("Extract k-Means Colors")
                                                        .primary()
                                                        .cursor_pointer()
                                                        .on_click(cx.listener(|this, _, _, cx| {
                                                            this.trigger_async_extraction(cx);
                                                        }))
                                                )
                                        )
                                )
                                .child(div().h_px().w_full().bg(cx.theme().border))
                                .child(
                                    if let Some(cols) = extracted_cols {
                                        v_flex().gap_6().w_full()
                                            .child(
                                                h_flex().w_full().h(px(32.0)).rounded_xl().overflow_hidden().border_1().border_color(cx.theme().border.opacity(0.3))
                                                    .children(cols.iter().map(|(hex, pct)| {
                                                        let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(0);
                                                        let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(0);
                                                        let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(0);
                                                        div().h_full().bg(gpui::Rgba { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a: 1.0 })
                                                            .w(gpui::relative(if pct.is_nan() { 0.0 } else { pct.clamp(0.0, 1.0) }))
                                                    }))
                                            )
                                            .child(
                                                div().flex().flex_wrap().gap_4().w_full()
                                                    .children(cols.clone().into_iter().map(|(hex, pct)| {
                                                        let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(0);
                                                        let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(0);
                                                        let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(0);
                                                        let pct_display = format!("{:.1}%", pct * 100.0);

                                                        v_flex().gap_2().w(px(140.0)).p_3().border_1().border_color(cx.theme().border).rounded_lg().bg(cx.theme().secondary)
                                                            .child(
                                                                h_flex().gap_3().items_center()
                                                                    .child(div().w(px(16.0)).h(px(16.0)).rounded_full().bg(gpui::Rgba { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a: 1.0 }).border_1().border_color(cx.theme().border.opacity(0.5)))
                                                                    .child(div().text_sm().font_bold().child(hex))
                                                            )
                                                            .child(
                                                                h_flex().justify_between()
                                                                    .child(div().text_xs().text_color(cx.theme().muted_foreground).child(format!("{},{},{}", r, g, b)))
                                                                    .child(div().text_xs().font_bold().text_color(cx.theme().primary).child(pct_display))
                                                            )
                                                    }))
                                            )
                                            .child(div().w_full().h_px().bg(cx.theme().border))
                                            .child(div().text_sm().font_bold().child("Generated Shades (Tailwind-like)"))
                                            .child(
                                                v_flex().gap_4().w_full().children(cols.clone().into_iter().map(|(hex, _pct)| {
                                                    let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(0);
                                                    let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(0);
                                                    let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(0);

                                                    let shades = generate_tailwind_shades(r, g, b);

                                                    v_flex().gap_2().w_full()
                                                        .child(div().text_xs().font_bold().text_color(cx.theme().muted_foreground).child(hex.clone()))
                                                        .child(
                                                            h_flex().gap_1().w_full().h(px(40.0)).rounded_lg().overflow_hidden().border_1().border_color(cx.theme().border)
                                                                .children(shades.into_iter().map(|(label, rgb)| {
                                                                    let hex_shade = format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2]);
                                                                    v_flex().flex_1().h_full().bg(gpui::Rgba { r: rgb[0] as f32 / 255.0, g: rgb[1] as f32 / 255.0, b: rgb[2] as f32 / 255.0, a: 1.0 })
                                                                        .justify_center().items_center()
                                                                        .child(div().text_xs().font_bold().text_color(
                                                                            if rgb[0] as u32 + rgb[1] as u32 + rgb[2] as u32 > 382 { gpui::Rgba { r: 0.1, g: 0.1, b: 0.1, a: 1.0 } } else { gpui::Rgba { r: 0.9, g: 0.9, b: 0.9, a: 1.0 } }
                                                                        ).child(label))
                                                                        .child(div().text_xs().text_color(
                                                                            if rgb[0] as u32 + rgb[1] as u32 + rgb[2] as u32 > 382 { gpui::Rgba { r: 0.3, g: 0.3, b: 0.3, a: 1.0 } } else { gpui::Rgba { r: 0.7, g: 0.7, b: 0.7, a: 1.0 } }
                                                                        ).child(hex_shade))
                                                                }))
                                                        )
                                                }))
                                            )
                                            .into_any_element()
                                    } else {
                                        div().flex_1().flex().items_center().justify_center().text_color(cx.theme().muted_foreground)
                                            .child("No colors extracted. Click the button above to run the K-Means extraction algorithm.")
                                            .into_any_element()
                                    }
                                )
                                .into_any_element()
                        }
                        WorkspaceView::PaletteEditor => {
                            if let crate::app::state::ThemeSource::CustomPalette(ref name, ref colors) = current_theme {
                                let colors_clone = colors.clone();
                                let selected_idx = view.app.selected_color_idx;

                                v_flex().gap_4().size_full().p_6()
                                    .child(
                                        h_flex().justify_between().items_center().flex_wrap().gap_2()
                                            .child(div().text_xl().font_bold().child(format!("Edit Palette: {}", name)))
                                            .child(
                                                h_flex().gap_2()
                                                    .child(Button::new("pe_copy_raw").child("Copy Raw").small().outline().cursor_pointer().on_click(cx.listener(|view, _, _, cx| { view.copy_palette_to_clipboard(cx, "raw"); })))
                                                    .child(Button::new("pe_copy_json").child("Copy JSON").small().outline().cursor_pointer().on_click(cx.listener(|view, _, _, cx| { view.copy_palette_to_clipboard(cx, "json"); })))
                                                    .child(Button::new("pe_copy_obj").child("Copy Object").small().outline().cursor_pointer().on_click(cx.listener(|view, _, _, cx| { view.copy_palette_to_clipboard(cx, "object"); })))
                                                    .child(
                                                        Button::new("btn_apply_edited_palette").disabled(is_loading).child(gpui::svg().path("wand.svg").size_4().text_color(cx.theme().primary)).child("Apply Theme")
                                                            .primary()
                                                            .cursor_pointer()
                                                            .on_click(cx.listener(|this, _, _, cx| {
                                                                this.trigger_async_processing(cx, "Applying edited palette...");
                                                            }))
                                                    )
                                            )
                                    )
                                    .child(div().h_px().w_full().bg(cx.theme().border))
                                    .child(
                                        h_flex().gap_6().w_full().flex_1()
                                            .child(
                                                div().flex_1().flex().flex_wrap().gap_2().content_start()
                                                    .children(colors_clone.into_iter().enumerate().map(|(i, rgb)| {
                                                        let r = rgb[0] as f32 / 255.0;
                                                        let g = rgb[1] as f32 / 255.0;
                                                        let b = rgb[2] as f32 / 255.0;
                                                        let is_selected = selected_idx == Some(i);

                                                        div().id(("palette_color", i)).w(px(48.0)).h(px(48.0)).rounded_md().border_2()
                                                            .border_color(if is_selected { cx.theme().primary } else { cx.theme().border })
                                                            .bg(gpui::Rgba { r, g, b, a: 1.0 })
                                                            .cursor_pointer()
                                                            .on_click(cx.listener(move |view, _, _, cx| {
                                                                view.app.selected_color_idx = Some(i); view.app.needs_hex_sync = true; view.app.needs_slider_sync = true;

                                                                if let crate::app::state::ThemeSource::CustomPalette(_, ref colors) = view.app.current_theme {
                                                                    if let Some(c) = colors.get(i) {
                                                                        let r_slider = cx.new(|_| gpui_component::slider::SliderState::new().min(0.0).max(255.0).step(1.0).default_value(c[0] as f32));
                                                                        let g_slider = cx.new(|_| gpui_component::slider::SliderState::new().min(0.0).max(255.0).step(1.0).default_value(c[1] as f32));
                                                                        let b_slider = cx.new(|_| gpui_component::slider::SliderState::new().min(0.0).max(255.0).step(1.0).default_value(c[2] as f32));

                                                                        view.subscriptions.push(cx.subscribe(&r_slider, |this, _, event: &gpui_component::slider::SliderEvent, cx| if let gpui_component::slider::SliderEvent::Change(val) = event {
                                                                            if let Some(idx) = this.app.selected_color_idx {
                                                                                if let crate::app::state::ThemeSource::CustomPalette(_, ref mut colors) = this.app.current_theme {
                                                                                    if let Some(c) = colors.get_mut(idx) { c[0] = val.start() as u8; }
                                                                                }
                                                                            }
                                                                            cx.notify();
                                                                        }));
                                                                        view.subscriptions.push(cx.subscribe(&g_slider, |this, _, event: &gpui_component::slider::SliderEvent, cx| if let gpui_component::slider::SliderEvent::Change(val) = event {
                                                                            if let Some(idx) = this.app.selected_color_idx {
                                                                                if let crate::app::state::ThemeSource::CustomPalette(_, ref mut colors) = this.app.current_theme {
                                                                                    if let Some(c) = colors.get_mut(idx) { c[1] = val.start() as u8; }
                                                                                }
                                                                            }
                                                                            cx.notify();
                                                                        }));
                                                                        view.subscriptions.push(cx.subscribe(&b_slider, |this, _, event: &gpui_component::slider::SliderEvent, cx| if let gpui_component::slider::SliderEvent::Change(val) = event {
                                                                            if let Some(idx) = this.app.selected_color_idx {
                                                                                if let crate::app::state::ThemeSource::CustomPalette(_, ref mut colors) = this.app.current_theme {
                                                                                    if let Some(c) = colors.get_mut(idx) { c[2] = val.start() as u8; }
                                                                                }
                                                                            }
                                                                            cx.notify();
                                                                        }));

                                                                        view.palette_r_slider = r_slider;
                                                                        view.palette_g_slider = g_slider;
                                                                        view.palette_b_slider = b_slider;
                                                                    }
                                                                }
                                                                cx.notify();
                                                            }))
                                                    }))
                                                    .child(
                                                        h_flex().id("btn_add_color").w(px(48.0)).h(px(48.0)).rounded_md().border_2().border_color(cx.theme().border).border_dashed()
                                                            .justify_center().items_center()
                                                            .child(gpui::svg().path("plus.svg").size_6().text_color(cx.theme().muted_foreground))
                                                            .cursor_pointer()
                                                            .on_click(cx.listener(|view, _, _, cx| {
                                                                if let crate::app::state::ThemeSource::CustomPalette(_, ref mut colors) = view.app.current_theme {
                                                                    colors.push([128, 128, 128]);
                                                                    cx.notify();
                                                                }
                                                            }))
                                                    )
                                            )
                                            .child(
                                                if let Some(idx) = selected_idx {
                                                    if let Some(rgb) = current_theme.as_custom_palette().and_then(|c| c.1.get(idx).copied()) {
                                                        let hex = format!("#{:02X}{:02X}{:02X}", rgb[0], rgb[1], rgb[2]);
                                                        if view.app.needs_hex_sync {
                                                            view.palette_hex_input.update(cx, |input, cx| {
                                                                input.set_value(hex.clone(), window, cx);
                                                            });
                                                            view.app.needs_hex_sync = false;
                                                        }
                                                        if view.app.needs_slider_sync {
                                                            view.palette_r_slider.update(cx, |s, cx| s.set_value(rgb[0] as f32, window, cx));
                                                            view.palette_g_slider.update(cx, |s, cx| s.set_value(rgb[1] as f32, window, cx));
                                                            view.palette_b_slider.update(cx, |s, cx| s.set_value(rgb[2] as f32, window, cx));
                                                            view.app.needs_slider_sync = false;
                                                        }
                                                        v_flex().w(px(250.0)).gap_4().p_4().border_1().border_color(cx.theme().border).rounded_xl().bg(cx.theme().secondary)
                                                            .child(
                                                                h_flex().justify_between().items_center()
                                                                    .child(div().text_lg().font_bold().child("Modify Color"))
                                                                    .child(
                                                                        div().id("btn_delete_color").w(px(32.0)).h(px(32.0)).rounded_md().flex().justify_center().items_center()
                                                                            .hover(|this| this.bg(cx.theme().border))
                                                                            .child(gpui::svg().path("trash.svg").size_4().text_color(gpui::rgb(0xff5555)))
                                                                            .cursor_pointer()
                                                                            .on_click(cx.listener(move |view, _, _, cx| {
                                                                                if let crate::app::state::ThemeSource::CustomPalette(_, ref mut colors) = view.app.current_theme {
                                                                                    if colors.len() > 1 {
                                                                                        colors.remove(idx);
                                                                                        view.app.selected_color_idx = None;
                                                                                        cx.notify();
                                                                                    } else {
                                                                                        view.app.state = crate::app::AppState::Error("Cannot delete the last color.".to_string());
                                                                                        cx.notify();
                                                                                    }
                                                                                }
                                                                            }))
                                                                    )
                                                            )
                                                            .child(
                                                                h_flex().gap_3().items_center()
                                                                    .child(div().w(px(40.0)).h(px(40.0)).rounded_md().bg(gpui::Rgba { r: rgb[0] as f32/255.0, g: rgb[1] as f32/255.0, b: rgb[2] as f32/255.0, a: 1.0 }).border_1().border_color(cx.theme().border))
                                                                    .child(gpui_component::input::Input::new(&view.palette_hex_input))
                                                            )
                                                            .child(div().h_px().w_full().bg(cx.theme().border))
                                                            .child(
                                                                v_flex().gap_1()
                                                                    .child(h_flex().justify_between().child(div().text_sm().font_bold().text_color(gpui::rgb(0xff5555)).child("Red")).child(div().text_sm().child(format!("{}", rgb[0]))))
                                                                    .child(gpui_component::slider::Slider::new(&view.palette_r_slider))
                                                            )
                                                            .child(
                                                                v_flex().gap_1()
                                                                    .child(h_flex().justify_between().child(div().text_sm().font_bold().text_color(gpui::rgb(0x55ff55)).child("Green")).child(div().text_sm().child(format!("{}", rgb[1]))))
                                                                    .child(gpui_component::slider::Slider::new(&view.palette_g_slider))
                                                            )
                                                            .child(
                                                                v_flex().gap_1()
                                                                    .child(h_flex().justify_between().child(div().text_sm().font_bold().text_color(gpui::rgb(0x5555ff)).child("Blue")).child(div().text_sm().child(format!("{}", rgb[2]))))
                                                                    .child(gpui_component::slider::Slider::new(&view.palette_b_slider))
                                                            )
                                                            .into_any_element()
                                                    } else {
                                                        div().into_any_element()
                                                    }
                                                } else {
                                                    v_flex().w(px(250.0)).p_4().border_1().border_color(cx.theme().border).rounded_xl().bg(cx.theme().secondary).items_center().justify_center()
                                                        .child(div().text_sm().text_color(cx.theme().muted_foreground).text_center().child("Click a color block to modify it."))
                                                        .into_any_element()
                                                }
                                            )
                                    )
                                    .into_any_element()
                            } else {
                                v_flex().gap_4().size_full().p_6().items_center().justify_center()
                                    .child(div().text_lg().text_color(cx.theme().muted_foreground).child("Please select a Custom Palette or extract colors first."))
                                    .into_any_element()
                            }
                        }
                        WorkspaceView::Albums => {
                            v_flex().gap_4().size_full()
                                .child(
                                    h_flex().justify_between().items_center()
                                        .child(
                                            if let Some(ref sel) = selected_album {
                                                h_flex().gap_2().items_center()
                                                    .child(Button::new("btn_back_alb").disabled(is_loading).child(gpui::svg().path("arrow-left.svg").size_4().text_color(cx.theme().primary)).child("Back to Albums").small().cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                        this.app.selected_album = None;
                                                        this.app.album_images.clear();
                                                        cx.notify();
                                                    })))
                                                    .child(div().text_lg().font_bold().child(format!("Album: {}", sel.file_name().and_then(|n| n.to_str()).unwrap_or("Folder"))))
                                            } else {
                                                h_flex().child(div().text_lg().font_bold().child("System Wallpaper Albums"))
                                            }
                                        )
                                        .child(Button::new("btn_scan_gal").disabled(is_loading).child(gpui::svg().path("search.svg").size_4().text_color(cx.theme().primary)).child("Scan System Gallery").primary().cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                            this.app.albums = WallmodApp::scan_system_gallery();
                                            this.app.selected_album = None;
                                            cx.notify();
                                        })))
                                )
                                .child(
                                    if selected_album.is_some() {
                                        if album_images.is_empty() {
                                            div().p_8().text_center().text_color(cx.theme().muted_foreground).child("No images found in this album folder.").into_any_element()
                                        } else {
                                            div().flex_1().w_full().overflow_y_scrollbar()
                                                .child(
                                                    h_flex().flex_wrap().gap_4().children(album_images.iter().enumerate().map(|(idx, img_p)| {
                                                        let p_clone = img_p.clone();
                                                        let name = img_p.file_name().and_then(|n| n.to_str()).unwrap_or("Image").to_string();
                                                        div()
                                                            .id(format!("alb_img_{}", idx))
                                                            .w(px(180.0)).h(px(160.0))
                                                            .p_2().border_1().border_color(cx.theme().border).rounded_lg().bg(cx.theme().secondary)
                                                            .flex().flex_col().justify_between().items_center().cursor_pointer()
                                                            .cursor_pointer().on_click(cx.listener(move |this, _, _, cx| {
                                                                let p = p_clone.clone();
                                                                this.app.preview_path = Some(p.clone());
                                                                this.app.workspace_view = WorkspaceView::Standard;
                                                                cx.notify();
                                                                cx.spawn(async move |this, cx| {
                                                                    cx.background_executor().timer(std::time::Duration::from_millis(1500)).await;
                                                                    let _ = this.update(cx, |view, cx| {
                                                                        view.app.state = crate::app::AppState::Loading(0.2, "Loading full image...".to_string());
                                                                        cx.notify();
                                                                    });
                                                                    let res = crate::backend::runtime::spawn_blocking({ let p = p.clone(); move || crate::app::helpers::open_image(&p) }).await;
                                                                    match res {
                                                                        Ok(Ok(dyn_img)) => {
                                                                            let _ = this.update(cx, |view, cx| {
                                                                                view.app.on_image_selected(p, dyn_img);
                                                                                view.trigger_async_processing(cx, "Processing album image...");
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
                                                                }).detach();
                                                            }))
                                                            .child(div().w_full().h(px(110.0)).overflow_hidden().rounded_md().child(img(img_p.clone()).size_full().object_fit(ObjectFit::Cover)))
                                                            .child(div().text_xs().truncate().w_full().text_center().pt_1().child(name))
                                                    }))
                                                )
                                                .into_any_element()
                                        }
                                    } else if albums.is_empty() {
                                        div().p_8().text_center().text_color(cx.theme().muted_foreground).child("No albums loaded. Click 'Scan System Gallery' above to discover system wallpapers.").into_any_element()
                                    } else {
                                        div().flex_1().w_full().overflow_y_scrollbar()
                                            .child(
                                                h_flex().flex_wrap().gap_4().children(albums.iter().enumerate().map(|(idx, alb)| {
                                                    let folder_path = alb.folder_path.clone();
                                                    let name = alb.folder_name.clone();
                                                    let count = alb.image_count;
                                                    let cover = alb.cover_image.clone();
                                                    div()
                                                        .id(format!("alb_folder_{}", idx))
                                                        .w(px(220.0)).h(px(180.0))
                                                        .p_3().border_1().border_color(cx.theme().border).rounded_xl().bg(cx.theme().secondary)
                                                        .flex().flex_col().justify_between().cursor_pointer()
                                                        .cursor_pointer().on_click(cx.listener(move |this, _, _, cx| {
                                                            let imgs = WallmodApp::scan_album_images(&folder_path);
                                                            this.app.selected_album = Some(folder_path.clone());
                                                            this.app.album_images = imgs;
                                                            cx.notify();
                                                        }))
                                                        .child(
                                                            if let Some(cp) = cover {
                                                                div().w_full().h(px(120.0)).overflow_hidden().rounded_lg().child(img(cp).size_full().object_fit(ObjectFit::Cover)).into_any_element()
                                                            } else {
                                                                div().w_full().h(px(120.0)).flex().items_center().justify_center().bg(cx.theme().background).rounded_lg().child(gpui::svg().path("folder.svg").size_8().text_color(cx.theme().muted_foreground)).into_any_element()
                                                            }
                                                        )
                                                        .child(
                                                            h_flex().justify_between().items_center().w_full().pt_2()
                                                                .child(div().text_sm().font_bold().truncate().child(name))
                                                                .child(div().text_xs().text_color(cx.theme().muted_foreground).child(format!("{} imgs", count)))
                                                        )
                                                }))
                                            )
                                            .into_any_element()
                                    }
                                )
                                .into_any_element()
                        }
                        WorkspaceView::NodePipeline => {
                            let chain = view.app.theme_chain.clone();
                            let chaining_mode = view.app.chaining_mode;
                            let global_bd = view.app.global_bit_depth;
                            let auto_apply = view.app.auto_apply_nodes;

                            v_flex().size_full().gap_6().p_6().overflow_y_scrollbar()
                                .child(
                                    h_flex().justify_between().items_center().w_full().p_4().border_1().border_color(cx.theme().border).rounded_xl().bg(cx.theme().secondary)
                                        .child(
                                            v_flex().gap_1()
                                                .child(div().text_lg().font_bold().child("Node-Based Theme Visualization"))
                                                .child(div().text_sm().text_color(cx.theme().muted_foreground).child(
                                                    if chaining_mode { "Chaining Mode Active: Themes and effects are applied sequentially." }
                                                    else { "Explore Mode Active: Single theme exploration on original image." }
                                                ))
                                        )
                                        .child(
                                            h_flex().gap_2().flex_wrap()
                                                .child(Button::new("btn_apply_pipeline").disabled(is_loading).child(gpui::svg().path("check.svg").size_4().text_color(cx.theme().primary)).tooltip("▶ Apply Pipeline").small().primary().cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                    this.trigger_async_processing(cx, "Applying node pipeline...");
                                                })))
                                                .child(Button::new("btn_toggle_auto_apply").disabled(is_loading).label(if auto_apply { "⚡ Auto-Apply: ON" } else { "⚡ Auto-Apply: OFF" }).small().outline().selected(auto_apply).cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                    this.app.auto_apply_nodes = !this.app.auto_apply_nodes;
                                                    if this.app.auto_apply_nodes {
                                                        this.trigger_async_processing(cx, "Auto-apply enabled, rendering pipeline...");
                                                    } else {
                                                        cx.notify();
                                                    }
                                                })))
                                                .child(Button::new("btn_pipe_mode").disabled(is_loading).child(gpui::svg().path("replace.svg").size_4().text_color(cx.theme().primary)).label(if chaining_mode { "Switch to Explore Mode" } else { "Switch to Chaining Mode" }).small().outline().cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                    this.app.chaining_mode = !this.app.chaining_mode;
                                                    cx.notify();
                                                })))
                                                .child(Button::new("btn_toggle_tracker").disabled(is_loading).label(if view.app.show_progress_panel { "Tracker: ON" } else { "Tracker: OFF" }).small().outline().selected(view.app.show_progress_panel).cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                    this.app.show_progress_panel = !this.app.show_progress_panel;
                                                    cx.notify();
                                                })))
                                                .child(Button::new("btn_pipe_clear").disabled(is_loading).child(gpui::svg().path("close.svg").size_4().text_color(gpui::Rgba { r: 0.9, g: 0.2, b: 0.2, a: 1.0 })).tooltip("Reset Chain").small().outline().cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                    let init = crate::app::state::ThemeSource::Preset("Default".to_string());
                                                    this.app.theme_chain = vec![crate::app::state::ThemeChainNode { id: 1, op: crate::app::state::PipelineOp::Theme(init.clone(), 1.0), theme: init.clone(), enabled: true, bit_depth: crate::app::state::BitDepthStyle::Bit32 }];
                                                    this.app.current_theme = init;
                                                    this.app.selected_preset = Some("Default".to_string());
                                                    this.trigger_node_processing(cx, "Resetting chain...");
                                                })))
                                        )
                                )
                                .child(
                                    v_flex().gap_3().w_full().p_4().border_1().border_color(cx.theme().border).rounded_xl().bg(cx.theme().background)
                                        .child(div().text_xs().font_bold().text_color(cx.theme().muted_foreground).child("PIPELINE ACTIONS & REUSABILITY (SINGLE & BATCH PROCESSING)"))
                                        .child(
                                            div().flex().flex_wrap().gap_2().w_full()
                                                .child(Button::new("btn_add_theme").disabled(is_loading).child("+ Theme Grade").small().primary().cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                    let next_id = this.app.theme_chain.iter().map(|n| n.id).max().unwrap_or(0) + 1;
                                                    let theme = this.app.current_theme.clone();
                                                    this.app.theme_chain.push(crate::app::state::ThemeChainNode {
                                                        id: next_id,
                                                        op: crate::app::state::PipelineOp::Theme(theme.clone(), 1.0),
                                                        theme,
                                                        enabled: true,
                                                        bit_depth: this.app.global_bit_depth,
                                                    });
                                                    this.app.chaining_mode = true;
                                                    this.trigger_node_processing(cx, "Added Theme Grade step...");
                                                })))
                                                .child(Button::new("btn_add_blur").disabled(is_loading).child("+ Blur Step").small().outline().cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                    let next_id = this.app.theme_chain.iter().map(|n| n.id).max().unwrap_or(0) + 1;
                                                    let theme = this.app.current_theme.clone();
                                                    this.app.theme_chain.push(crate::app::state::ThemeChainNode {
                                                        id: next_id,
                                                        op: crate::app::state::PipelineOp::Blur(10.0),
                                                        theme,
                                                        enabled: true,
                                                        bit_depth: this.app.global_bit_depth,
                                                    });
                                                    this.app.chaining_mode = true;
                                                    this.trigger_node_processing(cx, "Added Blur step...");
                                                })))
                                                .child(Button::new("btn_add_ps").disabled(is_loading).child("+ Color Adjust").small().outline().cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                    let next_id = this.app.theme_chain.iter().map(|n| n.id).max().unwrap_or(0) + 1;
                                                    let theme = this.app.current_theme.clone();
                                                    this.app.theme_chain.push(crate::app::state::ThemeChainNode {
                                                        id: next_id,
                                                        op: crate::app::state::PipelineOp::Photoshop(crate::modules::photoshop::PhotoshopParams {
                                                            brightness: 10,
                                                            contrast: 10.0,
                                                            saturation: 0.2,
                                                            hue: 0,
                                                        }),
                                                        theme,
                                                        enabled: true,
                                                        bit_depth: this.app.global_bit_depth,
                                                    });
                                                    this.app.chaining_mode = true;
                                                    this.trigger_node_processing(cx, "Added Color Adjust step...");
                                                })))
                                                .child(Button::new("btn_add_dither").disabled(is_loading).child("+ Dither").small().outline().cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                    let next_id = this.app.theme_chain.iter().map(|n| n.id).max().unwrap_or(0) + 1;
                                                    let theme = this.app.current_theme.clone();
                                                    this.app.theme_chain.push(crate::app::state::ThemeChainNode {
                                                        id: next_id,
                                                        op: crate::app::state::PipelineOp::Dither,
                                                        theme,
                                                        enabled: true,
                                                        bit_depth: this.app.global_bit_depth,
                                                    });
                                                    this.app.chaining_mode = true;
                                                    this.trigger_node_processing(cx, "Added Dithering step...");
                                                })))
                                                .child(Button::new("btn_add_sort").disabled(is_loading).child("+ Pixel Sort").small().outline().cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                    let next_id = this.app.theme_chain.iter().map(|n| n.id).max().unwrap_or(0) + 1;
                                                    let theme = this.app.current_theme.clone();
                                                    this.app.theme_chain.push(crate::app::state::ThemeChainNode {
                                                        id: next_id,
                                                        op: crate::app::state::PipelineOp::PixelSort,
                                                        theme,
                                                        enabled: true,
                                                        bit_depth: this.app.global_bit_depth,
                                                    });
                                                    this.app.chaining_mode = true;
                                                    this.trigger_node_processing(cx, "Added Pixel Sort step...");
                                                })))
                                                .child(Button::new("btn_add_gowall").disabled(is_loading).child("+ Gowall (Invert)").small().outline().cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                    let next_id = this.app.theme_chain.iter().map(|n| n.id).max().unwrap_or(0) + 1;
                                                    let theme = this.app.current_theme.clone();
                                                    this.app.theme_chain.push(crate::app::state::ThemeChainNode {
                                                        id: next_id,
                                                        op: crate::app::state::PipelineOp::Gowall(crate::app::gowall_state::GowallTool::Effects, "invert".to_string()),
                                                        theme,
                                                        enabled: true,
                                                        bit_depth: this.app.global_bit_depth,
                                                    });
                                                    this.app.chaining_mode = true;
                                                    this.trigger_node_processing(cx, "Added Gowall step...");
                                                })))
                                                .child(Button::new("btn_add_wgsl").disabled(is_loading).child("+ WGSL Shader").small().outline().cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                    let next_id = this.app.theme_chain.iter().map(|n| n.id).max().unwrap_or(0) + 1;
                                                    let theme = this.app.current_theme.clone();
                                                    this.app.theme_chain.push(crate::app::state::ThemeChainNode {
                                                        id: next_id,
                                                        op: crate::app::state::PipelineOp::Shader("CRT Scanlines".to_string(), [1.0, 1.0, 1.0, 1.0]),
                                                        theme,
                                                        enabled: true,
                                                        bit_depth: this.app.global_bit_depth,
                                                    });
                                                    this.app.chaining_mode = true;
                                                    this.trigger_node_processing(cx, "Added Shader step...");
                                                })))
                                                .child(div().w_px().h_6().bg(cx.theme().border))
                                                .child(Button::new("btn_export_pipe").disabled(is_loading).tooltip("Export Pipeline").small().outline().cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                    let chain_str = crate::app::state::export_pipeline_to_string(&this.app.theme_chain);
                                                    cx.spawn(async move |this, cx| {
                                                        if let Some(handle) = rfd::AsyncFileDialog::new().set_file_name("pipeline.wallpipe").save_file().await {
                                                            let _ = std::fs::write(handle.path(), chain_str);
                                                            let _ = this.update(cx, |view, cx| {
                                                                view.app.state = crate::app::AppState::Notice("Pipeline exported successfully!".to_string());
                                                                cx.notify();
                                                            });
                                                        }
                                                    }).detach();
                                                })))
                                                .child(Button::new("btn_import_pipe").disabled(is_loading).tooltip("Import Pipeline").small().outline().cursor_pointer().on_click(cx.listener(|_this, _, _, cx| {
                                                    cx.spawn(async move |this, cx| {
                                                        if let Some(handle) = rfd::AsyncFileDialog::new().add_filter("Wallmod Pipeline", &["wallpipe", "json"]).pick_file().await {
                                                            if let Ok(content) = std::fs::read_to_string(handle.path()) {
                                                                let imported = crate::app::state::import_pipeline_from_string(&content);
                                                                if !imported.is_empty() {
                                                                    let _ = this.update(cx, |view, cx| {
                                                                        view.app.theme_chain = imported;
                                                                        view.app.chaining_mode = true;
                                                                        view.trigger_node_processing(cx, "Imported pipeline chain loaded...");
                                                                    });
                                                                }
                                                            }
                                                        }
                                                    }).detach();
                                                })))
                                        )
                                )
                                .child(
                                    div().flex().flex_wrap().gap_4().items_center().w_full().pb_6()
                                        // Node 0: Original Image Source
                                        .child(
                                            v_flex().w(px(260.0)).p_4().border_2().border_color(cx.theme().primary).rounded_xl().bg(cx.theme().background).gap_3()
                                                .child(h_flex().gap_2().items_center().child(gpui::svg().path("folder.svg").size_5().text_color(cx.theme().primary)).child(div().font_bold().child("Input Source")))
                                                .child(div().text_xs().text_color(cx.theme().muted_foreground).child("Original un-graded wallpaper image data"))
                                                .child(div().text_xs().font_bold().p_2().bg(cx.theme().secondary).rounded_md().child({
                                                    if let Some(p) = &preview_path {
                                                        let s = p.file_name().unwrap_or_default().to_string_lossy().to_string();
                                                        if s.len() > 24 { format!("{}...", &s[..21]) } else { s }
                                                    } else {
                                                        "No file".to_string()
                                                    }
                                                }))
                                        )
                                        .child(div().text_xl().font_bold().text_color(cx.theme().primary).child("➔"))
                                        // Chain nodes
                                        .children(chain.iter().map(|node| {
                                            let nid = node.id;
                                            let nname = node.op.display_name();
                                            let short_name = if nname.len() > 22 { format!("{}...", &nname[..19]) } else { nname };
                                            let nenabled = node.enabled;
                                            let nbd = node.bit_depth;
                                            let node_el = v_flex().w(px(320.0)).p_4().border_1().border_color(if nenabled { cx.theme().primary } else { cx.theme().border }).rounded_xl().bg(cx.theme().background).gap_3().opacity(if nenabled { 1.0 } else { 0.5 })
                                                .child(
                                                    h_flex().justify_between().items_center()
                                                        .child(h_flex().gap_2().items_center().child(gpui::svg().path("palette.svg").size_4().text_color(cx.theme().primary)).child(div().font_bold().text_xs().child(format!("#{}: {}", nid, short_name))))
                                                        .child(Button::new(format!("toggle_n_{}", nid)).disabled(is_loading).child(gpui::svg().path(if nenabled { "check.svg" } else { "close.svg" }).size_4().text_color(if nenabled { cx.theme().primary } else { cx.theme().muted_foreground })).small().outline().cursor_pointer().on_click(cx.listener(move |this, _, _, cx| {
                                                            if let Some(n) = this.app.theme_chain.iter_mut().find(|x| x.id == nid) {
                                                                n.enabled = !n.enabled;
                                                            }
                                                            this.trigger_node_processing(cx, "Toggling node...");
                                                        })))
                                                )
                                                .child(div().text_xs().text_color(cx.theme().muted_foreground).child(format!("Color Style: {}", nbd.display_name())))
                                                .child(
                                                    h_flex().gap_1().w_full().pt_1()
                                                        .child(Button::new(format!("bd32_{}", nid)).disabled(is_loading).child("32b").small().flex_1().selected(nbd == crate::app::state::BitDepthStyle::Bit32).cursor_pointer().on_click(cx.listener(move |this, _, _, cx| {
                                                            if let Some(n) = this.app.theme_chain.iter_mut().find(|x| x.id == nid) { n.bit_depth = crate::app::state::BitDepthStyle::Bit32; }
                                                            this.trigger_node_processing(cx, "Updating node bit depth...");
                                                        })))
                                                        .child(Button::new(format!("bd16_{}", nid)).disabled(is_loading).child("16b").small().flex_1().selected(nbd == crate::app::state::BitDepthStyle::Bit16).cursor_pointer().on_click(cx.listener(move |this, _, _, cx| {
                                                            if let Some(n) = this.app.theme_chain.iter_mut().find(|x| x.id == nid) { n.bit_depth = crate::app::state::BitDepthStyle::Bit16; }
                                                            this.trigger_node_processing(cx, "Updating node bit depth...");
                                                        })))
                                                        .child(Button::new(format!("bd8_{}", nid)).disabled(is_loading).child("8b").small().flex_1().selected(nbd == crate::app::state::BitDepthStyle::Bit8).cursor_pointer().on_click(cx.listener(move |this, _, _, cx| {
                                                            if let Some(n) = this.app.theme_chain.iter_mut().find(|x| x.id == nid) { n.bit_depth = crate::app::state::BitDepthStyle::Bit8; }
                                                            this.trigger_node_processing(cx, "Updating node bit depth...");
                                                        })))
                                                )

                                                .child(
                                                    match &node.op {
                                                        crate::app::state::PipelineOp::Theme(_, op) => {
                                                            h_flex().gap_2().items_center().justify_between()
                                                                .child(div().text_xs().child(format!("Opacity: {:.0}%", op * 100.0)))
                                                                .child(h_flex().gap_1()
                                                                    .child(Button::new(format!("thm_sub_{}", nid)).child("-").small().outline().on_click(cx.listener(move |this, _, _, cx| {
                                                                        if let Some(n) = this.app.theme_chain.iter_mut().find(|x| x.id == nid) {
                                                                            if let crate::app::state::PipelineOp::Theme(_, o) = &mut n.op { *o = (*o - 0.1).max(0.0); }
                                                                        }
                                                                        this.trigger_node_processing(cx, "Tweaking opacity...");
                                                                    })))
                                                                    .child(Button::new(format!("thm_add_{}", nid)).child("+").small().outline().on_click(cx.listener(move |this, _, _, cx| {
                                                                        if let Some(n) = this.app.theme_chain.iter_mut().find(|x| x.id == nid) {
                                                                            if let crate::app::state::PipelineOp::Theme(_, o) = &mut n.op { *o = (*o + 0.1).min(1.0); }
                                                                        }
                                                                        this.trigger_node_processing(cx, "Tweaking opacity...");
                                                                    })))
                                                                ).into_any_element()
                                                        },
                                                        crate::app::state::PipelineOp::Blur(sigma) => {
                                                            h_flex().gap_2().items_center().justify_between()
                                                                .child(div().text_xs().child(format!("Blur (Sigma): {:.1}", sigma)))
                                                                .child(h_flex().gap_1()
                                                                    .child(Button::new(format!("blr_sub_{}", nid)).child("-").small().outline().on_click(cx.listener(move |this, _, _, cx| {
                                                                        if let Some(n) = this.app.theme_chain.iter_mut().find(|x| x.id == nid) {
                                                                            if let crate::app::state::PipelineOp::Blur(s) = &mut n.op { *s = (*s - 1.0).max(0.0); }
                                                                        }
                                                                        this.trigger_node_processing(cx, "Tweaking blur...");
                                                                    })))
                                                                    .child(Button::new(format!("blr_add_{}", nid)).child("+").small().outline().on_click(cx.listener(move |this, _, _, cx| {
                                                                        if let Some(n) = this.app.theme_chain.iter_mut().find(|x| x.id == nid) {
                                                                            if let crate::app::state::PipelineOp::Blur(s) = &mut n.op { *s = *s + 1.0; }
                                                                        }
                                                                        this.trigger_node_processing(cx, "Tweaking blur...");
                                                                    })))
                                                                ).into_any_element()
                                                        },
                                                        crate::app::state::PipelineOp::Photoshop(p) => {
                                                            h_flex().gap_2().items_center().justify_between()
                                                                .child(div().text_xs().child(format!("B: {}, C: {:.0}%", p.brightness, p.contrast)))
                                                                .child(h_flex().gap_1()
                                                                    .child(Button::new(format!("ps_sub_{}", nid)).child("-").small().outline().on_click(cx.listener(move |this, _, _, cx| {
                                                                        if let Some(n) = this.app.theme_chain.iter_mut().find(|x| x.id == nid) {
                                                                            if let crate::app::state::PipelineOp::Photoshop(p) = &mut n.op { p.brightness -= 1; }
                                                                        }
                                                                        this.trigger_node_processing(cx, "Tweaking PS...");
                                                                    })))
                                                                    .child(Button::new(format!("ps_add_{}", nid)).child("+").small().outline().on_click(cx.listener(move |this, _, _, cx| {
                                                                        if let Some(n) = this.app.theme_chain.iter_mut().find(|x| x.id == nid) {
                                                                            if let crate::app::state::PipelineOp::Photoshop(p) = &mut n.op { p.brightness += 1; }
                                                                        }
                                                                        this.trigger_node_processing(cx, "Tweaking PS...");
                                                                    })))
                                                                ).into_any_element()
                                                        },
                                                        crate::app::state::PipelineOp::Gowall(tool, param) => {
                                                            let ve = cx.entity().clone();
                                                            let param_clone = param.clone();
                                                            let tool_clone = *tool;
                                                            let is_recolor = *tool == crate::app::gowall_state::GowallTool::Recolor;

                                                            let tool_name = match tool_clone {
                                                                crate::app::gowall_state::GowallTool::Recolor => "Recolor",
                                                                crate::app::gowall_state::GowallTool::Effects => "Effects",
                                                                crate::app::gowall_state::GowallTool::Upscale => "Upscale",
                                                                crate::app::gowall_state::GowallTool::PixelArt => "PixelArt",
                                                                crate::app::gowall_state::GowallTool::ReplaceColor => "Remove BG",
                                                                crate::app::gowall_state::GowallTool::Resize => "Resize",
                                                                crate::app::gowall_state::GowallTool::Extract => "Extract",
                                                                crate::app::gowall_state::GowallTool::Compress => "Compress",

                                                                _ => "Unknown"
                                                            };

                                                            v_flex().gap_2().w_full().child(
                                                                h_flex().gap_2().items_center().justify_between()
                                                                    .child(div().text_xs().child("Tool:"))
                                                                    .child(
                                                                        Button::new(format!("gw_tool_dd_{}", nid)).disabled(is_loading)
                                                                            .label(tool_name)
                                                                            .small().outline()
                                                                            .dropdown_menu({ let ve = ve.clone(); move |mut menu, window, _| {
                                                                                let tools = [
                                                                                    (crate::app::gowall_state::GowallTool::Recolor, "Recolor"),
                                                                                    (crate::app::gowall_state::GowallTool::Effects, "Effects"),
                                                                                    (crate::app::gowall_state::GowallTool::Upscale, "Upscale"),
                                                                                    (crate::app::gowall_state::GowallTool::PixelArt, "PixelArt"),
                                                                                    (crate::app::gowall_state::GowallTool::ReplaceColor, "Remove BG"),
                                                                                    (crate::app::gowall_state::GowallTool::Resize, "Resize"),
                                                                                ];
                                                                                for (t, name) in tools {
                                                                                    let ve = ve.clone();
                                                                                    menu = menu.item(
                                                                                        PopupMenuItem::new(name)
                                                                                            .on_click(window.listener_for(&ve, move |this, _, _, cx| {
                                                                                                if let Some(n) = this.app.theme_chain.iter_mut().find(|x| x.id == nid) {
                                                                                                    if let crate::app::state::PipelineOp::Gowall(ref mut current_tool, ref mut p) = n.op {
                                                                                                        *current_tool = t;
                                                                                                        // set a reasonable default param
                                                                                                        if t == crate::app::gowall_state::GowallTool::Recolor {
                                                                                                            *p = "catppuccin-mocha".to_string();
                                                                                                        } else if t == crate::app::gowall_state::GowallTool::Effects {
                                                                                                            *p = "invert".to_string();
                                                                                                        } else {
                                                                                                            *p = "".to_string();
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                this.trigger_node_processing(cx, "Changed Gowall tool...");
                                                                                            }))
                                                                                    );
                                                                                }
                                                                                menu
                                                                            }})
                                                                    )
                                                            ).child(
                                                                h_flex().gap_2().items_center().justify_between()
                                                                    .child(div().text_xs().child("Config:"))
                                                                    .child(
                                                                        Button::new(format!("gw_dd_{}", nid)).disabled(is_loading)
                                                                            .label(if param_clone.is_empty() { "N/A".to_string() } else { param_clone })
                                                                            .small().outline()
                                                                            .dropdown_menu(move |mut menu, window, _| {
                                                                                let options = if is_recolor {
                                                                                    crate::app::state::PRESET_NAMES.iter().map(|s| s.to_string()).collect::<Vec<_>>()
                                                                                } else if tool_clone == crate::app::gowall_state::GowallTool::Effects {
                                                                                    vec!["invert".to_string(), "grayscale".to_string(), "flip".to_string(), "mirror".to_string()]
                                                                                } else {
                                                                                    vec![]
                                                                                };
                                                                                for opt in options {
                                                                                    let o = opt.clone();
                                                                                    let ve = ve.clone();
                                                                                    menu = menu.item(
                                                                                        PopupMenuItem::new(opt)
                                                                                            .on_click(window.listener_for(&ve, move |this, _, _, cx| {
                                                                                                if let Some(n) = this.app.theme_chain.iter_mut().find(|x| x.id == nid) {
                                                                                                    if let crate::app::state::PipelineOp::Gowall(_, ref mut p) = n.op {
                                                                                                        *p = o.clone();
                                                                                                    }
                                                                                                }
                                                                                                this.trigger_node_processing(cx, "Tweaking Gowall param...");
                                                                                            }))
                                                                                    );
                                                                                }
                                                                                menu
                                                                            })
                                                                    )
                                                            ).into_any_element()
                                                        },
                                                        crate::app::state::PipelineOp::Shader(name, params) => {
                                                            let ve = cx.entity().clone();
                                                            let name_clone = name.clone();
                                                            if !view.shader_inputs.contains_key(&nid) {
                                                                let arr = [
                                                                    cx.new(|cx| gpui_component::input::InputState::new(window, cx).default_value(&params[0].to_string())),
                                                                    cx.new(|cx| gpui_component::input::InputState::new(window, cx).default_value(&params[1].to_string())),
                                                                    cx.new(|cx| gpui_component::input::InputState::new(window, cx).default_value(&params[2].to_string())),
                                                                    cx.new(|cx| gpui_component::input::InputState::new(window, cx).default_value(&params[3].to_string())),
                                                                ];
                                                                view.shader_inputs.insert(nid, arr);
                                                            }
                                                            let inputs = view.shader_inputs.get(&nid).unwrap().clone();
                                                            let labels = crate::backend::shaders::get_shader_param_labels(&name_clone);
                                                            v_flex().w_full().gap_2()
                                                                .child(
                                                                    Button::new(format!("dd_shader_{}", nid)).disabled(is_loading)
                                                                        .label(name_clone.clone())
                                                                        .small().outline()
                                                                        .dropdown_menu({ let ve = ve.clone(); move |mut menu, window, _| {
                                                                            for (preset_name, _) in crate::backend::shaders::SHADER_PRESETS {
                                                                                let preset = preset_name.to_string();
                                                                                let ve = ve.clone();
                                                                                menu = menu.item(
                                                                                    PopupMenuItem::new(preset.clone())
                                                                                        .on_click(window.listener_for(&ve, move |this, _, _, cx| {
                                                                                            if let Some(n) = this.app.theme_chain.iter_mut().find(|x| x.id == nid) {
                                                                                                if let crate::app::state::PipelineOp::Shader(_, current_p) = n.op {
                                                                                                    n.op = crate::app::state::PipelineOp::Shader(preset.clone(), current_p);
                                                                                                } else {
                                                                                                    n.op = crate::app::state::PipelineOp::Shader(preset.clone(), [1.0, 1.0, 1.0, 1.0]);
                                                                                                }
                                                                                            }
                                                                                            this.trigger_node_processing(cx, "Tweaking Shader preset...");
                                                                                        }))
                                                                                );
                                                                            }
                                                                            menu
                                                                        }})
                                                                )
                                                                .child(h_flex().gap_2().items_center().w_full()
                                                                    .child(v_flex().w_full().child(div().text_xs().text_color(cx.theme().muted).child(labels[0])).child(gpui_component::input::Input::new(&inputs[0])))
                                                                    .child(v_flex().w_full().child(div().text_xs().text_color(cx.theme().muted).child(labels[1])).child(gpui_component::input::Input::new(&inputs[1])))
                                                                    .child(v_flex().w_full().child(div().text_xs().text_color(cx.theme().muted).child(labels[2])).child(gpui_component::input::Input::new(&inputs[2])))
                                                                    .child(v_flex().w_full().child(div().text_xs().text_color(cx.theme().muted).child(labels[3])).child(gpui_component::input::Input::new(&inputs[3])))
                                                                )
                                                                .child(Button::new(format!("shd_apply_{}", nid)).child("Apply Params").small().outline().on_click(cx.listener(move |this, _, _, cx| {
                                                                    if let Some(ins) = this.shader_inputs.get(&nid) {
                                                                        let p0 = ins[0].read(cx).text().to_string().parse::<f32>().unwrap_or(0.0);
                                                                        let p1 = ins[1].read(cx).text().to_string().parse::<f32>().unwrap_or(0.0);
                                                                        let p2 = ins[2].read(cx).text().to_string().parse::<f32>().unwrap_or(0.0);
                                                                        let p3 = ins[3].read(cx).text().to_string().parse::<f32>().unwrap_or(0.0);
                                                                        if let Some(n) = this.app.theme_chain.iter_mut().find(|x| x.id == nid) {
                                                                            if let crate::app::state::PipelineOp::Shader(_, ref mut p) = n.op {
                                                                                *p = [p0, p1, p2, p3];
                                                                            }
                                                                        }
                                                                        this.trigger_node_processing(cx, "Applying Shader params...");
                                                                    }
                                                                }))).into_any_element()
                                                        },
                                                        _ => div().into_any_element()
                                                    }
                                                )
                                                .child(
                                                    v_flex().gap_2().w_full().pt_1()
                                                        .child(
                                                            h_flex().gap_2().w_full()
                                                                .child(Button::new(format!("mv_l_{}", nid)).disabled(is_loading).child(gpui::svg().path("arrow-left.svg").size_3().text_color(cx.theme().primary)).tooltip("Move Left").small().outline().flex_1().cursor_pointer().on_click(cx.listener(move |this, _, _, cx| {
                                                                    if let Some(pos) = this.app.theme_chain.iter().position(|x| x.id == nid) {
                                                                        if pos > 0 {
                                                                            this.app.theme_chain.swap(pos, pos - 1);
                                                                            this.trigger_node_processing(cx, "Reordering pipeline step...");
                                                                        }
                                                                    }
                                                                })))
                                                                .child(Button::new(format!("mv_r_{}", nid)).disabled(is_loading).child(gpui::svg().path("arrow-right.svg").size_3().text_color(cx.theme().primary)).tooltip("Move Right").small().outline().flex_1().cursor_pointer().on_click(cx.listener(move |this, _, _, cx| {
                                                                    if let Some(pos) = this.app.theme_chain.iter().position(|x| x.id == nid) {
                                                                        if pos + 1 < this.app.theme_chain.len() {
                                                                            this.app.theme_chain.swap(pos, pos + 1);
                                                                            this.trigger_node_processing(cx, "Reordering pipeline step...");
                                                                        }
                                                                    }
                                                                })))
                                                        )
                                                        .child(
                                                            h_flex().gap_2().w_full()
                                                                .child(Button::new(format!("dup_{}", nid)).disabled(is_loading).child(gpui::svg().path("copy.svg").size_3().text_color(cx.theme().primary)).tooltip("Duplicate").small().outline().flex_1().cursor_pointer().on_click(cx.listener(move |this, _, _, cx| {
                                                                    if let Some(pos) = this.app.theme_chain.iter().position(|x| x.id == nid) {
                                                                        let mut cloned = this.app.theme_chain[pos].clone();
                                                                        let max_id = this.app.theme_chain.iter().map(|x| x.id).max().unwrap_or(0);
                                                                        cloned.id = max_id + 1;
                                                                        this.app.theme_chain.insert(pos + 1, cloned);
                                                                        this.trigger_node_processing(cx, "Duplicating node...");
                                                                    }
                                                                })))
                                                                .child(Button::new(format!("del_n_{}", nid)).disabled(is_loading).child(gpui::svg().path("trash.svg").size_3().text_color(gpui::Rgba { r: 0.9, g: 0.2, b: 0.2, a: 1.0 })).tooltip("Delete").small().outline().flex_1().cursor_pointer().on_click(cx.listener(move |this, _, _, cx| {
                                                                    this.app.theme_chain.retain(|x| x.id != nid);
                                                                    this.trigger_node_processing(cx, "Removing node from chain...");
                                                                })))
                                                        )
                                                );
                                            h_flex().gap_4().items_center().child(node_el).child(div().text_xl().font_bold().text_color(cx.theme().primary).child("➔")).into_any_element()
                                        }))
                                        // Final Output Node
                                        .child(
                                            v_flex().w(px(260.0)).p_4().border_2().border_color(cx.theme().primary).rounded_xl().bg(cx.theme().secondary).gap_3()
                                                .child(h_flex().gap_2().items_center().child(gpui::svg().path("check.svg").size_5().text_color(cx.theme().primary)).child(div().font_bold().child("Final Output")))
                                                .child(div().text_xs().text_color(cx.theme().muted_foreground).child(format!("Global Output Bit-Depth: {}", global_bd.display_name())))
                                                .child(Button::new("btn_view_out").disabled(is_loading).tooltip("View Output Visual").child(gpui::svg().path("check.svg").size_4().text_color(cx.theme().primary)).small().primary().w_full().cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                    this.app.workspace_view = crate::app::state::WorkspaceView::Standard;
                                                    cx.notify();
                                                })))
                                        )
                                )
                                .into_any_element()
                        }
                        WorkspaceView::Gowall => {
                            crate::ui::gowall_tab::render_gowall_tab(view, cx).into_any_element()
                        }
                    }
                )
        )
        .children(if is_loading {
            Some(
                v_flex()
                    .absolute()
                    .inset_0()
                    .bg(gpui::Rgba { r: 0.04, g: 0.04, b: 0.06, a: 0.35 })
                    .items_center()
                    .justify_center()
                    .child(
                        v_flex()
                            .px_8()
                            .py_6()
                            .rounded_2xl()
                            .bg(cx.theme().background.opacity(0.96))
                            .border_1()
                            .border_color(cx.theme().primary.opacity(0.4))
                            .shadow_2xl()
                            .items_center()
                            .gap_4()
                            .child(
                                h_flex()
                                    .gap_4()
                                    .items_center()
                                    .child(div().rounded_full().bg(gpui::Rgba { r: 0.2, g: 0.8, b: 1.0, a: 0.9 })
                                        .with_animation(
                                            "dot1",
                                            Animation::new(std::time::Duration::from_secs_f32(1.2)).repeat(),
                                            |this, delta| {
                                                let scale = 1.0 + (delta * std::f32::consts::PI * 2.0).sin() * 0.5;
                                                this.w(px(14.0 * scale)).h(px(14.0 * scale))
                                            }
                                        )
                                    )
                                    .child(div().rounded_full().bg(gpui::Rgba { r: 0.6, g: 0.4, b: 1.0, a: 0.9 })
                                        .with_animation(
                                            "dot2",
                                            Animation::new(std::time::Duration::from_secs_f32(1.2)).repeat(),
                                            |this, delta| {
                                                let scale = 1.0 + ((delta + 0.33) * std::f32::consts::PI * 2.0).sin() * 0.5;
                                                this.w(px(14.0 * scale)).h(px(14.0 * scale))
                                            }
                                        )
                                    )
                                    .child(div().rounded_full().bg(gpui::Rgba { r: 1.0, g: 0.3, b: 0.6, a: 0.9 })
                                        .with_animation(
                                            "dot3",
                                            Animation::new(std::time::Duration::from_secs_f32(1.2)).repeat(),
                                            |this, delta| {
                                                let scale = 1.0 + ((delta + 0.66) * std::f32::consts::PI * 2.0).sin() * 0.5;
                                                this.w(px(14.0 * scale)).h(px(14.0 * scale))
                                            }
                                        )
                                    )
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_bold()
                                    .text_color(cx.theme().foreground)
                                    .child(loading_msg)
                            )
                    )
                    .into_any_element()
            )
        } else { error_msg.map(|err| v_flex()
                    .absolute()
                    .inset_0()
                    .bg(gpui::Rgba { r: 0.0, g: 0.0, b: 0.0, a: 0.85 })
                    .items_center()
                    .justify_center()
                    .child(
                        v_flex()
                            .w(px(460.0))
                            .p_6()
                            .gap_4()
                            .bg(cx.theme().secondary)
                            .border_1()
                            .border_color(gpui::Rgba { r: 0.9, g: 0.2, b: 0.2, a: 0.8 })
                            .rounded_2xl()
                            .shadow_2xl()
                            .items_center()
                            .child(gpui::svg().path("circle-alert.svg").size_12().text_color(gpui::Rgba { r: 0.9, g: 0.2, b: 0.2, a: 1.0 }))
                            .child(div().text_lg().font_bold().text_color(cx.theme().foreground).child("Processing Diagnostic Error"))
                            .child(div().text_sm().text_center().text_color(cx.theme().muted_foreground).child(err))
                            .child(div().h_px().w_full().bg(cx.theme().border))
                            .child(div().text_xs().text_color(cx.theme().muted_foreground).child("Troubleshooting: Ensure image format is supported or try a lower resolution target."))
                            .child(
                                Button::new("btn_err_dismiss")
                                    .child(gpui::svg().path("check.svg").size_4().text_color(cx.theme().primary))
                                    .child("Dismiss & Return")
                                    .primary()
                                    .cursor_pointer()
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.app.state = crate::app::AppState::Idle;
                                        cx.notify();
                                    }))
                            )
                    )
                    .into_any_element()) })
}

fn generate_tailwind_shades(r: u8, g: u8, b: u8) -> Vec<(&'static str, [u8; 3])> {
    let mix = |c1: [u8; 3], c2: [u8; 3], weight: f32| -> [u8; 3] {
        [
            (c1[0] as f32 * weight + c2[0] as f32 * (1.0 - weight)) as u8,
            (c1[1] as f32 * weight + c2[1] as f32 * (1.0 - weight)) as u8,
            (c1[2] as f32 * weight + c2[2] as f32 * (1.0 - weight)) as u8,
        ]
    };
    let w = [255, 255, 255];
    let k = [0, 0, 0];
    let base = [r, g, b];
    vec![
        ("50", mix(w, base, 0.95)),
        ("100", mix(w, base, 0.9)),
        ("200", mix(w, base, 0.75)),
        ("300", mix(w, base, 0.6)),
        ("400", mix(w, base, 0.3)),
        ("500", base),
        ("600", mix(k, base, 0.25)),
        ("700", mix(k, base, 0.45)),
        ("800", mix(k, base, 0.65)),
        ("900", mix(k, base, 0.8)),
        ("950", mix(k, base, 0.9)),
    ]
}
