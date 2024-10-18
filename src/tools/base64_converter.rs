use base64::{engine::general_purpose, Engine as _};
use iced::{
    widget::{column, container, scrollable, text_editor},
    Element, Length,
};

pub const NAME: &str = "Base64 Converter";

#[derive(Debug, Clone)]
pub enum Message {
    DecodedTextChanged(text_editor::Action),
    EncodedTextChanged(text_editor::Action),
}

pub struct Base64Converter {
    tool_name: String,
    encoded_input: text_editor::Content,
    decoded_input: text_editor::Content,
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
            encoded_input: text_editor::Content::new(),
            decoded_input: text_editor::Content::new(),
        }
    }

    pub fn title(&self) -> String {
        self.tool_name.clone()
    }

    pub fn view(&self) -> Element<Message> {
        let decoded_panel = column![
            "Decoded",
            container(scrollable(
                text_editor(&self.decoded_input).on_action(Message::DecodedTextChanged)
            ))
            .height(Length::Fill),
        ];

        let encoded_panel = column![
            "Encoded",
            container(scrollable(
                text_editor(&self.encoded_input).on_action(Message::EncodedTextChanged)
            ))
            .height(Length::Fill),
        ];

        let content = container(column![decoded_panel, encoded_panel]).padding(10);
        content.into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::DecodedTextChanged(text_action) => {
                let old_text = self.decoded_input.text().to_owned();
                self.decoded_input.perform(text_action);
                let mut new_text = self.decoded_input.text().to_owned();
                let len = new_text.trim_end_matches(&['\r', '\n'][..]).len();
                new_text.truncate(len);
                if old_text != new_text {
                    let encoded = general_purpose::STANDARD.encode(new_text.as_bytes());
                    self.encoded_input = text_editor::Content::with_text(&encoded);
                }
            }
            Message::EncodedTextChanged(text_action) => {
                let old_text = self.encoded_input.text().to_owned();
                self.encoded_input.perform(text_action);
                let mut new_text = self.encoded_input.text().to_owned();
                let len = new_text.trim_end_matches(&['\r', '\n'][..]).len();
                new_text.truncate(len);
                if old_text != new_text {
                    match general_purpose::STANDARD.decode(new_text.as_str()) {
                        Ok(decoded) => {
                            self.decoded_input = text_editor::Content::with_text(
                                String::from_utf8(decoded).unwrap().as_str(),
                            )
                        }
                        Err(_) => (),
                    }
                }
            }
        }
    }
}
