use iced::{
    widget::{canvas, column, scrollable, space, stack, text},
    Element,
};

use crate::{chaos, App, Message, SUBTITLE_COLOR, TEXT_SIZE};

pub(crate) const MD_SUBSCRIPTIONS: &str = r#"
```rust
fn subscription(&self) -> Subscription<Message> {
    event::listen_with(|event, _, _| match event {
        Event::Keyboard(KeyPressed {
            key: Key::Named(Named::ArrowRight), ..
        }) => Some(Message::NextScreen),
        ...
    })
}
```
"#;

impl App {
    pub(crate) fn view_subscriptions_screen(&self) -> Element<'_, Message> {
        let content = scrollable(
            column![
                text("Subscriptions let your app react to external events.").size(TEXT_SIZE),
                space().height(12),
                self.md_container(&self.md_subscriptions),
                space().height(16),
                space().height(8),
                text("  • Arrow Right → next slide")
                    .size(TEXT_SIZE - 4)
                    .color(SUBTITLE_COLOR),
                text("  • Arrow Left → previous slide")
                    .size(TEXT_SIZE - 4)
                    .color(SUBTITLE_COLOR),
                text("  • Ctrl → show theme picker")
                    .size(TEXT_SIZE - 4)
                    .color(SUBTITLE_COLOR),
                space().height(16),
                text("Other common uses: timers, window events, WebSocket messages.")
                    .size(TEXT_SIZE),
            ]
            .spacing(8)
            .padding(30),
        );

        let chaos_overlay = canvas(chaos::ChaosOverlay {
            circles: &self.chaos_circles,
        })
        .width(iced::Fill)
        .height(iced::Fill);

        stack![content, chaos_overlay].into()
    }
}
