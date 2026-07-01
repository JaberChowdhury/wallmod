use crate::app::gowall_state::GowallTool;
use crate::app::state::PRESET_NAMES;
use crate::ui::WallmodView;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::menu::{DropdownMenu as _, PopupMenuItem};
use gpui_component::scroll::ScrollableElement;
use gpui_component::{
    button::Button, button::ButtonVariants, h_flex, v_flex, ActiveTheme, Selectable, StyledExt,
};

fn map_preset_to_gowall_theme(preset: &str) -> &'static str {
    match preset {
        "Arc Dark" => "arcdark",
        "Atom Dark" => "atomdark",
        "Ayu" => "ayu",
        "Ayu Dark" => "ayu-dark",
        "Ayu Light" => "ayu-light",
        "Ayu Mirage" => "ayu-mirage",
        "Catppuccin" => "catppuccin",
        "Catppuccin Frappe" => "cat-frappe",
        "Catppuccin Latte" => "cat-latte",
        "Cyberpunk" => "cyberpunk",
        "Dracula" => "dracula",
        "Everforest" => "everforest",
        "GitHub Light" => "github-light",
        "Gruvbox" => "gruvbox",
        "Kanagawa" => "kanagawa",
        "Material" => "material",
        "Melange Dark" => "melange-dark",
        "Melange Light" => "melange-light",
        "Monokai" => "monokai",
        "Night Owl" => "night-owl",
        "Nord" => "nord",
        "Oceanic Next" => "oceanic-next",
        "Onedark" => "onedark",
        "PaleNight" => "palenight",
        "Rose Pine" => "rose-pine",
        "Shades of Purple" => "shades-of-purple",
        "Solarized" => "solarized",
        "Srcery" => "srcery",
        "Sunset Aurant" => "sunset-aurant",
        "Sunset Saffron" => "sunset-saffron",
        "Sunset Tangerine" => "sunset-tangerine",
        "Synthwave 84" => "synthwave-84",
        "Tokyo Dark" => "tokyo-dark",
        "Tokyo Moon" => "tokyo-moon",
        "Tokyo Storm" => "tokyo-storm",
        _ => "catppuccin", // Fallback
    }
}

pub fn render_gowall_tab(
    view: &mut WallmodView,
    cx: &mut Context<WallmodView>,
) -> impl IntoElement {
    h_flex()
        .size_full()
        .overflow_hidden()
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

                // Sidebar Buttons
                .child(sidebar_tool_button(view, cx, "Recolor Theme", GowallTool::Recolor))
                .child(sidebar_tool_button(view, cx, "Basic Effects", GowallTool::Effects))
                .child(sidebar_tool_button(view, cx, "Compress & Format", GowallTool::Compress))
                .child(sidebar_tool_button(view, cx, "Extract Text (OCR)", GowallTool::Ocr))
                .child(sidebar_tool_button(view, cx, "AI Upscale", GowallTool::Upscale))
                .child(sidebar_tool_button(view, cx, "Remove Background", GowallTool::ReplaceColor))
                .child(sidebar_tool_button(view, cx, "Pixel Art", GowallTool::PixelArt))
                .child(sidebar_tool_button(view, cx, "Extract Palette", GowallTool::Extract))
                .child(sidebar_tool_button(view, cx, "Resize Image", GowallTool::Resize))
                .child(sidebar_tool_button(view, cx, "Daily Wallpaper", GowallTool::Daily)),
        )
        .child(
            v_flex()
                .flex_1()
                .h_full()
                .w_full()
                .min_w_0()
                .min_h_0()
                .overflow_hidden()
                .gap_4()
                .child(render_tool_header(view, cx))
                .child(
                    h_flex()
                        .flex_1()
                        .w_full()
                        .min_w_0()
                        .min_h_0()
                        .overflow_hidden()
                        .gap_4()
                        .child(render_tool_panel(view, cx))
                        .child(render_image_preview(view, cx)),
                ),
        )
}

