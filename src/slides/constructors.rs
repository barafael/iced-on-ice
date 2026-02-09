use iced::{
    Element,
    widget::{column, scrollable, space, text},
};

use crate::{App, Message, SUBTITLE_COLOR, TEXT_SIZE};

pub const MD_CONSTRUCTORS: &str = r#"
```rust
// these two are equivalent:
.on_input(Message::UrlChanged)
.on_input(|s| Message::UrlChanged(s))
```
"#;

pub const MD_WIDGET_MESSAGES: &str = r#"
```rust
// Each widget sends its state into your Message:
text_input(..).on_input(f)    // f: impl Fn(String) -> Message
toggler(..).on_toggle(f)      // f: impl Fn(bool) -> Message
button(..).on_press(message)  // message: Message (no fn, just a value)
```
"#;

impl App {
    pub fn view_constructors_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("Enum variants with data are enum constructors.").size(self.sz(TEXT_SIZE)),
                space().height(self.sp(8.0)),
                self.md_container(&self.md_constructors),
                space().height(self.sp(16.0)),
                text("Widgets pass their state into these constructors:")
                    .size(self.sz(TEXT_SIZE))
                    .color(SUBTITLE_COLOR),
                space().height(self.sp(8.0)),
                self.md_container(&self.md_widget_messages),
            ]
            .spacing(self.sp(8.0)),
        )
        .into()
    }
}
