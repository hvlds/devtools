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

pub const UUID_GENERATOR_NAME: &str = "UUID Generator";
pub const JSON_BEAUTIFIER_NAME: &str = "JSON Beautifier";
pub const BASE64_CONVERTER_NAME: &str = "Base64 Converter";

impl Tool {
    pub const ALL: [&str; 3] = [
        UUID_GENERATOR_NAME,
        JSON_BEAUTIFIER_NAME,
        BASE64_CONVERTER_NAME,
    ];
}

impl std::fmt::Display for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tool::UuidGenerator => UUID_GENERATOR_NAME,
                Tool::JsonBeautifier => JSON_BEAUTIFIER_NAME,
                Tool::Base64Converter => BASE64_CONVERTER_NAME,
            }
        )
    }
}

impl FromStr for Tool {
    type Err = ();

    fn from_str(input: &str) -> Result<Tool, Self::Err> {
        match input {
            UUID_GENERATOR_NAME => Ok(Tool::UuidGenerator),
            JSON_BEAUTIFIER_NAME => Ok(Tool::JsonBeautifier),
            BASE64_CONVERTER_NAME => Ok(Tool::Base64Converter),
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
