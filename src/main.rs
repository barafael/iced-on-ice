use iced::{
    Color, Element, Event, Font, Padding, Subscription, Task, Theme, event, keyboard,
    widget::{
        button, canvas, column, container, markdown, pick_list, row, space, stack, text, themer,
    },
};
use iced_anim::{Animated, Animation, Motion};
use lucide_icons::{
    LUCIDE_FONT_BYTES,
    iced::{icon_chevron_left, icon_chevron_right},
};
use strum::EnumCount;

use crate::screen::Screen;
use theme::AppTheme;

mod chaos;
mod screen;
mod slides;
mod sliding;
mod theme;

pub const BITTER: Font = Font::with_name("Bitter");
pub const FIRA_MONO: Font = Font::with_name("Fira Mono");

pub const ICED_LOGO: &[u8] = include_bytes!("../assets/iced-logo.svg");

pub const TEXT_SIZE: u32 = 22;
const CODE_SIZE: u32 = 20;
pub const ORANGE: Color = Color::from_rgb(1.0, 0.4, 0.0);
pub const SUBTITLE_COLOR: Color = Color::from_rgb(0.45, 0.45, 0.45);
pub const CORRECT_COLOR: Color = Color::from_rgb(0.18, 0.65, 0.35);
pub const INCORRECT_COLOR: Color = Color::from_rgb(0.85, 0.25, 0.25);

pub const ELM_CIRCLE_OF_LIFE: &[u8] = include_bytes!("../assets/elm.svg");

pub struct App {
    pub screen: Screen,
    slide_offset: Animated<sliding::SlideOffset>,
    pub page_poker: page_poker::PagePoker,
    pub theme: Theme,
    pub ctrl_held: bool,
    pub shift_held: bool,
    pub chaos_circles: Vec<chaos::ChaosCircle>,
    pub chaos_paused: bool,
    canvas_size: (f32, f32),
    pub button_clicks: u32,
    pub input_changes: u32,
    pub input_submits: u32,
    pub demo_input: String,
    pub demo_spacing: f32,
    pub demo_padding: f32,
    pub quiz_answer: Option<u8>,
    pub quiz_http_answer: Option<u8>,
    pub quiz_button_answer: Option<u8>,
    pub quiz_validation_answer: Option<u8>,
    pub term: iced_term::Terminal,

    // Cached markdown content for each screen
    pub md_intro: Vec<markdown::Item>,
    pub md_model: Vec<markdown::Item>,
    pub md_view: Vec<markdown::Item>,
    pub md_theme: Vec<markdown::Item>,
    pub md_row_col: Vec<markdown::Item>,
    pub md_container: Vec<markdown::Item>,
    pub md_spacing: Vec<markdown::Item>,
    pub md_button: Vec<markdown::Item>,
    pub md_text_input: Vec<markdown::Item>,
    pub md_message: Vec<markdown::Item>,
    pub md_update: Vec<markdown::Item>,
    pub md_tasks: Vec<markdown::Item>,
    pub md_subscriptions: Vec<markdown::Item>,
    pub md_constructors: Vec<markdown::Item>,
    pub md_widget_messages: Vec<markdown::Item>,
}

