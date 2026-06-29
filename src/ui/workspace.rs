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
                    Button::new("wv_std").label("Output Visual").small()
                        .selected(workspace_view == WorkspaceView::Standard)
                        .on_click(cx.listener(|this, _, _, cx| { this.app.workspace_view = WorkspaceView::Standard; cx.notify(); }))
                )
                .child(
                    Button::new("wv_diff").label("Split Diff").small()
                        .selected(workspace_view == WorkspaceView::SplitDiff)
                        .on_click(cx.listener(|this, _, _, cx| { this.app.workspace_view = WorkspaceView::SplitDiff; cx.notify(); }))
                )
                .child(
                    Button::new("wv_tel").label("Dashboard Info").small()
                        .selected(workspace_view == WorkspaceView::Telemetry)
                        .on_click(cx.listener(|this, _, _, cx| { this.app.workspace_view = WorkspaceView::Telemetry; cx.notify(); }))
                )
                .child(
                    Button::new("wv_gal").label("Album Gallery").small()
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
}
