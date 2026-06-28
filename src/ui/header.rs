//! Top navigation and Client-Side Decoration (CSD) header bar component.

use crate::app::{Message, WallmodApp};
use crate::ui::icon::{icon, Icon};
use crate::ui::theme::{button_style, panel_container_style, txt_muted, txt_primary, ButtonVariant};
use iced::widget::{button, container, row, space, text};
use iced::{Alignment, Element, Length};

/// Renders the top application header bar with title branding, theme toggle, and window controls.
pub fn view(app: &WallmodApp) -> Element<'_, Message> {
    let is_dark = app.is_dark_mode();
    let tp = txt_primary(is_dark);
    let tm = txt_muted(is_dark);

    let brand = row![
        icon(Icon::Layers).size(16).color(tp),
        text("wallmod studio").size(15).color(tp),
        text("ricer edition").size(11).color(tm),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let make_csd_btn = move |label: &'static str, msg: Message| -> Element<'_, Message> {
        button(text(label).size(12).color(tp))
            .padding([4, 8])
            .on_press(msg)
            .style(|theme, status| button_style(theme, status, ButtonVariant::Ghost))
            .into()
    };

    let theme_toggle = button(
        row![
            icon(if is_dark { Icon::Moon } else { Icon::Sun }).size(14).color(tp),
            text(if is_dark { "Dark Mode" } else { "Light Mode" }).size(12).color(tp)
        ].spacing(6).align_y(Alignment::Center)
    )
        .padding([6, 12])
        .on_press(Message::ToggleAppTheme)
        .style(|theme, status| button_style(theme, status, ButtonVariant::Secondary));

    let mut tabs_row = row![].spacing(8);
    for tab in crate::app::state::AppTab::ALL {
        let is_active = app.active_tab() == *tab;
        let variant = if is_active { ButtonVariant::Primary } else { ButtonVariant::Ghost };
        let tab_icon = match tab {
            crate::app::state::AppTab::Themer => Icon::Palette,
            crate::app::state::AppTab::Upscaler => Icon::ArrowsAngleExpand,
            crate::app::state::AppTab::Ocr => Icon::FileType,
            crate::app::state::AppTab::Compression => Icon::FileZip,
        };
        let c = if is_active { tp } else { tm };
        let btn = button(
            row![
                icon(tab_icon).size(14).color(c),
                text(tab.to_string()).size(13).color(c),
            ].spacing(6).align_y(Alignment::Center)
        )
        .padding([6, 12])
        .on_press(Message::AppTabChanged(*tab))
        .style(move |theme, status| button_style(theme, status, variant));
        
        tabs_row = tabs_row.push(btn);
    }

    container(
        row![
            brand,
            space().width(Length::Fill),
            tabs_row,
            space().width(Length::Fill),
            theme_toggle,
            space().width(12),
            row![
                make_csd_btn("_", Message::WindowMinimize),
                make_csd_btn("[ ]", Message::WindowMaximize),
                make_csd_btn("X", Message::WindowClose),
            ]
            .spacing(4)
            .align_y(Alignment::Center)
        ]
        .align_y(Alignment::Center),
    )
    .padding([12, 20])
    .style(panel_container_style)
    .into()
}
