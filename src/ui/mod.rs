//! Central UI module assembling modular Shadcn components.

pub mod header;
pub mod sidebar;
pub mod swatches;
pub mod theme;
pub mod workspace;
pub mod histogram;
pub mod icon;

use crate::app::{Message, WallmodApp};
use iced::widget::{column, row};
use iced::{Element, Length};

/// Master UI assembler function combining header, sidebar controls, and workspace preview.
pub fn view(app: &WallmodApp) -> Element<'_, Message> {
    let header_bar = header::view(app);
    let left_controls = sidebar::view(app);
    let right_preview = workspace::view(app);

    column![
        header_bar,
        row![left_controls, right_preview].height(Length::Fill)
    ]
    .height(Length::Fill)
    .into()
}
