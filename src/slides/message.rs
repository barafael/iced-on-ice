use iced::{
    widget::{column, scrollable, space, text},
    Element,
};

use crate::{App, Message, TEXT_SIZE};

pub(crate) const MD_MESSAGE: &str = r#"
```rust
enum Message {
    UrlChanged(String),
    SecureChanged(bool),
    ModeChanged(Mode),
    Action,
    Result(String),
}
```
"#;

impl App {
    pub(crate) fn view_message_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("Messages describe user actions or system events.").size(TEXT_SIZE),
                space().height(8),
                self.md_container(&self.md_message),
                space().height(12),
                text("Messages are produced by the view.").size(TEXT_SIZE)
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }
}
