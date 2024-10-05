use iced::widget::{button, column, container, horizontal_space, pick_list, row, text};
use iced::Alignment::Center;
use iced::Element;
use iced::Length::Fill;
use uuid::Uuid;

use crate::utils::Version;

impl Default for UUIDGenerator {
    fn default() -> Self {
        UUIDGenerator::new()
    }
}

pub struct UUIDGenerator {
    selected_version: Option<Version>,
    value: Uuid,
}

#[derive(Debug, Clone)]
pub enum Message {
    Generated,
    Selected(Version),
}

impl UUIDGenerator {
    pub fn new() -> Self {
        Self {
            selected_version: Some(Version::V4),
            value: Uuid::new_v4(),
        }
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        let header = container(
            row![horizontal_space(), "UUID Generator", horizontal_space(),]
                .padding(10)
                .align_y(Center),
        )
        .style(container::rounded_box);

        let content = container(
            column![
                pick_list(&Version::ALL[..], self.selected_version, Message::Selected,)
                    .placeholder("Choose a version"),
                button("Generate UUID").on_press(Message::Generated),
                text(format!("{}", self.value)).size(30)
            ]
            .spacing(20),
        )
        .padding(10)
        .height(Fill);

        column![header, content].into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Generated => match self.selected_version {
                Some(version) => match version {
                    Version::V4 => self.value = Uuid::new_v4(),
                    Version::V7 => self.value = Uuid::now_v7(),
                },
                None => self.value = Uuid::new_v4(),
            },
            Message::Selected(version) => {
                self.selected_version = Some(version);
                match version {
                    Version::V4 => self.value = Uuid::new_v4(),
                    Version::V7 => self.value = Uuid::now_v7(),
                }
            }
        }
    }
}
