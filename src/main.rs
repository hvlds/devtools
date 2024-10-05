use app_launcher::AppLauncher;
use iced::event::{self};
use iced::widget::{self};
use iced::{
    keyboard::{self, key},
    Element, Event, Subscription, Task,
};
use modal::modal;
use utils::Message;
use uuid_generator::UUIDGenerator;

mod app_launcher;
mod modal;
mod utils;
mod uuid_generator;

pub fn main() -> iced::Result {
    iced::application("Devtools", DevTools::update, DevTools::view)
        .subscription(DevTools::subscription)
        .run()
}

pub struct DevTools {
    screen: Screen,
    launcher: AppLauncher,
    is_modal_open: bool,
}

enum Screen {
    UUIDGenerator(UUIDGenerator),
}

impl Default for DevTools {
    fn default() -> Self {
        Self {
            launcher: AppLauncher::new(),
            screen: Screen::UUIDGenerator(UUIDGenerator::new()),
            is_modal_open: false,
        }
    }
}

impl DevTools {
    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }

    fn update(&mut self, event: Message) -> Task<Message> {
        match event {
            Message::UUIDGenerator(message) => {
                if let Screen::UUIDGenerator(uuid_generator) = &mut self.screen {
                    uuid_generator.update(message);
                    Task::none()
                } else {
                    Task::none()
                }
            }
            Message::AppLauncher(message) => {
                self.launcher.update(message);
                Task::none()
            }
            Message::HideModal => {
                self.is_modal_open = false;
                Task::none()
            }
            Message::Event(event) => match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Space),
                    ..
                }) => {
                    self.is_modal_open ^= true;
                    widget::focus_next()
                }
                _ => Task::none(),
            },
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match &self.screen {
            Screen::UUIDGenerator(uuid_generator) => uuid_generator.view(),
        };
        let launcher_content = self.launcher.view().map(Message::AppLauncher);
        if self.is_modal_open {
            modal(
                content.map(Message::UUIDGenerator),
                launcher_content,
                Message::HideModal,
            )
        } else {
            content.map(Message::UUIDGenerator)
        }
    }
}
