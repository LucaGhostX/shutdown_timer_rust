# Shutdown Timer (Rust Version) 2.4

**English** | [Italiano](README.it.md) | [Français](README.fr.md) | [Español](README.es.md) | [中文](README.zh.md)

[![Vibecoded](https://img.shields.io/badge/development-vibecoded-ff69b4?style=flat-square)](https://github.com/LucaGhostX/shutdown_timer_rust)
[![Rust](https://img.shields.io/badge/rust-2021-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)

> **Note**: This Rust port was completely **vibecoded** from the original project [**shutdown_timer**](https://github.com/LucaGhostX/shutdown_timer) (originally developed in Python/CustomTkinter).

A compact, modern, and responsive scheduled computer shutdown timer, featuring a native Dark/Light Mode GUI, multilingual support, cross-platform compatibility (**Windows**, **macOS**, **Linux**), and smart sleep prevention.

---

## Screenshots

![english (Personalizzato)](https://user-images.githubusercontent.com/64761832/210807640-a37306bb-3599-416f-9863-6f5b32117d58.png)
![italian (Personalizzato)](https://user-images.githubusercontent.com/64761832/210807707-e8c55222-fae6-4193-8b74-d32b1916ab25.png)
![french (Personalizzato)](https://user-images.githubusercontent.com/64761832/210807746-c0ef6038-158d-4b31-a13d-76437af47d99.png)
![spanish (Personalizzato)](https://user-images.githubusercontent.com/64761832/210807762-68fcffcb-4077-46dc-ba41-53b781ec15fa.png)
![chinese (Personalizzato)](https://user-images.githubusercontent.com/64761832/210807783-f678cf79-e9b6-4831-bd4c-13d4d835b829.png)
![pause_reset functions (Personalizzato)](https://user-images.githubusercontent.com/64761832/210807849-ce10fce9-3c73-4b4e-8164-3f08ab96c3da.png)

---

## Features

1. **Performance and Responsiveness**:
   - Compiled to native machine code with Rust and `eframe`/`egui`.
   - Standalone single executable, instant startup, minimal memory consumption (< 30 MB).
   - No external dependency on Python or heavy runtimes.

2. **Settings Panel and Themes**:
   - Gear icon at the bottom-right corner to open the quick settings modal.
   - **Theme Switch**: Quick toggle between:
     - **Dark** (*Dark Mode*)
     - **Light** (*Light Mode*)
     - **System** (*Auto* - follows OS preferences)
   - **Multilingual Support (i18n)**:
     - English (`en`)
     - Italian (`it`)
     - French (`fr`)
     - Spanish (`sp`)
     - Chinese (`ch`)
     - Automatic system language detection on first run.

3. **Hybrid Portable & Installed Mode**:
   - Zero footprint by default (no configuration file is created initially).
   - Once the user changes language or theme in settings, `config.json` is generated.
   - In **Portable** mode (or if `portable.txt` exists), `config.json` is saved in the application directory.
   - In **Installed** mode, `config.json` is saved in standard user directories:
     - **Windows**: `%APPDATA%\ShutdownTimer\config.json`
     - **macOS**: `~/Library/Application Support/ShutdownTimer/config.json`
     - **Linux**: `$XDG_CONFIG_HOME/ShutdownTimer/config.json` or `~/.config/ShutdownTimer/config.json`

4. **Native Cross-Platform Integration**:
   - **Windows**:
     - Automatic PC and display sleep prevention via `SetThreadExecutionState` and `PowerCreateRequest`.
     - System shutdown via `shutdown /s /t <sec>` and abort with `shutdown /a`.
     - Automatic Dark Mode detection from the Windows Registry.
   - **macOS**:
     - Sleep prevention using the native `caffeinate` utility.
     - Safe user-level shutdown via AppleScript (`osascript` with `System Events`), without requiring root or sudo permissions.
     - System language detection via `defaults read -g AppleLocale`.
   - **Linux**:
     - Standby inhibition via `systemd-inhibit`.
     - Scheduled shutdown with abort support via `systemctl poweroff` / `shutdown`.
     - Dark Mode detection from GNOME/freedesktop settings.

5. **Intuitive Timer Controls**:
   - 3 adjustable spinboxes for Hours (0-99), Minutes (0-59), and Seconds (0-59).
   - Quick `+` and `-` buttons with circular wrap-around and directly editable text field.
   - Large digital countdown display in `HH:MM:SS` format.
   - **Start**, **Pause**, **Resume**, and **Reset** buttons.

6. **Safety and Confirmation Modals**:
   - Error modal when attempting to start a timer with zero duration.
   - Confirmation modal on **Reset** to avoid accidental cancellations.
   - Window close interception `[X]` when a timer is running with an exit confirmation dialog.

---

## Build and Packaging with GitHub Actions

The repository includes a complete automated CI/CD workflow in [`.github/workflows/build-and-release.yml`](.github/workflows/build-and-release.yml) that automatically builds all release formats on every tag push (`v*`) or manually via *Run workflow*:

| Platform | Package Format | Generated Artifact |
| :--- | :--- | :--- |
| **Windows** | **Portable** (Standalone Zip) | `ShutdownTimer-Windows-Portable.zip` |
| **Windows** | **Installer** (Inno Setup) | `ShutdownTimer-Windows-Setup.exe` |
| **macOS** | **Apple Disk Image & Bundle** | `ShutdownTimer-macOS.dmg`, `.tar.gz` |
| **Linux** | **AppImage** | `ShutdownTimer-x86_64.AppImage` |
| **Linux** | **Flatpak Bundle** | `ShutdownTimer.flatpak` |

---

## Local Compilation

### Prerequisites
- **Rust & Cargo** (installable via [rustup.rs](https://rustup.rs))

### Release Build:
```bash
cargo build --release
```

### Run Tests:
```bash
cargo test
```

---

## Project Structure

```
shutdown_timer_rust/
├── .github/
│   └── workflows/
│       └── build-and-release.yml    # CI/CD GitHub Actions (Windows, macOS, AppImage, Flatpak)
├── assets/
│   ├── GFSNeohellenic-Bold.ttf      # Embedded font
│   ├── power.ico                    # Application icon (Windows)
│   ├── power_icon.png               # Window icon (cross-platform)
│   ├── settings_icon.png            # Gear icon for Dark Mode
│   ├── settings_icon_dark.png       # Gear icon for Light Mode
│   └── shutdown_timer_logo.png      # Integrated logo
├── installer/
│   ├── flatpak/
│   │   └── io.github.lucag.ShutdownTimer.yml  # Flatpak Manifest
│   ├── linux/
│   │   ├── io.github.lucag.ShutdownTimer.desktop
│   │   └── io.github.lucag.ShutdownTimer.metainfo.xml
│   ├── macos/
│   │   └── Info.plist               # macOS App Bundle configuration
│   └── windows/
│       └── setup.iss                # Inno Setup script for Windows Installer
├── src/
│   ├── main.rs                      # Entry point, window setup and font loading
│   ├── app.rs                       # Application logic, UI and settings modal
│   ├── spinbox.rs                   # Spinbox widget for hours, minutes, seconds
│   ├── config.rs                    # Hybrid configuration (Portable / System)
│   ├── i18n.rs                      # Translation management and system locale detection
│   ├── platform.rs                  # Native cross-platform abstraction (Windows, macOS, Linux)
│   ├── theme.rs                     # Palette management and Dark/Light/Auto theme
│   └── timer.rs                     # Countdown timer engine
├── Cargo.toml                       # Rust package manifest
├── LICENSE                          # MIT License
└── README.md
```

---

## License

This project is licensed under the **MIT** License. See the [`LICENSE`](LICENSE) file for the full terms and conditions.