fn sidebar_tool_button(
    view: &mut WallmodView,
    cx: &mut Context<WallmodView>,
    label: &'static str,
    tool: GowallTool,
) -> impl IntoElement {
    let is_active = view.app.gowall_state.current_tool == tool;
    Button::new(format!("btn_gowall_{:?}", tool))
        .child(label)
        .selected(is_active)
        .when(is_active, |this| this.primary())
        .when(!is_active, |this| this.outline())
        .on_click(cx.listener(move |this, _, _, cx| {
            this.app.gowall_state.current_tool = tool;
            cx.notify();
        }))
}

fn render_tool_header(view: &mut WallmodView, _cx: &mut Context<WallmodView>) -> impl IntoElement {
    h_flex().w_full().justify_between().items_center().child(div().text_2xl().font_bold().child(
        match view.app.gowall_state.current_tool {
            GowallTool::Recolor => "Theme Recolor",
            GowallTool::Effects => "Basic Effects",
            GowallTool::Compress => "Compress & Format",
            GowallTool::Ocr => "Extract Text (OCR)",
            GowallTool::Upscale => "AI Upscaler",
            GowallTool::PixelArt => "Pixel Art Generator",
            GowallTool::ReplaceColor => "Background Removal",
            GowallTool::Extract => "Extract Color Palette",
            GowallTool::Resize => "Resize Image",
            GowallTool::Daily => "Daily Wallpaper",
        },
    ))
}

fn render_image_preview(view: &mut WallmodView, cx: &mut Context<WallmodView>) -> impl IntoElement {
    let display_path = view.app.preview_path.as_ref().or(view.app.base_image_path.as_ref());

    let content = if view.app.gowall_state.is_processing {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .text_xl()
            .font_bold()
            .text_color(cx.theme().primary)
            .child("Processing... Please wait.")
            .into_any_element()
    } else if let Some(path) = display_path {
        div()
            .size_full()
            .p_4()
            .overflow_hidden()
            .child(img(path.clone()).size_full().object_fit(gpui::ObjectFit::Contain))
            .into_any_element()
    } else {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .text_lg()
            .text_color(cx.theme().muted_foreground)
            .child("No image loaded.")
            .into_any_element()
    };

    v_flex()
        .flex_1()
        .size_full()
        .min_w_0()
        .min_h_0()
        .bg(cx.theme().secondary)
        .rounded_xl()
        .border_1()
        .border_color(cx.theme().border)
        .overflow_hidden()
        .child(content)
}

fn render_tool_panel(view: &mut WallmodView, cx: &mut Context<WallmodView>) -> impl IntoElement {
    let panel = v_flex()
        .w(px(300.0))
        .h_full()
        .p_4()
        .gap_4()
        .bg(cx.theme().background)
        .rounded_xl()
        .border_1()
        .border_color(cx.theme().border);

    if view.app.base_image_path.is_none() && view.app.gowall_state.current_tool != GowallTool::Daily
    {
        return panel
            .child(
                div().text_color(cx.theme().muted_foreground).child("Please load an image first."),
            )
            .into_any_element();
    }

    match view.app.gowall_state.current_tool {
        GowallTool::Recolor => render_recolor_panel(view, cx, panel).into_any_element(),
        GowallTool::Effects => render_effects_panel(view, cx, panel).into_any_element(),
        GowallTool::Compress => render_compress_panel(view, cx, panel).into_any_element(),
        GowallTool::Ocr => render_ocr_panel(view, cx, panel).into_any_element(),
        GowallTool::Upscale => render_upscale_panel(view, cx, panel).into_any_element(),
        GowallTool::PixelArt => render_pixelart_panel(view, cx, panel).into_any_element(),
        GowallTool::ReplaceColor => render_bg_remove_panel(view, cx, panel).into_any_element(),
        GowallTool::Extract => render_extract_panel(view, cx, panel).into_any_element(),
        GowallTool::Resize => render_resize_panel(view, cx, panel).into_any_element(),
        GowallTool::Daily => render_daily_panel(view, cx, panel).into_any_element(),
    }
}

