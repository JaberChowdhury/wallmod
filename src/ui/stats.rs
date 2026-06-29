//! Floating statistics panel for the UI.

use gpui::*;
use gpui_component::{h_flex, ActiveTheme, StyledExt};
use crate::ui::WallmodView;

pub fn render_floating_stats(
    app: &crate::app::WallmodApp,
    cx: &mut Context<WallmodView>,
) -> AnyElement {
    let avg_cpu = if app.sys_cpu_threads.is_empty() {
        0.0
    } else {
        app.sys_cpu_threads.iter().sum::<f32>() / app.sys_cpu_threads.len() as f32
    };

    let mut items = Vec::new();
    if app.float_show_fps {
        items.push(
            h_flex()
                .justify_between()
                .gap_4()
                .items_center()
                .child(div().text_xs().text_color(cx.theme().muted_foreground).child("FPS"))
                .child(
                    div()
                        .text_xs()
                        .font_bold()
                        .text_color(gpui::rgb(0x22c55e))
                        .child(format!("{:.0}", app.current_fps)),
                )
                .into_any_element(),
        );
    }
    if app.float_show_ram {
        items.push(
            h_flex()
                .justify_between()
                .gap_4()
                .items_center()
                .child(div().text_xs().text_color(cx.theme().muted_foreground).child("RAM"))
                .child(
                    div()
                        .text_xs()
                        .font_bold()
                        .text_color(cx.theme().primary)
                        .child(format!("{:.1}%", app.sys_ram_percent)),
                )
                .into_any_element(),
        );
    }
    if app.float_show_cpu {
        items.push(
            h_flex()
                .justify_between()
                .gap_4()
                .items_center()
                .child(div().text_xs().text_color(cx.theme().muted_foreground).child("CPU (Avg)"))
                .child(
                    div().text_xs().font_bold().text_color(cx.theme().primary).child(format!("{:.0}%", avg_cpu)),
                )
                .into_any_element(),
        );
    }
    if items.is_empty() {
        items.push(
            div()
                .text_xs()
                .text_color(cx.theme().muted_foreground)
                .child("No stats enabled")
                .into_any_element(),
        );
    }

    div()
        .absolute()
        .bottom_6()
        .right_6()
        .p_3()
        .gap_2()
        .flex()
        .flex_col()
        .min_w(px(130.0))
        .rounded_xl()
        .bg(cx.theme().secondary.opacity(0.80))
        .border_1()
        .border_color(cx.theme().border.opacity(0.4))
        .shadow_2xl()
        .child(
            h_flex()
                .justify_between()
                .items_center()
                .pb_1()
                .border_b_1()
                .border_color(cx.theme().border.opacity(0.5))
                .child(
                    div()
                        .text_xs()
                        .font_bold()
                        .text_color(cx.theme().foreground)
                        .child("SYSTEM STATS"),
                )
                .child(gpui::svg().path("monitor.svg").size_3().text_color(cx.theme().primary)),
        )
        .children(items)
        .into_any_element()
}
