use gpui::*;
use std::time::Duration;

pub fn render_test(cx: &mut App) -> impl IntoElement {
    div()
        .with_animation(
            "bouncy",
            Animation::new(Duration::from_secs_f32(1.0)).repeat(),
            |this, delta| {
                this.transform(Transformation::scale(1.0 + (delta * std::f32::consts::PI * 2.0).sin() * 0.3))
            }
        )
}
