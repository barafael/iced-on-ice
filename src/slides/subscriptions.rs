use iced::{
    widget::{column, scrollable, space, text},
    Element,
};
use iced_anim::widget::button;

use crate::{App, Message, SUBTITLE_COLOR, TEXT_SIZE};

pub const MD_SUBSCRIPTIONS: &str = r#"
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
    pub fn view_subscriptions_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("Subscriptions let your app react to external events.").size(self.sz(TEXT_SIZE)),
                space().height(self.sp(12.0)),
                self.md_container(&self.md_subscriptions),
                space().height(self.sp(16.0)),
                space().height(self.sp(8.0)),
                text("  • Arrow Right → next slide")
                    .size(self.sz(TEXT_SIZE - 4))
                    .color(SUBTITLE_COLOR),
                text("  • Arrow Left → previous slide")
                    .size(self.sz(TEXT_SIZE - 4))
                    .color(SUBTITLE_COLOR),
                text("  • Ctrl → show theme picker")
                    .size(self.sz(TEXT_SIZE - 4))
                    .color(SUBTITLE_COLOR),
                space().height(self.sp(16.0)),
                text("Other common uses: timers, window events, WebSocket messages.")
                    .size(self.sz(TEXT_SIZE)),
                space().height(self.sp(12.0)),
                button("🚨 Panic!").on_press(Message::PanicChaos),
            ]
            .spacing(self.sp(8.0)),
        )
        .into()
    }
}
