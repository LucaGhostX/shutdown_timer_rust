#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Italian,
    French,
    Spanish,
    Chinese,
}

#[derive(Debug, Clone)]
pub struct Texts {
    pub title: &'static str,
    pub hour: &'static str,
    pub min: &'static str,
    pub sec: &'static str,
    pub start: &'static str,
    pub resume: &'static str,
    pub pause: &'static str,
    pub reset: &'static str,
    pub quit_title: &'static str,
    pub quit_msg: &'static str,
    pub okay: &'static str,
    pub cancel: &'static str,
    pub error_title: &'static str,
    pub error_msg: &'static str,
    pub cancel_timer_msg: &'static str,
    #[allow(dead_code)]
    pub select_language: &'static str,
    pub settings: &'static str,
    pub theme: &'static str,
    pub theme_dark: &'static str,
    pub theme_light: &'static str,
    pub theme_auto: &'static str,
    pub language_label: &'static str,
    pub save: &'static str,
}

pub const ALL_LANGUAGES: [Language; 5] = [
    Language::English,
    Language::Italian,
    Language::French,
    Language::Spanish,
    Language::Chinese,
];

impl Language {
    #[allow(dead_code)]
    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Italian => "it",
            Language::French => "fr",
            Language::Spanish => "sp",
            Language::Chinese => "ch",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Italian => "Italiano",
            Language::French => "Français",
            Language::Spanish => "Español",
            Language::Chinese => "中文",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        match code.trim().to_lowercase().as_str() {
            "en" => Some(Language::English),
            "it" => Some(Language::Italian),
            "fr" => Some(Language::French),
            "sp" | "es" => Some(Language::Spanish),
            "ch" | "zh" => Some(Language::Chinese),
            _ => None,
        }
    }

    /// Parses a locale tag (such as "it-IT", "en_US", "zh-CN", "es-419", "fr")
    /// into a supported `Language`, matching by code or primary language subtag.
    pub fn from_locale(locale: &str) -> Option<Self> {
        let trimmed = locale.trim().to_lowercase();
        if let Some(lang) = Self::from_code(&trimmed) {
            return Some(lang);
        }

        if let Some(primary) = trimmed.split(&['-', '_', '.'][..]).next() {
            if let Some(lang) = Self::from_code(primary) {
                return Some(lang);
            }
        }

        None
    }

    pub fn texts(&self) -> Texts {
        match self {
            Language::English => Texts {
                title: "Shutdown Timer",
                hour: "Hour",
                min: "Min.",
                sec: "Sec.",
                start: "Start",
                resume: "Resume",
                pause: "Pause",
                reset: "Reset",
                quit_title: "Quit?",
                quit_msg: "Do you want to quit?\nYou'll cancel the timer.",
                okay: "Okay",
                cancel: "Cancel",
                error_title: "Error!",
                error_msg: "Invalid value,\nenter data again",
                cancel_timer_msg: "You'll cancel the timer.",
                select_language: "Select language",
                settings: "Settings",
                theme: "Theme",
                theme_dark: "Dark",
                theme_light: "Light",
                theme_auto: "System",
                language_label: "Language",
                save: "Save",
            },
            Language::Italian => Texts {
                title: "Spegnimento Programmato",
                hour: "Ora",
                min: "Min.",
                sec: "Sec.",
                start: "Avvia",
                resume: "Riprendi",
                pause: "Pausa",
                reset: "Ripristina",
                quit_title: "Uscire?",
                quit_msg: "Vuoi uscire?\nIl timer verrà cancellato.",
                okay: "Okay",
                cancel: "Annulla",
                error_title: "Errore!",
                error_msg: "Valore non valido,\ninserisci nuovamente\ni dati.",
                cancel_timer_msg: "Il timer verrà cancellato.",
                select_language: "Seleziona lingua",
                settings: "Impostazioni",
                theme: "Tema",
                theme_dark: "Scuro",
                theme_light: "Chiaro",
                theme_auto: "Sistema",
                language_label: "Lingua",
                save: "Salva",
            },
            Language::French => Texts {
                title: "Arrêt Programmé",
                hour: "Heure",
                min: "Min.",
                sec: "Sec.",
                start: "Lancement",
                resume: "Continuer",
                pause: "Pause",
                reset: "Restaurer",
                quit_title: "Quitter?",
                quit_msg: "Voulez-vous quitter?\nLa minuterie sera\nannulée.",
                okay: "Ok",
                cancel: "Annuller",
                error_title: "Erreur!",
                error_msg: "Valeur non valide,\nEntrez encore\nles données.",
                cancel_timer_msg: "La minuterie sera\nannulée.",
                select_language: "Choisir la langue",
                settings: "Paramètres",
                theme: "Thème",
                theme_dark: "Sombre",
                theme_light: "Clair",
                theme_auto: "Système",
                language_label: "Langue",
                save: "Enregistrer",
            },
            Language::Spanish => Texts {
                title: "Programa apagado automatico",
                hour: "Horas",
                min: "Min.",
                sec: "Seg.",
                start: "Iniciar",
                resume: "Reiniciar",
                pause: "Pausa",
                reset: "Reset",
                quit_title: "Salir?",
                quit_msg: "Salir?\nEl timer será cancelado.",
                okay: "Okay",
                cancel: "Anular",
                error_title: "Error!",
                error_msg: "Valor invalido,\ninserir nuovamente\nlos datos.",
                cancel_timer_msg: "El timer será cancelado.",
                select_language: "Seleccionar idioma",
                settings: "Configuración",
                theme: "Tema",
                theme_dark: "Oscuro",
                theme_light: "Claro",
                theme_auto: "Sistema",
                language_label: "Idioma",
                save: "Guardar",
            },
            Language::Chinese => Texts {
                title: "关机计时器",
                hour: "小时",
                min: "分",
                sec: "秒",
                start: "开始",
                resume: "恢复",
                pause: "暂停",
                reset: "重启",
                quit_title: "退出?",
                quit_msg: "你想要退出吗？\n你将取消计时器",
                okay: "确定",
                cancel: "取消",
                error_title: "错误!",
                error_msg: "无效值\n重新输入数据",
                cancel_timer_msg: "你将取消计时器",
                select_language: "选择语言",
                settings: "设置",
                theme: "主题",
                theme_dark: "深色",
                theme_light: "浅色",
                theme_auto: "系统",
                language_label: "语言",
                save: "保存",
            },
        }
    }
}

