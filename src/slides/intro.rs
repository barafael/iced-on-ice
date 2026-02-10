use iced::{
    Element,
    widget::{column, scrollable, space, svg},
};

use crate::{App, ELM_CIRCLE_OF_LIFE, Message};

pub const MD_INTRO: &str = r#"
The **Elm Architecture** is a pattern for structuring interactive applications.

It separates concerns into four distinct parts:

1. **Model** — the application state
2. **Message** — events from user input or the system
3. **Update** — a function that applies messages to the model
4. **View** — transforms state into UI with event handlers
"#;

impl App {
    pub fn view_intro_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                self.md_container(&self.md_intro),
                space().height(self.sp(30.0)),
                svg(svg::Handle::from_memory(ELM_CIRCLE_OF_LIFE)).height(self.sp(220.0)),
                space().height(self.sp(30.0)),
            ]
            .align_x(iced::Alignment::Center),
        )
        .into()
    }
}
