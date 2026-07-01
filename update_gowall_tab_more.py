import re

with open("src/ui/gowall_tab.rs", "r") as f:
    content = f.read()

# Add to sidebar buttons
content = content.replace(
    '.child(sidebar_tool_button(view, cx, "Pixel Art", GowallTool::PixelArt))',
    '.child(sidebar_tool_button(view, cx, "Pixel Art", GowallTool::PixelArt))\n                .child(sidebar_tool_button(view, cx, "Extract Palette", GowallTool::Extract))\n                .child(sidebar_tool_button(view, cx, "Resize Image", GowallTool::Resize))'
)

# Add to render_tool_header
content = content.replace(
    'GowallTool::ReplaceColor => "Background Removal",',
    'GowallTool::ReplaceColor => "Background Removal",\n                    GowallTool::Extract => "Extract Color Palette",\n                    GowallTool::Resize => "Resize Image",'
)

# Add to match block in render_tool_panel
content = content.replace(
    'GowallTool::ReplaceColor => render_bg_remove_panel(view, cx, panel).into_any_element(),',
    'GowallTool::ReplaceColor => render_bg_remove_panel(view, cx, panel).into_any_element(),\n        GowallTool::Extract => render_extract_panel(view, cx, panel).into_any_element(),\n        GowallTool::Resize => render_resize_panel(view, cx, panel).into_any_element(),'
)

# Add new functions at the end of the file
new_fns = """

fn render_extract_panel(view: &mut WallmodView, cx: &mut Context<WallmodView>, panel: gpui::Div) -> impl IntoElement {
    panel
        .child(div().font_bold().child("Extract Color Palette:"))
        .child(
            Button::new("btn_execute_extract")
                .child("Extract Colors")
                .primary()
                .w_full()
                .on_click(cx.listener(|this, _, _, cx| {
                    if let Some(in_path) = this.app.base_image_path.clone() {
                        let out_path = std::path::PathBuf::new();
                        let args = vec![
                            "extract".to_string(),
                            in_path.to_string_lossy().to_string(),
                        ];
                        // Treat as OCR to capture stdout text
                        execute_gowall_cmd(this, cx, args, out_path, true);
                    }
                }))
        )
        .child(
            v_flex()
                .w_full()
                .h(px(200.0))
                .bg(cx.theme().secondary)
                .rounded_md()
                .p_2()
                .overflow_y_scrollbar()
                .child(div().child(view.app.gowall_state.extracted_text.clone()))
        )
}

fn render_resize_panel(_view: &mut WallmodView, cx: &mut Context<WallmodView>, panel: gpui::Div) -> impl IntoElement {
    panel
        .child(div().font_bold().child("Resize Image:"))
        .child(
            Button::new("btn_execute_resize_1080p")
                .child("Resize to 1920x1080")
                .primary()
                .w_full()
                .on_click(cx.listener(|this, _, _, cx| {
                    if let Some(in_path) = this.app.base_image_path.clone() {
                        let out_path = std::env::temp_dir().join("gowall_resize_out.png");
                        let args = vec![
                            "resize".to_string(),
                            in_path.to_string_lossy().to_string(),
                            "-d".to_string(), "1920x1080".to_string(),
                            "--output".to_string(), out_path.to_string_lossy().to_string()
                        ];
                        execute_gowall_cmd(this, cx, args, out_path, false);
                    }
                }))
        )
        .child(
            Button::new("btn_execute_resize_4k")
                .child("Resize to 3840x2160 (4K)")
                .outline()
                .w_full()
                .on_click(cx.listener(|this, _, _, cx| {
                    if let Some(in_path) = this.app.base_image_path.clone() {
                        let out_path = std::env::temp_dir().join("gowall_resize_out_4k.png");
                        let args = vec![
                            "resize".to_string(),
                            in_path.to_string_lossy().to_string(),
                            "-d".to_string(), "3840x2160".to_string(),
                            "--output".to_string(), out_path.to_string_lossy().to_string()
                        ];
                        execute_gowall_cmd(this, cx, args, out_path, false);
                    }
                }))
        )
}
"""

with open("src/ui/gowall_tab.rs", "w") as f:
    f.write(content + new_fns)
