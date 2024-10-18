use iced::{
    widget::{column, container, text_input},
    Element, Length,
};

pub const NAME: &str = "Base64 Converter";

#[derive(Debug, Clone)]
pub enum Message {
    DecodedTextChanged(String),
    EncodedTextChanged(String),
}

#[derive(Debug, Clone, Default)]
enum Mode {
    #[default]
    Encode,
    Decode,
}

pub struct Base64Converter {
    tool_name: String,
    encoded_text: String,
    decoded_text: String,
    active_mode: Mode,
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
            encoded_text: String::new(),
            decoded_text: String::new(),
            active_mode: Mode::default(),
        }
    }

    pub fn title(&self) -> String {
        self.tool_name.clone()
    }

    pub fn view(&self) -> Element<Message> {
        let controls = column![
            "Decoded",
            text_input("", self.decoded_text.as_str()).on_input_maybe(match self.active_mode {
                Mode::Encode => Some(Message::DecodedTextChanged),
                Mode::Decode => None,
            }),
            "Encoded",
            text_input("", self.encoded_text.as_str()).on_input_maybe(match self.active_mode {
                Mode::Encode => None,
                Mode::Decode => Some(Message::EncodedTextChanged),
            }),
        ];
        let content = container(controls).padding(10).height(Length::Fill);
        content.into()
    }
}
