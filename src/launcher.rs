use std::str::FromStr;

use iced::{
    border,
    widget::{column, container, keyed_column, mouse_area, text, text_input},
    Color, Element, Length,
};
use nucleo_matcher::{
    pattern::{CaseMatching, Normalization, Pattern},
    Config, Matcher,
};

use crate::utils::Tool;

impl Default for Launcher {
    fn default() -> Self {
        Launcher::new()
    }
}

pub struct Launcher {
    search_text: String,
    search_matches: Vec<String>,
    matcher: Matcher,
    result_hovered: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Search(String),
    SearchSubmitted,
    SearchClicked(String),
    ResultEntered(String),
    ResultExited(String),
}

impl Launcher {
    pub fn new() -> Self {
        Self {
            search_text: String::new(),
            matcher: Matcher::new(Config::DEFAULT),
            search_matches: vec![],
            result_hovered: None,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let input_app = text_input("Search a tool...", &self.search_text)
            .on_input(Message::Search)
            .on_submit(Message::SearchSubmitted)
            .id("app-launcher-text-input");

        let results = container(keyed_column(self.search_matches.iter().enumerate().map(
            |(i, search_match)| {
                (
                    i,
                    mouse_area(
                        container(text(search_match.as_str()).width(Length::Fill))
                            .padding(2)
                            .style(|_theme| container::Style {
                                background: if self
                                    .result_hovered
                                    .as_ref()
                                    .is_some_and(|v| *v == search_match.to_string())
                                {
                                    Some(
                                        Color {
                                            a: 0.7,
                                            r: 1.0,
                                            g: 0.5,
                                            b: 0.0,
                                        }
                                        .into(),
                                    )
                                } else {
                                    Some(
                                        Color {
                                            a: 1.0,
                                            r: 1.0,
                                            g: 1.0,
                                            b: 1.0,
                                        }
                                        .into(),
                                    )
                                },
                                ..container::Style::default()
                            }),
                    )
                    .on_press(Message::SearchClicked(search_match.to_string()))
                    .on_enter(Message::ResultEntered(search_match.to_string()))
                    .on_exit(Message::ResultExited(search_match.to_string()))
                    .into(),
                )
            },
        )))
        .style(|_theme| container::Style {
            background: Some(Color::WHITE.into()),
            ..container::Style::default()
        });

        let content = column![input_app, results].padding(10);

        container(content)
            .center_x(500)
            .style(|_theme| container::Style {
                border: border::rounded(10),
                background: Some(
                    Color {
                        a: 0.7,
                        r: 1.0,
                        g: 0.5,
                        b: 0.0,
                    }
                    .into(),
                ),
                ..container::Style::default()
            })
            .into()
    }

    pub fn update(&mut self, message: Message) -> Option<Tool> {
        match message {
            Message::Search(application) => {
                self.search_text = application;
                let binding = self.search_text.to_owned();

                self.search_matches = Pattern::parse(
                    &binding.as_str(),
                    CaseMatching::Ignore,
                    Normalization::Smart,
                )
                .match_list(Tool::ALL, &mut self.matcher)
                .into_iter()
                .map(|m| m.0.to_string())
                .collect();

                None
            }
            Message::SearchSubmitted => {
                if self.search_matches.len() >= 1 {
                    let best_match = self.search_matches.get(0).unwrap().as_str();
                    match Tool::from_str(best_match) {
                        Ok(v) => Some(v),
                        Err(_) => None,
                    }
                } else {
                    None
                }
            }
            Message::SearchClicked(search_match) => match Tool::from_str(search_match.as_str()) {
                Ok(v) => Some(v),
                Err(_) => None,
            },
            Message::ResultEntered(search_match) => {
                self.result_hovered = Some(search_match);
                None
            }
            Message::ResultExited(search_match) => {
                if self
                    .result_hovered
                    .as_mut()
                    .is_some_and(|v| *v == search_match)
                {
                    self.result_hovered = None;
                }
                None
            }
        }
    }

    pub fn reset(&mut self) {
        self.search_matches = vec![];
        self.search_text = String::new();
    }
}
