use std::fs;

use iced::widget::text_editor::Action;
use iced::widget::{
    button, checkbox, column, container, horizontal_space, pick_list, row, scrollable, text,
    text_editor, text_input, Space,
};
use iced::{Element, Length, Task};
use rfd::AsyncFileDialog;
use uuid::Uuid;

pub const NAME: &str = "UUID Generator";

impl Default for UuidGenerator {
    fn default() -> Self {
        UuidGenerator::new()
    }
}

pub struct UuidGenerator {
    selected_version: Option<Version>,
    output: text_editor::Content,
    tool_name: String,
    raw_amount: String,
    parsed_amount: u32,
    parsing_error: String,
    selected_quotes: Option<Quotes>,
    is_separated_by_comma: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Generated,
    Selected(Version),
    OutputActionPerformed(text_editor::Action),
    AmountChanged(String),
    QuotesSelected(Quotes),
    CommaSelected(bool),
    UuidList(String),
    SaveToFileClicked,
    FilePicked(Option<bool>),
}

impl UuidGenerator {
    pub fn new() -> Self {
        Self {
            selected_version: Some(Version::V4),
            output: text_editor::Content::with_text(""),
            tool_name: NAME.to_string(),
            raw_amount: String::from("1"),
            parsed_amount: 1,
            parsing_error: String::new(),
            selected_quotes: Some(Quotes::NoQuotes),
            is_separated_by_comma: false,
        }
    }

    pub fn title(&self) -> String {
        self.tool_name.clone()
    }

    fn can_display(&self) -> bool {
        if self.parsing_error.as_str() == "" && self.parsed_amount <= 1000 {
            true
        } else {
            false
        }
    }

    fn can_export(&self) -> bool {
        if self.parsing_error.as_str() == "" {
            true
        } else {
            false
        }
    }

    pub fn view(&self) -> Element<Message> {
        let configuration = column![
            "Configuration",
            row![
                "Version: ",
                pick_list(&Version::ALL[..], self.selected_version, Message::Selected,)
                    .placeholder("Choose a version")
            ],
            row![
                "Amount: ",
                text_input("Amount", self.raw_amount.as_str()).on_input(Message::AmountChanged),
                text(self.parsing_error.as_str()),
                horizontal_space()
            ],
            row![
                "Wrap with quotes",
                pick_list(
                    &Quotes::ALL[..],
                    self.selected_quotes,
                    Message::QuotesSelected
                )
            ],
            row![checkbox("Separate by comma", self.is_separated_by_comma)
                .on_toggle(Message::CommaSelected)],
            row![
                button("Generate UUID").on_press_maybe(match self.can_display() {
                    true => Some(Message::Generated),
                    false => None,
                }),
                Space::with_width(10),
                button("Save to file").on_press_maybe(match self.can_export() {
                    true => Some(Message::SaveToFileClicked),
                    false => None,
                })
            ],
        ]
        .padding(10)
        .spacing(10);

        let result = column![
            "Result",
            scrollable(text_editor(&self.output).on_action(Message::OutputActionPerformed))
        ]
        .padding(10)
        .spacing(10);

        let content = container(column![configuration, Space::with_height(20), result])
            .padding(10)
            .height(Length::Fill);

        content.into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Generated => {
                let parsed_amount = self.parsed_amount.clone();
                let selected_version = self.selected_version.clone();
                let selected_quotes = self.selected_quotes.clone();
                let is_separated_by_comma = self.is_separated_by_comma.clone();
                Task::perform(
                    async move {
                        generate_result(
                            parsed_amount,
                            selected_version,
                            selected_quotes,
                            is_separated_by_comma,
                        )
                    },
                    |res| Message::UuidList(res),
                )
            }
            Message::Selected(version) => {
                self.selected_version = Some(version);
                Task::none()
            }
            Message::OutputActionPerformed(action) => {
                match action {
                    Action::SelectAll | Action::SelectLine | Action::SelectWord => {
                        self.output.perform(action)
                    }

                    Action::Select(motion) => self.output.perform(Action::Select(motion)),
                    Action::Click(point) => self.output.perform(Action::Click(point)),
                    Action::Drag(point) => self.output.perform(Action::Drag(point)),
                    Action::Move(motion) => self.output.perform(Action::Move(motion)),
                    _ => (),
                };
                Task::none()
            }
            Message::AmountChanged(value) => {
                self.raw_amount = value.clone();
                match value.parse::<u32>() {
                    Ok(v) => {
                        if v > 0 {
                            self.parsed_amount = v;
                            self.parsing_error = String::new();
                        } else {
                            self.parsing_error = format!("Amount must be at least 1 '{}'", value);
                        }
                    }
                    Err(_) => {
                        self.parsing_error = format!("Cannot parse '{}'", value);
                    }
                };
                Task::none()
            }
            Message::QuotesSelected(quotes) => {
                self.selected_quotes = Some(quotes);
                Task::none()
            }
            Message::CommaSelected(value) => {
                self.is_separated_by_comma = value;
                Task::none()
            }
            Message::UuidList(result) => {
                self.output = text_editor::Content::with_text(result.as_str());
                Task::none()
            }
            Message::SaveToFileClicked => {
                let parsed_amount = self.parsed_amount.clone();
                let selected_version = self.selected_version.clone();
                let selected_quotes = self.selected_quotes.clone();
                let is_separated_by_comma = self.is_separated_by_comma.clone();
                Task::perform(
                    generate_and_save_to_file(
                        parsed_amount,
                        selected_version,
                        selected_quotes,
                        is_separated_by_comma,
                    ),
                    |res| Message::FilePicked(res),
                )
            }
            Message::FilePicked(file_handle) => match file_handle {
                Some(_) => {
                    println!("File created!");
                    Task::none()
                }
                None => Task::none(),
            },
        }
    }
}

