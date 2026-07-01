//! Centralized GPUI UI Presentation Layer.
//! Refactored into clean modular components following standard software engineering practices.

pub mod gowall_tab;
pub mod header;
pub mod histogram;
pub mod sidebar;
pub mod swatches;
pub mod workspace;

use crate::app::WallmodApp;
use gpui::*;
use gpui_component::slider::{SliderEvent, SliderState};
use gpui_component::{h_flex, v_flex, ActiveTheme, StyledExt};

pub struct WallmodView {
    pub app: WallmodApp,
    pub blur_slider: Entity<SliderState>,
    pub brightness_slider: Entity<SliderState>,
    pub contrast_slider: Entity<SliderState>,
    pub saturation_slider: Entity<SliderState>,
    pub hue_slider: Entity<SliderState>,
    pub palette_r_slider: Entity<SliderState>,
    pub palette_g_slider: Entity<SliderState>,
    pub palette_b_slider: Entity<SliderState>,
    pub palette_hex_input: Entity<gpui_component::input::InputState>,
    pub code_render_input: Entity<gpui_component::input::InputState>,
    pub shader_inputs:
        std::collections::HashMap<usize, [Entity<gpui_component::input::InputState>; 4]>,
    pub subscriptions: Vec<Subscription>,
}

impl WallmodView {
    pub fn new(window: &mut gpui::Window, cx: &mut Context<Self>) -> Self {
        let blur_slider = cx.new(|_| {
            SliderState::new()
                .min(0.0)
                .max(50.0)
                .step(0.5)
                .default_value(0.0)
        });
        let brightness_slider = cx.new(|_| {
            SliderState::new()
                .min(-100.0)
                .max(100.0)
                .step(1.0)
                .default_value(0.0)
        });
        let contrast_slider = cx.new(|_| {
            SliderState::new()
                .min(-100.0)
                .max(100.0)
                .step(1.0)
                .default_value(0.0)
        });
        let saturation_slider = cx.new(|_| {
            SliderState::new()
                .min(-1.0)
                .max(1.0)
                .step(0.05)
                .default_value(0.0)
        });
        let hue_slider = cx.new(|_| {
            SliderState::new()
                .min(0.0)
                .max(360.0)
                .step(1.0)
                .default_value(0.0)
        });

        let mut subscriptions = Vec::new();

        subscriptions.push(cx.subscribe(
            &blur_slider,
            |this, _, event: &SliderEvent, cx| match event {
                SliderEvent::Change(val) => {
                    this.app.blur_sigma = val.start();
                    cx.notify();
                },
                SliderEvent::Release(val) => {
                    this.app.blur_sigma = val.start();
                    this.trigger_async_processing(cx, "Applying blur...");
                },
            },
        ));

        subscriptions.push(
            cx.subscribe(
                &brightness_slider,
                |this, _, event: &SliderEvent, cx| match event {
                    SliderEvent::Change(val) => {
                        this.app.photoshop_params.brightness = val.start() as i32;
                        cx.notify();
                    },
                    SliderEvent::Release(val) => {
                        this.app.photoshop_params.brightness = val.start() as i32;
                        this.trigger_async_processing(cx, "Adjusting brightness...");
                    },
                },
            ),
        );

        subscriptions.push(
            cx.subscribe(
                &contrast_slider,
                |this, _, event: &SliderEvent, cx| match event {
                    SliderEvent::Change(val) => {
                        this.app.photoshop_params.contrast = val.start();
                        cx.notify();
                    },
                    SliderEvent::Release(val) => {
                        this.app.photoshop_params.contrast = val.start();
                        this.trigger_async_processing(cx, "Adjusting contrast...");
                    },
                },
            ),
        );

        subscriptions.push(
            cx.subscribe(
                &saturation_slider,
                |this, _, event: &SliderEvent, cx| match event {
                    SliderEvent::Change(val) => {
                        this.app.photoshop_params.saturation = val.start();
                        cx.notify();
                    },
                    SliderEvent::Release(val) => {
                        this.app.photoshop_params.saturation = val.start();
                        this.trigger_async_processing(cx, "Adjusting saturation...");
                    },
                },
            ),
        );

        subscriptions.push(cx.subscribe(
            &hue_slider,
            |this, _, event: &SliderEvent, cx| match event {
                SliderEvent::Change(val) => {
                    this.app.photoshop_params.hue = val.start() as i32;
                    cx.notify();
                },
                SliderEvent::Release(val) => {
                    this.app.photoshop_params.hue = val.start() as i32;
                    this.trigger_async_processing(cx, "Shifting hue...");
                },
            },
        ));

        let palette_r_slider = cx.new(|_| {
            SliderState::new()
                .min(0.0)
                .max(255.0)
                .step(1.0)
                .default_value(128.0)
        });
        let palette_g_slider = cx.new(|_| {
            SliderState::new()
                .min(0.0)
                .max(255.0)
                .step(1.0)
                .default_value(128.0)
        });
        let palette_b_slider = cx.new(|_| {
            SliderState::new()
                .min(0.0)
                .max(255.0)
                .step(1.0)
                .default_value(128.0)
        });

        subscriptions.push(
            cx.subscribe(&palette_r_slider, |this, _, event: &SliderEvent, cx| {
                if let SliderEvent::Change(val) = event {
                    if let Some(idx) = this.app.selected_color_idx {
                        if let crate::app::state::ThemeSource::CustomPalette(_, ref mut colors) =
                            this.app.current_theme
                        {
                            if let Some(c) = colors.get_mut(idx) {
                                c[0] = val.start() as u8;
                            }
                        }
                    }
                    this.app.needs_hex_sync = true;
                    cx.notify();
                }
            }),
        );
        subscriptions.push(
            cx.subscribe(&palette_g_slider, |this, _, event: &SliderEvent, cx| {
                if let SliderEvent::Change(val) = event {
                    if let Some(idx) = this.app.selected_color_idx {
                        if let crate::app::state::ThemeSource::CustomPalette(_, ref mut colors) =
                            this.app.current_theme
                        {
                            if let Some(c) = colors.get_mut(idx) {
                                c[1] = val.start() as u8;
                            }
                        }
                    }
                    this.app.needs_hex_sync = true;
                    cx.notify();
                }
            }),
        );
        subscriptions.push(
            cx.subscribe(&palette_b_slider, |this, _, event: &SliderEvent, cx| {
                if let SliderEvent::Change(val) = event {
                    if let Some(idx) = this.app.selected_color_idx {
                        if let crate::app::state::ThemeSource::CustomPalette(_, ref mut colors) =
                            this.app.current_theme
                        {
                            if let Some(c) = colors.get_mut(idx) {
                                c[2] = val.start() as u8;
                            }
                        }
                    }
                    this.app.needs_hex_sync = true;
                    cx.notify();
                }
            }),
        );

        cx.spawn(async move |this, cx| loop {
            cx.background_executor()
                .timer(std::time::Duration::from_secs(60))
                .await;
            let _ = this.update(cx, |view, cx| {
                if view.app.daemon_enabled && view.app.check_daemon_tick() {
                    view.trigger_async_processing(cx, "Automated time-of-day theme shift...");
                }
            });
        })
        .detach();

        cx.spawn(async move |this, cx| loop {
            cx.background_executor()
                .timer(std::time::Duration::from_millis(500))
                .await;
            let _ = this.update(cx, |view, cx| {
                view.app.update_system_stats();
                if view.app.sidebar_tab == crate::app::state::SidebarTab::Settings {
                    cx.notify();
                }
            });
        })
        .detach();

        let palette_hex_input = cx
            .new(|cx| gpui_component::input::InputState::new(window, cx).default_value("#1A1A1A"));

        subscriptions.push(cx.subscribe(
            &palette_hex_input,
            |this, _, event: &gpui_component::input::InputEvent, cx| {
                if let gpui_component::input::InputEvent::Change = event {
                    let text = this.palette_hex_input.read(cx).text().to_string();
                    if text.len() == 7 && text.starts_with('#') {
                        if let (Ok(r), Ok(g), Ok(b)) = (
                            u8::from_str_radix(&text[1..3], 16),
                            u8::from_str_radix(&text[3..5], 16),
                            u8::from_str_radix(&text[5..7], 16),
                        ) {
                            if let Some(idx) = this.app.selected_color_idx {
                                if let crate::app::state::ThemeSource::CustomPalette(
                                    _,
                                    ref mut colors,
                                ) = this.app.current_theme
                                {
                                    if let Some(c) = colors.get_mut(idx) {
                                        c[0] = r;
                                        c[1] = g;
                                        c[2] = b;
                                    }
                                }
                            }
                            this.app.needs_slider_sync = true;
                        }
                    }
                    cx.notify();
                }
            },
        ));

        let code_render_input = cx.new(|cx| {
            gpui_component::input::InputState::new(window, cx)
                .multi_line(true)
                .placeholder("Enter code here or select a file...")
        });

        Self {
            app: WallmodApp::new(),
            blur_slider,
            brightness_slider,
            contrast_slider,
            saturation_slider,
            hue_slider,
            palette_r_slider,
            palette_g_slider,
            palette_b_slider,
            palette_hex_input,
            code_render_input,
            shader_inputs: std::collections::HashMap::new(),
            subscriptions,
        }
    }

