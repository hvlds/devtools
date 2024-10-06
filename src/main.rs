use app_launcher::AppLauncher;
use apps::{JsonBeautifier, UUIDGenerator};
use iced::event::{self};
use iced::widget::{self};
use iced::{
    keyboard::{self, key},
    Element, Event, Subscription, Task,
};
use modal::modal;

use scale_factor::ScaleFactor;
use utils::{Application, Message};

mod app_launcher;
mod apps;
mod modal;
mod scale_factor;
mod utils;

pub fn main() -> iced::Result {
    iced::application("DevTools", DevTools::update, DevTools::view)
        .subscription(DevTools::subscription)
        .scale_factor(DevTools::get_scale_factor)
        .run()
}

pub struct DevTools {
    screen: Screen,
    launcher: AppLauncher,
    is_modal_open: bool,
    current_application: Application,
    scale_factor: ScaleFactor,
}

enum Screen {
    UUIDGenerator(UUIDGenerator),
    JsonBeautifier(JsonBeautifier),
}

impl Default for DevTools {
    fn default() -> Self {
        Self {
            launcher: AppLauncher::new(),
            screen: Screen::UUIDGenerator(UUIDGenerator::new()),
            current_application: Application::UUIDGenerator,
            is_modal_open: false,
            scale_factor: ScaleFactor::default(),
        }
    }
}

impl DevTools {
    fn get_scale_factor(&self) -> f64 {
        self.scale_factor.into()
    }

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
            Message::JsonBeautifier(message) => {
                if let Screen::JsonBeautifier(json_beautifier) = &mut self.screen {
                    json_beautifier.update(message);
                    Task::none()
                } else {
                    Task::none()
                }
            }
            Message::AppLauncher(message) => {
                let selected_application = self.launcher.update(message);
                match selected_application {
                    Some(application) => {
                        self.is_modal_open = false;
                        self.launcher.reset();
                        if application != self.current_application {
                            self.current_application = application;
                            self.screen = match application {
                                Application::JsonBeautifier => {
                                    Screen::JsonBeautifier(JsonBeautifier::new())
                                }
                                Application::UUIDGenerator => {
                                    Screen::UUIDGenerator(UUIDGenerator::new())
                                }
                            };
                            widget::text_input::focus("app-launcher-text-input")
                        } else {
                            Task::none()
                        }
                    }
                    None => Task::none(),
                }
            }
            Message::HideModal => {
                self.is_modal_open = false;
                self.launcher.reset();
                Task::none()
            }
            Message::Event(event) => match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Escape),
                    ..
                }) => {
                    self.is_modal_open ^= true;
                    self.launcher.reset();
                    widget::text_input::focus("app-launcher-text-input")
                }
                _ => Task::none(),
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match &self.screen {
            Screen::UUIDGenerator(uuid_generator) => {
                uuid_generator.view().map(Message::UUIDGenerator)
            }
            Screen::JsonBeautifier(json_beautifier) => {
                json_beautifier.view().map(Message::JsonBeautifier)
            }
        };
        let launcher_content = self.launcher.view().map(Message::AppLauncher);
        if self.is_modal_open {
            modal(content, launcher_content, Message::HideModal)
        } else {
            content
        }
    }
}