fn execute_gowall_cmd(
    view: &mut WallmodView,
    cx: &mut Context<WallmodView>,
    args: Vec<String>,
    out_path: std::path::PathBuf,
    is_ocr: bool,
) {
    let mut final_args = args;
    final_args.push("--preview".to_string());
    final_args.push("false".to_string());
    final_args.push("--yes".to_string());

    view.app.gowall_state.is_processing = true;
    cx.spawn(async move |this, cx| {
        let res = crate::backend::gowall_cli::run_gowall_command(final_args).await;

        let _ = this.update(cx, |this: &mut WallmodView, cx| {
            this.app.gowall_state.is_processing = false;
            if let Ok(output) = res {
                if is_ocr {
                    this.app.gowall_state.extracted_text = output.to_string_lossy().to_string();
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
    })
    .detach();
}

fn render_recolor_panel(
    view: &mut WallmodView,
    cx: &mut Context<WallmodView>,
    panel: gpui::Div,
) -> impl IntoElement {
    let selected_theme = view.app.gowall_state.selected_theme.clone();
    let display_name = if selected_theme.is_empty() {
        "Select Theme".to_string()
    } else {
        selected_theme.clone()
    };

    let dropdown = Button::new("btn_gowall_theme_dropdown")
        .child(h_flex().justify_between().w_full().child(display_name).child(
            gpui::svg().path("chevron_down.svg").size_4().text_color(cx.theme().muted_foreground),
        ))
        .w_full()
        .outline()
        .dropdown_menu({
            let ve = cx.entity().clone();
            move |mut menu, window, _| {
                for &name in PRESET_NAMES.iter() {
                    let n = name.to_string();
                    let ve = ve.clone();
                    menu = menu.item(PopupMenuItem::new(name).on_click(window.listener_for(
                        &ve,
                        move |this, _, _, cx| {
                            this.app.gowall_state.selected_theme = n.clone();
                            cx.notify();
                        },
                    )));
                }
                menu
            }
        });

    panel.child(div().font_bold().child("Select Theme Preset:")).child(dropdown).child(
        Button::new("btn_execute_recolor").child("Apply Theme").primary().w_full().on_click(
            cx.listener(|this, _, _, cx| {
                if let Some(in_path) = this.app.base_image_path.clone() {
                    let preset = this.app.gowall_state.selected_theme.clone();
                    let gowall_theme = map_preset_to_gowall_theme(&preset);
                    let out_path = std::env::temp_dir().join("gowall_recolor_out.png");

                    let args = vec![
                        "convert".to_string(),
                        in_path.to_string_lossy().to_string(),
                        "-t".to_string(),
                        gowall_theme.to_string(),
                        "--output".to_string(),
                        out_path.to_string_lossy().to_string(),
                    ];
                    execute_gowall_cmd(this, cx, args, out_path, false);
                }
            }),
        ),
    )
}

fn render_effects_panel(
    _view: &mut WallmodView,
    cx: &mut Context<WallmodView>,
    panel: gpui::Div,
) -> impl IntoElement {
    panel
        .child(div().font_bold().child("Quick Actions:"))
        .child(effect_button("Invert Colors", "invert", cx))
        .child(effect_button("Grayscale", "effects grayscale", cx))
        .child(effect_button("Flip Horizontal", "effects flip", cx))
        .child(effect_button("Mirror", "effects mirror", cx))
}

fn effect_button(
    label: &'static str,
    action: &'static str,
    cx: &mut Context<WallmodView>,
) -> impl IntoElement {
    Button::new(format!("btn_effect_{}", action.replace(" ", "_")))
        .child(label)
        .outline()
        .w_full()
        .on_click(cx.listener(move |this, _, _, cx| {
            if let Some(in_path) = this.app.base_image_path.clone() {
                let out_path = std::env::temp_dir()
                    .join(format!("gowall_{}_out.png", action.replace(" ", "_")));

                let mut args: Vec<String> =
                    action.split_whitespace().map(|s| s.to_string()).collect();
                args.push(in_path.to_string_lossy().to_string());
                args.push("--output".to_string());
                args.push(out_path.to_string_lossy().to_string());

                execute_gowall_cmd(this, cx, args, out_path, false);
            }
        }))
}

fn render_compress_panel(
    _view: &mut WallmodView,
    cx: &mut Context<WallmodView>,
    panel: gpui::Div,
) -> impl IntoElement {
    panel.child(div().font_bold().child("Compress & Format:")).child(
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
                        "-m".to_string(),
                        "losslesspng".to_string(),
                        "--output".to_string(),
                        out_path.to_string_lossy().to_string(),
                    ];
                    execute_gowall_cmd(this, cx, args, out_path, false);
                }
            })),
    )
}

