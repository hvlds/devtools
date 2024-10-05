use iced::{
    widget::{center, combo_box, container, mouse_area, opaque, stack, ComboBox},
    Color, Element,
};

use crate::utils::{Application, Message};

impl Default for AppLauncher {
    fn default() -> Self {
        AppLauncher::new()
    }
}

pub struct AppLauncher {
    pub is_modal_open: bool,
    applications: combo_box::State<Application>,
    selected_application: Option<Application>,
}

impl AppLauncher {
    pub fn new() -> Self {
        let state = combo_box::State::new(Application::ALL.to_vec());
        Self {
            is_modal_open: false,
            selected_application: None,
            applications: state,
        }
    }

    pub fn show_modal(&mut self) {
        self.is_modal_open = true;
    }

    pub fn hide_modal(&mut self) {
        self.is_modal_open = false;
    }

    pub fn modal<'a, 'b: 'a>(
        self: &'b Self,
        base: impl Into<Element<'a, Message>>,
    ) -> Element<'a, Message> {
        let combo_box: ComboBox<'_, Application, Message> = combo_box(
            &self.applications,
            "Type an Application...",
            self.selected_application.as_ref(),
            Message::ApplicationSelected,
        )
        .on_option_hovered(Message::ApplicationHovered)
        .width(250);

        let content = container(combo_box.padding(10)).style(container::rounded_box);

        stack![
            base.into(),
            opaque(
                mouse_area(center(opaque(content)).style(|_theme| {
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
                .on_press(Message::HideModal)
            )
        ]
        .into()
    }
}