    pub fn trigger_node_processing(&mut self, cx: &mut Context<Self>, msg: &str) {
        if self.app.auto_apply_nodes {
            self.trigger_async_processing(cx, msg);
        } else {
            self.app.state = crate::app::AppState::Notice(
                "Pipeline modified (Manual Trigger Mode). Click '▶ Apply Pipeline' to render."
                    .to_string(),
            );
            cx.notify();
        }
    }

    pub fn copy_palette_to_clipboard(&mut self, cx: &mut Context<Self>, format: &str) {
        let shades = self.app.current_theme.get_shades();
        if shades.is_empty() {
            self.app.state =
                crate::app::AppState::Notice("No palette shades available to copy.".to_string());
            cx.notify();
            return;
        }
        let hex_shades: Vec<String> = shades
            .iter()
            .map(|rgb| format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2]))
            .collect();
        let formatted = match format {
            "json" => format!(
                "[\n  {}\n]",
                hex_shades
                    .iter()
                    .map(|s| format!("\"{}\"", s))
                    .collect::<Vec<_>>()
                    .join(",\n  ")
            ),
            "object" => {
                let entries: Vec<String> = hex_shades
                    .iter()
                    .enumerate()
                    .map(|(i, s)| format!("  \"color_{}\": \"{}\"", i + 1, s))
                    .collect();
                format!("{{\n{}\n}}", entries.join(",\n"))
            },
            _ => hex_shades.join(", "),
        };
        cx.write_to_clipboard(gpui::ClipboardItem::new_string(formatted.clone()));
        self.app.state = crate::app::AppState::Notice(format!(
            "Copied {} format to clipboard!",
            format.to_uppercase()
        ));
        cx.notify();
    }

    pub fn trigger_async_processing(&mut self, cx: &mut Context<Self>, msg: &str) {
        self.app.state = crate::app::AppState::Loading(0.5, msg.to_string());
        cx.notify();

        let _base_image_dyn = self.app.base_image_dyn.clone();
        let current_theme = self.app.current_theme.clone();
        let photoshop_params = self.app.photoshop_params;
        let blur_sigma = self.app.blur_sigma;
        let dither_enabled = self.app.dither_enabled;
        let seam_carve_target = self.app.seam_carve_target;
        let pixel_sort_enabled = self.app.pixel_sort_enabled;
        let theme_chain = self.app.theme_chain.clone();
        let chaining_mode = self.app.chaining_mode;
        let global_bit_depth = self.app.global_bit_depth;
        let base_image_dyn = match self.app.base_image_dyn.clone() {
            Some(img) => img,
            None => {
                self.app.state = crate::app::AppState::Idle;
                cx.notify();
                return;
            },
        };
        let algorithm = self.app.algorithm;
        let preserve_luma = self.app.preserve_luma;
        let hald_level = self.app.hald_level;

        let is_done = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let is_done_bg = is_done.clone();
        let is_done_ui = is_done.clone();
        let status_tracker = std::sync::Arc::new(std::sync::Mutex::new(None::<String>));
        let status_tracker_bg = status_tracker.clone();
        let status_tracker_ui = status_tracker.clone();

        cx.spawn(async move |this_view, cx| {
            while !is_done_ui.load(std::sync::atomic::Ordering::Relaxed) {
                cx.background_executor()
                    .timer(std::time::Duration::from_millis(50))
                    .await;
                if let Ok(lock) = status_tracker_ui.lock() {
                    if let Some(msg) = lock.clone() {
                        let _ = this_view.update(cx, |view: &mut WallmodView, cx| {
                            view.app.processing_status = Some(msg);
                            cx.notify();
                        });
                    }
                }
            }
        })
        .detach();

        let on_progress = Box::new(move |msg: String| {
            if let Ok(mut lock) = status_tracker_bg.lock() {
                *lock = Some(msg);
            }
        }) as Box<dyn Fn(String) + Send>;

        cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(std::time::Duration::from_millis(100))
                .await;

            let result = cx
                .background_executor()
                .spawn(async move {
                    let res = crate::app::WallmodApp::process_image_sync(
                        Some(base_image_dyn.clone()),
                        current_theme,
                        photoshop_params,
                        blur_sigma,
                        dither_enabled,
                        seam_carve_target,
                        pixel_sort_enabled,
                        theme_chain,
                        chaining_mode,
                        global_bit_depth,
                        algorithm,
                        preserve_luma,
                        hald_level,
                        Some(on_progress),
                    );
                    is_done_bg.store(true, std::sync::atomic::Ordering::Relaxed);
                    res
                })
                .await;

            let _ = this.update(cx, |view, cx| {
                match result {
                    Ok(Some((processed_dyn, temp_path, histogram, wcag_contrast))) => {
                        view.app
                            .update_preview(processed_dyn, temp_path, histogram, wcag_contrast);
                        view.app.processing_status = None;
                    },
                    Ok(None) => {
                        view.app.state = crate::app::AppState::Idle;
                        view.app.processing_status = None;
                    },
                    Err(err) => {
                        eprintln!("Processing error: {}", err);
                        view.app.state = crate::app::AppState::Error(err);
                        view.app.processing_status = None;
                    },
                }
                cx.notify();
            });
        })
        .detach();
    }

    pub fn trigger_async_extraction(&mut self, cx: &mut Context<Self>) {
        if self.app.base_image_dyn.is_none() {
            return;
        }
        self.app.state =
            crate::app::AppState::Loading(0.5, "Extracting dominant colors...".to_string());
        cx.notify();

        let base_image_dyn = self.app.base_image_dyn.clone().unwrap();
        let algorithm = self.app.algorithm;

        cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(std::time::Duration::from_millis(100))
                .await;

            let result = cx
                .background_executor()
                .spawn(async move {
                    crate::modules::extractor::extract_dominant_colors(
                        &base_image_dyn,
                        8,
                        algorithm as usize,
                    )
                })
                .await;

            let _ = this.update(cx, |view, cx| {
                match result {
                    Ok(colors) => {
                        view.app.extracted_colors = Some(colors);
                        view.app.state = crate::app::AppState::Idle;
                    },
                    Err(err) => {
                        eprintln!("Extraction error: {}", err);
                        view.app.state = crate::app::AppState::Error(err);
                    },
                }
                cx.notify();
            });
        })
        .detach();
    }
}

