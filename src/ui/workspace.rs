//! Main visual workspace and preview component.

use crate::app::{WallmodApp, WorkspaceView};
use crate::ui::histogram::render_histogram;
use crate::ui::swatches::render_swatches;
use crate::ui::WallmodView;
use gpui::*;
use gpui_component::{
    button::*, h_flex, scroll::ScrollableElement, v_flex, ActiveTheme, Selectable,
    Sizable, StyledExt, Disableable,
};

/// Renders the central workspace preview, split diff overlay, dashboard info, or album gallery.
pub fn render_workspace(view: &mut WallmodView, cx: &mut Context<WallmodView>) -> impl IntoElement {
    let is_loading = view.app.state.is_loading();
    let loading_msg = if let crate::app::AppState::Loading(_, ref s) = view.app.state { s.clone() } else { "Processing...".to_string() };
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
                    Button::new("wv_std").child(gpui::svg().path("eye.svg").size_4().text_color(cx.theme().primary)).label("Output Visual").small()
                        .selected(workspace_view == WorkspaceView::Standard)
                        .cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.workspace_view = WorkspaceView::Standard; cx.notify(); }))
                )
                .child(
                    Button::new("wv_diff").child(gpui::svg().path("frame.svg").size_4().text_color(cx.theme().primary)).label("Split Diff").small()
                        .selected(workspace_view == WorkspaceView::SplitDiff)
                        .cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.workspace_view = WorkspaceView::SplitDiff; cx.notify(); }))
                )
                .child(
                    Button::new("wv_tel").child(gpui::svg().path("layout-dashboard.svg").size_4().text_color(cx.theme().primary)).label("Dashboard Info").small()
                        .selected(workspace_view == WorkspaceView::Telemetry)
                        .cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.workspace_view = WorkspaceView::Telemetry; cx.notify(); }))
                )
                .child(
                    Button::new("wv_extract").child(gpui::svg().path("palette.svg").size_4().text_color(cx.theme().primary)).label("Extract Color").small()
                        .selected(view.app.workspace_view == WorkspaceView::ExtractColor)
                        .cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.workspace_view = WorkspaceView::ExtractColor; cx.notify(); }))
                )
                .child(
                    Button::new("wv_gal").child(gpui::svg().path("folder.svg").size_4().text_color(cx.theme().primary)).label("Album Gallery").small()
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
                                        .child(Button::new("split_10").disabled(is_loading).child(gpui::svg().path("eye.svg").size_4().text_color(cx.theme().primary)).label("10% Orig").small().selected((split_ratio - 0.1).abs() < 0.01).cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.split_diff_ratio = 0.1; cx.notify(); })))
                                        .child(Button::new("split_30").disabled(is_loading).child(gpui::svg().path("eye.svg").size_4().text_color(cx.theme().primary)).label("30% Orig").small().selected((split_ratio - 0.3).abs() < 0.01).cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.split_diff_ratio = 0.3; cx.notify(); })))
                                        .child(Button::new("split_50").disabled(is_loading).child(gpui::svg().path("eye.svg").size_4().text_color(cx.theme().primary)).label("50% / 50%").small().selected((split_ratio - 0.5).abs() < 0.01).cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.split_diff_ratio = 0.5; cx.notify(); })))
                                        .child(Button::new("split_70").disabled(is_loading).child(gpui::svg().path("eye.svg").size_4().text_color(cx.theme().primary)).label("70% Orig").small().selected((split_ratio - 0.7).abs() < 0.01).cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.split_diff_ratio = 0.7; cx.notify(); })))
                                        .child(Button::new("split_90").disabled(is_loading).child(gpui::svg().path("eye.svg").size_4().text_color(cx.theme().primary)).label("90% Orig").small().selected((split_ratio - 0.9).abs() < 0.01).cursor_pointer().on_click(cx.listener(|this, _, _, cx| { this.app.split_diff_ratio = 0.9; cx.notify(); })))
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
                                                        Button::new("btn_apply_extracted").disabled(is_loading).child(gpui::svg().path("wand.svg").size_4().text_color(cx.theme().primary)).label("Use as Preset")
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
                                                                    this.app.current_theme = crate::app::state::ThemeSource::CustomPalette("Extracted".to_string(), colors);
                                                                    this.app.selected_preset = None;
                                                                    this.trigger_async_processing(cx, "Applying extracted palette...");
                                                                }
                                                            }))
                                                            .into_any_element()
                                                    } else {
                                                        div().into_any_element()
                                                    }
                                                )
                                                .child(
                                                    Button::new("btn_extract_cols").disabled(is_loading).child(gpui::svg().path("palette.svg").size_4().text_color(cx.theme().primary)).label("Extract k-Means Colors")
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
                                        v_flex().gap_4().w_full()
                                            .children(cols.into_iter().enumerate().map(|(_i, (hex, pct))| {
                                                let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(0);
                                                let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(0);
                                                let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(0);
                                                let pct_display = format!("{:.1}%", pct * 100.0);
                                                
                                                h_flex().gap_4().items_center().w_full()
                                                    .child(div().w(px(70.0)).text_sm().font_bold().child(hex.clone()))
                                                    .child(
                                                        div().flex_1().h(px(24.0)).bg(cx.theme().secondary).rounded_full().overflow_hidden().flex().items_center()
                                                            .child(
                                                                div().h_full().bg(gpui::Rgba { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a: 1.0 })
                                                                    .w(gpui::relative(if pct.is_nan() { 0.0 } else { pct.clamp(0.0, 1.0) }))
                                                            )
                                                    )
                                                    .child(div().w(px(50.0)).text_sm().text_color(cx.theme().muted_foreground).child(pct_display))
                                            }))
                                            .into_any_element()
                                    } else {
                                        div().flex_1().flex().items_center().justify_center().text_color(cx.theme().muted_foreground)
                                            .child("No colors extracted. Click the button above to run the K-Means extraction algorithm.")
                                            .into_any_element()
                                    }
                                )
                                .into_any_element()
                        }
                        WorkspaceView::Albums => {
                            v_flex().gap_4().size_full()
                                .child(
                                    h_flex().justify_between().items_center()
                                        .child(
                                            if let Some(ref sel) = selected_album {
                                                h_flex().gap_2().items_center()
                                                    .child(Button::new("btn_back_alb").disabled(is_loading).child(gpui::svg().path("arrow-left.svg").size_4().text_color(cx.theme().primary)).label("Back to Albums").small().cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                                        this.app.selected_album = None;
                                                        this.app.album_images.clear();
                                                        cx.notify();
                                                    })))
                                                    .child(div().text_lg().font_bold().child(format!("Album: {}", sel.file_name().and_then(|n| n.to_str()).unwrap_or("Folder"))))
                                            } else {
                                                h_flex().child(div().text_lg().font_bold().child("System Wallpaper Albums"))
                                            }
                                        )
                                        .child(Button::new("btn_scan_gal").disabled(is_loading).child(gpui::svg().path("search.svg").size_4().text_color(cx.theme().primary)).label("Scan System Gallery").primary().cursor_pointer().on_click(cx.listener(|this, _, _, cx| {
                                            this.app.albums = WallmodApp::scan_system_gallery();
                                            this.app.selected_album = None;
                                            cx.notify();
                                        })))
                                )
                                .child(
                                    if let Some(_) = selected_album {
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
        } else {
            None
        })
}