fn render_ocr_panel(
    view: &mut WallmodView,
    cx: &mut Context<WallmodView>,
    panel: gpui::Div,
) -> impl IntoElement {
    panel
        .child(div().font_bold().child("Extract Text (OCR):"))
        .child(Button::new("btn_execute_ocr").child("Run OCR").primary().w_full().on_click(
            cx.listener(|this, _, _, cx| {
                if let Some(in_path) = this.app.base_image_path.clone() {
                    // OCR doesn't need output image path, we just parse stdout
                    let out_path = std::path::PathBuf::new(); // Dummy
                    let args = vec![
                        "ocr".to_string(),
                        in_path.to_string_lossy().to_string(),
                        "-p".to_string(),
                        "tesseract".to_string(),
                        "-m".to_string(),
                        "eng".to_string(),
                    ];
                    execute_gowall_cmd(this, cx, args, out_path, true);
                }
            }),
        ))
        .child(
            v_flex()
                .gap_2()
                .child(
                    v_flex()
                        .w_full()
                        .h(px(200.0))
                        .bg(cx.theme().secondary)
                        .rounded_md()
                        .p_2()
                        .overflow_y_scrollbar()
                        .child(div().child(view.app.gowall_state.extracted_text.clone())),
                )
                .child(
                    Button::new("btn_copy_ocr")
                        .child("Copy to Clipboard")
                        .outline()
                        .w_full()
                        .on_click(cx.listener(|this, _, _, cx| {
                            let text = this.app.gowall_state.extracted_text.clone();
                            cx.write_to_clipboard(gpui::ClipboardItem::new_string(text));
                        })),
                ),
        )
}

fn render_upscale_panel(
    _view: &mut WallmodView,
    cx: &mut Context<WallmodView>,
    panel: gpui::Div,
) -> impl IntoElement {
    panel.child(div().font_bold().child("AI Upscale (Requires Vulkan):")).child(
        Button::new("btn_execute_upscale").child("Upscale 2x").primary().w_full().on_click(
            cx.listener(|this, _, _, cx| {
                if let Some(in_path) = this.app.base_image_path.clone() {
                    let out_path = std::env::temp_dir().join("gowall_upscale_out.png");
                    let args = vec![
                        "upscale".to_string(),
                        in_path.to_string_lossy().to_string(),
                        "-s".to_string(),
                        "2".to_string(),
                        "--output".to_string(),
                        out_path.to_string_lossy().to_string(),
                    ];
                    execute_gowall_cmd(this, cx, args, out_path, false);
                }
            }),
        ),
    )
}

fn render_pixelart_panel(
    _view: &mut WallmodView,
    cx: &mut Context<WallmodView>,
    panel: gpui::Div,
) -> impl IntoElement {
    panel.child(div().font_bold().child("Pixel Art Generator:")).child(
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
                        "-s".to_string(),
                        "15".to_string(),
                        "--output".to_string(),
                        out_path.to_string_lossy().to_string(),
                    ];
                    execute_gowall_cmd(this, cx, args, out_path, false);
                }
            })),
    )
}

