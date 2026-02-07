use iced::{
    widget::{column, container, row, scrollable, space, text},
    Color, Element,
};

use crate::{App, Message, SUBTITLE_COLOR, TEXT_SIZE};

pub(crate) const MD_ROW_COL: &str = r#"
```rust
// Horizontal layout
row![widget_a, widget_b, widget_c]

// Nested layouts
column![
    row![label, text_input],
    row![cancel_btn, submit_btn],
]
```
"#;

pub(crate) const MD_CONTAINER: &str = r#"
```rust
// Wrap content for positioning and styling
container(content)
    .center_x(Fill)
    .center_y(Fill)
    .padding(20)
    .style(container::rounded_box)
```
"#;

pub(crate) const MD_SPACING: &str = r#"
```rust
column![a, b, c]
    .spacing(10)           // Gap between children
    .padding(20)           // Space around the column
    .align_x(Center)       // Horizontal alignment
```
"#;

impl App {
    pub(crate) fn view_layout_row_col_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("Rows and columns are the building blocks of layout.").size(TEXT_SIZE),
                space().height(12),
                self.md_container(&self.md_row_col),
                space().height(20),
                text("Live example:").size(TEXT_SIZE).color(SUBTITLE_COLOR),
                space().height(10),
                container(
                    column![
                        row![
                            text("Row 1, Col A"),
                            text("Row 1, Col B"),
                            text("Row 1, Col C"),
                        ]
                        .spacing(20),
                        row![text("Row 2, Col A"), text("Row 2, Col B")].spacing(20),
                    ]
                    .spacing(10)
                )
                .padding(15)
                .style(container::rounded_box),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }

    pub(crate) fn view_layout_container_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("Container wraps content for positioning and styling.").size(TEXT_SIZE),
                space().height(12),
                self.md_container(&self.md_container),
                space().height(20),
                text("Live example:").size(TEXT_SIZE).color(SUBTITLE_COLOR),
                space().height(10),
                container(
                    container(text("Centered and styled"))
                        .padding(20)
                        .style(container::rounded_box),
                )
                .width(iced::Fill)
                .center_x(iced::Fill),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }

    pub(crate) fn view_layout_spacing_screen(&self) -> Element<'_, Message> {
        let row: Element<'_, Message> = row![
            container(
                column![text("A"), text("B"), text("C")]
                    .spacing(5)
                    .align_x(iced::Alignment::Start),
            )
            .padding(10)
            .style(container::rounded_box),
            container(
                column![text("A"), text("B"), text("C")]
                    .spacing(15)
                    .align_x(iced::Alignment::Center),
            )
            .padding(10)
            .style(container::rounded_box),
            container(
                column![text("A"), text("B"), text("C")]
                    .spacing(25)
                    .align_x(iced::Alignment::End),
            )
            .padding(10)
            .style(container::rounded_box),
        ]
        .spacing(20)
        .into();
        let row = if self.shift_held {
            row.explain(Color::from_rgb(0.4, 0.2, 0.8))
        } else {
            row
        };

        scrollable(
            column![
                text("Control gaps and alignment with spacing, padding, and align.")
                    .size(TEXT_SIZE),
                space().height(12),
                self.md_container(&self.md_spacing),
                space().height(20),
                text("Live example:").size(TEXT_SIZE).color(SUBTITLE_COLOR),
                space().height(10),
                row,
                text("hint: press shift")
                    .size(TEXT_SIZE)
                    .color(SUBTITLE_COLOR),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }
}
