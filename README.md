> ⚡ Built with the help of AI — fully open source and cross-platform.

# ADB Studio
![Build](https://github.com/aeke/adb-studio/actions/workflows/release.yml/badge.svg)
![Stars](https://img.shields.io/github/stars/aeke/adb-studio?style=social)
![Downloads](https://img.shields.io/github/downloads/aeke/adb-studio/total?label=downloads)
[![GitHub release](https://img.shields.io/github/v/release/aeke/adb-studio?color=brightgreen&label=version)](https://github.com/aeke/adb-studio/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Contributions welcome](https://img.shields.io/badge/contributions-welcome-orange.svg)](CONTRIBUTING.md)

---

Modern, cross-platform **Android Debug Bridge (ADB)** GUI application built with **Rust** and **Dioxus**.  
Clean, fast, and intuitive — for Android developers who want a desktop companion.

---

## 📸 Screenshots

![ADB Studio - Device list](assets/s1.jpeg)
![ADB Studio - App Manager](assets/s2.jpeg)

---

## 🚀 Features

- **Device Management** – Auto-detect and manage multiple Android devices  
- **App Manager** – Install/uninstall APKs with progress tracking  
- **File Operations** – Push/pull files between device and computer  
- **Terminal** – Execute ADB shell commands interactively  
- **Media** – Take screenshots and record screen  
- **Log Viewer** – Real-time logcat monitoring  
- **Dark/Light Theme** – Switch between system or manual themes  
- **Custom Device Selector** – Dropdown for easy device switching  

---

## 🧠 Tech Stack

- 🦀 **Rust** – Core logic and performance  
- ⚛️ **Dioxus** – Reactive UI framework  
- 📱 **ADB** – Android Debug Bridge integration  
- 🎨 **Font Awesome** – Vector icons and UI polish  

---

## ⚙️ Requirements

- Rust **1.70+**  
- ADB installed and available in `PATH`  
- Android device with **USB debugging** enabled  

---

## 🏗️ Installation

```bash
# Clone the repository
git clone https://github.com/aeke/adb-studio.git
cd adb-studio

# Build and run
cargo run
```

---

## 💡 Usage

1. **Connect Device** – via USB or WiFi  
2. **Select Device** – from the dropdown at the top  
3. **Navigate** – using the sidebar  
4. **Install APK** – go to *Apps → Install APK → Choose file*  
5. **Uninstall App** – select an app → *Uninstall*  
6. **Pull/Push Files** – manage file transfers seamlessly  
7. **Logs & Terminal** – monitor or interact with your device in real-time  

---

## ⚙️ Configuration

You can configure settings via the **Settings** page:
- Custom ADB path  
- Theme preference (Dark/Light)  
- Auto-refresh interval  

---

## 🧩 Project Structure

```
src/
├── main.rs           # Entry point with embedded CSS
├── app.rs            # Main app component with routing
├── adb.rs            # ADB command wrappers
├── app_manager.rs    # APK install/uninstall
├── fileops.rs        # File push/pull operations
├── terminal.rs       # Shell command execution
├── media.rs          # Screenshot/recording
├── log_viewer.rs     # Logcat viewer
├── settings.rs       # App settings
└── device.rs         # Device parsing
```

---

## 🧑‍💻 Development

### Recommended IDE
- [VS Code](https://code.visualstudio.com/)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [Dioxus extension](https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus)

### Build release version
```bash
cargo build --release
```

---

## 📦 Releases

Latest builds for macOS, Windows, and Linux are available here:  
👉 [Releases · aeke/adb-studio](https://github.com/aeke/adb-studio/releases)

---

## 🤝 Contributing

We welcome contributions!  
Please see the [CONTRIBUTING.md](CONTRIBUTING.md) guide before submitting a pull request.

---

## 🪪 License

This project is licensed under the [MIT License](LICENSE).

---

### 💬 Connect

- 💻 Repository: [https://github.com/aeke/adb-studio](https://github.com/aeke/adb-studio)
- 🐞 Issues: [Report a bug](https://github.com/aeke/adb-studio/issues)
- 💡 Feature requests: [Suggest an idea](https://github.com/aeke/adb-studio/issues/new?labels=enhancement)

---