impl Default for App {
    fn default() -> Self {
        use slides::*;

        Self {
            screen: Screen::default(),
            slide_offset: Animated::new(sliding::SlideOffset::settled(), Motion::SNAPPY),
            page_poker: page_poker::PagePoker::with_style(page_poker::StyleConfig {
                mono_font: FIRA_MONO,
                subtitle_color: SUBTITLE_COLOR,
                text_size: TEXT_SIZE,
            }),
            theme: Theme::GruvboxLight,
            ctrl_held: false,
            shift_held: false,
            chaos_circles: Vec::new(),
            chaos_paused: false,
            canvas_size: (800.0, 600.0),
            button_clicks: 0,
            input_changes: 0,
            input_submits: 0,
            demo_input: String::new(),
            demo_spacing: 10.0,
            demo_padding: 10.0,
            quiz_answer: None,
            quiz_http_answer: None,
            quiz_button_answer: None,
            quiz_validation_answer: None,
            term: shell_backend(),
            md_intro: markdown::parse(intro::MD_INTRO).collect(),
            md_model: markdown::parse(model::MD_MODEL).collect(),
            md_view: markdown::parse(view::MD_VIEW).collect(),
            md_theme: markdown::parse(view::MD_THEME).collect(),
            md_row_col: markdown::parse(layout::MD_ROW_COL).collect(),
            md_container: markdown::parse(layout::MD_CONTAINER).collect(),
            md_spacing: markdown::parse(layout::MD_SPACING).collect(),
            md_button: markdown::parse(button::MD_BUTTON).collect(),
            md_text_input: markdown::parse(text_input::MD_TEXT_INPUT).collect(),
            md_message: markdown::parse(message::MD_MESSAGE).collect(),
            md_update: markdown::parse(update::MD_UPDATE).collect(),
            md_tasks: markdown::parse(tasks::MD_TASKS).collect(),
            md_subscriptions: markdown::parse(subscriptions::MD_SUBSCRIPTIONS).collect(),
            md_constructors: markdown::parse(constructors::MD_CONSTRUCTORS).collect(),
            md_widget_messages: markdown::parse(constructors::MD_WIDGET_MESSAGES).collect(),
        }
    }
}

fn shell_backend() -> iced_term::Terminal {
    #[cfg(not(windows))]
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".into());
    #[cfg(windows)]
    let shell = "cmd.exe".to_string();
    let settings = iced_term::settings::Settings {
        font: iced_term::settings::FontSettings {
            size: 14.0,
            font_type: FIRA_MONO,
            ..Default::default()
        },
        backend: iced_term::settings::BackendSettings {
            program: shell,
            ..Default::default()
        },
        ..Default::default()
    };
    iced_term::Terminal::new(0, settings).expect("failed to create terminal")
}

#[derive(Debug, Clone)]
pub enum Message {
    // Navigation
    NextScreen,
    PrevScreen,

    // Page Poker (interactive screen)
    PagePoker(page_poker::Message),

    // No-op (used for markdown link clicks)
    Noop,

    // Demo
    ButtonClicked,
    DemoInputChanged(String),
    DemoInputSubmitted,
    DemoSpacingChanged(f32),
    DemoPaddingChanged(f32),

    // Animation
    SlideOffset(iced_anim::Event<sliding::SlideOffset>),

    // Theme
    ThemeChanged(Theme),
    CtrlPressed,
    CtrlReleased,
    ShiftPressed,
    ShiftReleased,

    // Chaos
    SpawnChaos,
    PanicChaos,
    Tick,
    WindowResized(f32, f32),

    // Quiz
    QuizAnswer(u8),
    QuizHttpAnswer(u8),
    QuizButtonAnswer(u8),
    QuizValidationAnswer(u8),

    // Terminal
    TermEvent(iced_term::Event),
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
            || self.slide_offset.value() != &sliding::SlideOffset::settled();

        let term_sub = self.term.subscription().map(Message::TermEvent);

