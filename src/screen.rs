use rotalubat::Rotalubat;
use strum::Display;

#[derive(Clone, Copy, PartialEq, Eq, Default, Display, Rotalubat)]
#[rotalubat(mode = "clamp")]
pub enum Screen {
    #[default]
    #[strum(serialize = "")]
    Title,

    #[strum(serialize = "0. The Elm Architecture")]
    Intro,

    // Part 1: The Elm Architecture
    #[strum(serialize = "1. Model")]
    Model,

    #[strum(serialize = "2. View")]
    View,

    #[strum(serialize = "3. Message")]
    Message,

    #[strum(serialize = "4. Update")]
    Update,

    // Part 2: Iced Widgets
    #[strum(serialize = "5. Widget: Button")]
    Button,

    #[strum(serialize = "6. Widget: Text Input")]
    TextInput,

    #[strum(serialize = "7. Tasks")]
    Tasks,

    #[strum(serialize = "8. Try It Out")]
    Interactive,

    #[strum(serialize = "9. Quiz")]
    Quiz,
}

impl Screen {
    pub fn is_first(&self) -> bool {
        *self == Screen::Title
    }

    pub fn is_last(&self) -> bool {
        *self == Screen::Quiz
    }
}
