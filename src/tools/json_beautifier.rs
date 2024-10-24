use std::collections::HashMap;

use iced::{
    highlighter,
    widget::{
        button, column, container, horizontal_space, row, scrollable, slider, text,
        text_editor::{self, Action},
        Space,
    },
    Element,
    Length::Fill,
};

pub const NAME: &str = "JSON Beautifier";

pub struct JsonBeautifier {
    input_content: text_editor::Content,
    output_content: text_editor::Content,
    error_text: Option<String>,
    theme: highlighter::Theme,
    indentation: u16,
    tool_name: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputActionPerformed(text_editor::Action),
    OutputActionPerformed(text_editor::Action),
    IndentationChanged(u16),
}

impl JsonBeautifier {
    pub fn new() -> Self {
        Self {
            input_content: text_editor::Content::with_text("{}"),
            output_content: text_editor::Content::with_text("{}"),
            error_text: None,
            theme: highlighter::Theme::InspiredGitHub,
            indentation: 4,
            tool_name: NAME.to_string(),
        }
    }

    pub fn title(&self) -> String {
        self.tool_name.clone()
    }

    pub fn view(&self) -> Element<Message> {
        let controls = row![
            slider(0..=8, self.indentation, Message::IndentationChanged),
            horizontal_space()
        ]
        .padding(20);

        let editor = container(scrollable(
            iced::widget::text_editor(&self.input_content)
                .on_action(Message::InputActionPerformed)
                .highlight("js", self.theme),
        ))
        .height(Fill);

        let status = row![
            horizontal_space(),
            text({
                let (line, column) = self.input_content.cursor_position();

                format!("{}:{}", line + 1, column + 1)
            })
        ]
        .spacing(10);

        let output = container(scrollable(
            iced::widget::text_editor(&self.output_content)
                .on_action(Message::OutputActionPerformed)
                .highlight("js", self.theme),
        ))
        .height(Fill);

        let json_rows = row![
            column![
                row![text("Input"), horizontal_space()].height(50),
                editor,
                status
            ]
            .padding(10),
            column![
                row![
                    text("Output"),
                    horizontal_space(),
                    button("Copy to Clipboard"),
                    Space::with_width(10),
                    button("Save to file")
                ]
                .height(50),
                output
            ]
            .padding(10),
        ]
        .padding(20);

        let mut all_content = column![controls, json_rows];
        match &self.error_text {
            Some(v) => {
                all_content =
                    all_content.push(row![text(v.to_owned()), horizontal_space()].padding(20));
            }
            None => (),
        }

        all_content.into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::InputActionPerformed(action) => {
                let old_text = self.input_content.text().to_owned();

                self.input_content.perform(action);

                let new_text = self.input_content.text().to_owned();
                if old_text != new_text {
                    let text_content = new_text.as_str();
                    match serde_json::from_str::<HashMap<String, serde_json::Value>>(text_content) {
                        Ok(serialized_json) => match serde_json::to_string_pretty(&serialized_json)
                        {
                            Ok(formatted_json) => {
                                self.error_text = None;
                                self.output_content =
                                    text_editor::Content::with_text(&formatted_json)
                            }
                            Err(e) => self.error_text = Some(e.to_string()),
                        },
                        Err(e) => self.error_text = Some(e.to_string()),
                    }
                }
            }
            Message::OutputActionPerformed(action) => match action {
                Action::SelectAll | Action::SelectLine | Action::SelectWord => {
                    self.output_content.perform(action);
                }
                _ => (),
            },
            Message::IndentationChanged(indentation) => self.indentation = indentation,
        }
    }
}
