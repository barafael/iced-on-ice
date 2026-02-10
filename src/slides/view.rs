use iced::{
    Color, Element, Theme,
    widget::{column, container, pick_list, row, scrollable, space, text},
    widget::button as iced_button,
};
use iced_anim::widget::button;

use crate::{App, Message, SUBTITLE_COLOR, TEXT_SIZE};

pub const MD_VIEW: &str = r#"
```rust
fn view(&self) -> Element<Message> {
    column![
        text_input("URL", &self.url)
            .on_input(Message::UrlChanged),
        button("Get").on_press(Message::Action),
    ].into()
}
```
"#;

impl App {
    pub fn view_theming_screen(&self) -> Element<'_, Message> {
        let hover_color = self.hover_color;

        let swatch = button(
            container(space().width(60).height(40)).style(move |_| container::Style {
                background: Some(hover_color.into()),
                ..Default::default()
            }),
        )
        .on_press(Message::OpenColorPicker);

        let picker = iced_aw::helpers::color_picker(
            self.show_color_picker,
            self.hover_color,
            swatch,
            Message::CancelColorPicker,
            Message::SubmitHoverColor,
        );

        let demo_button = button(
            text("Hover me").size(self.sz(TEXT_SIZE)),
        )
        .on_press(Message::Noop)
        .padding(self.sp(16.0))
        .style(move |theme: &Theme, status| match status {
            iced_button::Status::Hovered => iced_button::Style {
                background: Some(hover_color.into()),
                text_color: Color::WHITE,
                ..iced_button::primary(theme, status)
            },
            _ => iced_button::primary(theme, status),
        });

        scrollable(
            column![
                text("Every widget has a .style() method. Pick a hover color for the button:")
                    .size(self.sz(TEXT_SIZE)),
                space().height(self.sp(20.0)),
                row![
                    column![
                        text("Click to pick a color:").size(self.sz(TEXT_SIZE - 4)).color(SUBTITLE_COLOR),
                        space().height(self.sp(8.0)),
                        picker,
                    ],
                    space().width(self.sp(40.0)),
                    column![
                        text("Styled button:").size(self.sz(TEXT_SIZE - 4)).color(SUBTITLE_COLOR),
                        space().height(self.sp(8.0)),
                        demo_button,
                    ],
                ]
                .align_y(iced::Alignment::Start),
            ]
            .spacing(self.sp(8.0)),
        )
        .into()
    }

    pub fn view_theme_picker_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("Iced ships with built-in themes you can switch at runtime.")
                    .size(self.sz(TEXT_SIZE)),
                space().height(self.sp(20.0)),
                row![
                    text("Theme:").size(self.sz(TEXT_SIZE)),
                    pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged),
                ]
                .spacing(self.sp(12.0))
                .align_y(iced::Alignment::Center),
                space().height(self.sp(24.0)),
                text("Sample widgets with the current theme:").size(self.sz(TEXT_SIZE - 4)).color(SUBTITLE_COLOR),
                space().height(self.sp(12.0)),
                row![
                    button(text("Default").size(self.sz(TEXT_SIZE - 2)))
                        .on_press(Message::Noop)
                        .padding(self.sp(12.0)),
                    button(text("Primary").size(self.sz(TEXT_SIZE - 2)))
                        .on_press(Message::Noop)
                        .padding(self.sp(12.0))
                        .style(iced_button::primary),
                    button(text("Secondary").size(self.sz(TEXT_SIZE - 2)))
                        .on_press(Message::Noop)
                        .padding(self.sp(12.0))
                        .style(iced_button::secondary),
                    button(text("Danger").size(self.sz(TEXT_SIZE - 2)))
                        .on_press(Message::Noop)
                        .padding(self.sp(12.0))
                        .style(iced_button::danger),
                ]
                .spacing(self.sp(12.0)),
            ]
            .spacing(self.sp(8.0)),
        )
        .into()
    }

    pub fn view_view_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("The View visualizes the application state.").size(self.sz(TEXT_SIZE)),
                space().height(self.sp(12.0)),
                self.md_container(&self.md_view),
                space().height(self.sp(12.0)),
                text("Notice the method signature: &self (immutable borrow).").size(self.sz(TEXT_SIZE)),
                space().height(self.sp(8.0)),
                text("The View can read state but never modify it.").size(self.sz(TEXT_SIZE)),
            ]
            .spacing(self.sp(8.0)),
        )
        .into()
    }
}
