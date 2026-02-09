use iced::{
    widget::{column, scrollable, space, text},
    Element,
};

use crate::{App, Message, TEXT_SIZE};

pub const MD_MODEL: &str = r#"
```rust
enum Mode {
    Title,
    DownloadTime,
    DownloadSize,
}

struct UrlAnalyzer {
    url: String,
    secure: bool,
    mode: Mode,
}
```
"#;

impl App {
    pub fn view_model_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("The Model holds application state.").size(self.sz(TEXT_SIZE)),
                space().height(self.sp(12.0)),
                self.md_container(&self.md_model),
                space().height(self.sp(12.0)),
                text("Notice: completely UI-agnostic.").size(self.sz(TEXT_SIZE)),
            ]
            .spacing(self.sp(8.0)),
        )
        .into()
    }
}