/// Automatically detects system language from the operating system.
/// If the detected language is not present in `ALL_LANGUAGES`, it falls back to `Language::English`.
pub fn detect_system_language() -> Language {
    for locale in crate::platform::get_system_locales() {
        if let Some(lang) = Language::from_locale(&locale) {
            return lang;
        }
    }
    Language::English
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_languages_have_translations() {
        for lang in ALL_LANGUAGES {
            let texts = lang.texts();
            assert!(!texts.title.is_empty());
            assert!(!texts.hour.is_empty());
            assert!(!texts.min.is_empty());
            assert!(!texts.sec.is_empty());
            assert!(!texts.start.is_empty());
            assert!(!texts.pause.is_empty());
            assert!(!texts.resume.is_empty());
            assert!(!texts.reset.is_empty());
            assert!(!texts.quit_title.is_empty());
            assert!(!texts.quit_msg.is_empty());
            assert!(!texts.okay.is_empty());
            assert!(!texts.cancel.is_empty());
            assert!(!texts.error_title.is_empty());
            assert!(!texts.error_msg.is_empty());
        }
    }

    #[test]
    fn test_from_code() {
        assert_eq!(Language::from_code("en"), Some(Language::English));
        assert_eq!(Language::from_code("it"), Some(Language::Italian));
        assert_eq!(Language::from_code("fr"), Some(Language::French));
        assert_eq!(Language::from_code("sp"), Some(Language::Spanish));
        assert_eq!(Language::from_code("es"), Some(Language::Spanish));
        assert_eq!(Language::from_code("ch"), Some(Language::Chinese));
        assert_eq!(Language::from_code("zh"), Some(Language::Chinese));
        assert_eq!(Language::from_code("invalid"), None);
    }

    #[test]
    fn test_from_locale() {
        assert_eq!(Language::from_locale("it-IT"), Some(Language::Italian));
        assert_eq!(Language::from_locale("it_IT"), Some(Language::Italian));
        assert_eq!(Language::from_locale("it-CH"), Some(Language::Italian));
        assert_eq!(Language::from_locale("en-US"), Some(Language::English));
        assert_eq!(Language::from_locale("en-GB"), Some(Language::English));
        assert_eq!(Language::from_locale("en_US.UTF-8"), Some(Language::English));
        assert_eq!(Language::from_locale("fr-FR"), Some(Language::French));
        assert_eq!(Language::from_locale("fr-CA"), Some(Language::French));
        assert_eq!(Language::from_locale("es-ES"), Some(Language::Spanish));
        assert_eq!(Language::from_locale("es-419"), Some(Language::Spanish));
        assert_eq!(Language::from_locale("zh-CN"), Some(Language::Chinese));
        assert_eq!(Language::from_locale("zh-TW"), Some(Language::Chinese));
        assert_eq!(Language::from_locale("zh-Hans-CN"), Some(Language::Chinese));
        assert_eq!(Language::from_locale("de-DE"), None);
        assert_eq!(Language::from_locale("ja-JP"), None);
        assert_eq!(Language::from_locale(""), None);
    }

    #[test]
    fn test_detect_system_language() {
        let lang = detect_system_language();
        // ALL_LANGUAGES contains it
        assert!(ALL_LANGUAGES.contains(&lang));
    }

    #[test]
    fn test_language_code_and_display() {
        assert_eq!(Language::English.code(), "en");
        assert_eq!(Language::Italian.code(), "it");
        assert_eq!(Language::French.code(), "fr");
        assert_eq!(Language::Spanish.code(), "sp");
        assert_eq!(Language::Chinese.code(), "ch");

        assert_eq!(Language::English.display_name(), "English");
        assert_eq!(Language::Italian.display_name(), "Italiano");
        assert_eq!(Language::French.display_name(), "Français");
        assert_eq!(Language::Spanish.display_name(), "Español");
        assert_eq!(Language::Chinese.display_name(), "中文");
    }
}
