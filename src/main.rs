use iced::{
    Color, Element, Event, Font, Padding, Subscription, Task, Theme, event, keyboard,
    widget::{
        checkbox, column, container, markdown, pick_list, row, scrollable, space, svg, text,
        text_input, themer,
    },
};
use iced_anim::{Animated, Animation, Motion, widget::button};
use lucide_icons::LUCIDE_FONT_BYTES;
use serde::Serialize;
use strum::{Display, EnumIter, IntoEnumIterator};

mod screen;
mod theme;
use theme::AppTheme;

use crate::screen::Screen;

const BITTER: Font = Font::with_name("Bitter");
const FIRA_MONO: Font = Font::with_name("Fira Mono");

const ICED_LOGO: &[u8] = include_bytes!("../assets/iced-logo.svg");

const TEXT_SIZE: u32 = 22;
const CODE_SIZE: u32 = 20;
const ORANGE: Color = Color::from_rgb(1.0, 0.4, 0.0);

const ELM_CIRCLE_OF_LIFE: &[u8] = include_bytes!("../assets/elm.svg");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, EnumIter, Serialize)]
pub enum Mode {
    #[default]
    Title,

    #[strum(serialize = "Download Time")]
    DownloadTime,

    #[strum(serialize = "Download Size")]
    DownloadSize,
}

#[derive(Default, Serialize)]
pub struct UrlAnalyzer {
    url: String,
    secure: bool,
    mode: Mode,

    #[serde(skip_serializing_if = "String::is_empty")]
    result: String,

    #[serde(skip)]
    loading: bool,
}

pub struct App {
    screen: Screen,
    slide_offset: Animated<f32>,
    model: UrlAnalyzer,
    theme: Theme,
    ctrl_held: bool,
    button_clicks: u32,
    input_changes: u32,
    input_submits: u32,
    demo_input: String,
    quiz_answer: Option<u8>,

    // Cached markdown content for each screen
    md_intro: Vec<markdown::Item>,
    md_model: Vec<markdown::Item>,
    md_view: Vec<markdown::Item>,
    md_button: Vec<markdown::Item>,
    md_text_input: Vec<markdown::Item>,
    md_message: Vec<markdown::Item>,
    md_update: Vec<markdown::Item>,
    md_tasks: Vec<markdown::Item>,
}

impl Default for App {
    fn default() -> Self {
        let model = UrlAnalyzer::default();

        Self {
            screen: Screen::default(),
            slide_offset: Animated::new(0.0, Motion::SNAPPY.quick()),
            model,
            theme: Theme::GruvboxLight,
            ctrl_held: false,
            button_clicks: 0,
            input_changes: 0,
            input_submits: 0,
            demo_input: String::new(),
            quiz_answer: None,
            md_intro: markdown::parse(MD_INTRO).collect(),
            md_model: markdown::parse(MD_MODEL).collect(),
            md_view: markdown::parse(MD_VIEW).collect(),
            md_button: markdown::parse(MD_BUTTON).collect(),
            md_text_input: markdown::parse(MD_TEXT_INPUT).collect(),
            md_message: markdown::parse(MD_MESSAGE).collect(),
            md_update: markdown::parse(MD_UPDATE).collect(),
            md_tasks: markdown::parse(MD_TASKS).collect(),
        }
    }
}

const MD_INTRO: &str = r#"
The **Elm Architecture** is a way to structure interactive applications.

1. **Model** — application state
2. **Message** — events (user input, system events)
3. **Update** — applying messages to the model
4. **View** — state becomes UI (visualization and event handling)
"#;

const MD_MODEL: &str = r#"
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

const MD_BUTTON: &str = r#"
```rust
button("Get").on_press(Message::Action)
```
"#;

const MD_TEXT_INPUT: &str = r#"
```rust
text_input("Enter URL (e.g. example.com)", &self.model.url)
    .on_input(Message::UrlChanged)
    .on_submit(Message::Action)
```
"#;

const MD_MESSAGE: &str = r#"
```rust
enum Message {
    UrlChanged(String),
    SecureChanged(bool),
    ModeChanged(Mode),
    Action,
    Result(String),
}
```
"#;

