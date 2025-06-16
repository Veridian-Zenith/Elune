use crate::widgets::{Widget, WidgetMessage};
use iced::{
    Application, Command, Element, Settings, Theme,
    widget::{Text, container},
};
use std::collections::HashMap;

mod config;
mod layout;
mod theme;
mod widgets;

pub fn main() -> iced::Result {
    Elune::run(Settings {
        window: iced::window::Settings {
            transparent: true,
            decorations: false,
            ..Default::default()
        },
        ..Default::default()
    })
}

struct Elune {
    theme: Theme,
    widgets: HashMap<String, Box<dyn Widget>>,
    edit_mode: bool,
}

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(Theme),
    WidgetMessage(String, WidgetMessage),
    ToggleEditMode,
}

impl Application for Elune {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut widgets = HashMap::new();

        // Add default clock widget
        let clock = Box::new(widgets::clock::DigitalClock::new("clock-1".to_string()));
        widgets.insert("clock-1".to_string(), clock);

        (
            Self {
                theme: Theme::Dark,
                widgets,
                edit_mode: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Elune")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = theme;
                Command::none()
            }
            Message::WidgetMessage(id, msg) => {
                if let Some(widget) = self.widgets.get_mut(&id) {
                    widget.update(msg);
                }
                Command::none()
            }
            Message::ToggleEditMode => {
                self.edit_mode = !self.edit_mode;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        container(Text::new("Elune Widget System"))
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
