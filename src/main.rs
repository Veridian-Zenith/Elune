use chrono::Local;
use iced::time;
use iced::{
    executor, mouse,
    widget::{container, Column, Container, Text},
    window,
    window::Settings as WindowSettings,
    Application, Color, Command, Element, Event, Point, Settings, Size, Subscription, Theme,
};
use sysinfo::System;
use tray_icon::{
    menu::MenuEvent,
    menu::{Menu, MenuId, MenuItem},
    TrayIcon, TrayIconBuilder,
};
use window_shadows::set_shadow;

#[derive(Debug, Clone)]
enum Message {
    Tick,
    WindowDragged(Point),
    Exit,
    ToggleVisibility,
}

struct Elune {
    time_str: String,
    ram_usage: String,
    sys: System,
    window_position: Point,
    visible: bool,
    tray_icon: Option<TrayIcon>,
}

impl Application for Elune {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        // Create tray menu
        let tray_menu = Menu::new();
        tray_menu
            .append(&MenuItem::new("Show/Hide", MenuId::new("toggle"), true))
            .unwrap();
        tray_menu
            .append(&MenuItem::new("Exit", MenuId::new("exit"), true))
            .unwrap();

        // Create tray icon
        let icon = include_bytes!("../assets/icon.png");
        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("Elune")
            .with_icon_from_buffer(icon)
            .build()
            .ok();

        (
            Self {
                time_str: String::new(),
                ram_usage: String::new(),
                sys: System::new(),
                window_position: Point::new(0.0, 0.0),
                visible: true,
                tray_icon: tray_icon,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Elune")
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::subscription::events().map(|event| match event {
            Event::Window(window::Event::Moved { x, y }) => {
                Message::WindowDragged(Point::new(x as f32, y as f32))
            }
            _ => Message::Tick,
        })
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick => {
                self.time_str = Local::now().format("%H:%M:%S").to_string();

                self.sys.refresh_memory();
                let total_mem = self.sys.total_memory();
                let used_mem = self.sys.used_memory();
                self.ram_usage = format!(
                    "RAM: {:.1} / {:.1} GB",
                    used_mem as f32 / 1024.0 / 1024.0,
                    total_mem as f32 / 1024.0 / 1024.0
                );
                Command::none()
            }
            Message::WindowDragged(position) => {
                self.window_position = position;
                Command::none()
            }
            Message::Exit => {
                std::process::exit(0);
            }
            Message::ToggleVisibility => {
                self.visible = !self.visible;
                Command::batch(vec![window::change_mode(if self.visible {
                    window::Mode::Visible
                } else {
                    window::Mode::Hidden
                })])
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = Column::new()
            .padding(20)
            .spacing(15)
            .align_items(iced::alignment::Alignment::Center)
            .push(
                Text::new(&self.time_str)
                    .size(40)
                    .style(iced::theme::Text::Color(Color::from_rgb(1.0, 0.84, 0.0))),
            )
            .push(
                Text::new(&self.ram_usage)
                    .size(25)
                    .style(iced::theme::Text::Color(Color::from_rgb(1.0, 0.84, 0.0))),
            );

        Container::new(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .style(iced::theme::Container::Custom(Box::new(
                TransparentBackground,
            )))
            .into()
    }
}

struct TransparentBackground;

impl container::StyleSheet for TransparentBackground {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            border: iced::Border {
                radius: 10.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            shadow: iced::Shadow::default(),
            text_color: None, // Add this field
        }
    }
}

fn main() -> iced::Result {
    // Set up menu event listener
    let menu_channel = MenuEvent::receiver();
    std::thread::spawn(move || {
        while let Ok(event) = menu_channel.recv() {
            match event.id.0.as_str() {
                "exit" => std::process::exit(0),
                "toggle" => {
                    // Handle toggle visibility
                }
                _ => {}
            }
        }
    });

    let mut window_settings = WindowSettings::default();
    window_settings.transparent = true;
    window_settings.decorations = false;
    window_settings.size = Size::new(300.0, 120.0);

    let app = Elune::run(Settings {
        window: window_settings,
        ..Default::default()
    })?;

    // Add window shadow
    if let Ok(window) = app.window() {
        let _ = set_shadow(&window, true);
    }

    Ok(())
}
