use super::super::{Widget, WidgetMessage};
use crate::theme::Theme;
use iced::{Element, widget::*};
use mpris::{Player, PlayerFinder};

pub struct MediaControls {
    id: String,
    position: (f32, f32),
    size: (f32, f32),
    pinned: bool,
    current_player: Option<Player>,
}

impl MediaControls {
    pub fn new(id: String) -> Self {
        Self {
            id,
            position: (0.0, 0.0),
            size: (300.0, 100.0),
            pinned: false,
            current_player: None,
        }
    }

    pub fn update_player(&mut self) {
        if let Ok(finder) = PlayerFinder::new() {
            self.current_player = finder.find_active().ok();
        }
    }
}

impl Widget for MediaControls {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn view(&self, theme: &Theme) -> Element<WidgetMessage> {
        let content = if let Some(player) = &self.current_player {
            let metadata = player.get_metadata().unwrap_or_default();
            Column::new()
                .push(Text::new(metadata.title().unwrap_or("Unknown")))
                .push(Text::new(
                    metadata
                        .artists()
                        .unwrap_or(&vec!["Unknown".to_string()])
                        .join(", "),
                ))
                .spacing(5)
        } else {
            Column::new().push(Text::new("No media playing"))
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }

    // ... rest of implementation
}
