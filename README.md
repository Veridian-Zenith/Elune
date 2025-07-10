## ⚖️ Legal Disclaimer

**Veridian Zenith** is a digital label and project organization operated by **Jeremy Matlock**, also known as **Dae Euhwa**.
All works published under this name are the intellectual property of Jeremy Matlock unless otherwise specified.

---

## 🌙 Project Overview: *Elune*

**Elune** is a customizable desktop widget system written in **Rust**, designed to bring live, interactive, and theme-aware UI components to Linux desktops.

Inspired by *Rainmeter* (Windows), Elune integrates seamlessly with **Cosmic DE** and other **Wayland-based environments**, delivering elegant widgets such as clocks, system monitors, weather panels, and user-defined modules.

Built on the [`iced`](https://github.com/iced-rs/iced) GUI library, Elune offers GPU-accelerated performance with a reactive, declarative interface. Its modular design empowers users to tailor their desktop experience with native-feeling, minimalist widgets.

---

## ✨ Key Features

* **Modern Rust Architecture**: Built entirely in Rust for safety, performance, and concurrency.
* **Reactive GUI**: Powered by `iced` for smooth and responsive UI behavior.
* **Widget Engine** with support for:

  * Digital & analog clocks
  * CPU, RAM, disk, and network monitors
  * Weather integration (e.g., OpenWeatherMap)
  * Media controls (via MPRIS)
  * Custom shell/script-driven widgets
* **Theming Support**:

  * Cosmic DE-style theming and transparency
  * Default: **Black, gold, and fuchsia** with *Gamja Flower* font
  * Fully customizable: fonts, colors, transparency, spacing
* **Dynamic Layout**:

  * Right-click → Edit mode
  * Drag-and-drop positioning
  * Resizable, pinnable, and optionally snap-to-grid widgets
* **Configuration System**:

  * Uses `~/.config/Elune`
  * Auto-generates defaults on first run
* **Autostart Options**:

  * `.desktop` file support
  * Optional `systemd --user` service generation
* **Extensibility**:

  * Configurable with **TOML** or **YAML**
  * Plugin system *(planned)*
  * Future support for **Lua** or **WASM scripting**

---

## 🔌 Planned Integrations

* Desktop APIs (portals, system status, etc.)
* Weather APIs (OpenWeatherMap, AccuWeather)
* Media APIs (MPRIS)
* External Widget Repositories *(similar to Chrome Web Store)*

---

## 🧩 Use Cases

* Displaying system vitals with visual flair
* Creating overlays for stream setups
* Lightweight dashboards for info-at-a-glance
* Enhancing desktop aesthetic (cyberpunk, vintage, minimal, etc.)

---

## 👥 Target Audience

Linux users (especially Wayland + Cosmic DE), Rust devs, ricing enthusiasts, streamers, and anyone wanting dynamic, stylish overlays on their desktop.

---

## 📦 Status

**In active development**
Alpha release expected: **Q3 2025**

---

## 📝 License

**Dual-licensed** under the **GNU AGPLv3** for community use and the **Veridian Commercial License (VCL 1.0)** for proprietary applications.

See the [LICENSE](LICENSE) file for full details.

---

## 🌐 Repository

🔗 [Elune by Veridian Zenith — GitHub](https://github.com/Veridian-Zenith/Elune)

---
