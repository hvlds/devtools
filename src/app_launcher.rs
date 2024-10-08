use std::str::FromStr;

use iced::{
    border,
    widget::{button, column, container, keyed_column, text_input},
    Color, Element, Length,
};
use nucleo_matcher::{
    pattern::{CaseMatching, Normalization, Pattern},
    Config, Matcher,
};

use crate::utils::Application;

impl Default for AppLauncher {
    fn default() -> Self {
        AppLauncher::new()
    }
}

pub struct AppLauncher {
    search_text: String,
    search_matches: Vec<String>,
    matcher: Matcher,
}

#[derive(Debug, Clone)]
pub enum Message {
    Search(String),
    SearchSubmitted,
    SearchClicked(String),
}

impl AppLauncher {
    pub fn new() -> Self {
        Self {
            search_text: String::new(),
            matcher: Matcher::new(Config::DEFAULT),
            search_matches: vec![],
        }
    }

    pub fn view(&self) -> Element<Message> {
        let input_app = text_input("Search an app...", &self.search_text)
            .on_input(Message::Search)
            .on_submit(Message::SearchSubmitted)
            .id("app-launcher-text-input");

        let results = container(keyed_column(self.search_matches.iter().enumerate().map(
            |(i, search_match)| {
                (
                    i,
                    container(
                        button(search_match.as_str())
                            .on_press(Message::SearchClicked(search_match.to_string()))
                            .width(Length::Fill),
                    )
                    .padding(2)
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

    pub fn update(&mut self, message: Message) -> Option<Application> {
        match message {
            Message::Search(application) => {
                self.search_text = application;
                let binding = self.search_text.to_owned();

                self.search_matches = Pattern::parse(
                    &binding.as_str(),
                    CaseMatching::Ignore,
                    Normalization::Smart,
                )
                .match_list(Application::ALL, &mut self.matcher)
                .into_iter()
                .map(|m| m.0.to_string())
                .collect();

                None
            }
            Message::SearchSubmitted => {
                if self.search_matches.len() >= 1 {
                    let best_match = self.search_matches.get(0).unwrap().as_str();
                    match Application::from_str(best_match) {
                        Ok(v) => Some(v),
                        Err(_) => None,
                    }
                } else {
                    None
                }
            }
            Message::SearchClicked(search_match) => {
                match Application::from_str(search_match.as_str()) {
                    Ok(v) => Some(v),
                    Err(_) => None,
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.search_matches = vec![];
        self.search_text = String::new();
    }
}
