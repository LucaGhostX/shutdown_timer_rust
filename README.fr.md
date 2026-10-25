# Shutdown Timer (Version Rust) 2.4

[English](README.md) | [Italiano](README.it.md) | **Français** | [Español](README.es.md) | [中文](README.zh.md)

[![Vibecoded](https://img.shields.io/badge/development-vibecoded-ff69b4?style=flat-square)](https://github.com/LucaGhostX/shutdown_timer_rust)
[![Rust](https://img.shields.io/badge/rust-2021-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)

> **Note**: Ce portage en Rust a été entièrement **vibecoded** à partir du projet original [**shutdown_timer**](https://github.com/LucaGhostX/shutdown_timer) (initialement développé en Python/CustomTkinter).

Une minuterie d'arrêt programmé de l'ordinateur compacte, moderne et réactive, avec une interface graphique native en Mode Sombre/Clair, un support multilingue, une compatibilité multiplateforme (**Windows**, **macOS**, **Linux**) et une prévention intelligente de la mise en veille.

---

## Captures d'écran

![english (Personalizzato)](https://user-images.githubusercontent.com/64761832/210807640-a37306bb-3599-416f-9863-6f5b32117d58.png)
![italian (Personalizzato)](https://user-images.githubusercontent.com/64761832/210807707-e8c55222-fae6-4193-8b74-d32b1916ab25.png)
![french (Personalizzato)](https://user-images.githubusercontent.com/64761832/210807746-c0ef6038-158d-4b31-a13d-76437af47d99.png)
![spanish (Personalizzato)](https://user-images.githubusercontent.com/64761832/210807762-68fcffcb-4077-46dc-ba41-53b781ec15fa.png)
![chinese (Personalizzato)](https://user-images.githubusercontent.com/64761832/210807783-f678cf79-e9b6-4831-bd4c-13d4d835b829.png)
![pause_reset functions (Personalizzato)](https://user-images.githubusercontent.com/64761832/210807849-ce10fce9-3c73-4b4e-8164-3f08ab96c3da.png)

---

## Fonctionnalités

1. **Performances et Réactivité**:
   - Compilé en code machine natif avec Rust et `eframe`/`egui`.
   - Exécutable unique autonome, démarrage instantané et consommation de mémoire minimale (< 30 Mo).
   - Aucune dépendance externe envers Python ou des environnements d'exécution lourds.

2. **Panneau des Paramètres et Thèmes**:
   - Icône d'engrenage en bas à droite pour ouvrir la fenêtre contextuelle des paramètres rapides.
   - **Changement de Thème**: Bascule immédiate entre:
     - **Sombre** (*Dark Mode*)
     - **Clair** (*Light Mode*)
     - **Système** (*Auto* - suit les préférences du système d'exploitation)
   - **Support Multilingue (i18n)**:
     - Anglais (`en`)
     - Italien (`it`)
     - Français (`fr`)
     - Espagnol (`sp`)
     - Chinois (`ch`)
     - Détection automatique de la langue du système dès le premier lancement.

3. **Mode Hybride Portable et Installé**:
   - Aucune création de fichier de configuration par défaut (empreinte zéro).
   - Dès que l'utilisateur modifie la langue ou le thème, `config.json` est généré.
   - En mode **Portable** (ou si le fichier `portable.txt` est présent), `config.json` est enregistré dans le dossier de l'exécutable.
   - En mode **Installé**, `config.json` est enregistré dans les répertoires utilisateur standards:
     - **Windows**: `%APPDATA%\ShutdownTimer\config.json`
     - **macOS**: `~/Library/Application Support/ShutdownTimer/config.json`
     - **Linux**: `$XDG_CONFIG_HOME/ShutdownTimer/config.json` ou `~/.config/ShutdownTimer/config.json`

4. **Intégration Native Multiplateforme**:
   - **Windows**:
     - Prévention automatique de la mise en veille du PC et de l'écran via `SetThreadExecutionState` et `PowerCreateRequest`.
     - Arrêt programmé du système via `shutdown /s /t <sec>` et annulation via `shutdown /a`.
     - Détection automatique du Mode Sombre à partir du registre Windows.
   - **macOS**:
     - Prévention de la mise en veille à l'aide de l'utilitaire natif `caffeinate`.
     - Arrêt sécurisé au niveau utilisateur via AppleScript (`osascript` avec `System Events`), sans nécessiter de droits root ou sudo.
     - Détection de la langue du système via `defaults read -g AppleLocale`.
   - **Linux**:
     - Inhibition de la mise en veille via `systemd-inhibit`.
     - Arrêt programmé avec prise en charge de l'annulation via `systemctl poweroff` / `shutdown`.
     - Détection du Mode Sombre via les paramètres GNOME/freedesktop.

5. **Contrôles Intuitifs de la Minuterie**:
   - 3 Spinbox ajustables pour les Heures (0-99), Minutes (0-59) et Secondes (0-59).
   - Boutons rapides `+` et `-` avec bouclage circulaire et champ de texte directement modifiable.
   - Grand affichage numérique du compte à rebours au format `HH:MM:SS`.
   - Boutons **Lancement** (*Start*), **Pause**, **Continuer** (*Resume*) et **Restaurer** (*Reset*).

6. **Protection et Fenêtres de Confirmation**:
   - Fenêtre modale d'erreur en cas de tentative de démarrage avec une durée nulle.
   - Fenêtre de confirmation lors de la réinitialisation (*Restaurer*) pour éviter les annulations accidentelles.
   - Interception de la fermeture de la fenêtre `[X]` lorsqu'une minuterie est en cours avec boîte de dialogue de confirmation.

---

## Compilation et Empaquetage avec GitHub Actions

Le dépôt inclut un flux de travail CI/CD complet dans [`.github/workflows/build-and-release.yml`](.github/workflows/build-and-release.yml) qui compile automatiquement tous les formats de distribution à chaque tag (`v*`) ou manuellement via *Run workflow*:

| Plateforme | Format du Paquet | Artefact généré |
| :--- | :--- | :--- |
| **Windows** | **Portable** (Zip autonome) | `ShutdownTimer-Windows-Portable.zip` |
| **Windows** | **Installateur** (Inno Setup) | `ShutdownTimer-Windows-Setup.exe` |
| **macOS** | **Image Disque Apple & Bundle** | `ShutdownTimer-macOS.dmg`, `.tar.gz` |
| **Linux** | **AppImage** | `ShutdownTimer-x86_64.AppImage` |
| **Linux** | **Bundle Flatpak** | `ShutdownTimer.flatpak` |

---

## Compilation Locale

### Prérequis
- **Rust et Cargo** (installables via [rustup.rs](https://rustup.rs))

### Compilation Release:
```bash
cargo build --release
```

### Exécution des Tests:
```bash
cargo test
```

---

## Structure du Projet

```
shutdown_timer_rust/
├── .github/
│   └── workflows/
│       └── build-and-release.yml    # CI/CD GitHub Actions (Windows, macOS, AppImage, Flatpak)
├── assets/
│   ├── GFSNeohellenic-Bold.ttf      # Police intégrée
│   ├── power.ico                    # Icône d'application (Windows)
│   ├── power_icon.png               # Icône de fenêtre (multiplateforme)
│   ├── settings_icon.png            # Icône d'engrenage pour Mode Sombre
│   ├── settings_icon_dark.png       # Icône d'engrenage pour Mode Clair
│   └── shutdown_timer_logo.png      # Logo intégré
├── installer/
│   ├── flatpak/
│   │   └── io.github.lucag.ShutdownTimer.yml  # Manifeste Flatpak
│   ├── linux/
│   │   ├── io.github.lucag.ShutdownTimer.desktop
│   │   └── io.github.lucag.ShutdownTimer.metainfo.xml
│   ├── macos/
│   │   └── Info.plist               # Configuration App Bundle macOS
│   └── windows/
│       └── setup.iss                # Script Inno Setup pour installateur Windows
├── src/
│   ├── main.rs                      # Point d'entrée, configuration de fenêtre et chargement de polices
│   ├── app.rs                       # Logique applicative, interface utilisateur et paramètres
│   ├── spinbox.rs                   # Widget Spinbox pour heures, minutes, secondes
│   ├── config.rs                    # Gestion de configuration hybride (Portable / Système)
│   ├── i18n.rs                      # Gestion des traductions et détection de la langue système
│   ├── platform.rs                  # Abstraction native multiplateforme (Windows, macOS, Linux)
│   ├── theme.rs                     # Gestion de la palette et thème Sombre/Clair/Auto
│   └── timer.rs                     # Moteur du compte à rebours
├── Cargo.toml                       # Manifeste du paquet Rust
├── LICENSE                          # Licence MIT
└── README.md
```

---

## Licence

Ce projet est distribué sous licence **MIT**. Consultez le fichier [`LICENSE`](LICENSE) pour l'intégralité des termes et conditions.
