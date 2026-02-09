use iced::{
    Element,
    widget::{column, scrollable, space, text},
};

use crate::{App, Message, SUBTITLE_COLOR, TEXT_SIZE};

pub const MD_THEME: &str = r#"
```rust
button("Click me")
    .style(|theme, status| {
        match status {
            button::Status::Active => button::Style {
                background: Some(Color::from_rgb(0.2, 0.6, 1.0).into()),
                text_color: Color::WHITE,
                ..Default::default()
            },
            button::Status::Hovered => button::Style {
                background: Some(Color::from_rgb(0.3, 0.7, 1.0).into()),
                text_color: Color::WHITE,
                ..Default::default()
            },
            _ => button::primary(theme, status),
        }
    })
```
"#;

pub const MD_VIEW: &str = r#"
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
    pub fn view_theming_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("Every widget has a .style() method that takes a closure.").size(self.sz(TEXT_SIZE)),
                space().height(self.sp(12.0)),
                self.md_container(&self.md_theme),
                space().height(self.sp(12.0)),
                text("The closure receives the current Theme and widget Status (Active, Hovered, Pressed, …).")
                    .size(self.sz(TEXT_SIZE)),
                space().height(self.sp(8.0)),
                text("Return a Style struct — background, text_color, border, shadow, …")
                    .size(self.sz(TEXT_SIZE)),
                space().height(self.sp(16.0)),
                text("Iced also ships with built-in themes (GruvboxLight, Dracula, Nord, …) that you can switch at runtime.")
                    .size(self.sz(TEXT_SIZE - 2))
                    .color(SUBTITLE_COLOR),
                space().height(self.sp(4.0)),
                text("hint: hold Ctrl to try it yourself")
                    .size(self.sz(TEXT_SIZE - 4))
                    .color(SUBTITLE_COLOR),
            ]
            .spacing(self.sp(8.0)),
        )
        .into()
    }

    pub fn view_view_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("The View visualizes the application state.").size(self.sz(TEXT_SIZE)),
                space().height(self.sp(12.0)),
                self.md_container(&self.md_view),
                space().height(self.sp(12.0)),
                text("Notice the method signature: &self (immutable borrow).").size(self.sz(TEXT_SIZE)),
                space().height(self.sp(8.0)),
                text("The View can read state but never modify it.").size(self.sz(TEXT_SIZE)),
            ]
            .spacing(self.sp(8.0)),
        )
        .into()
    }
}
