use rotalubat::Rotalubat;
use strum::{Display, EnumCount};

#[derive(Clone, Copy, PartialEq, Eq, Default, Display, Rotalubat, EnumCount)]
#[rotalubat(mode = "clamp")]
pub enum Screen {
    #[default]
    #[strum(serialize = "")]
    Title,

    #[strum(serialize = "0. The Elm Architecture")]
    Intro,

    // The Elm Architecture
    #[strum(serialize = "1. Model")]
    Model,

    #[strum(serialize = "2. View")]
    View,

    // Layout
    #[strum(serialize = "3. Layout: Rows & Columns")]
    LayoutRowCol,

    #[strum(serialize = "4. Layout: Container")]
    LayoutContainer,

    #[strum(serialize = "5. Layout: Spacing & Alignment")]
    LayoutSpacing,

    // Widgets
    #[strum(serialize = "6. Widget: Button")]
    Button,

    #[strum(serialize = "7. Widget: Text Input")]
    TextInput,

    #[strum(serialize = "8. Styling Widgets")]
    Theming,

    #[strum(serialize = "9. Theming")]
    ThemePicker,

    // Back to Elm Architecture
    #[strum(serialize = "10. Message")]
    Message,

    #[strum(serialize = "11. Variants as Constructors")]
    Constructors,

    #[strum(serialize = "12. Update")]
    Update,

    #[strum(serialize = "13. Tasks")]
    Tasks,

    #[strum(serialize = "14. Subscriptions")]
    Subscriptions,

    // Demo
    #[strum(serialize = "15. Page Poker")]
    Interactive,

    // Community
    #[strum(serialize = "16. Community Widgets")]
    CommunityWidgets,

    // Quizzes
    #[strum(serialize = "17. Quiz: Where Does Logic Live?")]
    Quiz,

    #[strum(serialize = "18. Quiz: Async Operations")]
    QuizHttp,

    #[strum(serialize = "19. Quiz: Conditional UI")]
    QuizButton,

    #[strum(serialize = "20. Quiz: Validation Flow")]
    QuizValidation,

    // Recap
    #[strum(serialize = "21. Key Takeaways")]
    Takeaways,

    #[strum(serialize = "22. Recap")]
    Recap,
}

impl Screen {
    pub fn is_first(&self) -> bool {
        *self == Screen::Title
    }

    pub fn is_last(&self) -> bool {
        *self == Screen::Recap
    }
}
