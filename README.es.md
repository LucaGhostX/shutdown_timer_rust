# Shutdown Timer (Versión Rust) 2.4

[English](README.md) | [Italiano](README.it.md) | [Français](README.fr.md) | **Español** | [中文](README.zh.md)

[![Vibecoded](https://img.shields.io/badge/development-vibecoded-ff69b4?style=flat-square)](https://github.com/LucaGhostX/shutdown_timer_rust)
[![Rust](https://img.shields.io/badge/rust-2021-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)

> **Nota**: Esta adaptación en Rust ha sido completamente **vibecoded** a partir del proyecto original [**shutdown_timer**](https://github.com/LucaGhostX/shutdown_timer) (desarrollado originalmente en Python/CustomTkinter).

Un temporizador de apagado programado para ordenador compacto, moderno y reactivo, con interfaz gráfica nativa en Modo Oscuro/Claro, soporte multilingüe, compatibilidad multiplataforma (**Windows**, **macOS**, **Linux**) y prevención inteligente de suspensión.

---

## Capturas de pantalla

![english (Personalizzato)](https://user-images.githubusercontent.com/64761832/210807640-a37306bb-3599-416f-9863-6f5b32117d58.png)
![italian (Personalizzato)](https://user-images.githubusercontent.com/64761832/210807707-e8c55222-fae6-4193-8b74-d32b1916ab25.png)
![french (Personalizzato)](https://user-images.githubusercontent.com/64761832/210807746-c0ef6038-158d-4b31-a13d-76437af47d99.png)
![spanish (Personalizzato)](https://user-images.githubusercontent.com/64761832/210807762-68fcffcb-4077-46dc-ba41-53b781ec15fa.png)
![chinese (Personalizzato)](https://user-images.githubusercontent.com/64761832/210807783-f678cf79-e9b6-4831-bd4c-13d4d835b829.png)
![pause_reset functions (Personalizzato)](https://user-images.githubusercontent.com/64761832/210807849-ce10fce9-3c73-4b4e-8164-3f08ab96c3da.png)

---

## Características

1. **Rendimiento y Reactividad**:
   - Compilado en código máquina nativo con Rust y `eframe`/`egui`.
   - Ejecutable único independiente, inicio instantáneo y consumo mínimo de memoria (< 30 MB).
   - Sin dependencias externas de Python ni entornos de ejecución pesados.

2. **Panel de Configuración y Temas**:
   - Icono de engranaje en la esquina inferior derecha para abrir el modal de ajustes rápidos.
   - **Cambio de Tema**: Selección inmediata entre:
     - **Oscuro** (*Dark Mode*)
     - **Claro** (*Light Mode*)
     - **Sistema** (*Auto* - sigue las preferencias del sistema operativo)
   - **Soporte Multilingüe (i18n)**:
     - Inglés (`en`)
     - Italiano (`it`)
     - Francés (`fr`)
     - Español (`sp`)
     - Chino (`ch`)
     - Detección automática del idioma del sistema en el primer inicio.

3. **Modo Híbrido Portable e Instalado**:
   - Sin archivos de configuración creados por defecto (huella cero).
   - En cuanto el usuario modifica el idioma o el tema, se genera `config.json`.
   - En modo **Portable** (o si existe el archivo `portable.txt`), `config.json` se guarda en la carpeta del ejecutable.
   - En modo **Instalado**, `config.json` se guarda en las rutas estándar de usuario:
     - **Windows**: `%APPDATA%\ShutdownTimer\config.json`
     - **macOS**: `~/Library/Application Support/ShutdownTimer/config.json`
     - **Linux**: `$XDG_CONFIG_HOME/ShutdownTimer/config.json` o `~/.config/ShutdownTimer/config.json`

4. **Integración Nativa Multiplataforma**:
   - **Windows**:
     - Prevención automática de la suspensión del PC y de la pantalla mediante `SetThreadExecutionState` y `PowerCreateRequest`.
     - Apagado programado del sistema mediante `shutdown /s /t <sec>` y cancelación con `shutdown /a`.
     - Detección automática del Modo Oscuro desde el Registro de Windows.
   - **macOS**:
     - Prevención de la suspensión mediante la herramienta nativa `caffeinate`.
     - Apagado seguro a nivel de usuario mediante AppleScript (`osascript` con `System Events`), sin necesidad de permisos root o sudo.
     - Detección del idioma del sistema mediante `defaults read -g AppleLocale`.
   - **Linux**:
     - Inhibición de la suspensión mediante `systemd-inhibit`.
     - Apagado programado con soporte de cancelación mediante `systemctl poweroff` / `shutdown`.
     - Detección del Modo Oscuro mediante la configuración de GNOME/freedesktop.

5. **Controles Intuitivos del Temporizador**:
   - 3 Spinbox ajustables para Horas (0-99), Minutos (0-59) y Segundos (0-59).
   - Botones rápidos `+` y `-` con ciclo continuo y campo de texto editable directamente.
   - Gran pantalla digital de cuenta atrás en formato `HH:MM:SS`.
   - Botones **Iniciar** (*Start*), **Pausa**, **Reiniciar** (*Resume*) y **Reset**.

6. **Protección y Ventanas Modales de Confirmación**:
   - Modal de error al intentar iniciar el temporizador con duración cero.
   - Confirmación de restablecimiento (*Reset*) para evitar cancelaciones accidentales.
   - Intercepción del cierre de la ventana `[X]` cuando el temporizador está en marcha con cuadro de diálogo de confirmación.

---

## Compilación y Empaquetado con GitHub Actions

El repositorio incluye un flujo de trabajo CI/CD completo en [`.github/workflows/build-and-release.yml`](.github/workflows/build-and-release.yml) que compila automáticamente todos los formatos en cada etiqueta (`v*`) o manualmente mediante *Run workflow*:

| Plataforma | Formato del Paquete | Artefacto generado |
| :--- | :--- | :--- |
| **Windows** | **Portable** (Zip independiente) | `ShutdownTimer-Windows-Portable.zip` |
| **Windows** | **Instalador** (Inno Setup) | `ShutdownTimer-Windows-Setup.exe` |
| **macOS** | **Imagen de Disco Apple & Bundle** | `ShutdownTimer-macOS.dmg`, `.tar.gz` |
| **Linux** | **AppImage** | `ShutdownTimer-x86_64.AppImage` |
| **Linux** | **Bundle Flatpak** | `ShutdownTimer.flatpak` |

---

## Compilación Local

### Requisitos previos
- **Rust y Cargo** (instalable mediante [rustup.rs](https://rustup.rs))

### Compilación Release:
```bash
cargo build --release
```

### Ejecución de Pruebas:
```bash
cargo test
```

---

## Estructura del Proyecto

```
shutdown_timer_rust/
├── .github/
│   └── workflows/
│       └── build-and-release.yml    # CI/CD GitHub Actions (Windows, macOS, AppImage, Flatpak)
├── assets/
│   ├── GFSNeohellenic-Bold.ttf      # Fuente incrustada
│   ├── power.ico                    # Icono de la aplicación (Windows)
│   ├── power_icon.png               # Icono de la ventana (multiplataforma)
│   ├── settings_icon.png            # Icono de engranaje para Modo Oscuro
│   ├── settings_icon_dark.png       # Icono de engranaje para Modo Claro
│   └── shutdown_timer_logo.png      # Logo integrado
├── installer/
│   ├── flatpak/
│   │   └── io.github.lucag.ShutdownTimer.yml  # Manifiesto Flatpak
│   ├── linux/
│   │   ├── io.github.lucag.ShutdownTimer.desktop
│   │   └── io.github.lucag.ShutdownTimer.metainfo.xml
│   ├── macos/
│   │   └── Info.plist               # Configuración de App Bundle macOS
│   └── windows/
│       └── setup.iss                # Script de Inno Setup para instalador Windows
├── src/
│   ├── main.rs                      # Punto de entrada, configuración de ventana y fuentes
│   ├── app.rs                       # Lógica de la aplicación, interfaz de usuario y ajustes
│   ├── spinbox.rs                   # Widget Spinbox para horas, minutos y segundos
│   ├── config.rs                    # Gestión de configuración híbrida (Portable / Sistema)
│   ├── i18n.rs                      # Gestión de traducciones y detección de idioma
│   ├── platform.rs                  # Abstracción nativa multiplataforma (Windows, macOS, Linux)
│   ├── theme.rs                     # Gestión de paleta y tema Oscuro/Claro/Auto
│   └── timer.rs                     # Motor de cuenta atrás
├── Cargo.toml                       # Manifiesto del paquete Rust
├── LICENSE                          # Licencia MIT
└── README.md
```

---

## Licencia

Este proyecto está bajo la licencia **MIT**. Consulte el archivo [`LICENSE`](LICENSE) para conocer todos los términos y condiciones.
