//! Color swatch component displaying active palette shades.

use crate::app::ThemeSource;
use gpui::*;
use gpui_component::{button::Button, h_flex, v_flex, ActiveTheme, Sizable, StyledExt};

/// Renders a color swatch card representing the top shades of the selected theme palette.
pub fn render_swatches(
    current_theme: &ThemeSource,
    cx: &mut Context<crate::ui::WallmodView>,
) -> impl IntoElement {
    let shades = current_theme.get_shades();

    v_flex()
        .w_full()
        .p_3()
        .gap_2()
        .border_1()
        .border_color(cx.theme().border)
        .rounded_lg()
        .bg(cx.theme().secondary)
        .child(
            h_flex()
                .justify_between()
                .items_center()
                .child(
                    div()
                        .text_xs()
                        .font_bold()
                        .text_color(cx.theme().muted_foreground)
                        .child("ACTIVE PALETTE SHADES"),
                )
                .child(
                    div()
                        .text_xs()
                        .font_bold()
                        .text_color(cx.theme().primary)
                        .child(current_theme.display_name()),
                ),
        )
        .child(h_flex().w_full().h_6().gap_1().children(shades.iter().take(8).map(|&rgb| {
            let color = rgb_to_hsla(rgb[0], rgb[1], rgb[2]);
            div()
                .flex_1()
                .h_full()
                .rounded_sm()
                .bg(color)
                .border_1()
                .border_color(cx.theme().border.opacity(0.3))
        })))
        .child(
            Button::new("btn_edit_palette")
                .child(gpui::svg().path("wand.svg").size_4().text_color(cx.theme().primary))
                .child("Edit Palette...")
                .w_full()
                .small()
                .cursor_pointer()
                .on_click(cx.listener(|view, _, _, cx| {
                    if let ThemeSource::CustomPalette(_, _) = view.app.current_theme {
                        // Already a custom palette
                    } else {
                        // Convert to custom palette
                        let shades = view.app.current_theme.get_shades();
                        view.app.current_theme = ThemeSource::CustomPalette(
                            format!("Custom {}", view.app.current_theme.display_name()),
                            shades,
                        );
                        view.app.selected_preset = None;
                    }
                    view.app.workspace_view = crate::app::WorkspaceView::PaletteEditor;
                    view.app.selected_color_idx = None;
                    cx.notify();
                })),
        )
}

/// Helper to convert 8-bit RGB to GPUI Hsla color.
fn rgb_to_hsla(r: u8, g: u8, b: u8) -> Hsla {
    let rf = r as f32 / 255.0;
    let gf = g as f32 / 255.0;
    let bf = b as f32 / 255.0;

    let max = rf.max(gf).max(bf);
    let min = rf.min(gf).min(bf);
    let delta = max - min;

    let l = (max + min) / 2.0;

    let s = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * l - 1.0).abs())
    };

    let mut h = if delta == 0.0 {
        0.0
    } else if max == rf {
        ((gf - bf) / delta).rem_euclid(6.0) / 6.0
    } else if max == gf {
        ((bf - rf) / delta + 2.0) / 6.0
    } else {
        ((rf - gf) / delta + 4.0) / 6.0
    };

    if h < 0.0 {
        h += 1.0;
    }

    Hsla {
        h,
        s,
        l,
        a: 1.0,
    }
}
