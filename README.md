**Project: Elune**

**Description:**
Elune is a customizable desktop widget system, written in Rust, designed to bring live, interactive, and theme-aware UI components to Linux desktops. Inspired by Rainmeter on Windows, Elune integrates seamlessly with the Cosmic DE and other Wayland-based environments, offering lightweight yet rich visual widgets like clocks, system monitors, weather, and user-defined modules.

Elune is built using the `iced` GUI library, providing a reactive, declarative interface and smooth performance with GPU acceleration. Its modular system allows users to easily craft and extend their desktop experience with widgets that feel native, elegant, and minimal.

**Key Features:**

* **Modern Rust Architecture**: Fully written in Rust with safety, performance, and concurrency in mind.
* **Reactive GUI with Iced**: Built on top of the `iced` framework for responsive and beautiful UIs.
* **Widget Engine**: Support for a variety of widgets, such as:

  * Digital and analog clocks
  * CPU, RAM, disk, and network monitors
  * Weather integration (via APIs like OpenWeatherMap)
  * Media controls
  * Custom shell or script output widgets
* **Theming Support**: Elune supports theming and transparency with Cosmic DE in mind, ensuring it visually aligns with your desktop style.

  * Default theme: Black, gold, and fuchsia with Gamja Flower font and a custom app icon
  * User-selectable colors, fonts, transparencies, and spacing
* **Dynamic Layout**:

  * Right-click to enter edit mode
  * Drag-and-drop positioning
  * Resizable and pinnable widgets
  * Optional snap-to-grid layout
* **Configuration Directory:**

  * Uses `~/.config/Elune` for settings, themes, and widget definitions
  * Auto-creates default config files on first run
* **Startup Options:**

  * User can enable Elune via `.desktop` autostart entry
  * Optional `systemd --user` service generator for background launch
* **Extensibility:**

  * TOML/YAML config-driven widgets
  * Plugin architecture (planned)
  * Easy-to-extend architecture for future scripting (e.g., Lua or WASM)

**Planned Integrations:**

* Desktop APIs: integration with portals and system status providers
* Weather APIs: OpenWeatherMap, AccuWeather
* Media APIs: MPRIS
* External Extension Channels: Like Chrome Web Store (later)

**Use Cases:**

* Displaying system vitals elegantly on a desktop
* Creating information panels for stream overlays
* Embedding lightweight interactive dashboards on Linux
* Aesthetic customization for desktop setups (e.g., cyberpunk, minimal, vintage themes)

**Target Audience:**
Linux users (especially Wayland + Cosmic DE enthusiasts), Rust developers, ricing aficionados, streamers, and anyone wanting to add dynamic, stylish, and functional overlays to their workspace.

**Status:**
Active development. Alpha release planned Q3 2025.

**License:**
MIT

**Repository:**
[Elune by Veridian Zenith](https://github.com/Veridian-Zenith/Elune)
