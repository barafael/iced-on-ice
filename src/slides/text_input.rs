use iced::{
    widget::{column, scrollable, space, text, text_input},
    Element,
};

use crate::{App, Message, TEXT_SIZE};

pub(crate) const MD_TEXT_INPUT: &str = r#"
```rust
text_input("Enter URL (e.g. example.com)", &self.model.url)
    .on_input(Message::UrlChanged)
    .on_submit(Message::Action)
```
"#;

impl App {
    pub(crate) fn view_text_input_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("The Text Input widget produces messages as the user types.").size(TEXT_SIZE),
                space().height(8),
                self.md_container(&self.md_text_input),
                space().height(20),
                text_input("Enter URL (e.g. example.com)", &self.demo_input)
                    .on_input(Message::DemoInputChanged)
                    .on_submit(Message::DemoInputSubmitted),
                space().height(12),
                text!("Input Changed messages: {}", self.input_changes).size(TEXT_SIZE),
                text!("Input Submitted messages: {}", self.input_submits).size(TEXT_SIZE),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }
}
