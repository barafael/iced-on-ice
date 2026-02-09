use iced::{
    Color, Element,
    widget::{button, column, container, row, scrollable, slider, space, text, text_input},
};

use crate::{App, FIRA_MONO, Message, SUBTITLE_COLOR, TEXT_SIZE};

pub const MD_ROW_COL: &str = r#"
```rust
// Nested layouts
column![
    row![text("Name"), text_input("Type here...", &self.input)],
    row![button("Cancel"), button("Submit")],
]
```
"#;

pub const MD_CONTAINER: &str = r#"
```rust
// Wrap content for positioning and styling
container(content)
    .center_x(Fill)
    .center_y(Fill)
    .padding(20)
    .style(container::rounded_box)
```
"#;

pub const MD_SPACING: &str = r#"
```rust
column![a, b, c]
    .spacing(10)           // Gap between children
    .padding(20)           // Space around the column
    .align_x(Center)       // Horizontal alignment
```
"#;

impl App {
    pub fn view_layout_row_col_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("The building blocks of layout.").size(self.sz(TEXT_SIZE)),
                space().height(self.sp(12.0)),
                self.md_container(&self.md_row_col),
                space().height(self.sp(20.0)),
                text("Live example:").size(self.sz(TEXT_SIZE)).color(SUBTITLE_COLOR),
                space().height(self.sp(10.0)),
                container(
                    column![
                        row![
                            text("Name"),
                            text_input("Type here...", &self.demo_input)
                                .on_input(Message::DemoInputChanged)
                        ]
                        .spacing(self.sp(10.0))
                        .align_y(iced::Alignment::Center),
                        row![
                            button("Cancel").on_press(Message::Noop),
                            button("Submit").on_press(Message::Noop),
                        ]
                        .spacing(self.sp(10.0)),
                    ]
                    .spacing(self.sp(10.0))
                )
                .padding(self.sp(15.0))
                .style(container::rounded_box),
            ]
            .spacing(self.sp(8.0)),
        )
        .into()
    }

    pub fn view_layout_container_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("Container wraps content for positioning and styling.").size(self.sz(TEXT_SIZE)),
                space().height(self.sp(12.0)),
                self.md_container(&self.md_container),
                space().height(self.sp(20.0)),
                text("Live example:").size(self.sz(TEXT_SIZE)).color(SUBTITLE_COLOR),
                space().height(self.sp(10.0)),
                container(
                    container(text("Centered and styled"))
                        .padding(self.sp(20.0))
                        .style(container::rounded_box),
                )
                .width(iced::Fill)
                .center_x(iced::Fill),
            ]
            .spacing(self.sp(8.0)),
        )
        .into()
    }

    pub fn view_layout_spacing_screen(&self) -> Element<'_, Message> {
        let sp = self.demo_spacing;
        let pd = self.demo_padding;

        // Interactive preview driven by sliders
        let preview: Element<'_, Message> = container(
            column![text("A"), text("B"), text("C")]
                .spacing(sp)
                .align_x(iced::Alignment::Center),
        )
        .padding(pd)
        .style(container::rounded_box)
        .into();

        let preview = if self.shift_held {
            preview.explain(Color::from_rgb(0.4, 0.2, 0.8))
        } else {
            preview
        };

        let spacing_slider = row![
            text(format!(".spacing({:.0})", sp))
                .size(self.sz(16))
                .font(FIRA_MONO),
            slider(0.0..=40.0, sp, Message::DemoSpacingChanged).width(self.sp(200.0)),
        ]
        .spacing(self.sp(12.0))
        .align_y(iced::Alignment::Center);

        let padding_slider = row![
            text(format!(".padding({:.0})", pd))
                .size(self.sz(16))
                .font(FIRA_MONO),
            slider(0.0..=40.0, pd, Message::DemoPaddingChanged).width(self.sp(200.0)),
        ]
        .spacing(self.sp(12.0))
        .align_y(iced::Alignment::Center);

        scrollable(
            column![
                text("Control gaps and alignment with spacing, padding, and align.")
                    .size(self.sz(TEXT_SIZE)),
                space().height(self.sp(12.0)),
                self.md_container(&self.md_spacing),
                space().height(self.sp(20.0)),
                row![spacing_slider, padding_slider].spacing(self.sp(20.0)),
                space().height(self.sp(12.0)),
                preview,
                space().height(self.sp(8.0)),
                text("hint: press shift")
                    .size(self.sz(TEXT_SIZE - 4))
                    .color(SUBTITLE_COLOR),
            ]
            .spacing(self.sp(8.0)),
        )
        .into()
    }
}
