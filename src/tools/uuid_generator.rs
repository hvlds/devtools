use iced::widget::text_editor::Action;
use iced::widget::{
    button, column, container, horizontal_space, pick_list, row, scrollable, text, text_editor,
    text_input, Space,
};
use iced::{Element, Length};
use uuid::Uuid;

use crate::utils::{Version, UUID_GENERATOR_NAME};

impl Default for UuidGenerator {
    fn default() -> Self {
        UuidGenerator::new()
    }
}

pub struct UuidGenerator {
    selected_version: Option<Version>,
    output: text_editor::Content,
    tool_name: String,
    raw_amount: String,
    parsed_amount: u32,
    parsing_error: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Generated,
    Selected(Version),
    OutputActionPerformed(text_editor::Action),
    AmountChanged(String),
}

impl UuidGenerator {
    pub fn new() -> Self {
        Self {
            selected_version: Some(Version::V4),
            output: text_editor::Content::with_text(Uuid::new_v4().to_string().as_str()),
            tool_name: UUID_GENERATOR_NAME.to_string(),
            raw_amount: String::from("1"),
            parsed_amount: 1,
            parsing_error: String::new(),
        }
    }

    pub fn title(&self) -> String {
        self.tool_name.clone()
    }

    pub fn view(&self) -> Element<Message> {
        let configuration = column![
            "Configuration",
            row![
                "Version: ",
                pick_list(&Version::ALL[..], self.selected_version, Message::Selected,)
                    .placeholder("Choose a version")
            ],
            row![
                "Amount: ",
                text_input("Amount", self.raw_amount.as_str()).on_input(Message::AmountChanged),
                text(self.parsing_error.as_str()),
                horizontal_space()
            ],
            button("Generate UUID").on_press_maybe(match self.parsing_error.as_str() {
                "" => Some(Message::Generated),
                _ => None,
            }),
        ];

        let result = column![
            "Result",
            scrollable(text_editor(&self.output).on_action(Message::OutputActionPerformed))
        ];

        let content = container(column![configuration, Space::with_height(10), result].spacing(20))
            .padding(10)
            .height(Length::Fill);

        content.into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Generated => {
                let value = (0..self.parsed_amount)
                    .into_iter()
                    .map(|_| match self.selected_version {
                        Some(version) => match version {
                            Version::V4 => Uuid::new_v4().to_string(),
                            Version::V7 => Uuid::now_v7().to_string(),
                        },
                        None => Uuid::new_v4().to_string(),
                    })
                    .reduce(|cur: String, nxt: String| cur + "\n" + &nxt)
                    .unwrap();

                self.output = text_editor::Content::with_text(value.as_str());
            }
            Message::Selected(version) => {
                self.selected_version = Some(version);
                let value = match version {
                    Version::V4 => Uuid::new_v4().to_string(),
                    Version::V7 => Uuid::now_v7().to_string(),
                };
                self.output = text_editor::Content::with_text(value.as_str());
            }
            Message::OutputActionPerformed(action) => {
                match action {
                    Action::SelectAll | Action::SelectLine | Action::SelectWord => {
                        self.output.perform(action)
                    }

                    Action::Select(motion) => self.output.perform(Action::Select(motion)),
                    Action::Click(point) => self.output.perform(Action::Click(point)),
                    Action::Drag(point) => self.output.perform(Action::Drag(point)),
                    Action::Move(motion) => self.output.perform(Action::Move(motion)),
                    _ => (),
                };
            }
            Message::AmountChanged(value) => {
                self.raw_amount = value.clone();
                match value.parse::<u32>() {
                    Ok(v) => {
                        if v > 0 {
                            self.parsed_amount = v;
                            self.parsing_error = String::new();
                        } else {
                            self.parsing_error = format!("Amount must be at least 1 '{}'", value);
                        }
                    }
                    Err(_) => {
                        self.parsing_error = format!("Cannot parse '{}'", value);
                    }
                };
            }
        }
    }
}
