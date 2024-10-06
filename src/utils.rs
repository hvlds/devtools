use std::str::FromStr;

use iced::Event;

use crate::app_launcher;
use crate::apps::{json_beautifier, uuid_generator};

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
    UUIDGenerator,
    JsonBeautifier,
}

impl Application {
    pub const ALL: [Application; 2] = [Application::UUIDGenerator, Application::JsonBeautifier];
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Application::UUIDGenerator => "UUID Generator",
                Application::JsonBeautifier => "JSON Beautifier",
            }
        )
    }
}

impl FromStr for Application {
    type Err = ();

    fn from_str(input: &str) -> Result<Application, Self::Err> {
        match input {
            "UUID Generator" => Ok(Application::UUIDGenerator),
            "JSON Beautifier" => Ok(Application::JsonBeautifier),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    HideModal,
    UUIDGenerator(uuid_generator::Message),
    AppLauncher(app_launcher::Message),
    JsonBeautifier(json_beautifier::Message),
    Event(Event),
}
