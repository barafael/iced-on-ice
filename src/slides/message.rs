use iced::{
    widget::{column, scrollable, space, text},
    Element,
};

use crate::{App, Message, TEXT_SIZE};

pub const MD_MESSAGE: &str = r#"
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
    pub fn view_message_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("Messages describe user actions or system events.").size(self.sz(TEXT_SIZE)),
                space().height(self.sp(8.0)),
                self.md_container(&self.md_message),
                space().height(self.sp(12.0)),
                text("Messages are produced by the view.").size(self.sz(TEXT_SIZE))
            ]
            .spacing(self.sp(8.0)),
        )
        .into()
    }
}
