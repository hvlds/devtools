use iced::{
    widget::{combo_box, container},
    Element,
};

use crate::utils::Application;

impl Default for AppLauncher {
    fn default() -> Self {
        AppLauncher::new()
    }
}

pub struct AppLauncher {
    applications: combo_box::State<Application>,
    selected_application: Option<Application>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Selected(Application),
    Hovered(Application),
}

impl AppLauncher {
    pub fn new() -> Self {
        let state = combo_box::State::new(Application::ALL.to_vec());
        Self {
            selected_application: None,
            applications: state,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let combo_box = combo_box(
            &self.applications,
            "Type an Application...",
            self.selected_application.as_ref(),
            Message::Selected,
        )
        .on_option_hovered(Message::Hovered)
        .width(250);

        container(combo_box.padding(10))
            .style(container::rounded_box)
            .into()
    }

    pub fn update(&mut self, message: Message) -> Option<Application> {
        match message {
            Message::Hovered(application) => {
                self.selected_application = Some(application);
                None
            }
            Message::Selected(application) => {
                self.selected_application = Some(application);
                Some(application)
            }
        }
    }
}
