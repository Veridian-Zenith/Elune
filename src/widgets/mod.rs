use crate::theme::Theme;
use iced::Element;

#[derive(Debug, Clone)]
pub enum WidgetMessage {
    Resize(f32, f32),
    Move(f32, f32),
    Pin(bool),
    Close,
    Custom(Box<dyn std::any::Any>),
}

pub trait Widget {
    fn id(&self) -> String;
    fn view(&self, theme: &Theme) -> Element<WidgetMessage>;
    fn update(&mut self, message: WidgetMessage);
    fn size(&self) -> (f32, f32);
    fn position(&self) -> (f32, f32);
    fn is_pinned(&self) -> bool;
}

pub mod clock;
pub mod media;
pub mod system;
pub mod weather;
