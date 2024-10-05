use iced::Event;

use crate::{app_launcher, uuid_generator};

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
}

impl Application {
    pub const ALL: [Application; 1] = [Application::UUIDGenerator];
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Application::UUIDGenerator => "UUID Generator",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    HideModal,
    UUIDGenerator(uuid_generator::Message),
    AppLauncher(app_launcher::Message),
    Event(Event),
}
