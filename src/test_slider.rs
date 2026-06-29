use gpui::{Context, Window};
use gpui_component::slider::SliderState;

pub fn test(s: &mut SliderState, window: &mut Window, cx: &mut Context<SliderState>) {
    s.set_value(10.0, window, cx);
}
