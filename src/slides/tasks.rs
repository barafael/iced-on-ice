use iced::{
    widget::{column, scrollable, space, text},
    Element,
};

use crate::{App, Message, TEXT_SIZE};

pub const MD_TASKS: &str = r#"
```rust
fn update(&mut self, message: Message) -> Task<Message> {
    ...
    Message::Action => {
        return Task::perform(
            fetch_url(self.url.clone(), self.secure, self.mode),
            Message::Result,
        );
    }
    ...
}
```
"#;

impl App {
    pub fn view_tasks_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("The update function may produce a Task for async background operations.")
                    .size(self.sz(TEXT_SIZE)),
                space().height(self.sp(8.0)),
                self.md_container(&self.md_tasks),
                space().height(self.sp(12.0)),
                text("Task::perform takes an async function and a message constructor.")
                    .size(self.sz(TEXT_SIZE)),
                space().height(self.sp(8.0)),
                text(
                    "When the async work completes, the result is usually wrapped in the message."
                )
                .size(self.sz(TEXT_SIZE)),
            ]
            .spacing(self.sp(8.0)),
        )
        .into()
    }
}
