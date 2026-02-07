use iced::{
    Color, Element, Event, Font, Padding, Subscription, Task, Theme, event, keyboard,
    widget::{button, column, container, markdown, pick_list, row, space, text, themer},
};
use iced_anim::{Animate, Animated, Animation, Motion};
use lucide_icons::LUCIDE_FONT_BYTES;
use lucide_icons::iced::{icon_chevron_left, icon_chevron_right};
use serde::Serialize;
use strum::{Display, EnumCount, EnumIter};

mod chaos;
mod screen;
mod slides;
mod theme;
use theme::AppTheme;

use crate::screen::Screen;

#[derive(Debug, Clone, PartialEq, Animate)]
pub struct SlideOffset {
    left: f32,
    right: f32,
}

impl SlideOffset {
    fn settled() -> Self {
        Self {
            left: 0.0,
            right: 0.0,
        }
    }

    fn entering_forward() -> Self {
        Self {
            left: 40.0,
            right: 0.0,
        }
    }

    fn entering_backward() -> Self {
        Self {
            left: 0.0,
            right: 40.0,
        }
    }
}

pub(crate) const BITTER: Font = Font::with_name("Bitter");
pub(crate) const FIRA_MONO: Font = Font::with_name("Fira Mono");

pub(crate) const ICED_LOGO: &[u8] = include_bytes!("../assets/iced-logo.svg");

pub(crate) const TEXT_SIZE: u32 = 22;
const CODE_SIZE: u32 = 20;
pub(crate) const ORANGE: Color = Color::from_rgb(1.0, 0.4, 0.0);
pub(crate) const SUBTITLE_COLOR: Color = Color::from_rgb(0.45, 0.45, 0.45);
pub(crate) const CORRECT_COLOR: Color = Color::from_rgb(0.18, 0.65, 0.35);
pub(crate) const INCORRECT_COLOR: Color = Color::from_rgb(0.85, 0.25, 0.25);

pub(crate) const ELM_CIRCLE_OF_LIFE: &[u8] = include_bytes!("../assets/elm.svg");

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
    pub(crate) url: String,
    pub(crate) secure: bool,
    pub(crate) mode: Mode,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) result: String,

    #[serde(skip)]
    pub(crate) loading: bool,
}

pub struct App {
    pub(crate) screen: Screen,
    slide_offset: Animated<SlideOffset>,
    pub(crate) model: UrlAnalyzer,
    pub(crate) theme: Theme,
    pub(crate) ctrl_held: bool,
    pub(crate) shift_held: bool,
    pub(crate) chaos_circles: Vec<chaos::ChaosCircle>,
    canvas_size: (f32, f32),
    pub(crate) button_clicks: u32,
    pub(crate) input_changes: u32,
    pub(crate) input_submits: u32,
    pub(crate) demo_input: String,
    pub(crate) quiz_answer: Option<u8>,
    pub(crate) quiz_http_answer: Option<u8>,
    pub(crate) quiz_button_answer: Option<u8>,
    pub(crate) quiz_validation_answer: Option<u8>,
    pub(crate) message_log: Vec<String>,

    // Cached markdown content for each screen
    pub(crate) md_intro: Vec<markdown::Item>,
    pub(crate) md_model: Vec<markdown::Item>,
    pub(crate) md_view: Vec<markdown::Item>,
    pub(crate) md_row_col: Vec<markdown::Item>,
    pub(crate) md_container: Vec<markdown::Item>,
    pub(crate) md_spacing: Vec<markdown::Item>,
    pub(crate) md_button: Vec<markdown::Item>,
    pub(crate) md_text_input: Vec<markdown::Item>,
    pub(crate) md_message: Vec<markdown::Item>,
    pub(crate) md_update: Vec<markdown::Item>,
    pub(crate) md_tasks: Vec<markdown::Item>,
    pub(crate) md_subscriptions: Vec<markdown::Item>,
}

