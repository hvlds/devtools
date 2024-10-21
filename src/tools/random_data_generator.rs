use iced::{widget::text, Element};

pub const NAME: &str = "Random Data Generator";

pub struct RandomDataGenerator {
    tool_name: String,
}

#[derive(Debug, Clone)]
pub enum Message {}

impl RandomDataGenerator {
    pub fn new() -> Self {
        Self {
            tool_name: NAME.to_string(),
        }
    }

    pub fn title(&self) -> String {
        self.tool_name.clone()
    }

    pub fn view(&self) -> Element<Message> {
        text("Test").into()
    }
}
