use iced::{
    Border, Color, Shadow, Theme,
    widget::{column, container, row, space, text},
    widget::button as iced_button,
    Element,
};
use iced_anim::widget::button;
use lucide_icons::iced::{icon_circle_check, icon_circle_x};

use crate::{App, CORRECT_COLOR, FIRA_MONO, INCORRECT_COLOR, Message, ORANGE, SUBTITLE_COLOR};

// WWM dark navy colors
const WWM_BG: Color = Color::from_rgb(0.08, 0.12, 0.22);
const WWM_BG_HOVER: Color = Color::from_rgb(0.14, 0.20, 0.35);
const WWM_BG_DIMMED: Color = Color::from_rgb(0.06, 0.08, 0.14);
const WWM_BORDER: Color = Color::from_rgb(0.25, 0.35, 0.55);

fn wwm_style(
    is_selected: bool,
    is_correct: bool,
    answered: bool,
) -> impl Fn(&Theme, iced_button::Status) -> iced_button::Style {
    move |_theme, status| {
        let border = Border {
            width: 1.5,
            radius: 6.0.into(),
            color: WWM_BORDER,
        };

        if !answered {
            // Not answered yet — normal WWM style
            let bg = match status {
                iced_button::Status::Hovered => WWM_BG_HOVER,
                iced_button::Status::Pressed => WWM_BG,
                _ => WWM_BG,
            };
            iced_button::Style {
                background: Some(bg.into()),
                text_color: Color::WHITE,
                border,
                shadow: Shadow::default(),
                snap: false,
            }
        } else if is_selected && is_correct {
            // Picked this, and it's correct → green
            iced_button::Style {
                background: Some(CORRECT_COLOR.into()),
                text_color: Color::WHITE,
                border: Border { color: CORRECT_COLOR, ..border },
                shadow: Shadow::default(),
                snap: false,
            }
        } else if is_selected && !is_correct {
            // Picked this, but it's wrong → red
            iced_button::Style {
                background: Some(INCORRECT_COLOR.into()),
                text_color: Color::WHITE,
                border: Border { color: INCORRECT_COLOR, ..border },
                shadow: Shadow::default(),
                snap: false,
            }
        } else if !is_selected && is_correct {
            // Not picked, but this is the correct answer → reveal green
            iced_button::Style {
                background: Some(Color::from_rgba(0.18, 0.65, 0.35, 0.7).into()),
                text_color: Color::WHITE,
                border: Border { color: CORRECT_COLOR, ..border },
                shadow: Shadow::default(),
                snap: false,
            }
        } else {
            // Not picked, not correct → dim
            iced_button::Style {
                background: Some(WWM_BG_DIMMED.into()),
                text_color: Color::from_rgba(1.0, 1.0, 1.0, 0.4),
                border: Border {
                    color: Color::from_rgba(0.25, 0.35, 0.55, 0.3),
                    ..border
                },
                shadow: Shadow::default(),
                snap: false,
            }
        }
    }
}

const LETTERS: [&str; 4] = ["A:", "B:", "C:", "D:"];

