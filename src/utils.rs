use std::str::FromStr;

use iced::Event;

use crate::app_launcher;
use crate::apps::{base64_converter, json_beautifier, uuid_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Version {
    V4,
    #[default]
    V7,
}

impl Version {
    pub const ALL: [Version; 2] = [Version::V4, Version::V7];
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Version::V4 => "Version 4",
                Version::V7 => "Version 7",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Application {
    #[default]
    UuidGenerator,
    JsonBeautifier,
    Base64Converter,
}

const UUID_GENERATOR_NAME: &str = "UUID Generator";
const JSON_BEAUTIFIER_NAME: &str = "JSON Beautifier";
const BASE64_CONVERTER_NAME: &str = "Base64 Converter";

impl Application {
    pub const ALL: [&str; 3] = [
        UUID_GENERATOR_NAME,
        JSON_BEAUTIFIER_NAME,
        BASE64_CONVERTER_NAME,
    ];
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Application::UuidGenerator => UUID_GENERATOR_NAME,
                Application::JsonBeautifier => JSON_BEAUTIFIER_NAME,
                Application::Base64Converter => BASE64_CONVERTER_NAME,
            }
        )
    }
}

impl FromStr for Application {
    type Err = ();

    fn from_str(input: &str) -> Result<Application, Self::Err> {
        match input {
            UUID_GENERATOR_NAME => Ok(Application::UuidGenerator),
            JSON_BEAUTIFIER_NAME => Ok(Application::JsonBeautifier),
            BASE64_CONVERTER_NAME => Ok(Application::Base64Converter),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    HideModal,
    UuidGenerator(uuid_generator::Message),
    AppLauncher(app_launcher::Message),
    JsonBeautifier(json_beautifier::Message),
    Base64Converter(base64_converter::Message),
    Event(Event),
}
