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
    _subscriptions: Vec<Subscription>,
}

impl WallmodView {
    pub fn new(cx: &mut Context<Self>) -> Self {
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

        subscriptions.push(cx.subscribe(&blur_slider, |this, _, event: &SliderEvent, cx| match event {
            SliderEvent::Change(val) => {
                this.app.blur_sigma = val.start();
                cx.notify();
            }
            SliderEvent::Release(val) => {
                this.app.blur_sigma = val.start();
                this.trigger_async_processing(cx, "Applying blur...");
            }
        }));

        subscriptions.push(cx.subscribe(&brightness_slider, |this, _, event: &SliderEvent, cx| match event {
            SliderEvent::Change(val) => {
                this.app.photoshop_params.brightness = val.start() as i32;
                cx.notify();
            }
            SliderEvent::Release(val) => {
                this.app.photoshop_params.brightness = val.start() as i32;
                this.trigger_async_processing(cx, "Adjusting brightness...");
            }
        }));

        subscriptions.push(cx.subscribe(&contrast_slider, |this, _, event: &SliderEvent, cx| match event {
            SliderEvent::Change(val) => {
                this.app.photoshop_params.contrast = val.start();
                cx.notify();
            }
            SliderEvent::Release(val) => {
                this.app.photoshop_params.contrast = val.start();
                this.trigger_async_processing(cx, "Adjusting contrast...");
            }
        }));

        subscriptions.push(cx.subscribe(&saturation_slider, |this, _, event: &SliderEvent, cx| match event {
            SliderEvent::Change(val) => {
                this.app.photoshop_params.saturation = val.start();
                cx.notify();
            }
            SliderEvent::Release(val) => {
                this.app.photoshop_params.saturation = val.start();
                this.trigger_async_processing(cx, "Adjusting saturation...");
            }
        }));

        subscriptions.push(cx.subscribe(&hue_slider, |this, _, event: &SliderEvent, cx| match event {
            SliderEvent::Change(val) => {
                this.app.photoshop_params.hue = val.start() as i32;
                cx.notify();
            }
            SliderEvent::Release(val) => {
                this.app.photoshop_params.hue = val.start() as i32;
                this.trigger_async_processing(cx, "Shifting hue...");
            }
        }));

        Self {
            app: WallmodApp::new(),
            blur_slider,
            brightness_slider,
            contrast_slider,
            saturation_slider,
            hue_slider,
            _subscriptions: subscriptions,
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
        let algorithm = self.app.algorithm.clone();
        let preserve_luma = self.app.preserve_luma;
        let hald_level = self.app.hald_level;

        cx.spawn(async move |this, cx| {
            cx.background_executor().timer(std::time::Duration::from_millis(100)).await;

            let result = cx.background_executor().spawn(async move {
                crate::app::WallmodApp::process_image_sync(
                    base_image_dyn,
                    current_theme,
                    photoshop_params,
                    blur_sigma,
                    dither_enabled,
                    algorithm,
                    preserve_luma,
                    hald_level,
                )
            }).await;

            let _ = this.update(cx, |view, cx| {
                match result {
                    Ok(Some((processed_dyn, temp_path, histogram, wcag_contrast))) => {
                        view.app.update_preview(processed_dyn, temp_path, histogram, wcag_contrast);
                    }
                    Ok(None) => {
                        view.app.state = crate::app::AppState::Idle;
                    }
                    Err(err) => {
                        eprintln!("Processing error: {}", err);
                        view.app.state = crate::app::AppState::Error(err);
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }
}

impl Render for WallmodView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
            )
    }
}
