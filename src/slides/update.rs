use iced::{
    Element,
    widget::{column, scrollable, space, text},
};

use crate::{App, Message, TEXT_SIZE};

pub const MD_UPDATE: &str = r#"
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
    pub fn view_update_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("Update modifies state based on messages.").size(self.sz(TEXT_SIZE)),
                space().height(self.sp(8.0)),
                self.md_container(&self.md_update),
                space().height(self.sp(12.0)),
                text("Notice the method signature! (&mut)").size(self.sz(TEXT_SIZE)),
            ]
            .spacing(self.sp(8.0)),
        )
        .into()
    }
}
