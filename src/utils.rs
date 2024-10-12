use std::str::FromStr;

use iced::Event;

use crate::launcher;
use crate::tools::{base64_converter, json_beautifier, uuid_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tool {
    #[default]
    UuidGenerator,
    JsonBeautifier,
    Base64Converter,
}

impl Tool {
    pub const ALL: [&str; 3] = [
        uuid_generator::NAME,
        json_beautifier::NAME,
        base64_converter::NAME,
    ];
}

impl std::fmt::Display for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tool::UuidGenerator => uuid_generator::NAME,
                Tool::JsonBeautifier => json_beautifier::NAME,
                Tool::Base64Converter => base64_converter::NAME,
            }
        )
    }
}

impl FromStr for Tool {
    type Err = ();

    fn from_str(input: &str) -> Result<Tool, Self::Err> {
        match input {
            uuid_generator::NAME => Ok(Tool::UuidGenerator),
            json_beautifier::NAME => Ok(Tool::JsonBeautifier),
            base64_converter::NAME => Ok(Tool::Base64Converter),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    HideModal,
    UuidGenerator(uuid_generator::Message),
    Launcher(launcher::Message),
    JsonBeautifier(json_beautifier::Message),
    Base64Converter(base64_converter::Message),
    Event(Event),
}
