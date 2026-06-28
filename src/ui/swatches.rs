//! Color swatch component displaying active palette shades.

use crate::app::ThemeSource;
use crate::ui::theme::{txt_muted, txt_primary, RADIUS_SM};
use iced::widget::{column, container, row, space, text};
use iced::{Background, Border, Color, Element, Length, Theme};

/// Renders a color swatch row representing the shades of the selected theme palette.
pub fn view(current_theme: &ThemeSource, is_dark: bool) -> Element<'_, crate::app::Message> {
    let tp = txt_primary(is_dark);
    let tm = txt_muted(is_dark);
    let shades = current_theme.get_shades();
    let mut swatches_row = row![].spacing(4).width(Length::Fill);

    for &rgb in shades.iter().take(8) {
        let swatch = container(space())
            .width(Length::FillPortion(1))
            .height(24)
            .style(move |_theme: &Theme| container::Style {
                background: Some(Background::Color(Color::from_rgb(
                    rgb[0] as f32 / 255.0,
                    rgb[1] as f32 / 255.0,
                    rgb[2] as f32 / 255.0,
                ))),
                border: Border { radius: RADIUS_SM.into(), ..Default::default() },
                ..Default::default()
            });
        swatches_row = swatches_row.push(swatch);
    }

    column![
        row![
            text("ACTIVE COLOR SHADES").size(11).color(tm),
            space().width(Length::Fill),
            text(current_theme.display_name()).size(11).color(tp),
        ],
        swatches_row
    ]
    .spacing(6)
    .into()
}