impl Default for App {
    fn default() -> Self {
        use slides::*;

        let model = UrlAnalyzer::default();

        Self {
            screen: Screen::default(),
            slide_offset: Animated::new(SlideOffset::settled(), Motion::SNAPPY),
            model,
            theme: Theme::GruvboxLight,
            ctrl_held: false,
            shift_held: false,
            chaos_circles: Vec::new(),
            canvas_size: (800.0, 600.0),
            button_clicks: 0,
            input_changes: 0,
            input_submits: 0,
            demo_input: String::new(),
            quiz_answer: None,
            quiz_http_answer: None,
            quiz_button_answer: None,
            quiz_validation_answer: None,
            message_log: Vec::new(),
            md_intro: markdown::parse(intro::MD_INTRO).collect(),
            md_model: markdown::parse(model::MD_MODEL).collect(),
            md_view: markdown::parse(view::MD_VIEW).collect(),
            md_row_col: markdown::parse(layout::MD_ROW_COL).collect(),
            md_container: markdown::parse(layout::MD_CONTAINER).collect(),
            md_spacing: markdown::parse(layout::MD_SPACING).collect(),
            md_button: markdown::parse(button::MD_BUTTON).collect(),
            md_text_input: markdown::parse(text_input::MD_TEXT_INPUT).collect(),
            md_message: markdown::parse(message::MD_MESSAGE).collect(),
            md_update: markdown::parse(update::MD_UPDATE).collect(),
            md_tasks: markdown::parse(tasks::MD_TASKS).collect(),
            md_subscriptions: markdown::parse(subscriptions::MD_SUBSCRIPTIONS).collect(),
        }
    }
}

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
    SlideOffset(iced_anim::Event<SlideOffset>),

    // Theme
    ThemeChanged(Theme),
    CtrlPressed,
    CtrlReleased,
    ShiftPressed,
    ShiftReleased,

    // Chaos
    SpawnChaos,
    Tick,
    WindowResized(f32, f32),

    // Quiz
    QuizAnswer(u8),
    QuizHttpAnswer(u8),
    QuizButtonAnswer(u8),
    QuizValidationAnswer(u8),
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Iced Tutorial")
        .theme(App::theme)
        .subscription(App::subscription)
        .antialiasing(true)
        .font(LUCIDE_FONT_BYTES)
        .font(include_bytes!("../fonts/Bitter-Regular.ttf"))
        .font(include_bytes!("../fonts/FiraMono-Regular.ttf"))
        .default_font(BITTER)
        .run()
}