const MD_UPDATE: &str = r#"
```rust
fn update(&mut self, message: Message) {
    match message {
        Message::UrlChanged(url) => self.url = url,
        Message::SecureChanged(secure) => self.secure = secure,
        Message::ModeChanged(mode) => self.mode = mode,
        Message::Action => todo!("Start fetching URL"),
        Message::Result(result) => self.result = result,
    }
}
```
"#;

const MD_TASKS: &str = r#"
```rust
Message::Action => {
    return Task::perform(
        fetch_url(self.url.clone(), self.secure, self.mode),
        Message::Result,
    );
}
```
"#;

const MD_VIEW: &str = r#"
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

#[derive(Debug, Clone)]
pub enum Message {
    // Navigation
    NextScreen,
    PrevScreen,

    // Model updates
    UrlChanged(String),
    SecureChanged(bool),
    ModeChanged(Mode),
    Action,
    Result(String),

    // Demo
    ButtonClicked,
    DemoInputChanged(String),
    DemoInputSubmitted,

    // Animation
    SlideOffset(iced_anim::Event<f32>),

    // Theme
    ThemeChanged(Theme),
    CtrlPressed,
    CtrlReleased,

    // Quiz
    QuizAnswer(u8),
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Iced Tutorial")
        .theme(App::theme)
        .subscription(App::subscription)
        .font(LUCIDE_FONT_BYTES)
        .font(include_bytes!("../fonts/Bitter-Regular.ttf"))
        .font(include_bytes!("../fonts/FiraMono-Regular.ttf"))
        .default_font(BITTER)
        .run()
}

