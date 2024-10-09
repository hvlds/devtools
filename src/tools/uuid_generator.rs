use iced::widget::text_editor::Action;
use iced::widget::{
    button, column, container, mouse_area, pick_list, row, text, text_editor, Space,
};
use iced::{clipboard, Element, Length, Task};
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
}

#[derive(Debug, Clone)]
pub enum Message {
    Generated,
    Selected(Version),
    OutputActionPerformed(text_editor::Action),
}

impl UuidGenerator {
    pub fn new() -> Self {
        Self {
            selected_version: Some(Version::V4),
            output: text_editor::Content::with_text(Uuid::new_v4().to_string().as_str()),
            tool_name: UUID_GENERATOR_NAME.to_string(),
        }
    }

    pub fn title(&self) -> String {
        self.tool_name.clone()
    }

    pub fn view(&self) -> Element<Message> {
        let content = container(
            column![
                "Configuration",
                row![
                    "Version: ",
                    pick_list(&Version::ALL[..], self.selected_version, Message::Selected,)
                        .placeholder("Choose a version")
                ],
                button("Generate UUID").on_press(Message::Generated),
                Space::with_height(10),
                "Result",
                text_editor(&self.output).on_action(Message::OutputActionPerformed)
            ]
            .spacing(20),
        )
        .padding(10)
        .height(Length::Fill);

        content.into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Generated => {
                let value = match self.selected_version {
                    Some(version) => match version {
                        Version::V4 => Uuid::new_v4().to_string(),
                        Version::V7 => Uuid::now_v7().to_string(),
                    },
                    None => Uuid::new_v4().to_string(),
                };
                self.output = text_editor::Content::with_text(value.as_str());
                Task::none()
            }
            Message::Selected(version) => {
                self.selected_version = Some(version);
                let value = match version {
                    Version::V4 => Uuid::new_v4().to_string(),
                    Version::V7 => Uuid::now_v7().to_string(),
                };
                self.output = text_editor::Content::with_text(value.as_str());
                Task::none()
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
                Task::none()
            }
        }
    }
}
