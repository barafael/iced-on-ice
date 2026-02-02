use iced::theme::{self, Style};
use iced::widget::{checkbox, container, markdown, rule, scrollable, table, text};
use iced::{Border, Color, Theme};

/// Custom theme wrapper that provides styled code blocks.
#[derive(Debug, Clone)]
pub struct AppTheme(pub Theme);

impl theme::Base for AppTheme {
    fn default(preference: theme::Mode) -> Self {
        AppTheme(Theme::default(preference))
    }

    fn name(&self) -> &str {
        self.0.name()
    }

    fn mode(&self) -> theme::Mode {
        self.0.mode()
    }

    fn base(&self) -> Style {
        self.0.base()
    }

    fn palette(&self) -> Option<iced::theme::Palette> {
        Some(self.0.palette())
    }
}

// Gruvbox dark colors for code blocks (high contrast)
const CODE_BG: Color = Color::from_rgba(0.18, 0.18, 0.18, 0.8);
const CODE_BORDER: Color = Color::from_rgb(0.30, 0.30, 0.28);
const CODE_TEXT: Color = Color::from_rgb(0.92, 0.86, 0.70);

impl container::Catalog for AppTheme {
    type Class<'a> = <Theme as container::Catalog>::Class<'a>;

    fn default<'a>() -> Self::Class<'a> {
        <Theme as container::Catalog>::default()
    }

    fn style(&self, class: &Self::Class<'_>) -> container::Style {
        self.0.style(class)
    }
}

impl scrollable::Catalog for AppTheme {
    type Class<'a> = <Theme as scrollable::Catalog>::Class<'a>;

    fn default<'a>() -> Self::Class<'a> {
        <Theme as scrollable::Catalog>::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: scrollable::Status) -> scrollable::Style {
        self.0.style(class, status)
    }
}

impl text::Catalog for AppTheme {
    type Class<'a> = <Theme as text::Catalog>::Class<'a>;

    fn default<'a>() -> Self::Class<'a> {
        <Theme as text::Catalog>::default()
    }

    fn style(&self, class: &Self::Class<'_>) -> text::Style {
        self.0.style(class)
    }
}

impl rule::Catalog for AppTheme {
    type Class<'a> = <Theme as rule::Catalog>::Class<'a>;

    fn default<'a>() -> Self::Class<'a> {
        <Theme as rule::Catalog>::default()
    }

    fn style(&self, class: &Self::Class<'_>) -> rule::Style {
        self.0.style(class)
    }
}

impl checkbox::Catalog for AppTheme {
    type Class<'a> = <Theme as checkbox::Catalog>::Class<'a>;

    fn default<'a>() -> Self::Class<'a> {
        <Theme as checkbox::Catalog>::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: checkbox::Status) -> checkbox::Style {
        self.0.style(class, status)
    }
}

impl table::Catalog for AppTheme {
    type Class<'a> = <Theme as table::Catalog>::Class<'a>;

    fn default<'a>() -> Self::Class<'a> {
        <Theme as table::Catalog>::default()
    }

    fn style(&self, class: &Self::Class<'_>) -> table::Style {
        self.0.style(class)
    }
}

impl markdown::Catalog for AppTheme {
    fn code_block<'a>() -> <Self as container::Catalog>::Class<'a> {
        Box::new(|_theme| container::Style {
            background: Some(CODE_BG.into()),
            border: Border {
                color: CODE_BORDER,
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Some(CODE_TEXT),
            ..Default::default()
        })
    }
}