impl App {
    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn subscription(&self) -> Subscription<Message> {
        use keyboard::Key;
        use keyboard::key::Named;

        event::listen_with(|event, _status, _id| match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(Named::Control),
                ..
            }) => Some(Message::CtrlPressed),
            Event::Keyboard(keyboard::Event::KeyReleased {
                key: Key::Named(Named::Control),
                ..
            }) => Some(Message::CtrlReleased),
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(Named::ArrowLeft),
                ..
            }) => Some(Message::PrevScreen),
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(Named::ArrowRight),
                ..
            }) => Some(Message::NextScreen),
            _ => None,
        })
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // Navigation
            Message::NextScreen => {
                if !self.screen.is_last() {
                    self.screen.forward();
                    // Start offset at positive (content coming from right), animate to 0
                    self.slide_offset.set_target(0.0);
                    self.slide_offset = Animated::new(60.0, Motion::SNAPPY.quick());
                    self.slide_offset.set_target(0.0);
                }
                Task::none()
            }
            Message::PrevScreen => {
                if !self.screen.is_first() {
                    self.screen.backward();
                    // Start offset at negative (content coming from left), animate to 0
                    self.slide_offset = Animated::new(-60.0, Motion::SNAPPY.quick());
                    self.slide_offset.set_target(0.0);
                }
                Task::none()
            }
            Message::SlideOffset(event) => {
                self.slide_offset.update(event);
                Task::none()
            }

            // Model updates
            Message::UrlChanged(url) => {
                self.model.url = url;
                Task::none()
            }
            Message::SecureChanged(secure) => {
                self.model.secure = secure;
                Task::none()
            }
            Message::ModeChanged(mode) => {
                self.model.mode = mode;
                Task::none()
            }
            Message::Action => {
                if self.model.url.is_empty() {
                    self.model.result = "Please enter a URL".to_string();
                    Task::none()
                } else {
                    self.model.loading = true;
                    self.model.result.clear();
                    let url = self.model.url.clone();
                    let secure = self.model.secure;
                    let mode = self.model.mode;
                    Task::perform(fetch_url(url, secure, mode), Message::Result)
                }
            }
            Message::Result(result) => {
                self.model.loading = false;
                self.model.result = result;
                Task::none()
            }
            Message::ButtonClicked => {
                self.button_clicks += 1;
                Task::none()
            }
            Message::DemoInputChanged(value) => {
                self.demo_input = value;
                self.input_changes += 1;
                Task::none()
            }
            Message::DemoInputSubmitted => {
                self.input_submits += 1;
                Task::none()
            }
            Message::ThemeChanged(theme) => {
                self.theme = theme;
                Task::none()
            }
            Message::CtrlPressed => {
                self.ctrl_held = true;
                Task::none()
            }
            Message::CtrlReleased => {
                self.ctrl_held = false;
                Task::none()
            }
            Message::QuizAnswer(answer) => {
                self.quiz_answer = Some(answer);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let title = text(self.screen.to_string())
            .size(28)
            .font(FIRA_MONO)
            .color(ORANGE);

        let content: Element<Message> = match self.screen {
            Screen::Title => self.view_title_screen(),
            Screen::Intro => self.view_intro_screen(),
            Screen::Model => self.view_model_screen(),
            Screen::View => self.view_view_screen(),
            Screen::Button => self.view_button_screen(),
            Screen::TextInput => self.view_text_input_screen(),
            Screen::Message => self.view_message_screen(),
            Screen::Update => self.view_update_screen(),
            Screen::Tasks => self.view_tasks_screen(),
            Screen::Interactive => self.view_interactive_screen(),
            Screen::Quiz => self.view_quiz_screen(),
        };

        let nav = self.view_navigation();
        let nav_bar = container(nav).center_x(iced::Fill).padding(20);

        // Orange stripe at the top
        let orange_stripe =
            container(space().height(6))
                .width(iced::Fill)
                .style(|_| container::Style {
                    background: Some(ORANGE.into()),
                    ..Default::default()
                });

        let offset = *self.slide_offset.value();
        let main_content = container(
            column![title, content]
                .spacing(20)
                .padding(30)
                .width(iced::Fill),
        )
        .padding(Padding {
            left: offset.max(0.0),
            right: (-offset).max(0.0),
            ..Padding::ZERO
        });

        let animated_content: Element<'_, Message> =
            Animation::new(&self.slide_offset, main_content)
                .on_update(Message::SlideOffset)
                .into();

        let layout = column![
            orange_stripe,
            container(animated_content).height(iced::Fill),
            nav_bar
        ];

        container(layout)
            .width(iced::Fill)
            .height(iced::Fill)
            .into()
    }

    fn view_navigation(&self) -> Element<'_, Message> {
        let prev_btn = if self.screen.is_first() {
            button("< Previous")
        } else {
            button("< Previous").on_press(Message::PrevScreen)
        };

        let next_btn = if self.screen.is_last() {
            button("Next >")
        } else {
            button("Next >").on_press(Message::NextScreen)
        };

        let mut nav_row = row![prev_btn, next_btn].spacing(20);

        if self.ctrl_held {
            let theme_picker = row![
                text("Theme: "),
                pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged),
            ]
            .spacing(10);
            nav_row = nav_row.push(theme_picker);
        }

        nav_row.into()
    }

    fn view_title_screen(&self) -> Element<'_, Message> {
        container(
            column![
                text("Discover Iced").size(64),
                space().height(40),
                svg(svg::Handle::from_memory(ICED_LOGO))
                    .width(64)
                    .height(64),
                space().height(40),
                text("An exploration of Iced").size(24),
                text("Made with Iced").size(24)
            ]
            .width(iced::Fill)
            .align_x(iced::Alignment::Center),
        )
        .width(iced::Fill)
        .height(iced::Fill)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
    }

    fn md_settings(&self) -> markdown::Settings {
        let mut settings = markdown::Settings::with_text_size(TEXT_SIZE, self.theme.clone());
        settings.code_size = CODE_SIZE.into();
        settings
    }

    fn view_intro_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                self.md_container(&self.md_intro),
                space().height(20),
                svg(svg::Handle::from_memory(ELM_CIRCLE_OF_LIFE)).height(320),
                space().height(20),
            ]
            .align_x(iced::Alignment::Center),
        )
        .into()
    }

    fn md_container<'a>(&self, md: &'a [markdown::Item]) -> Element<'a, Message> {
        let md_view: Element<'a, Message, AppTheme, _> =
            markdown::view(md, self.md_settings()).map(|_| Message::Action);
        themer(Some(AppTheme(self.theme.clone())), md_view).into()
    }

    fn view_model_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("The Model holds application state.").size(TEXT_SIZE),
                space().height(12),
                self.md_container(&self.md_model),
                text("").size(12),
                text("Notice: completely UI-agnostic.").size(TEXT_SIZE),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }

    fn view_view_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("The View visualizes the application state.").size(TEXT_SIZE),
                space().height(12),
                self.md_container(&self.md_view),
                text("").size(12),
                text("Notice the method signature! (&)").size(TEXT_SIZE),
                text("").size(12),
                text("Also, notice on_input and on_press.").size(TEXT_SIZE),
                text("").size(12),
                text("Layout etc. is out of scope here.").size(TEXT_SIZE),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }

    fn view_button_screen(&self) -> Element<'_, Message> {
        let click_text = if self.button_clicks == 0 {
            String::from("Click the button!")
        } else {
            format!(
                "Clicked {} time{}",
                self.button_clicks,
                if self.button_clicks == 1 { "" } else { "s" }
            )
        };

        scrollable(
            column![
                text("The Button widget produces messages when clicked.").size(TEXT_SIZE),
                space().height(8),
                self.md_container(&self.md_button),
                space().height(20),
                row![
                    button("Get").on_press(Message::ButtonClicked),
                    text(click_text).size(TEXT_SIZE),
                ]
                .align_y(iced::Center)
                .spacing(15)
                .align_y(iced::Alignment::Center),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }

    fn view_text_input_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("The Text Input widget produces messages as the user types.").size(TEXT_SIZE),
                space().height(8),
                self.md_container(&self.md_text_input),
                space().height(20),
                text_input("Enter URL (e.g. example.com)", &self.demo_input)
                    .on_input(Message::DemoInputChanged)
                    .on_submit(Message::DemoInputSubmitted),
                space().height(12),
                text!("Input Changed messages: {}", self.input_changes).size(TEXT_SIZE),
                text!("Input Submitted messages: {}", self.input_submits).size(TEXT_SIZE),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }

    fn view_message_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("Messages describe user actions or system events.").size(TEXT_SIZE),
                text("").size(8),
                self.md_container(&self.md_message),
                text("").size(12),
                text("Messages are produced by the view.").size(TEXT_SIZE)
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }

    fn view_update_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("Update modifies state based on messages.").size(TEXT_SIZE),
                text("").size(8),
                self.md_container(&self.md_update),
                text("").size(12),
                text("Notice the method signature! (&mut)").size(TEXT_SIZE),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }

    fn view_tasks_screen(&self) -> Element<'_, Message> {
        scrollable(
            column![
                text("The update function may produce a Task for async background operations.")
                    .size(TEXT_SIZE),
                text("").size(8),
                self.md_container(&self.md_tasks),
                text("").size(12),
                text("Task::perform takes an async function and a message constructor.")
                    .size(TEXT_SIZE),
                text("When the async work completes, the result is wrapped in the message.")
                    .size(TEXT_SIZE),
                text("").size(12),
                text("In this app, fetch_url performs an HTTP request asynchronously.")
                    .size(TEXT_SIZE),
                text("The response is applied via Message::Result.").size(TEXT_SIZE),
            ]
            .spacing(8)
            .padding(30),
        )
        .into()
    }

    fn view_interactive_screen(&self) -> Element<'_, Message> {
        let mode_options: Vec<Mode> = Mode::iter().collect();

        let url_input = text_input("Enter URL (e.g. example.com)", &self.model.url)
            .on_input(Message::UrlChanged)
            .on_submit(Message::Action);

        let secure_checkbox = checkbox(self.model.secure)
            .label("Secure (HTTPS)")
            .on_toggle(Message::SecureChanged);

        let mode_picker = pick_list(mode_options, Some(self.model.mode), Message::ModeChanged);

        let get_button = if self.model.loading {
            button("Loading...")
        } else {
            button("Get").on_press(Message::Action)
        };

        let controls = row![secure_checkbox, mode_picker, get_button]
            .spacing(20)
            .align_y(iced::Alignment::Center);

        let result_display: Element<'_, Message> = if self.model.loading {
            text("Fetching...").size(TEXT_SIZE).into()
        } else if !self.model.result.is_empty() {
            container(text(&self.model.result).size(TEXT_SIZE))
                .padding(10)
                .style(container::rounded_box)
                .into()
        } else {
            space().into()
        };

        // RON state visualization
        let ron_config = ron::ser::PrettyConfig::default();
        let state_ron = ron::ser::to_string_pretty(&self.model, ron_config)
            .unwrap_or_else(|e| format!("Error: {e}"));

        let state_display = column![
            text("Current State").size(20).font(FIRA_MONO),
            row![
                space().width(iced::Length::FillPortion(1)),
                container(text(state_ron).size(18).font(FIRA_MONO))
                    .width(iced::Length::FillPortion(4))
                    .padding(20)
                    .style(container::rounded_box),
                space().width(iced::Length::FillPortion(1)),
            ],
        ]
        .spacing(10)
        .align_x(iced::Alignment::Center);

        let form = container(
            column![
                url_input,
                space().height(10),
                controls,
                space().height(15),
                result_display,
            ]
            .width(iced::Fill),
        )
        .padding(20)
        .center_x(iced::Fill);

        column![
            text("Try out the URL Analyzer - all the concepts in action.").size(TEXT_SIZE),
            space().height(20),
            form,
            space().height(20),
            state_display,
        ]
        .spacing(10)
        .into()
    }

    fn view_quiz_screen(&self) -> Element<'_, Message> {
        let feedback: Element<'_, Message> = match self.quiz_answer {
            None => space().into(),
            Some(2) => text("Correct! The Update function is where you process input and validate data before updating the Model.")
                .size(18)
                .color(Color::from_rgb(0.2, 0.7, 0.3))
                .into(),
            Some(0) => text("Incorrect. The View only renders UI from state — it shouldn't contain logic.")
                .size(18)
                .color(Color::from_rgb(0.8, 0.2, 0.2))
                .into(),
            Some(1) => text("Incorrect. Messages are just data describing what happened — they don't contain logic.")
                .size(18)
                .color(Color::from_rgb(0.8, 0.2, 0.2))
                .into(),
            Some(3) => text("Incorrect. The Model only holds state.")
                .size(18)
                .color(Color::from_rgb(0.8, 0.2, 0.2))
                .into(),
            Some(_) => space().into(),
        };

        container(
            column![
                text("Where should validation of a text input happen?").size(TEXT_SIZE),
                space().height(30),
                column![
                    button("A) In the View").on_press(Message::QuizAnswer(0)),
                    button("B) In the Message").on_press(Message::QuizAnswer(1)),
                    button("C) In the Update").on_press(Message::QuizAnswer(2)),
                    button("D) In the Model").on_press(Message::QuizAnswer(3)),
                ]
                .spacing(15),
                space().height(20),
                feedback,
            ]
            .spacing(20)
            .align_x(iced::Alignment::Center),
        )
        .width(iced::Fill)
        .height(iced::Fill)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
    }
}

