use std::collections::HashMap;

use iced::{
    highlighter,
    widget::{
        button, column, container, horizontal_space, row, text, text_editor, text_editor::Action,
    },
    Alignment::Center,
    Element,
    Length::Fill,
};

pub struct JsonBeautifier {
    input_content: text_editor::Content,
    output_content: text_editor::Content,
    error_text: Option<String>,
    theme: highlighter::Theme,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputActionPerformed(text_editor::Action),
    OutputActionPerformed(text_editor::Action),
    Beautified,
}

impl JsonBeautifier {
    pub fn new() -> Self {
        Self {
            input_content: text_editor::Content::new(),
            output_content: text_editor::Content::new(),
            error_text: None,
            theme: highlighter::Theme::InspiredGitHub,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let header = container(
            row![horizontal_space(), "Json Beautifier", horizontal_space(),]
                .padding(10)
                .align_y(Center),
        )
        .style(container::rounded_box);

        let controls = row![
            button("Beautify!").on_press(Message::Beautified),
            horizontal_space()
        ]
        .padding(20);

        let editor = container(
            text_editor(&self.input_content)
                .on_action(Message::InputActionPerformed)
                .highlight("js", self.theme)
                .height(Fill),
        )
        .height(Fill);

        let status = row![
            horizontal_space(),
            text({
                let (line, column) = self.input_content.cursor_position();

                format!("{}:{}", line + 1, column + 1)
            })
        ]
        .spacing(10);

        let output = container(
            text_editor(&self.output_content)
                .on_action(Message::OutputActionPerformed)
                .highlight("js", self.theme)
                .height(Fill),
        )
        .height(Fill);

        let json_rows = row![
            column![row![text("Input"), horizontal_space()], editor, status].padding(10),
            column![row![text("Output"), horizontal_space()], output].padding(10),
        ]
        .padding(20);

        let mut all_content = column![header, controls, json_rows];
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
                self.input_content.perform(action);
            }
            Message::OutputActionPerformed(action) => match action {
                Action::SelectAll => {
                    self.output_content.perform(action);
                }
                _ => (),
            },
            Message::Beautified => {
                let binding = self.input_content.text().to_owned();
                let text_content = binding.as_str();
                match serde_json::from_str::<HashMap<String, serde_json::Value>>(text_content) {
                    Ok(serialized_json) => match serde_json::to_string_pretty(&serialized_json) {
                        Ok(formatted_json) => {
                            self.error_text = None;
                            self.output_content = text_editor::Content::with_text(&formatted_json)
                        }
                        Err(e) => self.error_text = Some(e.to_string()),
                    },
                    Err(e) => self.error_text = Some(e.to_string()),
                }
            }
        }
    }
}
