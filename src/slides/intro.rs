use iced::{
    widget::{column, scrollable, space, svg},
    Element, Padding,
};

use crate::{App, ELM_CIRCLE_OF_LIFE, Message};

pub(crate) const MD_INTRO: &str = r#"
The **Elm Architecture** is a pattern for structuring interactive applications.

It separates concerns into four distinct parts:

1. **Model** — the application state
2. **Message** — events from user input or the system
3. **Update** — a function that applies messages to the model
4. **View** — transforms state into UI with event handlers
"#;

impl App {
    pub(crate) fn view_intro_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                self.md_container(&self.md_intro),
                space().height(30),
                svg(svg::Handle::from_memory(ELM_CIRCLE_OF_LIFE)).height(260),
                space().height(30),
            ]
            .align_x(iced::Alignment::Center)
            .padding(Padding::new(20.0).left(40.0).right(40.0)),
        )
        .into()
    }
}
