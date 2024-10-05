use iced::{
    widget::{column, container, horizontal_space, row, text},
    Alignment::Center,
    Element,
    Length::Fill,
};

pub struct JsonBeautifier {}

#[derive(Debug, Clone)]
pub enum Message {
    Test,
}

impl JsonBeautifier {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> Element<Message> {
        let header = container(
            row![horizontal_space(), "Json Beautifier", horizontal_space(),]
                .padding(10)
                .align_y(Center),
        )
        .style(container::rounded_box);

        let content = container(text("TODO")).height(Fill);

        column![header, content].into()
    }
}