fn generate_result(
    parsed_amount: u32,
    selected_version: Option<Version>,
    selected_quotes: Option<Quotes>,
    is_separated_by_comma: bool,
) -> String {
    (0..parsed_amount)
        .into_iter()
        .map(|_| match selected_version {
            Some(version) => match version {
                Version::V4 => Uuid::new_v4().to_string(),
                Version::V7 => Uuid::now_v7().to_string(),
            },
            None => Uuid::new_v4().to_string(),
        })
        .map(|v| wrap_with_quotes(v, selected_quotes))
        .reduce(|cur: String, nxt: String| match is_separated_by_comma {
            true => cur + ",\n" + &nxt,
            false => cur + "\n" + &nxt,
        })
        .unwrap()
}

async fn generate_and_save_to_file(
    parsed_amount: u32,
    selected_version: Option<Version>,
    selected_quotes: Option<Quotes>,
    is_separated_by_comma: bool,
) -> Option<bool> {
    let file = AsyncFileDialog::new().set_directory("/").save_file().await;
    match file {
        Some(f) => {
            let output = generate_result(
                parsed_amount,
                selected_version,
                selected_quotes,
                is_separated_by_comma,
            );
            match fs::write(f.path().as_os_str(), output) {
                Ok(_) => Some(true),
                Err(_) => None,
            }
        }
        None => None,
    }
}

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
pub enum Quotes {
    #[default]
    NoQuotes,
    SingleQuotes,
    DoubleQuotes,
}

impl Quotes {
    const ALL: [Quotes; 3] = [Quotes::NoQuotes, Quotes::SingleQuotes, Quotes::DoubleQuotes];
}

impl std::fmt::Display for Quotes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Quotes::NoQuotes => "No Quotes",
                Quotes::SingleQuotes => "Single Quotes",
                Quotes::DoubleQuotes => "Double Quotes",
            }
        )
    }
}

fn wrap_with_quotes(text: String, quote_type: Option<Quotes>) -> String {
    match quote_type {
        Some(v) => match v {
            Quotes::NoQuotes => text,
            Quotes::DoubleQuotes => format!("\"{}\"", text),
            Quotes::SingleQuotes => format!("'{}'", text),
        },
        None => text,
    }
}
