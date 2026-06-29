//! Histogram waveform display component.

use crate::modules::histogram::HistogramData;
use gpui::*;
use gpui_component::{h_flex, v_flex, ActiveTheme, StyledExt};

/// Renders a 64-bar frequency equalizer representing the image luma and color distribution.
pub fn render_histogram(
    data: Option<&HistogramData>,
    cx: &mut Context<crate::ui::WallmodView>,
) -> impl IntoElement {
    v_flex()
        .w_full()
        .p_4()
        .gap_2()
        .border_1()
        .border_color(cx.theme().border)
        .rounded_xl()
        .bg(cx.theme().secondary)
        .child(
            h_flex()
                .justify_between()
                .items_center()
                .child(
                    div()
                        .text_sm()
                        .font_bold()
                        .text_color(cx.theme().primary)
                        .child("Color Waveform Histogram"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("Logarithmic Scale"),
                ),
        )
        .child(if let Some(hist) = data {
            let max = hist.max_count as f32;
            let max_log = if max > 0.0 { (max + 1.0).ln() } else { 1.0 };

            // Downsample 256 bins into 64 visual frequency bars
            let mut bars = [0.0f32; 64];
            for i in 0..64 {
                let mut sum = 0u32;
                for j in 0..4 {
                    sum += hist.luma[i * 4 + j];
                }
                let avg = sum as f32 / 4.0;
                let norm = if avg > 0.0 {
                    (avg + 1.0).ln() / max_log
                } else {
                    0.05
                };
                bars[i] = norm.clamp(0.05, 1.0);
            }

            h_flex()
                .w_full()
                .h_32()
                .items_end()
                .gap_px()
                .pt_2()
                .children(bars.iter().map(|&height_ratio| {
                    div()
                        .flex_1()
                        .h(gpui::Length::Definite(gpui::DefiniteLength::Fraction(
                            height_ratio,
                        )))
                        .rounded_t_sm()
                        .bg(cx.theme().primary.opacity(0.85))
                }))
                .into_any_element()
        } else {
            div()
                .w_full()
                .h_32()
                .flex()
                .items_center()
                .justify_center()
                .text_xs()
                .text_color(cx.theme().muted_foreground)
                .child("No histogram computed (load an image to analyze channels)")
                .into_any_element()
        })
}
