//! Right preview pane (70% width) rendering Idle, Loading, Notice, Error, or Category C Views.

use crate::app::{AppState, Message, WallmodApp, WorkspaceView};
use crate::ui::icon::{icon, Icon};
use crate::ui::theme::{button_style, canvas_container_style, card_container_style, txt_muted, txt_primary, ButtonVariant, BORDER_COLOR, ERROR_COLOR};
use iced::widget::{button, column, container, image as iced_image, progress_bar, row, scrollable, space, stack, text};
use iced::{Alignment, Background, Color, ContentFit, Element, Length};

/// Renders the right preview panel with full Category C telemetry, diffing, and WCAG contrast auditing.
#[allow(non_snake_case)]
pub fn view(app: &WallmodApp) -> Element<'_, Message> {
    let TEXT_PRIMARY = txt_primary(app.is_dark_mode());
    let TEXT_MUTED = txt_muted(app.is_dark_mode());
    let state = app.state();
    let content: Element<'_, Message> = match state {
        AppState::Idle => container(
            column![
                icon(Icon::PlusSquare).size(32).color(BORDER_COLOR),
                space().height(12),
                text("No wallpaper loaded").size(16).color(TEXT_PRIMARY),
                text("Select a base image from Category A controls on the left to begin ricing.")
                    .size(13)
                    .color(TEXT_MUTED),
            ]
            .align_x(Alignment::Center)
            .spacing(4),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .into(),

        AppState::Loading(prog, msg) => {
            let percent = (*prog * 100.0).round() as u32;
            let loading_card = container(
                column![
                    row![
                    row![icon(Icon::Layers).size(14).color(TEXT_MUTED), text("PROCESSING ENGINE").size(11).color(TEXT_MUTED)].spacing(6),
                        space().width(Length::Fill),
                        text(format!("{}%", percent)).size(13).color(Color::from_rgb(0.54, 0.71, 0.98)),
                    ]
                    .width(Length::Fixed(340.0)),
                    space().height(10),
                    text(msg).size(14).color(TEXT_PRIMARY),
                    space().height(14),
                    progress_bar(0.0..=1.0, *prog)
                        .length(340.0)
                        .girth(8.0),
                ]
                .align_x(Alignment::Center),
            )
            .padding(24)
            .style(|theme| card_container_style(theme));

            if let Some(handle) = app.preview_handle().or(app.base_image_handle()) {
                let bg_img = container(
                    iced_image(handle.clone()).content_fit(ContentFit::Contain)
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center);

                let overlay = container(
                    column![
                        space().height(Length::Fill),
                        loading_card,
                        space().height(32),
                    ]
                    .align_x(Alignment::Center)
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::End);

                stack![bg_img, overlay].into()
            } else {
                container(loading_card)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .into()
            }
        }

        AppState::Notice(msg) => container(
            column![
                row![icon(Icon::Check).size(16).color(Color::from_rgb(0.3, 0.85, 0.5)), text("Notification / Success").size(14).color(Color::from_rgb(0.3, 0.85, 0.5))].spacing(8),
                space().height(12),
                text(msg).size(14).color(TEXT_PRIMARY),
                space().height(16),
                button(row![icon(Icon::ArrowLeftCircle).size(14).color(TEXT_PRIMARY), text("Return to Workspace").size(12).color(TEXT_PRIMARY)].spacing(6))
                    .padding([8, 14])
                    .on_press(Message::DismissNotice)
                    .style(|theme, status| button_style(theme, status, ButtonVariant::Secondary)),
            ]
            .padding(24)
            .align_x(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .into(),

        AppState::PreviewReady(handle) => {
            let wcag = app.wcag_contrast();
            let wcag_status = if wcag >= 7.0 {
                "Pass AAA (Excellent Legibility)"
            } else if wcag >= 4.5 {
                "Pass AA (Standard Legibility)"
            } else {
                "Warning (Low Text Contrast)"
            };

            let wcag_color = if wcag >= 4.5 {
                Color::from_rgb(0.3, 0.85, 0.5)
            } else {
                Color::from_rgb(0.95, 0.7, 0.3)
            };

            let info_pill = container(
                row![
                    text("[ # ]").size(12).color(TEXT_MUTED),
                    text(app.image_filename()).size(12).color(TEXT_PRIMARY),
                    text("•").size(12).color(TEXT_MUTED),
                    text(format!("{} × {} px", app.image_width(), app.image_height())).size(12).color(TEXT_MUTED),
                    text("•").size(12).color(TEXT_MUTED),
                    row![icon(Icon::InfoCircle).size(14).color(wcag_color), text(format!("WCAG Contrast: {:.1}:1 ({})", wcag, wcag_status)).size(12).color(wcag_color)].spacing(6),
                ]
                .spacing(8)
                .align_y(Alignment::Center)
            )
            .padding([8, 14])
            .style(card_container_style);

            match app.workspace_view() {
                WorkspaceView::Standard => {
                    let img_widget = iced_image(handle.clone())
                        .content_fit(ContentFit::Contain)
                        .width(Length::Fill)
                        .height(Length::Fill);

                    container(
                        column![
                            info_pill,
                            space().height(12),
                            container(img_widget).width(Length::Fill).height(Length::Fill),
                        ]
                        .align_x(Alignment::Center)
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(20)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .into()
                }

                WorkspaceView::SplitDiff => {
                    let base_img = if let Some(base_handle) = app.base_image_handle() {
                        iced_image(base_handle.clone()).content_fit(ContentFit::Contain).width(Length::Fill).height(Length::Fill)
                    } else {
                        iced_image(handle.clone()).content_fit(ContentFit::Contain).width(Length::Fill).height(Length::Fill)
                    };

                    let riced_img = iced_image(handle.clone()).content_fit(ContentFit::Contain).width(Length::Fill).height(Length::Fill);

                    let diff_row = row![
                        container(column![text("ORIGINAL BASE").size(11).color(TEXT_MUTED), space().height(4), base_img].align_x(Alignment::Center)).width(Length::FillPortion(1)),
                        container(space()).width(2).style(|_| container::Style { background: Some(iced::Background::Color(BORDER_COLOR)), ..Default::default() }),
                        container(column![text("RICED THEME OUTPUT").size(11).color(TEXT_PRIMARY), space().height(4), riced_img].align_x(Alignment::Center)).width(Length::FillPortion(1)),
                    ]
                    .spacing(12)
                    .width(Length::Fill)
                    .height(Length::Fill);

                    container(
                        column![
                            info_pill,
                            space().height(12),
                            diff_row,
                        ]
                        .align_x(Alignment::Center)
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(20)
                    .into()
                }

                WorkspaceView::Telemetry => {
                    let telemetry_card = container(
                        column![
                            row![icon(Icon::InfoCircle).size(16).color(TEXT_PRIMARY), text("DEEP IMAGE TELEMETRY & ANALYTICS (`imagineer`)").size(14).color(TEXT_PRIMARY)].spacing(8),
                            space().height(12),
                            row![text("File Name:").size(13).color(TEXT_MUTED).width(160), text(app.image_filename()).size(13).color(TEXT_PRIMARY)],
                            row![text("Resolution Dimensions:").size(13).color(TEXT_MUTED).width(160), text(format!("{} × {} pixels", app.image_width(), app.image_height())).size(13).color(TEXT_PRIMARY)],
                            row![text("Aspect Ratio:").size(13).color(TEXT_MUTED).width(160), text(format!("{:.2}", app.image_width() as f32 / app.image_height().max(1) as f32)).size(13).color(TEXT_PRIMARY)],
                            row![text("Color Grading Profile:").size(13).color(TEXT_MUTED).width(160), text(app.current_theme().display_name()).size(13).color(TEXT_PRIMARY)],
                            row![text("Remapping Algorithm:").size(13).color(TEXT_MUTED).width(160), text(format!("{}", app.algorithm())).size(13).color(TEXT_PRIMARY)],
                            row![text("Luminance Lock:").size(13).color(TEXT_MUTED).width(160), text(if app.preserve_luma() { "Active (Preserve Y Channel)" } else { "Disabled (Direct Color Shift)" }).size(13).color(TEXT_PRIMARY)],
                            row![text("HaldCLUT Density:").size(13).color(TEXT_MUTED).width(160), text(format!("Level {} ({}³ Matrix)", app.hald_level(), app.hald_level() as u32 * app.hald_level() as u32)).size(13).color(TEXT_PRIMARY)],
                            space().height(8),
                            row![text("WCAG Contrast Rating:").size(13).color(TEXT_MUTED).width(160), text(format!("{:.2}:1 — {}", wcag, wcag_status)).size(13).color(wcag_color)],
                        ]
                        .spacing(8)
                    )
                    .padding(24)
                    .style(card_container_style)
                    .width(Length::Fill);

                    let mut telemetry_col = column![
                        info_pill,
                        space().height(20),
                        telemetry_card,
                    ].align_x(Alignment::Center).max_width(640);

                    if let Some(hist_data) = app.histogram_data() {
                        let hist_card = container(
                            column![
                                row![icon(Icon::Display).size(16).color(TEXT_PRIMARY), text("CHANNEL HISTOGRAMS (RGB + Luma)").size(14).color(TEXT_PRIMARY)].spacing(8),
                                space().height(12),
                                crate::ui::histogram::HistogramChart::new(hist_data).view()
                            ]
                        )
                        .padding(24)
                        .style(card_container_style)
                        .width(Length::Fill);

                        telemetry_col = telemetry_col.push(space().height(20)).push(hist_card);
                    }

                    container(telemetry_col)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(20)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .into()
                }
                WorkspaceView::Gallery => container(space()).into(),
            }
        }

        AppState::Error(err) => {
            let diag_card = container(
                column![
                    row![
                        row![icon(Icon::ExclamationTriangle).size(16).color(ERROR_COLOR), text("DIAGNOSTIC NOTICE / ENGINE ERROR").size(14).color(ERROR_COLOR)].spacing(8),
                    ].align_y(Alignment::Center),
                    space().height(12),
                    text("An issue occurred while executing the pipeline command:").size(12).color(TEXT_MUTED),
                    space().height(6),
                    container(
                        text(err).size(13).color(TEXT_PRIMARY)
                    )
                    .padding(12)
                    .width(Length::Fill)
                    .style(|theme| card_container_style(theme)),
                    space().height(16),
                    text("Troubleshooting Steps:").size(12).color(TEXT_MUTED),
                    space().height(4),
                    text("• Verify that the selected file is a valid image (PNG/JPEG/WEBP).").size(12).color(TEXT_PRIMARY),
                    text("• If setting wallpaper, ensure a daemon (swww, swaybg, feh) is running.").size(12).color(TEXT_PRIMARY),
                    text("• Check file read/write permissions on the target directory.").size(12).color(TEXT_PRIMARY),
                    space().height(20),
                    button(row![icon(Icon::ArrowLeftCircle).size(14).color(TEXT_PRIMARY), text("Dismiss & Return to Workspace").size(12).color(TEXT_PRIMARY)].spacing(6))
                        .padding([10, 16])
                        .on_press(Message::DismissNotice)
                        .style(|theme, status| button_style(theme, status, ButtonVariant::Secondary)),
                ]
                .padding(24)
                .max_width(540)
            )
            .style(|theme| card_container_style(theme));

            container(diag_card)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .into()
        }
    };

    let view_mode = app.workspace_view();
    let tab_bar = container(
        row![
            button(row![icon(Icon::Image).size(14).color(if view_mode == WorkspaceView::Standard { TEXT_PRIMARY } else { TEXT_MUTED }), text("Output Visual").size(12).color(if view_mode == WorkspaceView::Standard { TEXT_PRIMARY } else { TEXT_MUTED })].spacing(6).align_y(Alignment::Center))
                .padding([6, 14])
                .on_press(Message::WorkspaceViewChanged(WorkspaceView::Standard))
                .style(move |theme, status| button_style(theme, status, if view_mode == WorkspaceView::Standard { ButtonVariant::Primary } else { ButtonVariant::Ghost })),
            
            button(row![icon(Icon::Grid).size(14).color(if view_mode == WorkspaceView::SplitDiff { TEXT_PRIMARY } else { TEXT_MUTED }), text("Split Diff").size(12).color(if view_mode == WorkspaceView::SplitDiff { TEXT_PRIMARY } else { TEXT_MUTED })].spacing(6).align_y(Alignment::Center))
                .padding([6, 14])
                .on_press(Message::WorkspaceViewChanged(WorkspaceView::SplitDiff))
                .style(move |theme, status| button_style(theme, status, if view_mode == WorkspaceView::SplitDiff { ButtonVariant::Primary } else { ButtonVariant::Ghost })),

            button(row![icon(Icon::InfoCircle).size(14).color(if view_mode == WorkspaceView::Telemetry { TEXT_PRIMARY } else { TEXT_MUTED }), text("Dashboard Info").size(12).color(if view_mode == WorkspaceView::Telemetry { TEXT_PRIMARY } else { TEXT_MUTED })].spacing(6).align_y(Alignment::Center))
                .padding([6, 14])
                .on_press(Message::WorkspaceViewChanged(WorkspaceView::Telemetry))
                .style(move |theme, status| button_style(theme, status, if view_mode == WorkspaceView::Telemetry { ButtonVariant::Primary } else { ButtonVariant::Ghost })),

            button(row![icon(Icon::Images).size(14).color(if view_mode == WorkspaceView::Gallery { TEXT_PRIMARY } else { TEXT_MUTED }), text("Album Gallery").size(12).color(if view_mode == WorkspaceView::Gallery { TEXT_PRIMARY } else { TEXT_MUTED })].spacing(6).align_y(Alignment::Center))
                .padding([6, 14])
                .on_press(Message::WorkspaceViewChanged(WorkspaceView::Gallery))
                .style(move |theme, status| button_style(theme, status, if view_mode == WorkspaceView::Gallery { ButtonVariant::Primary } else { ButtonVariant::Ghost })),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
    )
    .padding([10, 16])
    .width(Length::Fill)
    .align_x(Alignment::Center);

    let main_body: Element<'_, Message> = if view_mode == WorkspaceView::Gallery {
        if app.scanning_gallery() {
            container(
                column![
                    row![icon(Icon::ArrowRepeat).size(18).color(TEXT_PRIMARY), text("Multi-Thread Scanning System Images...").size(16).color(TEXT_PRIMARY)].spacing(8),
                    space().height(8),
                    text("Searching Pictures, Downloads, Wallpapers & /usr/share/backgrounds").size(13).color(TEXT_MUTED),
                ]
                .align_x(Alignment::Center)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into()
        } else if let Some(selected_path) = app.selected_album() {
            let folder_name = selected_path.file_name().unwrap_or_default().to_string_lossy().to_string();
            let header = row![
                button(row![icon(Icon::ArrowLeftCircle).size(14).color(TEXT_PRIMARY), text("Back to Albums").size(12).color(TEXT_PRIMARY)].spacing(6))
                    .padding([6, 12])
                    .on_press(Message::SelectAlbum(None))
                    .style(|theme, status| button_style(theme, status, ButtonVariant::Ghost)),
                space().width(12),
                text(format!("Album: {} ({} images)", folder_name, app.album_images().len())).size(14).color(TEXT_PRIMARY),
            ]
            .align_y(Alignment::Center);

            if let AppState::Loading(_, ref msg) = app.state() {
                container(
                    column![
                        header,
                        space().height(32),
                        container(
                            column![
                                icon(Icon::ArrowRepeat).size(32).color(TEXT_MUTED),
                                space().height(12),
                                text(msg).size(14).color(TEXT_PRIMARY),
                                space().height(8),
                                text("Reading directory and decoding thumbnails...").size(12).color(TEXT_MUTED)
                            ]
                            .align_x(Alignment::Center)
                        )
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                    ]
                    .padding(20)
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
            } else {
                let mut grid_col = column![].spacing(12);
                let mut current_row = row![].spacing(12);
                let mut count = 0;
                for (img_path, handle) in app.album_images() {
                    let name = img_path.file_name().unwrap_or_default().to_string_lossy().to_string();
                    let img_thumb = iced_image(handle.clone())
                        .content_fit(ContentFit::Cover)
                        .width(Length::Fill)
                        .height(Length::Fixed(130.0));

                    let name_badge = container(text(name).size(11).color(Color::WHITE))
                        .padding([4, 8])
                        .width(Length::Fill)
                        .style(|_theme| container::Style {
                            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.65))),
                            ..Default::default()
                        });
                    
                    let img_box = stack![
                        container(img_thumb)
                            .width(Length::Fill)
                            .height(Length::Fixed(140.0))
                            .style(|theme| card_container_style(theme)),
                        container(name_badge)
                            .width(Length::Fill)
                            .height(Length::Fixed(140.0))
                            .align_y(Alignment::End)
                    ];

                    let img_btn = button(img_box)
                    .padding(0)
                    .width(Length::Fixed(190.0))
                    .on_press(Message::SelectGalleryImage(img_path.clone()))
                    .style(|theme, status| button_style(theme, status, ButtonVariant::Ghost));
                    
                    current_row = current_row.push(img_btn);
                    count += 1;
                    if count >= 3 {
                        grid_col = grid_col.push(current_row);
                        current_row = row![].spacing(12);
                        count = 0;
                    }
                }
                if count > 0 {
                    grid_col = grid_col.push(current_row);
                }

                container(
                    column![
                        header,
                        space().height(16),
                        scrollable(grid_col).height(Length::Fill),
                    ]
                    .padding(20)
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
            }
        } else {
            let mut grid_col = column![].spacing(14);
            let mut current_row = row![].spacing(14);
            let mut count = 0;
            for album in app.albums() {
                let cover_elem: Element<'_, Message> = if let Some(ref cover) = album.cover_image {
                    iced_image(iced_image::Handle::from_path(cover.clone()))
                        .content_fit(ContentFit::Cover)
                        .width(Length::Fill)
                        .height(Length::Fixed(140.0))
                        .into()
                } else {
                    container(text("[ # ]").size(24).color(TEXT_MUTED))
                        .width(Length::Fill)
                        .height(Length::Fixed(140.0))
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .into()
                };

                let album_btn = button(
                    column![
                        container(cover_elem)
                            .width(Length::Fill)
                            .height(Length::Fixed(140.0))
                            .style(|theme| card_container_style(theme)),
                        space().height(8),
                        text(&album.folder_name).size(13).color(TEXT_PRIMARY),
                        text(format!("{} images", album.image_count)).size(11).color(TEXT_MUTED),
                    ]
                    .align_x(Alignment::Center)
                )
                .padding(10)
                .width(Length::Fixed(210.0))
                .on_press(Message::SelectAlbum(Some(album.folder_path.clone())))
                .style(|theme, status| button_style(theme, status, ButtonVariant::Secondary));

                current_row = current_row.push(album_btn);
                count += 1;
                if count >= 3 {
                    grid_col = grid_col.push(current_row);
                    current_row = row![].spacing(14);
                    count = 0;
                }
            }
            if count > 0 {
                grid_col = grid_col.push(current_row);
            }

            let header = row![
                text(format!("System Image Albums (Found {} folders)", app.albums().len())).size(15).color(TEXT_PRIMARY),
                space().width(Length::Fill),
                button(row![icon(Icon::ArrowRepeat).size(14).color(TEXT_PRIMARY), text("Rescan Albums").size(12).color(TEXT_PRIMARY)].spacing(6))
                    .padding([6, 12])
                    .on_press(Message::ScanSystemGallery)
                    .style(|theme, status| button_style(theme, status, ButtonVariant::Ghost)),
            ]
            .align_y(Alignment::Center);

            container(
                column![
                    header,
                    space().height(16),
                    scrollable(grid_col).height(Length::Fill),
                ]
                .padding(20)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        }
    } else {
        content
    };

    container(
        column![
            tab_bar,
            container(main_body).width(Length::Fill).height(Length::Fill),
        ]
    )
    .width(Length::FillPortion(7))
    .height(Length::Fill)
    .style(canvas_container_style)
    .into()
}
