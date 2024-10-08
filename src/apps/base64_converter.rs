use iced::{
    widget::{column, container, horizontal_space, row, text, vertical_space},
    Alignment::Center,
    Element,
};

#[derive(Debug, Clone)]
pub enum Message {}

pub struct Base64Converter {}

impl Default for Base64Converter {
    fn default() -> Self {
        Base64Converter::new()
    }
}

impl Base64Converter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> Element<Message> {
        let header = container(
            row![text("Base64 Converter").size(30), horizontal_space(),]
                .padding(10)
                .align_y(Center),
        )
        .style(container::rounded_box);

        column![header, vertical_space()].into()
    }
}
