use iced::{
    widget::{button, column, row, scrollable, space, text},
    Element,
};

use crate::{App, Message, TEXT_SIZE};

pub(crate) const MD_BUTTON: &str = r#"
```rust
button("Get").on_press(Message::Action)
```
"#;

impl App {
    pub(crate) fn view_button_screen(&self) -> Element<'_, Message> {
        let click_text = if self.button_clicks == 0 {
            String::from("Click the button!")
        } else {
            format!(
                "Clicked {} time{}",
                self.button_clicks,
                if self.button_clicks == 1 { "" } else { "s" }
            )
        };

        scrollable(
            column![
                text("The Button widget produces messages when clicked.").size(TEXT_SIZE),
                space().height(8),
                self.md_container(&self.md_button),
                space().height(20),
                row![
                    button("Get").on_press(Message::ButtonClicked),
                    text(click_text).size(TEXT_SIZE),
                ]
                .spacing(15)
                .align_y(iced::Alignment::Center),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }
}
