# Shutdown Timer (Rust Version) 2.4

Riscrittura ad alte prestazioni in **Rust** del programma **Shutdown Timer** (originariamente sviluppato in Python/CustomTkinter).

Un timer per lo spegnimento programmato del computer compatto, moderno e reattivo, con interfaccia grafica Dark/Light Mode nativa, supporto multilingua, compatibilità multipiattaforma (**Windows**, **macOS**, **Linux**) e prevenzione intelligente della sospensione.

---

## 🚀 Caratteristiche

1. **Prestazioni e Reattività**:
   - Compilato in codice macchina nativo con Rust ed `eframe`/`egui`.
   - Eseguibile singolo autonomo, avvio istantaneo e consumo di memoria minimo (< 30 MB).
   - Nessuna dipendenza esterna da Python o runtime pesanti.

2. **Pannello Impostazioni (Ingranaggio ⚙) e Temi**:
   - Icona a ingranaggio in basso a destra per accedere al pannello impostazioni rapido.
   - **Switch Tema**: Scelta immediata tra:
     - 🌙 **Scuro** (*Dark Mode*)
     - ☀️ **Chiaro** (*Light Mode*)
     - 💻 **Sistema** (*Auto* - segue le preferenze dell'OS)
   - **Supporto Multilingua (i18n)**:
     - 🇬🇧 Inglese (`en`)
     - 🇮🇹 Italiano (`it`)
     - 🇫🇷 Francese (`fr`)
     - 🇪🇸 Spagnolo (`sp`)
     - 🇨🇳 Cinese (`ch`)
     - Rilevamento automatico della lingua di sistema al primo avvio.

3. **Modalità Ibrida Portable & Installato**:
   - Di default non viene creato alcun file di configurazione (zero footprint).
   - Se l'utente modifica lingua o tema dal pannello impostazioni, viene generato `config.json`.
   - In modalità **Portable** (o con file `portable.txt`), `config.json` viene salvato nella cartella dell'eseguibile.
   - In modalità **Installata**, `config.json` viene salvato nei percorsi utente standard:
     - **Windows**: `%APPDATA%\ShutdownTimer\config.json`
     - **macOS**: `~/Library/Application Support/ShutdownTimer/config.json`
     - **Linux**: `$XDG_CONFIG_HOME/ShutdownTimer/config.json` o `~/.config/ShutdownTimer/config.json`

4. **Integrazione Nativa Multipiattaforma**:
   - **Windows**:
     - Prevenzione automatica della sospensione del PC e del display tramite `SetThreadExecutionState` e `PowerCreateRequest`.
     - Arresto del sistema tramite `shutdown /s /t <sec>` e cancellazione con `shutdown /a`.
     - Rilevamento automatico del tema Dark dal registro di Windows.
   - **macOS**:
     - Prevenzione dello sleep tramite il tool nativo `caffeinate`.
     - Arresto sicuro a livello utente via AppleScript (`osascript` con `System Events`), senza necessità di permessi di root o sudo.
     - Rilevamento lingua tramite `defaults read -g AppleLocale`.
   - **Linux**:
     - Inibizione dello standby tramite `systemd-inhibit`.
     - Arresto programmato con cancellazione tramite `systemctl poweroff` / `shutdown`.
     - Rilevamento tema Dark Mode tramite impostazioni GNOME/freedesktop.

5. **Controlli Timer Intuitivi**:
   - 3 Spinbox regolabili per Ore (0-99), Minuti (0-59) e Secondi (0-59).
   - Pulsanti rapidi `+` e `-` con wrap-around circolare e casella di testo modificabile direttamente.
   - Grande display digitale del conto alla rovescia in formato `HH:MM:SS`.
   - Pulsanti **Avvia** (*Start*), **Pausa** (*Pause*), **Riprendi** (*Resume*) e **Ripristina** (*Reset*).

6. **Protezione e Modali di Conferma**:
   - Finestra modale di errore se si tenta di avviare il timer con valore zero.
   - Conferma di ripristino (*Reset*) per evitare azzeramenti accidentali.
   - Intercettazione della chiusura della finestra `[X]` se il timer è in esecuzione con conferma di uscita.

---

## 📦 Compilazione e Packaging con GitHub Actions

Il repository include un flusso di lavoro completo in [`.github/workflows/build-and-release.yml`](.github/workflows/build-and-release.yml) che compila automaticamente tutti i formati a ogni push su tag di rilascio (`v*`) o manualmente tramite *Run workflow*:

| Piattaforma | Formato Pacchetto | Artefatto generato |
| :--- | :--- | :--- |
| **Windows** | **Portable** (Zip autonomo) | `ShutdownTimer-Windows-Portable.zip` |
| **Windows** | **Installer** (Inno Setup) | `ShutdownTimer-Windows-Setup.exe` |
| **macOS** | **Apple Disk Image & Bundle** | `ShutdownTimer-macOS.dmg`, `.tar.gz` |
| **Linux** | **AppImage** | `ShutdownTimer-x86_64.AppImage` |
| **Linux** | **Flatpak Bundle** | `ShutdownTimer.flatpak` |

---

## 🛠️ Compilazione Locale

### Prerequisiti
- **Rust & Cargo** (installabile tramite [rustup.rs](https://rustup.rs))

### Compilazione Release:
```bash
cargo build --release
```

### Esecuzione Test:
```bash
cargo test
```

---

## 📂 Struttura del Progetto

```
shutdown_timer_rust/
├── .github/
│   └── workflows/
│       └── build-and-release.yml    # CI/CD GitHub Actions (Windows, macOS, AppImage, Flatpak)
├── assets/
│   ├── GFSNeohellenic-Bold.ttf      # Font incorporato
│   ├── power.ico                    # Icona applicazione (Windows)
│   ├── power_icon.png               # Icona finestra (multipiattaforma)
│   ├── settings_icon.png            # Icona ingranaggio per Dark Mode
│   ├── settings_icon_dark.png       # Icona ingranaggio per Light Mode
│   └── shutdown_timer_logo.png      # Logo integrato
├── installer/
│   ├── flatpak/
│   │   └── io.github.lucag.ShutdownTimer.yml  # Manifest Flatpak
│   ├── linux/
│   │   ├── io.github.lucag.ShutdownTimer.desktop
│   │   └── io.github.lucag.ShutdownTimer.metainfo.xml
│   ├── macos/
│   │   └── Info.plist               # Configurazione App Bundle macOS
│   └── windows/
│       └── setup.iss                # Script Inno Setup per Windows Installer
├── src/
│   ├── main.rs                      # Entry point, configurazione finestra e font
│   ├── app.rs                       # Logica applicativa, interfaccia utente e modale impostazioni
│   ├── spinbox.rs                   # Widget Spinbox per ore, minuti e secondi
│   ├── config.rs                    # Gestione configurazione ibrida (Portable / System)
│   ├── i18n.rs                      # Gestione traduzioni e rilevamento lingua di sistema
│   ├── platform.rs                  # Astrazione nativa multipiattaforma (Windows, macOS, Linux)
│   ├── theme.rs                     # Gestione palette e tema Dark/Light/Auto
│   └── timer.rs                     # Motore del conto alla rovescia
├── Cargo.toml                       # Configurazione dipendenze Rust
├── LICENSE                          # Licenza MIT
└── README.md
```

---

## 📄 Licenza

Questo progetto è rilasciato sotto licenza **MIT**. Consulta il file [`LICENSE`](LICENSE) per il testo completo dei termini e condizioni.

