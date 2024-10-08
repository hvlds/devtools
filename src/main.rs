use app_launcher::AppLauncher;
use apps::{Base64Converter, JsonBeautifier, UuidGenerator};
use iced::event::{self};
use iced::keyboard::{self};
use iced::widget::{self};
use iced::{Element, Event, Subscription, Task};
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
    UuidGenerator(UuidGenerator),
    JsonBeautifier(JsonBeautifier),
    Base64Converter(Base64Converter),
}

impl Default for DevTools {
    fn default() -> Self {
        Self {
            launcher: AppLauncher::new(),
            screen: Screen::UuidGenerator(UuidGenerator::new()),
            current_application: Application::UuidGenerator,
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
            Message::UuidGenerator(message) => {
                if let Screen::UuidGenerator(uuid_generator) = &mut self.screen {
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
                                Application::UuidGenerator => {
                                    Screen::UuidGenerator(UuidGenerator::new())
                                }
                                Application::Base64Converter => {
                                    Screen::Base64Converter(Base64Converter::new())
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
                    key: keyboard::Key::Named(keyboard::key::Named::Escape),
                    ..
                }) => {
                    if self.is_modal_open {
                        self.is_modal_open = false;
                        self.launcher.reset();
                    }
                    Task::none()
                }
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(keyboard::key::Named::Space),
                    modifiers,
                    ..
                }) if modifiers.control() => {
                    self.is_modal_open ^= true;
                    self.launcher.reset();
                    widget::text_input::focus("app-launcher-text-input")
                }
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Character(c),
                    modifiers,
                    ..
                }) if c.as_str() == "+" && modifiers.control() => {
                    self.scale_factor.increment();
                    Task::none()
                }
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Character(c),
                    modifiers,
                    ..
                }) if c.as_str() == "-" && modifiers.control() => {
                    self.scale_factor.decrement();
                    Task::none()
                }
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Character(c),
                    modifiers,
                    ..
                }) if c.as_str() == "0" && modifiers.control() => {
                    self.scale_factor.to_default();
                    Task::none()
                }
                _ => Task::none(),
            },
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match &self.screen {
            Screen::UuidGenerator(uuid_generator) => {
                uuid_generator.view().map(Message::UuidGenerator)
            }
            Screen::JsonBeautifier(json_beautifier) => {
                json_beautifier.view().map(Message::JsonBeautifier)
            }
            Screen::Base64Converter(base64_converter) => {
                base64_converter.view().map(Message::Base64Converter)
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
