use gpui::*;
use gpui_component::{h_flex, v_flex, ActiveTheme, StyledExt, button::Button};
use crate::ui::WallmodView;

pub fn render_gowall_tab(view: &mut WallmodView, cx: &mut Context<WallmodView>) -> impl IntoElement {
    h_flex()
        .size_full()
        .gap_4()
        .p_6()
        .child(
            v_flex()
                .w(px(250.0))
                .h_full()
                .gap_4()
                .border_r_1()
                .border_color(cx.theme().border)
                .pr_4()
                .child(div().text_xl().font_bold().child("Gowall Tools"))
                .child(Button::new("btn_gowall_recolor").child("Recolor Theme").outline())
                .child(Button::new("btn_gowall_compress").child("Compress & Convert").outline())
                .child(Button::new("btn_gowall_ocr").child("Extract Text (OCR)").outline())
                .child(Button::new("btn_gowall_upscale").child("AI Upscale").outline())
                .child(Button::new("btn_gowall_pixelart").child("Pixel Art").outline())
                .child(Button::new("btn_gowall_rembg").child("Remove Background").outline())
        )
        .child(
            v_flex()
                .flex_1()
                .h_full()
                .items_center()
                .justify_center()
                .bg(cx.theme().secondary)
                .rounded_xl()
                .border_1()
                .border_color(cx.theme().border)
                .child(
                    div().text_lg().text_color(cx.theme().muted_foreground).child("Select a tool from the sidebar to begin.")
                )
        )
}
