use iced::{
    widget::{column, container, horizontal_space, row, text, vertical_space},
    Alignment::Center,
    Element,
};

use crate::utils::BASE64_CONVERTER_NAME;

#[derive(Debug, Clone)]
pub enum Message {}

pub struct Base64Converter {
    application_name: String,
}

impl Default for Base64Converter {
    fn default() -> Self {
        Base64Converter::new()
    }
}

impl Base64Converter {
    pub fn new() -> Self {
        Self {
            application_name: BASE64_CONVERTER_NAME.to_string(),
        }
    }

    pub fn title(&self) -> String {
        self.application_name.clone()
    }

    pub fn view(&self) -> Element<Message> {
        text("dummy").into()
    }
}
