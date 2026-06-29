//! Centralized GPUI UI Presentation Layer.
//! Refactored into clean modular components following standard software engineering practices.

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
    pub subscriptions: Vec<Subscription>,
}

impl WallmodView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let blur_slider =
            cx.new(|_| SliderState::new().min(0.0).max(50.0).step(0.5).default_value(0.0));
        let brightness_slider =
            cx.new(|_| SliderState::new().min(-100.0).max(100.0).step(1.0).default_value(0.0));
        let contrast_slider =
            cx.new(|_| SliderState::new().min(-100.0).max(100.0).step(1.0).default_value(0.0));
        let saturation_slider =
            cx.new(|_| SliderState::new().min(-1.0).max(1.0).step(0.05).default_value(0.0));
        let hue_slider =
            cx.new(|_| SliderState::new().min(0.0).max(360.0).step(1.0).default_value(0.0));

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

        subscriptions.push(cx.subscribe(&brightness_slider, |this, _, event: &SliderEvent, cx| {
            match event {
                SliderEvent::Change(val) => {
                    this.app.photoshop_params.brightness = val.start() as i32;
                    cx.notify();
                },
                SliderEvent::Release(val) => {
                    this.app.photoshop_params.brightness = val.start() as i32;
                    this.trigger_async_processing(cx, "Adjusting brightness...");
                },
            }
        }));

        subscriptions.push(cx.subscribe(&contrast_slider, |this, _, event: &SliderEvent, cx| {
            match event {
                SliderEvent::Change(val) => {
                    this.app.photoshop_params.contrast = val.start();
                    cx.notify();
                },
                SliderEvent::Release(val) => {
                    this.app.photoshop_params.contrast = val.start();
                    this.trigger_async_processing(cx, "Adjusting contrast...");
                },
            }
        }));

        subscriptions.push(cx.subscribe(&saturation_slider, |this, _, event: &SliderEvent, cx| {
            match event {
                SliderEvent::Change(val) => {
                    this.app.photoshop_params.saturation = val.start();
                    cx.notify();
                },
                SliderEvent::Release(val) => {
                    this.app.photoshop_params.saturation = val.start();
                    this.trigger_async_processing(cx, "Adjusting saturation...");
                },
            }
        }));

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

        let palette_r_slider =
            cx.new(|_| SliderState::new().min(0.0).max(255.0).step(1.0).default_value(128.0));
        let palette_g_slider =
            cx.new(|_| SliderState::new().min(0.0).max(255.0).step(1.0).default_value(128.0));
        let palette_b_slider =
            cx.new(|_| SliderState::new().min(0.0).max(255.0).step(1.0).default_value(128.0));

        subscriptions.push(cx.subscribe(&palette_r_slider, |this, _, event: &SliderEvent, cx| {
            match event {
                SliderEvent::Change(val) => {
                    if let Some(idx) = this.app.selected_color_idx {
                        if let crate::app::state::ThemeSource::CustomPalette(_, ref mut colors) =
                            this.app.current_theme
                        {
                            if let Some(c) = colors.get_mut(idx) {
                                c[0] = val.start() as u8;
                            }
                        }
                    }
                    cx.notify();
                },
                _ => {},
            }
        }));
        subscriptions.push(cx.subscribe(&palette_g_slider, |this, _, event: &SliderEvent, cx| {
            match event {
                SliderEvent::Change(val) => {
                    if let Some(idx) = this.app.selected_color_idx {
                        if let crate::app::state::ThemeSource::CustomPalette(_, ref mut colors) =
                            this.app.current_theme
                        {
                            if let Some(c) = colors.get_mut(idx) {
                                c[1] = val.start() as u8;
                            }
                        }
                    }
                    cx.notify();
                },
                _ => {},
            }
        }));
        subscriptions.push(cx.subscribe(&palette_b_slider, |this, _, event: &SliderEvent, cx| {
            match event {
                SliderEvent::Change(val) => {
                    if let Some(idx) = this.app.selected_color_idx {
                        if let crate::app::state::ThemeSource::CustomPalette(_, ref mut colors) =
                            this.app.current_theme
                        {
                            if let Some(c) = colors.get_mut(idx) {
                                c[2] = val.start() as u8;
                            }
                        }
                    }
                    cx.notify();
                },
                _ => {},
            }
        }));

        cx.spawn(async move |this, cx| loop {
            cx.background_executor().timer(std::time::Duration::from_secs(60)).await;
            let _ = this.update(cx, |view, cx| {
                if view.app.daemon_enabled {
                    if view.app.check_daemon_tick() {
                        view.trigger_async_processing(cx, "Automated time-of-day theme shift...");
                    }
                }
            });
        })
        .detach();

        cx.spawn(async move |this, cx| loop {
            cx.background_executor().timer(std::time::Duration::from_millis(500)).await;
            let _ = this.update(cx, |view, cx| {
                view.app.update_system_stats();
                if view.app.sidebar_tab == crate::app::state::SidebarTab::Settings {
                    cx.notify();
                }
            });
        })
        .detach();

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

    pub fn trigger_async_processing(&mut self, cx: &mut Context<Self>, msg: &str) {
        self.app.state = crate::app::AppState::Loading(0.5, msg.to_string());
        cx.notify();

        let base_image_dyn = self.app.base_image_dyn.clone();
        let current_theme = self.app.current_theme.clone();
        let photoshop_params = self.app.photoshop_params.clone();
        let blur_sigma = self.app.blur_sigma;
        let dither_enabled = self.app.dither_enabled;
        let seam_carve_target = self.app.seam_carve_target;
        let pixel_sort_enabled = self.app.pixel_sort_enabled;
        let theme_chain = self.app.theme_chain.clone();
        let chaining_mode = self.app.chaining_mode;
        let global_bit_depth = self.app.global_bit_depth;
        let algorithm = self.app.algorithm.clone();
        let preserve_luma = self.app.preserve_luma;
        let hald_level = self.app.hald_level;

        cx.spawn(async move |this, cx| {
            cx.background_executor().timer(std::time::Duration::from_millis(100)).await;

            let result = cx
                .background_executor()
                .spawn(async move {
                    crate::app::WallmodApp::process_image_sync(
                        base_image_dyn,
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
                    )
                })
                .await;

            let _ = this.update(cx, |view, cx| {
                match result {
                    Ok(Some((processed_dyn, temp_path, histogram, wcag_contrast))) => {
                        view.app.update_preview(processed_dyn, temp_path, histogram, wcag_contrast);
                    },
                    Ok(None) => {
                        view.app.state = crate::app::AppState::Idle;
                    },
                    Err(err) => {
                        eprintln!("Processing error: {}", err);
                        view.app.state = crate::app::AppState::Error(err);
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

        cx.spawn(async move |this, cx| {
            cx.background_executor().timer(std::time::Duration::from_millis(100)).await;

            let result = cx
                .background_executor()
                .spawn(async move {
                    crate::modules::extractor::extract_dominant_colors(&base_image_dyn, 8)
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

fn render_floating_stats(
    app: &crate::app::WallmodApp,
    cx: &mut Context<WallmodView>,
) -> AnyElement {
    let avg_cpu = if app.sys_cpu_threads.is_empty() {
        0.0
    } else {
        app.sys_cpu_threads.iter().sum::<f32>() / app.sys_cpu_threads.len() as f32
    };
    let any_high = app.sys_ram_percent > 80.0
        || avg_cpu > 80.0
        || app.sys_cpu_threads.iter().any(|&t| t > 80.0);

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
        let ram_col = if app.sys_ram_percent > 80.0 {
            gpui::rgb(0xffffff).into()
        } else {
            cx.theme().primary
        };
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
                        .text_color(ram_col)
                        .child(format!("{:.1}%", app.sys_ram_percent)),
                )
                .into_any_element(),
        );
    }
    if app.float_show_cpu {
        let color = if avg_cpu > 80.0 {
            gpui::rgb(0xffffff)
        } else if avg_cpu > 50.0 {
            gpui::rgb(0xf59e0b)
        } else {
            gpui::rgb(0x22c55e)
        };
        items.push(
            h_flex()
                .justify_between()
                .gap_4()
                .items_center()
                .child(div().text_xs().text_color(cx.theme().muted_foreground).child("CPU (Avg)"))
                .child(
                    div().text_xs().font_bold().text_color(color).child(format!("{:.0}%", avg_cpu)),
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

    let bg_color = if any_high {
        gpui::Rgba {
            r: 0.85,
            g: 0.15,
            b: 0.15,
            a: 0.65,
        }
        .into()
    } else {
        cx.theme().secondary.opacity(0.60)
    };
    let border_color = if any_high {
        gpui::Rgba {
            r: 0.95,
            g: 0.30,
            b: 0.30,
            a: 0.80,
        }
        .into()
    } else {
        cx.theme().border.opacity(0.4)
    };

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
        .bg(bg_color)
        .border_1()
        .border_color(border_color)
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

impl Render for WallmodView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                            .child(workspace::render_workspace(self, cx)),
                    ),
            )
            .children(if self.app.show_floating_stats {
                Some(render_floating_stats(&self.app, cx))
            } else {
                None
            })
    }
}
