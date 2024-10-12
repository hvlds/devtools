use iced::{
    widget::{horizontal_space, row, text},
    Element, Length,
};

pub const NAME: &str = "Base64 Converter";

#[derive(Debug, Clone)]
pub enum Message {}

pub struct Base64Converter {
    tool_name: String,
}

impl Default for Base64Converter {
    fn default() -> Self {
        Base64Converter::new()
    }
}

impl Base64Converter {
    pub fn new() -> Self {
        Self {
            tool_name: NAME.to_string(),
        }
    }

    pub fn title(&self) -> String {
        self.tool_name.clone()
    }

    pub fn view(&self) -> Element<Message> {
        row![text("dummy"), horizontal_space()]
            .height(Length::Fill)
            .into()
    }
}
