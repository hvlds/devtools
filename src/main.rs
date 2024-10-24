use iced::event::{self};
use iced::keyboard::{self};
use iced::widget::{self, column, container, horizontal_space, row, text};
use iced::Alignment::Center;
use iced::{Element, Event, Subscription, Task, Theme};
use launcher::Launcher;
use modal::modal;
use tools::{Base64Converter, JsonBeautifier, RandomDataGenerator, UuidGenerator};

use scale_factor::ScaleFactor;
use utils::{Message, Tool};

mod launcher;
mod modal;
mod scale_factor;
mod tools;
mod utils;

pub fn main() -> iced::Result {
    iced::application("DevTools", DevTools::update, DevTools::view)
        .theme(DevTools::theme)
        .subscription(DevTools::subscription)
        .scale_factor(DevTools::get_scale_factor)
        .run()
}

pub struct DevTools {
    screen: Screen,
    launcher: Launcher,
    is_modal_open: bool,
    current_tool: Tool,
    scale_factor: ScaleFactor,
    theme: Theme,
}

enum Screen {
    UuidGenerator(UuidGenerator),
    JsonBeautifier(JsonBeautifier),
    Base64Converter(Base64Converter),
    RandomDataGenerator(RandomDataGenerator),
}

impl Default for DevTools {
    fn default() -> Self {
        Self {
            launcher: Launcher::new(),
            screen: Screen::Base64Converter(Base64Converter::new()),
            current_tool: Tool::Base64Converter,
            is_modal_open: false,
            scale_factor: ScaleFactor::default(),
            theme: Theme::Light,
        }
    }
}

impl DevTools {
    fn theme(&self) -> Theme {
        self.theme.clone()
    }

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
                    uuid_generator.update(message).map(Message::UuidGenerator)
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
            Message::Base64Converter(message) => {
                if let Screen::Base64Converter(base64_converter) = &mut self.screen {
                    base64_converter.update(message);
                    Task::none()
                } else {
                    Task::none()
                }
            }
            Message::Launcher(message) => {
                let selected_application = self.launcher.update(message);
                match selected_application {
                    Some(application) => {
                        self.is_modal_open = false;
                        self.launcher.reset();
                        if application != self.current_tool {
                            self.current_tool = application;
                            self.screen = match application {
                                Tool::JsonBeautifier => {
                                    Screen::JsonBeautifier(JsonBeautifier::new())
                                }
                                Tool::UuidGenerator => Screen::UuidGenerator(UuidGenerator::new()),
                                Tool::Base64Converter => {
                                    Screen::Base64Converter(Base64Converter::new())
                                }
                                Tool::RandomDataGenerator => {
                                    Screen::RandomDataGenerator(RandomDataGenerator::new())
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
        let (content, title) = match &self.screen {
            Screen::UuidGenerator(uuid_generator) => (
                uuid_generator.view().map(Message::UuidGenerator),
                uuid_generator.title(),
            ),
            Screen::JsonBeautifier(json_beautifier) => (
                json_beautifier.view().map(Message::JsonBeautifier),
                json_beautifier.title(),
            ),
            Screen::Base64Converter(base64_converter) => (
                base64_converter.view().map(Message::Base64Converter),
                base64_converter.title(),
            ),
            Screen::RandomDataGenerator(random_data_generator) => (
                random_data_generator
                    .view()
                    .map(Message::RandomDataGenerator),
                random_data_generator.title(),
            ),
        };

        let header: Element<Message> = container(
            row![text(title).size(20), horizontal_space(),]
                .padding(10)
                .align_y(Center),
        )
        .into();

        let content_with_header = column![header, content].into();

        let launcher_content = self.launcher.view().map(Message::Launcher);
        if self.is_modal_open {
            modal(content_with_header, launcher_content, Message::HideModal)
        } else {
            content_with_header
        }
    }
}
