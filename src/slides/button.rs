use iced::{
    widget::{column, row, scrollable, space, text},
    Element,
};
use iced_anim::widget::button;

use crate::{App, Message, TEXT_SIZE};

pub const MD_BUTTON: &str = r#"
```rust
button("Get").on_press(Message::Action)
```
"#;

impl App {
    pub fn view_button_screen(&self) -> Element<'_, Message> {
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
                text("The Button widget produces messages when clicked.").size(self.sz(TEXT_SIZE)),
                space().height(self.sp(8.0)),
                self.md_container(&self.md_button),
                space().height(self.sp(20.0)),
                row![
                    button("Get").on_press(Message::ButtonClicked),
                    text(click_text).size(self.sz(TEXT_SIZE)),
                ]
                .spacing(self.sp(15.0))
                .align_y(iced::Alignment::Center),
            ]
            .spacing(self.sp(8.0)),
        )
        .into()
    }
}
