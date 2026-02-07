use iced::{
    widget::{column, scrollable, space, text},
    Element,
};

use crate::{App, Message, TEXT_SIZE};

pub(crate) const MD_UPDATE: &str = r#"
```rust
fn update(&mut self, message: Message) {
    match message {
        Message::UrlChanged(url) => self.url = url,
        Message::SecureChanged(secure) => self.secure = secure,
        Message::ModeChanged(mode) => self.mode = mode,
        Message::Action => todo!("Start fetching URL"),
        Message::Result(result) => self.result = result,
    }
}
```
"#;

impl App {
    pub(crate) fn view_update_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("Update modifies state based on messages.").size(TEXT_SIZE),
                space().height(8),
                self.md_container(&self.md_update),
                space().height(12),
                text("Notice the method signature! (&mut)").size(TEXT_SIZE),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }
}
