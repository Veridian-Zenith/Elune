use super::{Widget, WidgetMessage};
use crate::theme::Theme;
use chrono::{DateTime, Local};
use iced::{
    Element,
    widget::{Container, Text},
};

pub struct DigitalClock {
    id: String,
    position: (f32, f32),
    size: (f32, f32),
    pinned: bool,
    format: String,
}

impl DigitalClock {
    pub fn new(id: String) -> Self {
        Self {
            id,
            position: (0.0, 0.0),
            size: (200.0, 50.0),
            pinned: false,
            format: "%H:%M:%S".to_string(),
        }
    }
}

impl Widget for DigitalClock {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn view(&self, theme: &Theme) -> Element<WidgetMessage> {
        let time: DateTime<Local> = Local::now();
        let time_str = time.format(&self.format).to_string();

        Container::new(Text::new(time_str))
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn update(&mut self, message: WidgetMessage) {
        match message {
            WidgetMessage::Move(x, y) => self.position = (x, y),
            WidgetMessage::Resize(w, h) => self.size = (w, h),
            WidgetMessage::Pin(pinned) => self.pinned = pinned,
            _ => {}
        }
    }

    fn size(&self) -> (f32, f32) {
        self.size
    }

    fn position(&self) -> (f32, f32) {
        self.position
    }

    fn is_pinned(&self) -> bool {
        self.pinned
    }
}
