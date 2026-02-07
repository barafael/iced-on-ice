use iced::{
    widget::{column, container, space, svg, text},
    Element,
};

use crate::{App, ELM_CIRCLE_OF_LIFE, Message, ORANGE, SUBTITLE_COLOR, TEXT_SIZE};

impl App {
    pub(crate) fn view_recap_screen(&self) -> Element<'_, Message> {
        container(
            column![
                text("The Elm Architecture").size(40).color(ORANGE),
                space().height(30),
                svg(svg::Handle::from_memory(ELM_CIRCLE_OF_LIFE)).height(280),
                space().height(30),
                text("Model → View → Message → Update → (Task) → Model...")
                    .size(TEXT_SIZE)
                    .color(SUBTITLE_COLOR),
                space().height(20),
            ]
            .align_x(iced::Alignment::Center),
        )
        .width(iced::Fill)
        .height(iced::Fill)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
    }
}
