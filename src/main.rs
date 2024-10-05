use app_launcher::AppLauncher;
use iced::event::{self};
use iced::widget::{self};
use iced::{
    keyboard::{self, key},
    Element, Event, Subscription, Task,
};
use utils::Message;
use uuid_generator::UUIDGenerator;

mod app_launcher;
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
}

enum Screen {
    UUIDGenerator(UUIDGenerator),
}

impl Default for DevTools {
    fn default() -> Self {
        Self {
            launcher: AppLauncher::new(),
            screen: Screen::UUIDGenerator(UUIDGenerator::new()),
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

            Message::HideModal => {
                self.launcher.hide_modal();
                Task::none()
            }
            Message::Event(event) => match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Space),
                    ..
                }) => {
                    self.launcher.is_modal_open ^= true;
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
        if self.launcher.is_modal_open {
            self.launcher.modal(content.map(Message::UUIDGenerator))
        } else {
            content.map(Message::UUIDGenerator)
        }
    }
}
