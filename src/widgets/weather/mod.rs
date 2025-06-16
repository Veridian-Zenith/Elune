use super::{Widget, WidgetMessage};
use crate::theme::Theme;
use iced::{Element, widget::*};

mod openweather;
pub use openweather::OpenWeatherWidget;

#[derive(Debug, Clone)]
pub enum WeatherMessage {
    UpdateWeather(WeatherData),
    Error(String),
}

#[derive(Debug, Clone)]
pub struct WeatherData {
    pub temperature: f32,
    pub condition: String,
    pub icon: String,
}
