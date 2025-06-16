use super::{WeatherData, WeatherMessage};
use crate::{
    theme::Theme,
    widgets::{Widget, WidgetMessage},
};
use iced::{Element, widget::*};
use reqwest::Client;
use serde::Deserialize;

pub struct OpenWeatherWidget {
    id: String,
    position: (f32, f32),
    size: (f32, f32),
    pinned: bool,
    api_key: String,
    weather_data: Option<WeatherData>,
    client: Client,
}

impl OpenWeatherWidget {
    pub fn new(id: String, api_key: String) -> Self {
        Self {
            id,
            position: (0.0, 0.0),
            size: (200.0, 150.0),
            pinned: false,
            api_key,
            weather_data: None,
            client: Client::new(),
        }
    }

    pub async fn update_weather(&mut self, lat: f64, lon: f64) {
        let url = format!(
            "http://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units=metric",
            lat, lon, self.api_key
        );

        // Implementation for weather fetching
    }
}

impl Widget for OpenWeatherWidget {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn view(&self, theme: &Theme) -> Element<WidgetMessage> {
        let content = if let Some(weather) = &self.weather_data {
            Column::new()
                .push(Text::new(format!("{}°C", weather.temperature)))
                .push(Text::new(&weather.condition))
                .spacing(10)
        } else {
            Column::new().push(Text::new("Loading..."))
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    // ... rest of implementation
}