fn render_floating_tracker(
    app: &crate::app::WallmodApp,
    cx: &mut Context<WallmodView>,
) -> AnyElement {
    let (status_text, is_idle) = match &app.processing_status {
        Some(msg) => (msg.clone(), false),
        None => {
            if matches!(
                app.state,
                crate::app::AppState::Idle | crate::app::AppState::PreviewReady(_)
            ) {
                ("Idle".to_string(), true)
            } else {
                ("Processing...".to_string(), false)
            }
        },
    };

    let bg_color = if is_idle {
        gpui::Hsla {
            h: 0.3,
            s: 0.8,
            l: 0.4,
            a: 0.9,
        }
    } else {
        cx.theme().secondary
    };
    div()
        .absolute()
        .bottom(px(16.0))
        .right(px(16.0))
        .w(px(250.0))
        .p_4()
        .rounded_xl()
        .bg(bg_color)
        .border_1()
        .border_color(cx.theme().border)
        .shadow_lg()
        .child(
            v_flex()
                .gap_2()
                .items_center()
                .child(div().text_sm().font_bold().child(if is_idle {
                    "Pipeline Ready"
                } else {
                    "Pipeline Running"
                }))
                .child(
                    div()
                        .text_xs()
                        .text_color(if is_idle {
                            gpui::black()
                        } else {
                            cx.theme().muted_foreground
                        })
                        .child(status_text),
                ),
        )
        .into_any_element()
}

