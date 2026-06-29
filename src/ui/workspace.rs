//! Main visual workspace and preview component.

use crate::app::{WallmodApp, WorkspaceView};
use crate::ui::WallmodView;
use crate::ui::histogram::render_histogram;
use crate::ui::swatches::render_swatches;
use gpui::*;
use gpui_component::{
    button::*, scroll::ScrollableElement,
    v_flex, h_flex, ActiveTheme, Icon, IconName, Sizable, Selectable, StyledExt,
};

/// Renders the central workspace preview, split diff overlay, dashboard info, or album gallery.
pub fn render_workspace(view: &mut WallmodView, cx: &mut Context<WallmodView>) -> impl IntoElement {
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
                    Button::new("wv_std").label("Output Visual").icon(IconName::Eye).small()
                        .selected(workspace_view == WorkspaceView::Standard)
                        .on_click(cx.listener(|this, _, _, cx| { this.app.workspace_view = WorkspaceView::Standard; cx.notify(); }))
                )
                .child(
                    Button::new("wv_diff").label("Split Diff").icon(IconName::Frame).small()
                        .selected(workspace_view == WorkspaceView::SplitDiff)
                        .on_click(cx.listener(|this, _, _, cx| { this.app.workspace_view = WorkspaceView::SplitDiff; cx.notify(); }))
                )
                .child(
                    Button::new("wv_tel").label("Dashboard Info").icon(IconName::LayoutDashboard).small()
                        .selected(workspace_view == WorkspaceView::Telemetry)
                        .on_click(cx.listener(|this, _, _, cx| { this.app.workspace_view = WorkspaceView::Telemetry; cx.notify(); }))
                )
                .child(
                    Button::new("wv_gal").label("Album Gallery").icon(IconName::Folder).small()
                        .selected(workspace_view == WorkspaceView::Gallery)
                        .on_click(cx.listener(|this, _, _, cx| { this.app.workspace_view = WorkspaceView::Gallery; cx.notify(); }))
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
                                    .border_1()
                                    .border_color(cx.theme().border)
                                    .rounded_lg()
                                    .overflow_hidden()
                                    .child(img(path).size_full().object_fit(ObjectFit::Contain))
                                    .into_any_element()
                            } else {
                                v_flex().gap_4().items_center().justify_center().p_12().border_1().border_color(cx.theme().border).rounded_xl().bg(cx.theme().secondary)
                                    .child(Icon::new(IconName::Inbox).size_12().text_color(cx.theme().muted_foreground))
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
                                        .child(Button::new("split_10").label("10% Orig").icon(IconName::Eye).small().selected((split_ratio - 0.1).abs() < 0.01).on_click(cx.listener(|this, _, _, cx| { this.app.split_diff_ratio = 0.1; cx.notify(); })))
                                        .child(Button::new("split_30").label("30% Orig").icon(IconName::Eye).small().selected((split_ratio - 0.3).abs() < 0.01).on_click(cx.listener(|this, _, _, cx| { this.app.split_diff_ratio = 0.3; cx.notify(); })))
                                        .child(Button::new("split_50").label("50% / 50%").icon(IconName::Eye).small().selected((split_ratio - 0.5).abs() < 0.01).on_click(cx.listener(|this, _, _, cx| { this.app.split_diff_ratio = 0.5; cx.notify(); })))
                                        .child(Button::new("split_70").label("70% Orig").icon(IconName::Eye).small().selected((split_ratio - 0.7).abs() < 0.01).on_click(cx.listener(|this, _, _, cx| { this.app.split_diff_ratio = 0.7; cx.notify(); })))
                                        .child(Button::new("split_90").label("90% Orig").icon(IconName::Eye).small().selected((split_ratio - 0.9).abs() < 0.01).on_click(cx.listener(|this, _, _, cx| { this.app.split_diff_ratio = 0.9; cx.notify(); })))
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
                        WorkspaceView::Gallery => {
                            v_flex().gap_4().size_full()
                                .child(
                                    h_flex().justify_between().items_center()
                                        .child(
                                            if let Some(ref sel) = selected_album {
                                                h_flex().gap_2().items_center()
                                                    .child(Button::new("btn_back_alb").label("Back to Albums").icon(IconName::ArrowLeft).small().on_click(cx.listener(|this, _, _, cx| {
                                                        this.app.selected_album = None;
                                                        this.app.album_images.clear();
                                                        cx.notify();
                                                    })))
                                                    .child(div().text_lg().font_bold().child(format!("Album: {}", sel.file_name().and_then(|n| n.to_str()).unwrap_or("Folder"))))
                                            } else {
                                                h_flex().child(div().text_lg().font_bold().child("System Wallpaper Albums"))
                                            }
                                        )
                                        .child(Button::new("btn_scan_gal").label("Scan System Gallery").icon(IconName::Search).primary().on_click(cx.listener(|this, _, _, cx| {
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
                                                            .on_click(cx.listener(move |_, _, _, cx| {
                                                                let p = p_clone.clone();
                                                                cx.spawn(async move |this, cx| {
                                                                    if let Ok(dyn_img) = crate::backend::runtime::spawn_blocking({ let p = p.clone(); move || image::open(&p) }).await.unwrap() {
                                                                        let _ = this.update(cx, |view, cx| {
                                                                            view.app.on_image_selected(p, dyn_img);
                                                                            view.app.workspace_view = WorkspaceView::Standard;
                                                                            cx.notify();
                                                                        });
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
                                                        .on_click(cx.listener(move |this, _, _, cx| {
                                                            let imgs = WallmodApp::scan_album_images(&folder_path);
                                                            this.app.selected_album = Some(folder_path.clone());
                                                            this.app.album_images = imgs;
                                                            cx.notify();
                                                        }))
                                                        .child(
                                                            if let Some(cp) = cover {
                                                                div().w_full().h(px(120.0)).overflow_hidden().rounded_lg().child(img(cp).size_full().object_fit(ObjectFit::Cover)).into_any_element()
                                                            } else {
                                                                div().w_full().h(px(120.0)).flex().items_center().justify_center().bg(cx.theme().background).rounded_lg().child(Icon::new(IconName::Folder).size_8().text_color(cx.theme().muted_foreground)).into_any_element()
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
}
