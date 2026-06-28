//! Left control panel (30% width) containing all interactive controls.
//! Organized into clean tabs (Theme & LUT, Desktop Engine, Export & Sync).

use crate::app::{AppState, Message, RemapAlgorithm, SidebarTab, WallpaperBackend, PRESET_NAMES, SWWW_TRANSITIONS, TARGET_DISPLAYS};
use crate::ui::icon::{icon, Icon};
use crate::ui::swatches;
use crate::ui::theme::{button_style, card_container_style, txt_muted, txt_primary, ButtonVariant};
use iced::widget::{button, checkbox, column, container, pick_list, row, scrollable, slider, space, text, text_input, toggler};
use iced::{Alignment, Element, Length};

/// Renders the complete left control panel assembling all unified feature categories.
#[allow(non_snake_case)]
pub fn view(app: &crate::app::WallmodApp) -> Element<'_, Message> {
    let TEXT_PRIMARY = txt_primary(app.is_dark_mode());
    let TEXT_MUTED = txt_muted(app.is_dark_mode());

    let parse_hex = |hex: &str| -> iced::Color {
        let hex = hex.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            iced::Color::from_rgb8(r, g, b)
        } else {
            iced::Color::BLACK
        }
    };
    let state = app.state();
    let has_image = app.has_image();
    let current_tab = app.sidebar_tab();

    // 1. Top Tabs Row
    let tab_bar = row(
        SidebarTab::ALL.iter().map(|&tab| {
            let is_active = tab == current_tab;
            let variant = if is_active { ButtonVariant::Primary } else { ButtonVariant::Secondary };
            let tab_text_color = if app.is_dark_mode() {
                if is_active { iced::Color::WHITE } else { iced::Color::from_rgb(0.9, 0.9, 0.95) }
            } else {
                if is_active { iced::Color::from_rgb(0.05, 0.05, 0.1) } else { iced::Color::from_rgb(0.3, 0.3, 0.35) }
            };
            button(text(tab.to_string()).size(11).color(tab_text_color))
                .padding([8, 8])
                .width(Length::Fill)
                .on_press(Message::SidebarTabChanged(tab))
                .style(move |theme, status| button_style(theme, status, variant))
                .into()
        })
    )
    .spacing(4);

    // CATEGORY A: Image Input & Source Management
    let cat_a_section = container(
        column![
            text("CATEGORY A (SOURCE & THEME)").size(11).color(TEXT_MUTED),
            button(
                row![icon(Icon::Image).size(14).color(TEXT_PRIMARY), text("Open Base Image").size(13).color(TEXT_PRIMARY)].spacing(8).align_y(Alignment::Center)
            )
            .width(Length::Fill)
            .padding([10, 16])
            .on_press(Message::SelectImage)
            .style(|theme, status| button_style(theme, status, ButtonVariant::Primary)),
            button(
                row![icon(Icon::ArrowRepeat).size(14).color(TEXT_PRIMARY), text("Batch Process Directory").size(13).color(TEXT_PRIMARY)].spacing(8).align_y(Alignment::Center)
            )
            .width(Length::Fill)
            .padding([10, 16])
            .on_press(Message::SelectBatchFolder)
            .style(|theme, status| button_style(theme, status, ButtonVariant::Secondary)),
            button(
                row![icon(Icon::Download).size(14).color(TEXT_PRIMARY), text("Import Custom LUT (.cube/.png)").size(13).color(TEXT_PRIMARY)].spacing(8).align_y(Alignment::Center)
            )
            .width(Length::Fill)
            .padding([10, 16])
            .on_press(Message::SelectCustomTheme)
            .style(|theme, status| button_style(theme, status, ButtonVariant::Secondary)),
        ]
        .spacing(6)
    );

    // CATEGORY B: Color Grading & Palette Engine (`lutgen-rs`)
    let preset_picker = column![
        text("PRESET PALETTE").size(11).color(TEXT_MUTED),
        pick_list(
            PRESET_NAMES,
            app.selected_preset(),
            |name| Message::ThemePresetSelected(name.to_string())
        )
        .padding([8, 12])
        .width(Length::Fill)
    ]
    .spacing(4);

    let algo_picker = column![
        text("INTERPOLATION ALGORITHM").size(11).color(TEXT_MUTED),
        pick_list(
            RemapAlgorithm::ALL,
            Some(app.algorithm()),
            Message::AlgorithmChanged
        )
        .padding([8, 12])
        .width(Length::Fill)
    ]
    .spacing(4);

    let hald_level_picker = column![
        text("HALDCLUT RESOLUTION").size(11).color(TEXT_MUTED),
        pick_list(
            vec![8u8, 16u8],
            Some(app.hald_level()),
            Message::HaldLevelChanged
        )
        .padding([8, 12])
        .width(Length::Fill)
    ]
    .spacing(4);

    let luma_toggle = row![
        checkbox(app.preserve_luma()).on_toggle(Message::TogglePreserveLuma).size(16),
        text("Preserve Luminance (Luma Lock)").size(13).color(TEXT_PRIMARY)
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let mut extract_palette_content = column![].spacing(8);

    if has_image {
        extract_palette_content = extract_palette_content.push(
            button(
                row![icon(Icon::Magic).size(13).color(TEXT_PRIMARY), text("Extract Dominant Oklab Colors").size(12).color(TEXT_PRIMARY)].spacing(6).align_y(Alignment::Center)
            )
            .padding([8, 14])
            .width(Length::Fill)
            .on_press(Message::ExtractDominantColors)
            .style(|theme, status| button_style(theme, status, ButtonVariant::Secondary))
        );
    } else {
        extract_palette_content = extract_palette_content.push(
            button(
                row![icon(Icon::Magic).size(13).color(TEXT_MUTED), text("Extract Palette (No Image)").size(12).color(TEXT_MUTED)].spacing(6).align_y(Alignment::Center)
            )
            .padding([8, 14])
            .width(Length::Fill)
            .style(|theme, status| button_style(theme, status, ButtonVariant::Disabled))
        );
    }

    if let Some(colors) = app.extracted_colors() {
        let mut swatches_row = row![].spacing(4).width(Length::Fill);
        for hex in colors.iter().take(8) {
            let color = parse_hex(hex);
            let swatch = container(space())
                .width(Length::FillPortion(1))
                .height(24)
                .style(move |_theme: &iced::Theme| iced::widget::container::Style {
                    background: Some(iced::Background::Color(color)),
                    border: iced::Border { radius: crate::ui::theme::RADIUS_SM.into(), ..Default::default() },
                    ..Default::default()
                });
            swatches_row = swatches_row.push(swatch);
        }
        extract_palette_content = extract_palette_content.push(swatches_row);
    }

    let extract_palette_btn = extract_palette_content;

    let swatches_block = swatches::view(app.current_theme(), app.is_dark_mode());

    let custom_palette_section = container(
        column![
            text("CUSTOM HEX PALETTE BUILDER").size(11).color(TEXT_MUTED),
            text_input("Hex codes (#89b4fa, #f38ba8)", app.custom_palette_input())
                .on_input(Message::CustomPaletteInputChanged)
                .padding([8, 12])
                .size(12),
            button(
                row![icon(Icon::Check).size(13).color(TEXT_PRIMARY), text("Compile & Apply Palette").size(12).color(TEXT_PRIMARY)].spacing(6).align_y(Alignment::Center)
            )
                .padding([8, 14])
                .width(Length::Fill)
                .on_press(Message::ApplyCustomPalette)
                .style(|theme, status| button_style(theme, status, ButtonVariant::Secondary)),
        ]
        .spacing(8)
    )
    .padding(12)
    .style(card_container_style);

    let blur_section = container(
        column![
            row![
                text("GAUSSIAN BLUR INTENSITY").size(11).color(TEXT_MUTED),
                space().width(Length::Fill),
                text(format!("Sigma: {:.1}", app.blur_sigma())).size(11).color(TEXT_PRIMARY),
            ],
            slider(0.0..=25.0, app.blur_sigma(), Message::BlurSigmaChanged).step(0.5),
            button(
                row![icon(Icon::Images).size(13).color(TEXT_PRIMARY), text("Apply Background Blur").size(12).color(TEXT_PRIMARY)].spacing(6).align_y(Alignment::Center)
            )
                .padding([8, 14])
                .width(Length::Fill)
                .on_press_maybe(if has_image && app.blur_sigma() > 0.0 { Some(Message::ApplyBlur) } else { None })
                .style(|theme, status| button_style(theme, status, ButtonVariant::Secondary)),
        ]
        .spacing(8)
    )
    .padding(12)
    .style(card_container_style);

    let max_w = app.image_width().max(1);
    let min_w = (max_w as f32 * 0.1) as u32; // 10% min width
    let current_target = if app.seam_carve_target() == 0 { max_w } else { app.seam_carve_target() };
    
    let seam_carve_section = container(
        column![
            row![
                text("CONTENT-AWARE SCALING").size(11).color(TEXT_MUTED),
                space().width(Length::Fill),
                text(format!("Width: {}px", current_target)).size(11).color(TEXT_PRIMARY),
            ],
            slider(min_w..=max_w, current_target, Message::SeamCarveTargetChanged).step(1u32),
            button(
                row![icon(Icon::Magic).size(13).color(TEXT_PRIMARY), text("Apply Seam Carving").size(12).color(TEXT_PRIMARY)].spacing(6).align_y(Alignment::Center)
            )
                .padding([8, 14])
                .width(Length::Fill)
                .on_press_maybe(if has_image && current_target < max_w { Some(Message::ApplySeamCarving) } else { None })
                .style(|theme, status| button_style(theme, status, ButtonVariant::Secondary)),
        ]
        .spacing(8)
    )
    .padding(12)
    .style(card_container_style);

    let dither_section = container(
        column![
            text("ALGORITHMIC DITHERING").size(11).color(TEXT_MUTED),
            toggler(app.dither_enabled())
                .label("Enable Floyd-Steinberg Diffusion")
                .on_toggle(|_| Message::ToggleDither)
                .size(16),
            button(
                row![icon(Icon::Magic).size(13).color(TEXT_PRIMARY), text("Apply Dithering").size(12).color(TEXT_PRIMARY)].spacing(6).align_y(Alignment::Center)
            )
                .padding([8, 14])
                .width(Length::Fill)
                .on_press_maybe(if has_image && app.dither_enabled() { Some(Message::ApplyDither) } else { None })
                .style(|theme, status| button_style(theme, status, ButtonVariant::Secondary)),
        ]
        .spacing(8)
    )
    .padding(12)
    .style(card_container_style);

    let cat_b_section = column![
        text("CATEGORY B: GRADING ENGINE").size(11).color(TEXT_MUTED),
        preset_picker,
        algo_picker,
        hald_level_picker,
        luma_toggle,
        extract_palette_btn,
        blur_section,
        seam_carve_section,
        dither_section,
        swatches_block,
        custom_palette_section,
    ]
    .spacing(10);

    // CATEGORY D: Desktop Environment & Wallpaper Engine (`wallrust`)
    let backend_picker = column![
        text("WALLPAPER APPLY ENGINE").size(11).color(TEXT_MUTED),
        pick_list(
            WallpaperBackend::ALL,
            Some(app.wallpaper_backend()),
            Message::WallpaperBackendChanged
        )
        .padding([8, 12])
        .width(Length::Fill)
    ]
    .spacing(4);

    let backend_desc_card = container(
        column![
            text("Engine Description:").size(11).color(TEXT_MUTED),
            text(app.wallpaper_backend().description()).size(12).color(TEXT_PRIMARY),
        ]
        .spacing(4)
    )
    .padding(10)
    .width(Length::Fill)
    .style(card_container_style);

    let swww_picker = column![
        text("WAYLAND TRANSITION (SWWW)").size(11).color(TEXT_MUTED),
        pick_list(
            SWWW_TRANSITIONS,
            Some(app.swww_transition()),
            |s| Message::SwwwTransitionChanged(s.to_string())
        )
        .padding([8, 12])
        .width(Length::Fill)
    ]
    .spacing(4);

    let display_picker = column![
        text("TARGET DISPLAY OUTPUT").size(11).color(TEXT_MUTED),
        pick_list(
            TARGET_DISPLAYS,
            Some(app.target_display()),
            |s| Message::TargetDisplayChanged(s.to_string())
        )
        .padding([8, 12])
        .width(Length::Fill)
    ]
    .spacing(4);

    let daemon_toggler = iced::widget::toggler(app.daemon_enabled())
        .label("Enable Time-of-Day Daemon")
        .on_toggle(Message::ToggleDaemon)
        .width(Length::Fill)
        .text_size(13);

    let daemon_settings = if app.daemon_enabled() {
        column![
            text("DAYTIME THEME (06:00 - 18:00)").size(11).color(TEXT_MUTED),
            pick_list(
                crate::app::state::PRESET_NAMES,
                Some(app.day_theme()),
                |s| Message::SetDayTheme(s.to_string())
            )
            .padding([8, 12])
            .width(Length::Fill),
            space().height(4),
            text("NIGHTTIME THEME (18:00 - 06:00)").size(11).color(TEXT_MUTED),
            pick_list(
                crate::app::state::PRESET_NAMES,
                Some(app.night_theme()),
                |s| Message::SetNightTheme(s.to_string())
            )
            .padding([8, 12])
            .width(Length::Fill),
        ].spacing(4)
    } else {
        column![]
    };

    let cat_d_section = column![
        text("CATEGORY D: DESKTOP ENGINE").size(11).color(TEXT_MUTED),
        backend_picker,
        backend_desc_card,
        swww_picker,
        display_picker,
        space().height(6),
        daemon_toggler,
        daemon_settings,
    ]
    .spacing(10);

    // CATEGORY E: Export & Save Integration
    let save_image_btn = if has_image {
        button(
            row![icon(Icon::Folder).size(14).color(TEXT_PRIMARY), text("Save Processed Image to Folder").size(13).color(TEXT_PRIMARY)].spacing(8).align_y(Alignment::Center)
        )
            .width(Length::Fill)
            .padding([10, 16])
            .on_press(Message::SaveImageToFolder)
            .style(|theme, status| button_style(theme, status, ButtonVariant::Secondary))
    } else {
        button(
            row![icon(Icon::Folder).size(14).color(TEXT_MUTED), text("Save Processed Image (Disabled)").size(13).color(TEXT_MUTED)].spacing(8).align_y(Alignment::Center)
        )
            .width(Length::Fill)
            .padding([10, 16])
            .style(|theme, status| button_style(theme, status, ButtonVariant::Disabled))
    };

    let export_scheme_btn = button(
        row![icon(Icon::Terminal).size(14).color(TEXT_PRIMARY), text("Export Terminal Scheme").size(13).color(TEXT_PRIMARY)].spacing(8).align_y(Alignment::Center)
    )
        .width(Length::Fill)
        .padding([10, 16])
        .on_press(Message::ExportTerminalScheme)
        .style(|theme, status| button_style(theme, status, ButtonVariant::Secondary));

    let sync_alacritty_cb = row![
        checkbox(app.sync_alacritty()).on_toggle(Message::ToggleSyncAlacritty).size(16),
        text("Sync Alacritty (~/.config/alacritty)").size(12).color(TEXT_PRIMARY)
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let sync_kitty_cb = row![
        checkbox(app.sync_kitty()).on_toggle(Message::ToggleSyncKitty).size(16),
        text("Sync Kitty (~/.config/kitty)").size(12).color(TEXT_PRIMARY)
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let cat_e_section = column![
        text("CATEGORY E: EXPORT & SAVE").size(11).color(TEXT_MUTED),
        save_image_btn,
        export_scheme_btn,
        space().height(4),
        sync_alacritty_cb,
        sync_kitty_cb,
    ]
    .spacing(6);

    // Select Active Tab Content
    let active_content = match current_tab {
        SidebarTab::ThemeLut => column![cat_a_section, space().height(15), cat_b_section].spacing(10),
        SidebarTab::DesktopEngine => column![cat_d_section].spacing(10),
        SidebarTab::ExportSync => column![cat_e_section].spacing(10),
    };

    // Action Buttons
    let is_loading = matches!(state, AppState::Loading(_, _));
    let apply_theme_btn = if has_image && !is_loading {
        button(
            row![icon(Icon::Check).size(14).color(TEXT_PRIMARY), text("Apply Theme").size(13).color(TEXT_PRIMARY)].spacing(8).align_y(Alignment::Center)
        )
            .width(Length::Fill)
            .padding([10, 16])
            .on_press(Message::ApplyTheme)
            .style(|theme, status| button_style(theme, status, ButtonVariant::Primary))
    } else {
        button(
            row![icon(Icon::Check).size(14).color(TEXT_MUTED), text("Apply Theme (Disabled)").size(13).color(TEXT_MUTED)].spacing(8).align_y(Alignment::Center)
        )
            .width(Length::Fill)
            .padding([10, 16])
            .style(|theme, status| button_style(theme, status, ButtonVariant::Disabled))
    };

    let is_ready = matches!(state, AppState::PreviewReady(_));
    let apply_wallpaper_btn = if is_ready {
        button(
            row![icon(Icon::Display).size(14).color(iced::Color::from_rgb(0.05, 0.05, 0.05)), text("Set as Desktop Wallpaper").size(13).color(iced::Color::from_rgb(0.05, 0.05, 0.05))].spacing(8).align_y(Alignment::Center)
        )
            .width(Length::Fill)
            .padding([10, 16])
            .on_press(Message::SetWallpaper)
            .style(|theme, status| button_style(theme, status, ButtonVariant::Accent))
    } else {
        button(
            row![icon(Icon::Display).size(14).color(TEXT_MUTED), text("Set as Wallpaper (Disabled)").size(13).color(TEXT_MUTED)].spacing(8).align_y(Alignment::Center)
        )
            .width(Length::Fill)
            .padding([10, 16])
            .style(|theme, status| button_style(theme, status, ButtonVariant::Disabled))
    };

    column![
        tab_bar,
        space().height(10),
        scrollable(active_content.width(Length::Fill)).height(Length::Fill),
        space().height(10),
        apply_theme_btn,
        space().height(6),
        apply_wallpaper_btn,
    ]
    .spacing(6)
    .padding(15)
    .width(Length::FillPortion(3))
    .height(Length::Fill)
    .into()
}
