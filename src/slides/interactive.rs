use iced::{
    widget::{button, checkbox, column, container, pick_list, row, scrollable, space, text, text_input},
    Element, Length,
};
use strum::IntoEnumIterator;

use crate::{App, FIRA_MONO, Message, Mode, SUBTITLE_COLOR, TEXT_SIZE};

impl App {
    pub(crate) fn view_interactive_screen(&self) -> Element<'_, Message> {
        let mode_options: Vec<Mode> = Mode::iter().collect();

        let get_button = if self.model.loading {
            button("Loading...")
        } else {
            button("Get").on_press(Message::Action)
        };

        let result_text = if self.model.loading {
            "Fetching...".to_string()
        } else if !self.model.result.is_empty() {
            self.model.result.clone()
        } else {
            "Enter a URL and click Get".to_string()
        };

        // RON state visualization
        let ron_config = ron::ser::PrettyConfig::default();
        let state_ron = ron::ser::to_string_pretty(&self.model, ron_config)
            .unwrap_or_else(|e| format!("Error: {e}"));

        // Message log
        let message_log_content: Element<'_, Message> = if self.message_log.is_empty() {
            text("Messages will appear here...")
                .size(14)
                .color(SUBTITLE_COLOR)
                .into()
        } else {
            column(
                self.message_log
                    .iter()
                    .map(|msg| {
                        row![
                            text(msg).size(14).font(FIRA_MONO),
                            space().width(Length::Fill)
                        ]
                        .into()
                    })
                    .collect::<Vec<_>>(),
            )
            .spacing(4)
            .into()
        };

        column![
            // Input row
            row![
                text_input("Enter URL (e.g. example.com)", &self.model.url)
                    .on_input(Message::UrlChanged)
                    .on_submit(Message::Action),
                checkbox(self.model.secure)
                    .label("HTTPS")
                    .on_toggle(Message::SecureChanged),
                pick_list(mode_options, Some(self.model.mode), Message::ModeChanged),
                get_button,
            ]
            .spacing(12)
            .align_y(iced::Alignment::Center),
            space().height(24),
            // Result
            text(result_text).size(TEXT_SIZE),
            space().height(36),
            // State and messages side by side (half width each)
            row![
                column![
                    text("Current State")
                        .size(16)
                        .font(FIRA_MONO)
                        .color(SUBTITLE_COLOR),
                    space().height(8),
                    container(text(state_ron).size(14).font(FIRA_MONO))
                        .width(iced::Fill)
                        .padding(12)
                        .style(container::rounded_box),
                ]
                .height(iced::Fill)
                .width(iced::Length::FillPortion(1)),
                column![
                    text("Recent Messages")
                        .size(16)
                        .font(FIRA_MONO)
                        .color(SUBTITLE_COLOR),
                    space().height(8),
                    container(scrollable(message_log_content).height(150))
                        .width(iced::Fill)
                        .padding(12)
                        .style(container::rounded_box),
                ]
                .height(iced::Fill)
                .width(iced::Length::FillPortion(1)),
            ]
            .spacing(20),
        ]
        .width(iced::Fill)
        .into()
    }
}