async fn fetch_url(url: String, secure: bool, mode: Mode) -> String {
    let protocol = if secure { "https" } else { "http" };
    let full_url = if url.starts_with("http://") || url.starts_with("https://") {
        url
    } else {
        format!("{protocol}://{url}")
    };

    let start = std::time::Instant::now();

    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    let response = match client.get(&full_url).send().await {
        Ok(resp) => resp,
        Err(e) => return format!("Error: {}", e),
    };

    let elapsed = start.elapsed();

    match mode {
        Mode::Title => {
            let body = match response.text().await {
                Ok(text) => text,
                Err(e) => return format!("Error reading body: {}", e),
            };

            // Extract title from HTML
            if let Some(start_idx) = body.find("<title>")
                && let Some(end_idx) = body.find("</title>")
            {
                let title = &body[start_idx + 7..end_idx];
                return format!("Title: {}", title.trim());
            }
            "No <title> found".to_string()
        }
        Mode::DownloadTime => {
            // Consume the body to measure full download time
            match response.bytes().await {
                Ok(_) => format!("Download time: {elapsed:.2?}"),
                Err(error) => format!("Error: {error}"),
            }
        }
        Mode::DownloadSize => match response.bytes().await {
            Ok(bytes) => format!(
                "Size: {}",
                humansize::format_size(bytes.len(), humansize::DECIMAL)
            ),
            Err(error) => format!("Error: {error}"),
        },
    }
}
