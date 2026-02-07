use iced::{
    widget::{column, scrollable, space, text},
    Element,
};

use crate::{App, Message, TEXT_SIZE};

pub(crate) const MD_VIEW: &str = r#"
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
    pub(crate) fn view_view_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("The View visualizes the application state.").size(TEXT_SIZE),
                space().height(12),
                self.md_container(&self.md_view),
                space().height(12),
                text("Notice the method signature: &self (immutable borrow).").size(TEXT_SIZE),
                space().height(8),
                text("The View can read state but never modify it.").size(TEXT_SIZE),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }
}
