use iced::{
    widget::{center, container, mouse_area, opaque, stack},
    Color, Element,
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
            mouse_area(center(opaque(top_content)).style(|_theme| {
                container::Style {
                    background: Some(
                        Color {
                            a: 0.6,
                            ..Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            }))
            .on_press(on_press)
        )
    ]
    .into()
}