fn render_floating_stats(
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
                .child(
                    div()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("FPS"),
                )
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
                .child(
                    div()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("RAM"),
                )
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
                .child(
                    div()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("CPU (Avg)"),
                )
                .child(
                    div()
                        .text_xs()
                        .font_bold()
                        .text_color(cx.theme().primary)
                        .child(format!("{:.0}%", avg_cpu)),
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
                .child(
                    gpui::svg()
                        .path("monitor.svg")
                        .size_3()
                        .text_color(cx.theme().primary),
                ),
        )
        .children(items)
        .into_any_element()
}

impl Render for WallmodView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.app.record_frame();
        div()
            .size_full()
            .relative()
            .child(
                v_flex()
                    .size_full()
                    .bg(cx.theme().background)
                    .text_color(cx.theme().foreground)
                    .child(header::render_header(self, cx))
                    .child(
                        h_flex()
                            .flex_1()
                            .w_full()
                            .overflow_hidden()
                            .child(sidebar::render_sidebar(self, cx))
                            .child(workspace::render_workspace(self, window, cx)),
                    ),
            )
            .children(if self.app.show_floating_stats {
                Some(render_floating_stats(&self.app, cx))
            } else {
                None
            })
            .children(if self.app.show_progress_panel {
                Some(render_floating_tracker(&self.app, cx))
            } else {
                None
            })
    }
}

