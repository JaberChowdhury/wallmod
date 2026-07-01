import re

with open("src/ui/gowall_tab.rs", "r") as f:
    content = f.read()

# I want to replace everything from match view.app.gowall_state.current_tool { ... to the end of the file.
# The match block starts around line 161. Let's find it.
start_idx = content.find("match view.app.gowall_state.current_tool {")

new_code = """match view.app.gowall_state.current_tool {
        GowallTool::Recolor => render_recolor_panel(view, cx, panel).into_any_element(),
        GowallTool::Effects => render_effects_panel(view, cx, panel).into_any_element(),
        GowallTool::Compress => render_compress_panel(view, cx, panel).into_any_element(),
        GowallTool::Ocr => render_ocr_panel(view, cx, panel).into_any_element(),
        GowallTool::Upscale => render_upscale_panel(view, cx, panel).into_any_element(),
        GowallTool::PixelArt => render_pixelart_panel(view, cx, panel).into_any_element(),
        GowallTool::ReplaceColor => render_bg_remove_panel(view, cx, panel).into_any_element(),
    }
}

fn execute_gowall_cmd(
    view: &mut WallmodView,
    cx: &mut Context<WallmodView>,
    args: Vec<String>,
    out_path: std::path::PathBuf,
    is_ocr: bool,
) {
    view.app.gowall_state.is_processing = true;
    cx.spawn(async move |this, cx| {
        let res = crate::backend::gowall_cli::run_gowall_command(args).await;
        
        let _ = this.update(cx, |this: &mut WallmodView, cx| {
            this.app.gowall_state.is_processing = false;
            if let Ok(output) = res {
                if is_ocr {
                    // For OCR, output path is not an image. The extracted text is in output.
                    this.app.gowall_state.extracted_text = output;
                } else {
                    this.app.preview_path = Some(out_path.clone());
                    if let Ok(img) = image::open(&out_path) {
                        this.app.processed_dyn = Some(img);
                    }
                }
            } else {
                eprintln!("Error running gowall command: {:?}", res.err());
            }
            cx.notify();
        });
    }).detach();
}

fn render_recolor_panel(view: &mut WallmodView, cx: &mut Context<WallmodView>, panel: gpui::Div) -> impl IntoElement {
    let selected_theme = view.app.gowall_state.selected_theme.clone();
    let display_name = if selected_theme.is_empty() {
        "Select Theme".to_string()
    } else {
        selected_theme.clone()
    };
    
    let dropdown = Button::new("btn_gowall_theme_dropdown")
        .child(h_flex().justify_between().w_full()
            .child(display_name)
            .child(gpui::svg().path("chevron_down.svg").size_4().text_color(cx.theme().muted_foreground))
        )
        .w_full()
        .outline()
        .dropdown_menu({
            let ve = cx.entity().clone();
            move |mut menu, window, _| {
                for &name in PRESET_NAMES.iter() {
                    let n = name.to_string();
                    let ve = ve.clone();
                    menu = menu.item(
                        PopupMenuItem::new(name).on_click(window.listener_for(&ve, move |this, _, _, cx| {
                            this.app.gowall_state.selected_theme = n.clone();
                            cx.notify();
                        }))
                    );
                }
                menu
            }
        });
    
    panel
        .child(div().font_bold().child("Select Theme Preset:"))
        .child(dropdown)
        .child(
            Button::new("btn_execute_recolor")
                .child("Apply Theme")
                .primary()
                .w_full()
                .on_click(cx.listener(|this, _, _, cx| {
                    if let Some(in_path) = this.app.base_image_path.clone() {
                        let preset = this.app.gowall_state.selected_theme.clone();
                        let gowall_theme = map_preset_to_gowall_theme(&preset);
                        let out_path = std::env::temp_dir().join("gowall_recolor_out.png");
                        
                        let args = vec![
                            "convert".to_string(),
                            in_path.to_string_lossy().to_string(),
                            "-t".to_string(), gowall_theme.to_string(),
                            "--output".to_string(), out_path.to_string_lossy().to_string()
                        ];
                        execute_gowall_cmd(this, cx, args, out_path, false);
                    }
                }))
        )
}

fn render_effects_panel(_view: &mut WallmodView, cx: &mut Context<WallmodView>, panel: gpui::Div) -> impl IntoElement {
    panel
        .child(div().font_bold().child("Quick Actions:"))
        .child(effect_button("Invert Colors", "invert", cx))
        .child(effect_button("Grayscale", "effects grayscale", cx))
        .child(effect_button("Flip Horizontal", "effects flip", cx))
        .child(effect_button("Mirror", "effects mirror", cx))
}

fn effect_button(label: &'static str, action: &'static str, cx: &mut Context<WallmodView>) -> impl IntoElement {
    Button::new(format!("btn_effect_{}", action.replace(" ", "_")))
        .child(label)
        .outline()
        .w_full()
        .on_click(cx.listener(move |this, _, _, cx| {
            if let Some(in_path) = this.app.base_image_path.clone() {
                let out_path = std::env::temp_dir().join(format!("gowall_{}_out.png", action.replace(" ", "_")));
                
                let mut args: Vec<String> = action.split_whitespace().map(|s| s.to_string()).collect();
                args.push(in_path.to_string_lossy().to_string());
                args.push("--output".to_string());
                args.push(out_path.to_string_lossy().to_string());
                
                execute_gowall_cmd(this, cx, args, out_path, false);
            }
        }))
}

fn render_compress_panel(_view: &mut WallmodView, cx: &mut Context<WallmodView>, panel: gpui::Div) -> impl IntoElement {
    panel
        .child(div().font_bold().child("Compress & Format:"))
        .child(
            Button::new("btn_compress_pngquant")
                .child("Compress (pngquant)")
                .primary()
                .w_full()
                .on_click(cx.listener(|this, _, _, cx| {
                    if let Some(in_path) = this.app.base_image_path.clone() {
                        let out_path = std::env::temp_dir().join("gowall_compress_out.png");
                        let args = vec![
                            "compress".to_string(),
                            in_path.to_string_lossy().to_string(),
                            "-m".to_string(), "pngquant".to_string(),
                            "--output".to_string(), out_path.to_string_lossy().to_string()
                        ];
                        execute_gowall_cmd(this, cx, args, out_path, false);
                    }
                }))
        )
}

fn render_ocr_panel(view: &mut WallmodView, cx: &mut Context<WallmodView>, panel: gpui::Div) -> impl IntoElement {
    panel
        .child(div().font_bold().child("Extract Text (OCR):"))
        .child(
            Button::new("btn_execute_ocr")
                .child("Run OCR")
                .primary()
                .w_full()
                .on_click(cx.listener(|this, _, _, cx| {
                    if let Some(in_path) = this.app.base_image_path.clone() {
                        // OCR doesn't need output image path, we just parse stdout
                        let out_path = std::path::PathBuf::new(); // Dummy
                        let args = vec![
                            "ocr".to_string(),
                            in_path.to_string_lossy().to_string(),
                        ];
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
                .overflow_y_scroll()
                .child(div().child(view.app.gowall_state.extracted_text.clone()))
        )
}

fn render_upscale_panel(_view: &mut WallmodView, cx: &mut Context<WallmodView>, panel: gpui::Div) -> impl IntoElement {
    panel
        .child(div().font_bold().child("AI Upscale (Requires Vulkan):"))
        .child(
            Button::new("btn_execute_upscale")
                .child("Upscale 2x")
                .primary()
                .w_full()
                .on_click(cx.listener(|this, _, _, cx| {
                    if let Some(in_path) = this.app.base_image_path.clone() {
                        let out_path = std::env::temp_dir().join("gowall_upscale_out.png");
                        let args = vec![
                            "upscale".to_string(),
                            in_path.to_string_lossy().to_string(),
                            "-s".to_string(), "2".to_string(),
                            "--output".to_string(), out_path.to_string_lossy().to_string()
                        ];
                        execute_gowall_cmd(this, cx, args, out_path, false);
                    }
                }))
        )
}

fn render_pixelart_panel(_view: &mut WallmodView, cx: &mut Context<WallmodView>, panel: gpui::Div) -> impl IntoElement {
    panel
        .child(div().font_bold().child("Pixel Art Generator:"))
        .child(
            Button::new("btn_execute_pixelate")
                .child("Pixelate (Scale 15)")
                .primary()
                .w_full()
                .on_click(cx.listener(|this, _, _, cx| {
                    if let Some(in_path) = this.app.base_image_path.clone() {
                        let out_path = std::env::temp_dir().join("gowall_pixelate_out.png");
                        let args = vec![
                            "pixelate".to_string(),
                            in_path.to_string_lossy().to_string(),
                            "-s".to_string(), "15".to_string(),
                            "--output".to_string(), out_path.to_string_lossy().to_string()
                        ];
                        execute_gowall_cmd(this, cx, args, out_path, false);
                    }
                }))
        )
}

fn render_bg_remove_panel(_view: &mut WallmodView, cx: &mut Context<WallmodView>, panel: gpui::Div) -> impl IntoElement {
    panel
        .child(div().font_bold().child("Background Removal:"))
        .child(
            Button::new("btn_execute_bg_remove")
                .child("Remove BG")
                .primary()
                .w_full()
                .on_click(cx.listener(|this, _, _, cx| {
                    if let Some(in_path) = this.app.base_image_path.clone() {
                        let out_path = std::env::temp_dir().join("gowall_bg_remove_out.png");
                        let args = vec![
                            "bg".to_string(),
                            in_path.to_string_lossy().to_string(),
                            "--output".to_string(), out_path.to_string_lossy().to_string()
                        ];
                        execute_gowall_cmd(this, cx, args, out_path, false);
                    }
                }))
        )
}
"""

with open("src/ui/gowall_tab.rs", "w") as f:
    f.write(content[:start_idx] + new_code)
