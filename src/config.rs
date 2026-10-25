use std::fs;
use std::path::{Path, PathBuf};

use crate::i18n::Language;
use crate::theme::ThemeMode;

pub const APP_NAME: &str = "ShutdownTimer";
pub const CONFIG_FILE_NAME: &str = "config.json";
pub const PORTABLE_MARKER: &str = "portable.txt";

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AppConfig {
    pub language: Option<Language>,
    pub theme: Option<ThemeMode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigLocation {
    Portable,
    Roaming,
}

/// Returns the directory where the current executable is located.
pub fn get_exe_dir() -> Option<PathBuf> {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
}

/// Returns the configuration directory for installed mode:
/// - Windows: %APPDATA%/ShutdownTimer
/// - macOS: ~/Library/Application Support/ShutdownTimer
/// - Linux/Unix: $XDG_CONFIG_HOME/ShutdownTimer or ~/.config/ShutdownTimer
pub fn get_roaming_dir() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        if let Ok(appdata) = std::env::var("APPDATA") {
            return Some(PathBuf::from(appdata).join(APP_NAME));
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            return Some(
                PathBuf::from(home)
                    .join("Library")
                    .join("Application Support")
                    .join(APP_NAME),
            );
        }
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME") {
            return Some(PathBuf::from(config_home).join(APP_NAME));
        }
        if let Ok(home) = std::env::var("HOME") {
            return Some(PathBuf::from(home).join(".config").join(APP_NAME));
        }
    }
    None
}

/// Checks whether a directory is inside a standard system installation directory
/// (Program Files on Windows, /Applications on macOS, /usr, /opt, /bin on Linux).
fn is_in_system_dirs(dir: &Path) -> bool {
    #[cfg(windows)]
    {
        let dir_str = dir.to_string_lossy().to_lowercase();
        if let Ok(pf) = std::env::var("ProgramFiles") {
            if dir_str.starts_with(&pf.to_lowercase()) {
                return true;
            }
        }
        if let Ok(pf86) = std::env::var("ProgramFiles(x86)") {
            if dir_str.starts_with(&pf86.to_lowercase()) {
                return true;
            }
        }
        if let Ok(pfall) = std::env::var("ProgramW6432") {
            if dir_str.starts_with(&pfall.to_lowercase()) {
                return true;
            }
        }
    }
    #[cfg(target_os = "macos")]
    {
        let dir_str = dir.to_string_lossy();
        if dir_str.starts_with("/Applications")
            || dir_str.starts_with("/System")
            || dir_str.starts_with("/Library")
        {
            return true;
        }
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        let dir_str = dir.to_string_lossy();
        if dir_str.starts_with("/usr")
            || dir_str.starts_with("/opt")
            || dir_str.starts_with("/bin")
            || dir_str.starts_with("/var")
        {
            return true;
        }
    }
    let _ = dir;
    false
}

/// Checks whether a directory is writable by attempting a temporary write.
fn is_dir_writable(dir: &Path) -> bool {
    let test_file = dir.join(".write_test_tmp");
    if fs::write(&test_file, b"").is_ok() {
        let _ = fs::remove_file(&test_file);
        true
    } else {
        false
    }
}

/// Determines where to save the configuration file:
/// 1. If `config.json` already exists next to the executable, use Portable mode.
/// 2. If `config.json` already exists in Roaming/Config directory, use Roaming mode.
/// 3. If a `portable.txt` marker exists in the executable folder, use Portable mode.
/// 4. If the executable is in standard system dirs (Program Files, /Applications, /usr/bin), use Roaming mode.
/// 5. Otherwise, if the executable folder is writable, use Portable mode (ideal for USB drives/folders).
///    If not writable, fall back to Roaming mode.
pub fn determine_target_config_path() -> Option<(PathBuf, ConfigLocation)> {
    let exe_dir = get_exe_dir()?;
    let local_config = exe_dir.join(CONFIG_FILE_NAME);
    let local_marker = exe_dir.join(PORTABLE_MARKER);

    // 1. Existing local config
    if local_config.exists() {
        return Some((local_config, ConfigLocation::Portable));
    }

    // 2. Existing Roaming/Application Support/Config
    if let Some(roaming_dir) = get_roaming_dir() {
        let roaming_config = roaming_dir.join(CONFIG_FILE_NAME);
        if roaming_config.exists() {
            return Some((roaming_config, ConfigLocation::Roaming));
        }
    }

    // 3. Explicit portable marker
    if local_marker.exists() {
        return Some((local_config, ConfigLocation::Portable));
    }

    // 4. Installed in system directory -> Roaming
    if is_in_system_dirs(&exe_dir) {
        if let Some(roaming_dir) = get_roaming_dir() {
            return Some((roaming_dir.join(CONFIG_FILE_NAME), ConfigLocation::Roaming));
        }
    }

    // 5. Check if local directory is writable (standalone folder / USB drive)
    if is_dir_writable(&exe_dir) {
        Some((local_config, ConfigLocation::Portable))
    } else if let Some(roaming_dir) = get_roaming_dir() {
        Some((roaming_dir.join(CONFIG_FILE_NAME), ConfigLocation::Roaming))
    } else {
        Some((local_config, ConfigLocation::Portable))
    }
}