impl WallmodView {
    pub fn open_image_from_path(&mut self, path: std::path::PathBuf, cx: &mut Context<Self>) {
        cx.spawn(async move |this, cx| {
            let path_clone = path.clone();
            let res = crate::backend::runtime::spawn_blocking(move || {
                crate::app::helpers::open_image(&path_clone)
            })
            .await;
            match res {
                Ok(Ok(dyn_img)) => {
                    let _ = this.update(cx, |view, cx| {
                        view.app.on_image_selected(path, dyn_img);
                        view.trigger_async_processing(cx, "Applying theme...");
                    });
                },
                Ok(Err(e)) => {
                    let _ = this.update(cx, |view, _cx| {
                        view.app.state =
                            crate::app::AppState::Error(format!("Failed to decode image: {}", e));
                    });
                },
                Err(e) => {
                    let _ = this.update(cx, |view, _cx| {
                        view.app.state =
                            crate::app::AppState::Error(format!("Thread panicked: {}", e));
                    });
                },
            }
        })
        .detach();
    }

    pub fn open_image_dialog(&mut self, cx: &mut Context<Self>) {
        cx.spawn(async move |this, cx| {
            if let Some(file) = rfd::AsyncFileDialog::new()
                .add_filter(
                    "Image",
                    &[
                        "png", "jpg", "jpeg", "webp", "avif", "bmp", "tiff", "tga", "gif", "ico",
                        "hdr", "exr", "qoi", "pnm",
                    ],
                )
                .pick_file()
                .await
            {
                let path = file.path().to_path_buf();
                let _ = this.update(cx, |view, cx| {
                    view.open_image_from_path(path, cx);
                });
            }
        })
        .detach();
    }
}
