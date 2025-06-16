# Elune

A COSMIC-compatible system monitoring widget for Linux, inspired by Rainmeter.

## Features
- Real-time system monitoring
- Plugin support
- Gold/Fuchsia theme with Gamja Flower font
- COSMIC DE integration
- Transparent interface with shadows
- Hot-loadable plugins

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/Elune.git
cd Elune

# Install dependencies
Gamja Flower font comes from Google Fonts

https://fonts.google.com/specimen/Gamja+Flower

# Build and install
cargo install --path .
```

## Configuration

Elune stores its configuration in `~/.config/Elune/`:
- `config.toml` - Main configuration
- `plugins/` - Directory for plugin files
- `themes/` - Custom theme files

On first run, Elune will prompt for startup preferences:
- COSMIC DE autostart
- Systemd user service

## Plugin Development

Create plugins without recompiling Elune:

1. Create a new plugin file:
```rust
use async_trait::async_trait;
use elune::plugin::Plugin;
use elune::theme::EluneTheme;

#[derive(Default)]
pub struct MyPlugin {
    state: String,
}

#[async_trait]
impl Plugin for MyPlugin {
    fn name(&self) -> &str { "My Plugin" }
    fn description(&self) -> &str { "Plugin description" }

    async fn update(&mut self) {
        // Update logic here
    }

    fn view(&self, theme: &EluneTheme) -> Element<Message> {
        // View layout here
    }
}

#[no_mangle]
pub fn _plugin_create() -> Box<dyn Plugin> {
    Box::new(MyPlugin::default())
}
```

2. Compile plugin:
```bash
rustc --crate-type cdylib my_plugin.rs -o my_plugin.so
```

3. Move to plugins directory:
```bash
mv my_plugin.so ~/.config/Elune/plugins/
```

4. Restart Elune

## Default Layout
```
elune/
├── assets/
│   ├── fonts/
│   │   └── GamjaFlower-Regular.ttf
│   └── images/
│       └── icon.png
├── src/
│   ├── theme/
│   ├── plugins/
│   ├── widgets/
│   └── main.rs
├── examples/
├── Cargo.toml
└── README.md
```

## Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push branch (`git push origin feature/amazing-feature`)
5. Open Pull Request

## License

MIT License - See [LICENSE](LICENSE) for details
