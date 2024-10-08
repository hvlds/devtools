use iced::widget::{button, column, container, mouse_area, pick_list, row, text};
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
    value: Uuid,
    application_name: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Generated,
    Selected(Version),
    ResultCopied,
}

impl UuidGenerator {
    pub fn new() -> Self {
        Self {
            selected_version: Some(Version::V4),
            value: Uuid::new_v4(),
            application_name: UUID_GENERATOR_NAME.to_string(),
        }
    }

    pub fn title(&self) -> String {
        self.application_name.clone()
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
                "Result",
                mouse_area(container(text(format!("{}", self.value)).size(18)))
                    .on_press(Message::ResultCopied)
            ]
            .spacing(20),
        )
        .padding(10)
        .height(Length::Fill);

        content.into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Generated => match self.selected_version {
                Some(version) => {
                    match version {
                        Version::V4 => self.value = Uuid::new_v4(),
                        Version::V7 => self.value = Uuid::now_v7(),
                    };
                    Task::none()
                }
                None => {
                    self.value = Uuid::new_v4();
                    Task::none()
                }
            },
            Message::Selected(version) => {
                self.selected_version = Some(version);
                match version {
                    Version::V4 => self.value = Uuid::new_v4(),
                    Version::V7 => self.value = Uuid::now_v7(),
                };
                Task::none()
            }
            Message::ResultCopied => clipboard::write::<Message>(self.value.to_string()),
        }
    }
}