fn render_bg_remove_panel(
    _view: &mut WallmodView,
    cx: &mut Context<WallmodView>,
    panel: gpui::Div,
) -> impl IntoElement {
    panel.child(div().font_bold().child("Background Removal:")).child(
        Button::new("btn_execute_bg_remove").child("Remove BG").primary().w_full().on_click(
            cx.listener(|this, _, _, cx| {
                if let Some(in_path) = this.app.base_image_path.clone() {
                    let out_path = std::env::temp_dir().join("gowall_bg_remove_out.png");
                    let args = vec![
                        "bg".to_string(),
                        in_path.to_string_lossy().to_string(),
                        "--output".to_string(),
                        out_path.to_string_lossy().to_string(),
                    ];
                    execute_gowall_cmd(this, cx, args, out_path, false);
                }
            }),
        ),
    )
}

fn render_extract_panel(
    view: &mut WallmodView,
    cx: &mut Context<WallmodView>,
    panel: gpui::Div,
) -> impl IntoElement {
    panel
        .child(div().font_bold().child("Extract Color Palette:"))
        .child(
            Button::new("btn_execute_extract").child("Extract Colors").primary().w_full().on_click(
                cx.listener(|this, _, _, cx| {
                    if let Some(in_path) = this.app.base_image_path.clone() {
                        let out_path = std::path::PathBuf::new();
                        let args =
                            vec!["extract".to_string(), in_path.to_string_lossy().to_string()];
                        // Treat as OCR to capture stdout text
                        execute_gowall_cmd(this, cx, args, out_path, true);
                    }
                }),
            ),
        )
        .child(
            v_flex()
                .w_full()
                .h(px(200.0))
                .bg(cx.theme().secondary)
                .rounded_md()
                .p_2()
                .overflow_y_scrollbar()
                .child(div().child(view.app.gowall_state.extracted_text.clone())),
        )
}

fn render_resize_panel(
    _view: &mut WallmodView,
    cx: &mut Context<WallmodView>,
    panel: gpui::Div,
) -> impl IntoElement {
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
                            "-d".to_string(),
                            "1920x1080".to_string(),
                            "--output".to_string(),
                            out_path.to_string_lossy().to_string(),
                        ];
                        execute_gowall_cmd(this, cx, args, out_path, false);
                    }
                })),
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
                            "-d".to_string(),
                            "3840x2160".to_string(),
                            "--output".to_string(),
                            out_path.to_string_lossy().to_string(),
                        ];
                        execute_gowall_cmd(this, cx, args, out_path, false);
                    }
                })),
        )
}

fn render_daily_panel(
    _view: &mut WallmodView,
    cx: &mut Context<WallmodView>,
    panel: gpui::Div,
) -> impl IntoElement {
    panel.child(div().font_bold().child("Daily Wallpaper:")).child(
        Button::new("btn_fetch_daily")
            .child("Fetch Wallpaper of the Day")
            .primary()
            .w_full()
            .on_click(cx.listener(|this, _, _, cx| {
                this.app.gowall_state.is_processing = true;
                cx.notify();
                cx.spawn(async move |this, cx| {
                    let url_res = crate::backend::gowall_cli::run_gowall_command(vec![
                        "daily-url".to_string()
                    ])
                    .await;

                    if let Ok(url_path) = url_res {
                        let url_str = url_path.to_string_lossy().to_string();
                        if !url_str.is_empty() {
                            let out_path = std::env::temp_dir().join("wallmod_daily.jpg");
                            let out_path_clone = out_path.clone();
                            let download_res = crate::backend::runtime::spawn_blocking(move || {
                                std::process::Command::new("curl")
                                    .arg("-sL")
                                    .arg("-o")
                                    .arg(&out_path_clone)
                                    .arg(&url_str)
                                    .status()
                            })
                            .await;

                            if let Ok(Ok(status)) = download_res {
                                if status.success() {
                                    let _ = this.update(cx, |view, cx| {
                                        view.open_image_from_path(out_path.clone(), cx);
                                    });
                                }
                            }
                        }
                    }

                    let _ = this.update(cx, |view, cx| {
                        view.app.gowall_state.is_processing = false;
                        cx.notify();
                    });
                })
                .detach();
            })),
    )
}