/// Loads configuration from either local folder or Roaming folder if existing.
/// Returns None if no config file exists in either location (without creating any file).
pub fn load_config() -> Option<AppConfig> {
    // 1. Check local directory first (portable takes precedence if present)
    if let Some(exe_dir) = get_exe_dir() {
        let local_config = exe_dir.join(CONFIG_FILE_NAME);
        if local_config.exists() {
            if let Ok(content) = fs::read_to_string(&local_config) {
                if let Some(cfg) = parse_config(&content) {
                    return Some(cfg);
                }
            }
        }
    }

    // 2. Check Roaming directory
    if let Some(roaming_dir) = get_roaming_dir() {
        let roaming_config = roaming_dir.join(CONFIG_FILE_NAME);
        if roaming_config.exists() {
            if let Ok(content) = fs::read_to_string(&roaming_config) {
                if let Some(cfg) = parse_config(&content) {
                    return Some(cfg);
                }
            }
        }
    }

    None
}

/// Parses the configuration file content (supports JSON, INI-style, or plain language code).
pub fn parse_config(content: &str) -> Option<AppConfig> {
    let mut language = None;
    let mut theme = None;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('"') {
            if let Some((key_part, val_part)) = trimmed.split_once(':') {
                let key = key_part.trim().trim_matches('"');
                let val = val_part
                    .trim()
                    .trim_matches(|c| c == '"' || c == ',' || c == ' ');
                if key.eq_ignore_ascii_case("language") {
                    language = Language::from_locale(val);
                } else if key.eq_ignore_ascii_case("theme") {
                    theme = ThemeMode::from_str(val);
                }
            }
        } else if let Some((key, val)) = trimmed.split_once('=') {
            let key = key.trim();
            let val = val.trim().trim_matches('"');
            if key.eq_ignore_ascii_case("language") {
                language = Language::from_locale(val);
            } else if key.eq_ignore_ascii_case("theme") {
                theme = ThemeMode::from_str(val);
            }
        }
    }

    if language.is_some() || theme.is_some() {
        return Some(AppConfig { language, theme });
    }

    // Fallback: try whole string as a locale/language code
    if let Some(lang) = Language::from_locale(content.trim()) {
        return Some(AppConfig {
            language: Some(lang),
            theme: None,
        });
    }

    None
}

/// Serializes configuration into clean, formatted JSON.
pub fn serialize_config(cfg: &AppConfig) -> String {
    let lang_code = cfg.language.map(|l| l.code()).unwrap_or("en");
    let theme_str = cfg.theme.unwrap_or_default().as_str();
    format!(
        "{{\n  \"language\": \"{}\",\n  \"theme\": \"{}\"\n}}\n",
        lang_code, theme_str
    )
}

/// Saves the user's preferences (language and theme) to the configuration file.
pub fn save_preferences(lang: Language, theme: ThemeMode) {
    if let Some((target_path, _location)) = determine_target_config_path() {
        if let Some(parent) = target_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let config = AppConfig {
            language: Some(lang),
            theme: Some(theme),
        };
        let content = serialize_config(&config);
        let _ = fs::write(target_path, content);
    }
}

/// Saves the user's forced language preference to the appropriate config location.
#[allow(dead_code)]
pub fn save_language_preference(lang: Language) {
    let current_theme = load_config().and_then(|c| c.theme).unwrap_or_default();
    save_preferences(lang, current_theme);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config_json() {
        let json = "{\n  \"language\": \"it\",\n  \"theme\": \"dark\"\n}";
        let cfg = parse_config(json).expect("Failed to parse JSON config");
        assert_eq!(cfg.language, Some(Language::Italian));
        assert_eq!(cfg.theme, Some(ThemeMode::Dark));

        let json_zh = "{\n  \"language\": \"zh-CN\",\n  \"theme\": \"light\"\n}";
        let cfg_zh = parse_config(json_zh).expect("Failed to parse JSON config");
        assert_eq!(cfg_zh.language, Some(Language::Chinese));
        assert_eq!(cfg_zh.theme, Some(ThemeMode::Light));
    }

    #[test]
    fn test_parse_config_ini() {
        let ini = "language = fr";
        let cfg = parse_config(ini).expect("Failed to parse INI config");
        assert_eq!(cfg.language, Some(Language::French));
    }

    #[test]
    fn test_parse_config_plain() {
        let plain = "es";
        let cfg = parse_config(plain).expect("Failed to parse plain config");
        assert_eq!(cfg.language, Some(Language::Spanish));
    }

    #[test]
    fn test_serialize_config() {
        let cfg = AppConfig {
            language: Some(Language::Italian),
            theme: Some(ThemeMode::Dark),
        };
        let serialized = serialize_config(&cfg);
        assert!(serialized.contains("\"language\": \"it\""));
        assert!(serialized.contains("\"theme\": \"dark\""));
    }

    #[test]
    fn test_determine_target_path() {
        let result = determine_target_config_path();
        assert!(result.is_some());
        let (path, location) = result.unwrap();
        assert!(path.ends_with(CONFIG_FILE_NAME));
        println!("Target config path: {:?}, location: {:?}", path, location);
    }
}