impl App {
    fn log_message(&mut self, msg: String) {
        self.message_log.push(msg);
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn subscription(&self) -> Subscription<Message> {
        use keyboard::Key;
        use keyboard::key::Named;

        let events = event::listen_with(|event, _status, _id| match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(Named::Control),
                ..
            }) => Some(Message::CtrlPressed),
            Event::Keyboard(keyboard::Event::KeyReleased {
                key: Key::Named(Named::Control),
                ..
            }) => Some(Message::CtrlReleased),
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(Named::Shift),
                ..
            }) => Some(Message::ShiftPressed),
            Event::Keyboard(keyboard::Event::KeyReleased {
                key: Key::Named(Named::Shift),
                ..
            }) => Some(Message::ShiftReleased),
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(Named::ArrowLeft),
                ..
            }) => Some(Message::PrevScreen),
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(Named::ArrowRight),
                ..
            }) => Some(Message::NextScreen),
            Event::Window(iced::window::Event::Resized(size)) => {
                Some(Message::WindowResized(size.width, size.height))
            }
            _ => None,
        });

        let needs_tick = self.screen == Screen::Subscriptions
            || self.slide_offset.value() != &SlideOffset::settled();

        if self.screen == Screen::Subscriptions {
            let tick =
                iced::time::every(std::time::Duration::from_millis(16)).map(|_| Message::Tick);
            let spawn_timer =
                iced::time::every(std::time::Duration::from_secs(3)).map(|_| Message::SpawnChaos);
            Subscription::batch([events, tick, spawn_timer])
        } else if needs_tick {
            let tick =
                iced::time::every(std::time::Duration::from_millis(16)).map(|_| Message::Tick);
            Subscription::batch([events, tick])
        } else {
            events
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // Navigation
            Message::NextScreen => {
                if !self.screen.is_last() {
                    self.screen.forward();
                    self.slide_offset =
                        Animated::new(SlideOffset::entering_forward(), Motion::SNAPPY);
                    self.slide_offset.set_target(SlideOffset::settled());
                }
                Task::none()
            }
            Message::PrevScreen => {
                if !self.screen.is_first() {
                    self.screen.backward();
                    self.slide_offset =
                        Animated::new(SlideOffset::entering_backward(), Motion::SNAPPY);
                    self.slide_offset.set_target(SlideOffset::settled());
                }
                Task::none()
            }
            Message::SlideOffset(event) => {
                self.slide_offset.update(event);
                Task::none()
            }

            // Model updates
            Message::UrlChanged(url) => {
                self.log_message(format!("UrlChanged({:?})", url));
                self.model.url = url;
                Task::none()
            }
            Message::SecureChanged(secure) => {
                self.log_message(format!("SecureChanged({})", secure));
                self.model.secure = secure;
                Task::none()
            }
            Message::ModeChanged(mode) => {
                self.log_message(format!("ModeChanged({})", mode));
                self.model.mode = mode;
                Task::none()
            }
            Message::Action => {
                self.log_message("Action".to_string());
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
                self.log_message(format!("Result({:?})", result));
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
            Message::ShiftPressed => {
                self.shift_held = true;
                Task::none()
            }
            Message::ShiftReleased => {
                self.shift_held = false;
                Task::none()
            }
            Message::SpawnChaos => {
                let (w, h) = self.canvas_size;
                self.chaos_circles.push(chaos::ChaosCircle::random(w, h));
                Task::none()
            }
            Message::Tick => {
                let (w, h) = self.canvas_size;
                for circle in &mut self.chaos_circles {
                    circle.update(w, h);
                }
                Task::none()
            }
            Message::WindowResized(width, height) => {
                self.canvas_size = (width, height);
                Task::none()
            }
            Message::QuizAnswer(answer) => {
                self.quiz_answer = Some(answer);
                Task::none()
            }
            Message::QuizHttpAnswer(answer) => {
                self.quiz_http_answer = Some(answer);
                Task::none()
            }
            Message::QuizButtonAnswer(answer) => {
                self.quiz_button_answer = Some(answer);
                Task::none()
            }
            Message::QuizValidationAnswer(answer) => {
                self.quiz_validation_answer = Some(answer);
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
            Screen::LayoutRowCol => self.view_layout_row_col_screen(),
            Screen::LayoutContainer => self.view_layout_container_screen(),
            Screen::LayoutSpacing => self.view_layout_spacing_screen(),
            Screen::Button => self.view_button_screen(),
            Screen::TextInput => self.view_text_input_screen(),
            Screen::Message => self.view_message_screen(),
            Screen::Update => self.view_update_screen(),
            Screen::Tasks => self.view_tasks_screen(),
            Screen::Subscriptions => self.view_subscriptions_screen(),
            Screen::Interactive => self.view_interactive_screen(),
            Screen::Quiz => self.view_quiz_screen(),
            Screen::QuizHttp => self.view_quiz_http_screen(),
            Screen::QuizButton => self.view_quiz_button_screen(),
            Screen::QuizValidation => self.view_quiz_validation_screen(),
            Screen::Recap => self.view_recap_screen(),
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

        let offset = self.slide_offset.value();
        let main_content = container(
            column![title, content]
                .spacing(20)
                .padding(30)
                .width(iced::Fill),
        )
        .padding(Padding {
            left: offset.left,
            right: offset.right,
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
        let prev_label = row![icon_chevron_left(), text("Previous")]
            .spacing(4)
            .align_y(iced::Alignment::Center);
        let next_label = row![text("Next"), icon_chevron_right()]
            .spacing(4)
            .align_y(iced::Alignment::Center);

        let prev_btn = if self.screen.is_first() {
            button(prev_label)
        } else {
            button(prev_label).on_press(Message::PrevScreen)
        };

        let next_btn = if self.screen.is_last() {
            button(next_label)
        } else {
            button(next_label).on_press(Message::NextScreen)
        };

        // Slide indicator
        let current = self.screen as usize;
        let total = Screen::COUNT;
        let slide_indicator = text(format!("{} / {}", current + 1, total))
            .size(14)
            .color(SUBTITLE_COLOR);

        let mut nav_row = row![prev_btn, slide_indicator, next_btn]
            .spacing(20)
            .align_y(iced::Alignment::Center);

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

    pub(crate) fn md_settings(&self) -> markdown::Settings {
        let mut settings = markdown::Settings::with_text_size(TEXT_SIZE, self.theme.clone());
        settings.code_size = CODE_SIZE.into();
        settings
    }

    pub(crate) fn md_container<'a>(&self, md: &'a [markdown::Item]) -> Element<'a, Message> {
        let md_view: Element<'a, Message, AppTheme, _> =
            markdown::view(md, self.md_settings()).map(|_| Message::Action);
        themer(Some(AppTheme(self.theme.clone())), md_view).into()
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

    let client = match reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
    {
        Ok(c) => c,
        Err(e) => return format!("Error creating client: {e}"),
    };

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

            // Extract title from HTML (naive: case-sensitive, no attribute handling)
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