impl App {
    fn view_quiz<'a>(
        &self,
        question: &'a str,
        options: &'a [(&'a str, Message)],
        answer: Option<u8>,
        feedbacks: &'a [(u8, &'a str, bool)],
    ) -> Element<'a, Message> {
        assert_eq!(options.len(), 4, "WWM quiz requires exactly 4 options");

        // Find which index is correct
        let correct_idx = feedbacks
            .iter()
            .find(|(_, _, c)| *c)
            .map(|(i, _, _)| *i);

        // Build 4 styled buttons
        let buttons: Vec<Element<'a, Message>> = options
            .iter()
            .enumerate()
            .map(|(i, (label, msg))| {
                let idx = i as u8;
                let is_selected = answer == Some(idx);
                let is_correct = correct_idx == Some(idx);
                let answered = answer.is_some();

                let content = row![
                    text(LETTERS[i]).size(self.sz(18)).font(FIRA_MONO).color(ORANGE),
                    text(*label).size(self.sz(18)).color(Color::WHITE),
                ]
                .spacing(self.sp(10.0))
                .align_y(iced::Alignment::Center);

                button(content)
                    .on_press(msg.clone())
                    .width(iced::Fill)
                    .padding([12, 20])
                    .style(wwm_style(is_selected, is_correct, answered))
                    .into()
            })
            .collect();

        // 2×2 grid
        let mut buttons = buttons.into_iter();
        let a = buttons.next().unwrap();
        let b = buttons.next().unwrap();
        let c = buttons.next().unwrap();
        let d = buttons.next().unwrap();

        let grid = column![
            row![a, b].spacing(self.sp(16.0)),
            row![c, d].spacing(self.sp(16.0)),
        ]
        .spacing(self.sp(16.0));

        // Feedback text below
        let feedback: Element<'_, Message> = match answer {
            None => text("Select an answer")
                .size(self.sz(16))
                .color(SUBTITLE_COLOR)
                .into(),
            Some(idx) => {
                if let Some((_, fb, is_correct)) = feedbacks.iter().find(|(i, _, _)| *i == idx) {
                    let icon: Element<'_, Message> = if *is_correct {
                        icon_circle_check().size(self.sz(18)).color(CORRECT_COLOR).into()
                    } else {
                        icon_circle_x().size(self.sz(18)).color(INCORRECT_COLOR).into()
                    };
                    let color = if *is_correct {
                        CORRECT_COLOR
                    } else {
                        INCORRECT_COLOR
                    };
                    row![icon, text(*fb).size(self.sz(16)).color(color)]
                        .spacing(self.sp(8.0))
                        .align_y(iced::Alignment::Center)
                        .into()
                } else {
                    space().into()
                }
            }
        };

        container(
            column![
                text(question).size(self.sz(28)).color(ORANGE),
                space().height(self.sp(30.0)),
                grid,
                space().height(self.sp(20.0)),
                feedback,
            ]
            .spacing(self.sp(10.0))
            .padding(self.sp(20.0)),
        )
        .width(iced::Fill)
        .height(iced::Fill)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
    }

    pub fn view_quiz_screen(&self) -> Element<'_, Message> {
        self.view_quiz(
            "Where should validation of a text input happen?",
            &[
                ("In the View", Message::QuizAnswer(0)),
                ("In the Message", Message::QuizAnswer(1)),
                ("In the Update", Message::QuizAnswer(2)),
                ("In the Model", Message::QuizAnswer(3)),
            ],
            self.quiz_answer,
            &[
                (2, "Correct! The Update function processes input and validates data before updating the Model.", true),
                (0, "Not quite. The View only renders UI from state — it shouldn't contain logic.", false),
                (1, "Not quite. Messages are just data describing what happened — they don't contain logic.", false),
                (3, "Not quite. The Model only holds state, not logic.", false),
            ],
        )
    }

    pub fn view_quiz_http_screen(&self) -> Element<'_, Message> {
        self.view_quiz(
            "Where should you make an HTTP request?",
            &[
                ("In the View", Message::QuizHttpAnswer(0)),
                ("In the Message", Message::QuizHttpAnswer(1)),
                ("In a Task from Update", Message::QuizHttpAnswer(2)),
                ("In the Model", Message::QuizHttpAnswer(3)),
            ],
            self.quiz_http_answer,
            &[
                (2, "Correct! HTTP requests are async operations, so they belong in a Task returned from Update.", true),
                (0, "Not quite. The View only renders UI — it can't perform side effects.", false),
                (1, "Not quite. Messages are just data — they describe events, not perform actions.", false),
                (3, "Not quite. The Model only holds state — it doesn't perform operations.", false),
            ],
        )
    }

    pub fn view_quiz_button_screen(&self) -> Element<'_, Message> {
        self.view_quiz(
            "How do you disable a button when a field is empty?",
            &[
                ("Conditional on_press in View", Message::QuizButtonAnswer(0)),
                ("Flag in Model, View reads it", Message::QuizButtonAnswer(1)),
                ("Send a DisableButton message", Message::QuizButtonAnswer(2)),
                ("Add a disabled bool to Model", Message::QuizButtonAnswer(3)),
            ],
            self.quiz_button_answer,
            &[
                (0, "Correct! The View can check the condition directly and conditionally call on_press.", true),
                (1, "Also correct! For complex logic, Update can set a flag that the View reads.", true),
                (2, "Not quite. Messages describe events, not UI commands.", false),
                (3, "Not quite. A separate flag is unnecessary — the View can derive disabled state from existing data.", false),
            ],
        )
    }

    pub fn view_quiz_validation_screen(&self) -> Element<'_, Message> {
        self.view_quiz(
            "How does input validation with error display work?",
            &[
                ("Update → Model → View", Message::QuizValidationAnswer(0)),
                ("View validates directly", Message::QuizValidationAnswer(1)),
                ("Update shows error directly", Message::QuizValidationAnswer(2)),
                ("Message sends ValidateInput", Message::QuizValidationAnswer(3)),
            ],
            self.quiz_validation_answer,
            &[
                (0, "Correct! Update validates, stores errors in Model, View displays them.", true),
                (1, "Not quite. The View shouldn't contain validation logic — it only renders.", false),
                (2, "Not quite. Update validates, but the error must be stored in the Model for View to display.", false),
                (3, "Not quite. Messages carry data about what happened, not commands for what to do.", false),
            ],
        )
    }
}
