use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub theme: ThemeConfig,
    pub widgets: Vec<WidgetConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub colors: ColorScheme,
    pub font: FontConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColorScheme {
    pub primary: String,   // Black
    pub secondary: String, // Gold
    pub accent: String,    // Fuchsia
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FontConfig {
    pub main_font: String,
    pub font_size: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WidgetConfig {
    pub widget_type: String,
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub settings: toml::Value,
}

pub fn get_config_dir() -> Option<PathBuf> {
    ProjectDirs::from("com", "VeridianZenith", "Elune")
        .map(|proj_dirs| proj_dirs.config_dir().to_path_buf())
}
