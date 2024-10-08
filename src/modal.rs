use iced::{
    padding::top,
    widget::{container, mouse_area, opaque, stack},
    Element, Length,
};

use crate::utils::Message;

pub fn modal<'a, 'b: 'a>(
    base: impl Into<Element<'a, Message>>,
    top_content: impl Into<Element<'b, Message>>,
    on_press: Message,
) -> Element<'a, Message> {
    stack![
        base.into(),
        opaque(
            mouse_area(
                container(opaque(top_content))
                    .padding(top(30))
                    .center_x(Length::Fill)
                    .height(Length::Fill)
            )
            .on_press(on_press.clone())
        ),
    ]
    .into()
}
