//! Centralized Shadcn UI Design Tokens & Component Styling
//!
//! Just like `components/ui/` or `tailwind.config.ts` / `theme.css` in modern web development,
//! this file is the single source of truth for all aesthetics in WallMod Studio.
//! Changing tokens or button styling here will update every component across the entire application.

use iced::widget::{button, container};
use iced::{Background, Border, Color, Theme};

// ==========================================
// 🎨 DESIGN TOKENS (Shadcn Zinc / Slate Dark)
// ==========================================

pub const BG_COLOR: Color = Color::from_rgb(0.035, 0.035, 0.043);       // #09090b (Main Canvas)
pub const PANEL_BG: Color = Color::from_rgb(0.094, 0.094, 0.106);       // #18181b (Sidebar Panel)
pub const CARD_BG: Color = Color::from_rgb(0.06, 0.06, 0.07);           // #0f0f12 (Inner Cards)
pub const BORDER_COLOR: Color = Color::from_rgb(0.153, 0.153, 0.165);   // #27272a (Borders & Separators)
pub const TEXT_PRIMARY: Color = Color::WHITE;                           // #fafafa (Primary Typography)
pub const TEXT_MUTED: Color = Color::from_rgb(0.631, 0.631, 0.667);     // #a1a1aa (Subtitles & Labels)
pub const ACCENT_COLOR: Color = Color::from_rgb(0.98, 0.98, 1.0);       // #fafafa (Primary Buttons)
pub const ERROR_COLOR: Color = Color::from_rgb(0.95, 0.35, 0.35);       // #f25f5c (Alerts & Errors)

pub const RADIUS_MD: f32 = 6.0;                                         // Standard Component Radius
pub const RADIUS_SM: f32 = 4.0;                                         // Swatch & Badge Radius

pub fn txt_primary(is_dark: bool) -> Color {
    if is_dark { TEXT_PRIMARY } else { Color::from_rgb(0.09, 0.09, 0.11) }
}

pub fn txt_muted(is_dark: bool) -> Color {
    if is_dark { TEXT_MUTED } else { Color::from_rgb(0.42, 0.42, 0.48) }
}

// ==========================================
// 🔘 BUTTON STYLING PRIMITIVES
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Accent,
    Disabled,
    Ghost,
}