        if self.screen == Screen::Subscriptions {
            let tick =
                iced::time::every(std::time::Duration::from_millis(16)).map(|_| Message::Tick);
            let spawn_timer =
                iced::time::every(std::time::Duration::from_secs(3)).map(|_| Message::SpawnChaos);
            Subscription::batch([events, tick, spawn_timer, term_sub])
        } else if needs_tick {
            let tick =
                iced::time::every(std::time::Duration::from_millis(16)).map(|_| Message::Tick);
            Subscription::batch([events, tick, term_sub])
        } else {
            Subscription::batch([events, term_sub])
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Noop => Task::none(),

            // Navigation
            Message::NextScreen => {
                if !self.screen.is_last() {
                    self.chaos_circles.clear();
                    self.chaos_paused = false;
                    self.screen.forward();
                    self.slide_offset =
                        Animated::new(sliding::SlideOffset::entering_forward(), Motion::SNAPPY);
                    self.slide_offset
                        .set_target(sliding::SlideOffset::settled());
                }
                Task::none()
            }
            Message::PrevScreen => {
                if !self.screen.is_first() {
                    self.chaos_circles.clear();
                    self.chaos_paused = false;
                    self.screen.backward();
                    self.slide_offset =
                        Animated::new(sliding::SlideOffset::entering_backward(), Motion::SNAPPY);
                    self.slide_offset
                        .set_target(sliding::SlideOffset::settled());
                }
                Task::none()
            }
            Message::SlideOffset(event) => {
                self.slide_offset.update(event);
                Task::none()
            }

            // Page Poker
            Message::PagePoker(msg) => self.page_poker.update(msg).map(Message::PagePoker),
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
            Message::DemoSpacingChanged(val) => {
                self.demo_spacing = val;
                Task::none()
            }
            Message::DemoPaddingChanged(val) => {
                self.demo_padding = val;
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
                if !self.chaos_paused {
                    let (w, h) = self.canvas_size;
                    self.chaos_circles.push(chaos::ChaosCircle::random(w, h));
                }
                Task::none()
            }
            Message::PanicChaos => {
                self.chaos_circles.clear();
                self.chaos_paused = true;
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
            Message::TermEvent(iced_term::Event::BackendCall(_, cmd)) => {
                self.term.handle(iced_term::Command::ProxyToBackend(cmd));
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let title = text(self.screen.to_string())
            .size(self.sz(28))
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
            Screen::Theming => self.view_theming_screen(),
            Screen::Message => self.view_message_screen(),
            Screen::Constructors => self.view_constructors_screen(),
            Screen::Update => self.view_update_screen(),
            Screen::Tasks => self.view_tasks_screen(),
            Screen::Subscriptions => self.view_subscriptions_screen(),
            Screen::Interactive => self.view_interactive_screen(),
            Screen::CommunityWidgets => self.view_community_widgets_screen(),
            Screen::Quiz => self.view_quiz_screen(),
            Screen::QuizHttp => self.view_quiz_http_screen(),
            Screen::QuizButton => self.view_quiz_button_screen(),
            Screen::QuizValidation => self.view_quiz_validation_screen(),
            Screen::Takeaways => self.view_takeaways_screen(),
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
                .spacing(self.sp(20.0))
                .padding(self.sp(30.0))
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

        if self.screen == Screen::Subscriptions {
            let chaos_overlay = canvas(chaos::ChaosOverlay {
                circles: &self.chaos_circles,
            })
            .width(iced::Fill)
            .height(iced::Fill);

            container(stack![layout, chaos_overlay])
                .width(iced::Fill)
                .height(iced::Fill)
                .into()
        } else {
            container(layout)
                .width(iced::Fill)
                .height(iced::Fill)
                .into()
        }
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
            .size(self.sz(14))
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

    /// Scale factor based on window size relative to 1024×768 base.
    pub fn scale(&self) -> f32 {
        let (w, h) = self.canvas_size;
        (w / 1024.0).min(h / 768.0).max(0.5)
    }

    /// Scale a pixel size (for text sizes, icon sizes, etc).
    pub fn sz(&self, base: u32) -> u32 {
        ((base as f32) * self.scale()) as u32
    }

    /// Scale a float value (for spacing, padding, heights).
    pub fn sp(&self, base: f32) -> f32 {
        base * self.scale()
    }

    pub fn md_settings(&self) -> markdown::Settings {
        let mut settings =
            markdown::Settings::with_text_size(self.sz(TEXT_SIZE), self.theme.clone());
        settings.code_size = self.sz(CODE_SIZE).into();
        settings
    }

    pub fn md_container<'a>(&self, md: &'a [markdown::Item]) -> Element<'a, Message> {
        let md_view: Element<'a, Message, AppTheme, _> =
            markdown::view(md, self.md_settings()).map(|_| Message::Noop);
        themer(Some(AppTheme(self.theme.clone())), md_view).into()
    }
}
