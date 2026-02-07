use iced::{
    widget::{column, container, row, space, svg, text},
    Element,
};
use lucide_icons::iced::icon_keyboard;

use crate::{App, Message, ICED_LOGO, ORANGE, SUBTITLE_COLOR};

impl App {
    pub(crate) fn view_title_screen(&self) -> Element<'_, Message> {
        container(
            column![
                svg(svg::Handle::from_memory(ICED_LOGO))
                    .width(96)
                    .height(96),
                space().height(30),
                text("Discover Iced").size(56).color(ORANGE),
                space().height(16),
                text("A quick tour of Iced (done with Iced)")
                    .size(22)
                    .color(SUBTITLE_COLOR),
                space().height(50),
                row![
                    icon_keyboard().size(16).color(SUBTITLE_COLOR),
                    text("Use arrow keys to navigate")
                        .size(16)
                        .color(SUBTITLE_COLOR),
                ]
                .spacing(8)
                .align_y(iced::Alignment::Center),
            ]
            .width(iced::Fill)
            .align_x(iced::Alignment::Center),
        )
        .width(iced::Fill)
        .height(iced::Fill)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
    }
}