pub fn button_style(theme: &Theme, status: button::Status, variant: ButtonVariant) -> button::Style {
    let is_light = theme == &Theme::Light;
    let txt = if is_light { Color::from_rgb(0.09, 0.09, 0.11) } else { Color::WHITE };
    match variant {
        ButtonVariant::Primary => match status {
            button::Status::Hovered => button::Style {
                background: Some(Background::Color(if is_light { Color::from_rgb(0.8, 0.84, 0.9) } else { Color::from_rgb(0.2, 0.22, 0.28) })),
                text_color: txt,
                border: Border { radius: RADIUS_MD.into(), ..Default::default() },
                ..Default::default()
            },
            _ => button::Style {
                background: Some(Background::Color(if is_light { Color::from_rgb(0.88, 0.9, 0.95) } else { Color::from_rgb(0.15, 0.16, 0.2) })),
                text_color: txt,
                border: Border { radius: RADIUS_MD.into(), ..Default::default() },
                ..Default::default()
            },
        },
        ButtonVariant::Secondary => match status {
            button::Status::Hovered => button::Style {
                background: Some(Background::Color(if is_light { Color::from_rgb(0.85, 0.85, 0.88) } else { Color::from_rgb(0.14, 0.14, 0.16) })),
                text_color: txt,
                border: Border { color: if is_light { Color::from_rgb(0.7, 0.7, 0.75) } else { Color::from_rgb(0.3, 0.3, 0.35) }, width: 1.0, radius: RADIUS_MD.into() },
                ..Default::default()
            },
            _ => button::Style {
                background: Some(Background::Color(if is_light { Color::from_rgb(0.92, 0.92, 0.94) } else { Color::from_rgb(0.09, 0.09, 0.1) })),
                text_color: txt,
                border: Border { color: if is_light { Color::from_rgb(0.8, 0.8, 0.84) } else { Color::from_rgb(0.2, 0.2, 0.25) }, width: 1.0, radius: RADIUS_MD.into() },
                ..Default::default()
            },
        },
        ButtonVariant::Accent => match status {
            button::Status::Hovered => button::Style {
                background: Some(Background::Color(if is_light { Color::from_rgb(0.15, 0.15, 0.18) } else { Color::from_rgb(0.9, 0.9, 0.95) })),
                text_color: if is_light { Color::WHITE } else { Color::BLACK },
                border: Border { radius: RADIUS_MD.into(), ..Default::default() },
                ..Default::default()
            },
            _ => button::Style {
                background: Some(Background::Color(if is_light { Color::from_rgb(0.1, 0.1, 0.12) } else { ACCENT_COLOR })),
                text_color: if is_light { Color::WHITE } else { Color::BLACK },
                border: Border { radius: RADIUS_MD.into(), ..Default::default() },
                ..Default::default()
            },
        },
        ButtonVariant::Disabled => button::Style {
            background: Some(Background::Color(if is_light { Color::from_rgb(0.9, 0.9, 0.92) } else { Color::from_rgb(0.07, 0.07, 0.08) })),
            text_color: if is_light { Color::from_rgb(0.6, 0.6, 0.65) } else { Color::from_rgb(0.4, 0.4, 0.45) },
            border: Border { color: if is_light { Color::from_rgb(0.8, 0.8, 0.82) } else { Color::from_rgb(0.12, 0.12, 0.14) }, width: 1.0, radius: RADIUS_MD.into() },
            ..Default::default()
        },
        ButtonVariant::Ghost => match status {
            button::Status::Hovered => button::Style {
                background: Some(Background::Color(if is_light { Color::from_rgb(0.86, 0.86, 0.89) } else { Color::from_rgb(0.18, 0.18, 0.2) })),
                text_color: txt,
                border: Border { radius: RADIUS_SM.into(), ..Default::default() },
                ..Default::default()
            },
            _ => button::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: txt,
                border: Border { radius: RADIUS_SM.into(), ..Default::default() },
                ..Default::default()
            },
        },
    }
}

// ==========================================
// 📦 CONTAINER CARD STYLING PRIMITIVES
// ==========================================

pub fn card_container_style(theme: &Theme) -> container::Style {
    if theme == &Theme::Light {
        container::Style {
            background: Some(Background::Color(Color::from_rgb(0.98, 0.98, 0.99))),
            border: Border { color: Color::from_rgb(0.85, 0.85, 0.88), width: 1.0, radius: RADIUS_MD.into() },
            ..Default::default()
        }
    } else {
        container::Style {
            background: Some(Background::Color(CARD_BG)),
            border: Border { color: BORDER_COLOR, width: 1.0, radius: RADIUS_MD.into() },
            ..Default::default()
        }
    }
}

pub fn panel_container_style(theme: &Theme) -> container::Style {
    if theme == &Theme::Light {
        container::Style {
            background: Some(Background::Color(Color::from_rgb(0.94, 0.94, 0.96))),
            border: Border { color: Color::from_rgb(0.82, 0.82, 0.86), width: 1.0, ..Default::default() },
            ..Default::default()
        }
    } else {
        container::Style {
            background: Some(Background::Color(PANEL_BG)),
            border: Border { color: BORDER_COLOR, width: 1.0, ..Default::default() },
            ..Default::default()
        }
    }
}

pub fn canvas_container_style(theme: &Theme) -> container::Style {
    if theme == &Theme::Light {
        container::Style {
            background: Some(Background::Color(Color::from_rgb(0.90, 0.90, 0.93))),
            border: Border { color: Color::from_rgb(0.82, 0.82, 0.86), width: 1.0, ..Default::default() },
            ..Default::default()
        }
    } else {
        container::Style {
            background: Some(Background::Color(BG_COLOR)),
            border: Border { color: BORDER_COLOR, width: 1.0, ..Default::default() },
            ..Default::default()
        }
    }
}
