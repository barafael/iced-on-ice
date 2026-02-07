use iced::{
    widget::{button, column, container, row, space, text},
    Element,
};
use lucide_icons::iced::{icon_circle_check, icon_circle_x};

use crate::{App, CORRECT_COLOR, INCORRECT_COLOR, Message, ORANGE, SUBTITLE_COLOR};

impl App {
    fn view_quiz<'a>(
        question: &'a str,
        options: &'a [(&'a str, Message)],
        answer: Option<u8>,
        feedbacks: &'a [(u8, &'a str, bool)],
    ) -> Element<'a, Message> {
        let feedback: Element<'_, Message> = match answer {
            None => text("Select an answer above")
                .size(16)
                .color(SUBTITLE_COLOR)
                .into(),
            Some(idx) => {
                if let Some((_, fb, is_correct)) = feedbacks.iter().find(|(i, _, _)| *i == idx) {
                    let icon: Element<'_, Message> = if *is_correct {
                        icon_circle_check().size(18).color(CORRECT_COLOR).into()
                    } else {
                        icon_circle_x().size(18).color(INCORRECT_COLOR).into()
                    };
                    let color = if *is_correct {
                        CORRECT_COLOR
                    } else {
                        INCORRECT_COLOR
                    };
                    row![icon, text(*fb).size(18).color(color)]
                        .spacing(8)
                        .align_y(iced::Alignment::Center)
                        .into()
                } else {
                    space().into()
                }
            }
        };

        let option_buttons = column(
            options
                .iter()
                .map(|(label, msg)| button(*label).on_press(msg.clone()).into())
                .collect::<Vec<_>>(),
        )
        .spacing(12);

        container(
            column![
                text(question).size(32).color(ORANGE),
                space().height(30),
                option_buttons,
                space().height(25),
                feedback,
            ]
            .spacing(10)
            .align_x(iced::Alignment::Center),
        )
        .width(iced::Fill)
        .height(iced::Fill)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
    }

    pub(crate) fn view_quiz_screen(&self) -> Element<'_, Message> {
        Self::view_quiz(
            "Where should validation of a text input happen?",
            &[
                ("A) In the View", Message::QuizAnswer(0)),
                ("B) In the Message", Message::QuizAnswer(1)),
                ("C) In the Update", Message::QuizAnswer(2)),
                ("D) In the Model", Message::QuizAnswer(3)),
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

    pub(crate) fn view_quiz_http_screen(&self) -> Element<'_, Message> {
        Self::view_quiz(
            "Where should you make an HTTP request?",
            &[
                ("A) In the View", Message::QuizHttpAnswer(0)),
                ("B) In the Message", Message::QuizHttpAnswer(1)),
                ("C) In a Task returned from Update", Message::QuizHttpAnswer(2)),
                ("D) In the Model", Message::QuizHttpAnswer(3)),
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

    pub(crate) fn view_quiz_button_screen(&self) -> Element<'_, Message> {
        Self::view_quiz(
            "How do you disable a button when a field is empty?",
            &[
                ("A) View checks the condition with conditional on_press", Message::QuizButtonAnswer(0)),
                ("B) Update sets a flag in the Model, View reads it", Message::QuizButtonAnswer(1)),
                ("C) Send a DisableButton message", Message::QuizButtonAnswer(2)),
            ],
            self.quiz_button_answer,
            &[
                (0, "Correct! For simple conditions, the View can check directly with conditional on_press.", true),
                (1, "Correct! For complex logic, Update can set a flag in the Model that the View reads.", true),
                (2, "Not quite. Messages don't control UI state — they describe events.", false),
            ],
        )
    }

    pub(crate) fn view_quiz_validation_screen(&self) -> Element<'_, Message> {
        Self::view_quiz(
            "How does input validation with error display work?",
            &[
                ("A) Update validates, stores error in Model, View displays it", Message::QuizValidationAnswer(0)),
                ("B) View validates and shows error directly", Message::QuizValidationAnswer(1)),
                ("C) Update validates and shows error directly", Message::QuizValidationAnswer(2)),
            ],
            self.quiz_validation_answer,
            &[
                (0, "Correct! Update validates and stores errors in the Model. The View reads those errors and displays them. Messages carry the input data.", true),
                (1, "Not quite. The View shouldn't contain validation logic — it only renders based on Model state.", false),
                (2, "Not quite. While Update does the validation, the error must be stored in the Model for the View to display it.", false),
            ],
        )
    }
}
