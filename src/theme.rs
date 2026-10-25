use egui::{Color32, CornerRadius, Visuals};

pub const BUTTON_PRIMARY: Color32 = Color32::from_rgb(57, 94, 156); // CustomTkinter #395E9C
pub const BUTTON_DISABLED: Color32 = Color32::from_rgb(60, 65, 75);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    #[default]
    Auto,
    Dark,
    Light,
}

impl ThemeMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ThemeMode::Auto => "auto",
            ThemeMode::Dark => "dark",
            ThemeMode::Light => "light",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "auto" | "system" => Some(ThemeMode::Auto),
            "dark" => Some(ThemeMode::Dark),
            "light" => Some(ThemeMode::Light),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
}

impl Theme {
    /// Resolves the effective Theme given a user preference and UI context.
    pub fn resolve(mode: ThemeMode, ctx: &egui::Context) -> Self {
        match mode {
            ThemeMode::Dark => Theme::Dark,
            ThemeMode::Light => Theme::Light,
            ThemeMode::Auto => Self::from_system(ctx),
        }
    }

    /// Detects the theme automatically from the operating system.
    pub fn from_system(ctx: &egui::Context) -> Self {
        match ctx.system_theme() {
            Some(egui::Theme::Dark) => Theme::Dark,
            Some(egui::Theme::Light) => Theme::Light,
            None => {
                if crate::platform::is_system_dark_mode() {
                    Theme::Dark
                } else {
                    Theme::Light
                }
            }
        }
    }

    pub fn is_dark(&self) -> bool {
        *self == Theme::Dark
    }

    pub fn window_background(&self) -> Color32 {
        if self.is_dark() {
            Color32::from_rgb(36, 36, 36) // CustomTkinter dark #242424
        } else {
            Color32::from_rgb(235, 235, 235) // CustomTkinter light #EBEBEB
        }
    }

    pub fn clear_color(&self) -> [f32; 4] {
        let (r, g, b) = if self.is_dark() {
            (36, 36, 36)
        } else {
            (235, 235, 235)
        };
        egui::Rgba::from_srgba_unmultiplied(r, g, b, 255).to_array()
    }

    pub fn label_text(&self) -> Color32 {
        if self.is_dark() {
            Color32::from_rgb(220, 220, 220)
        } else {
            Color32::from_rgb(30, 30, 30)
        }
    }

    pub fn countdown_text(&self) -> Color32 {
        if self.is_dark() {
            Color32::from_rgb(220, 228, 238) // #DCE4EE
        } else {
            Color32::from_rgb(26, 26, 26) // #1A1A1A
        }
    }

    pub fn entry_background(&self) -> Color32 {
        if self.is_dark() {
            Color32::from_rgb(52, 54, 56) // CustomTkinter dark #343638
        } else {
            Color32::from_rgb(249, 249, 250) // CustomTkinter light #F9F9FA
        }
    }

    pub fn entry_border(&self) -> Color32 {
        if self.is_dark() {
            Color32::from_rgb(86, 91, 94) // CustomTkinter dark #565B5E
        } else {
            Color32::from_rgb(151, 157, 162) // CustomTkinter light #979DA2
        }
    }

    pub fn entry_text(&self) -> Color32 {
        if self.is_dark() {
            Color32::WHITE
        } else {
            Color32::from_rgb(26, 26, 26)
        }
    }

    pub fn modal_background(&self) -> Color32 {
        if self.is_dark() {
            Color32::from_rgb(36, 36, 36)
        } else {
            Color32::from_rgb(245, 245, 245)
        }
    }

    pub fn modal_border(&self) -> Color32 {
        if self.is_dark() {
            Color32::from_rgb(60, 60, 60)
        } else {
            Color32::from_rgb(180, 180, 180)
        }
    }

    pub fn modal_text(&self) -> Color32 {
        if self.is_dark() {
            Color32::WHITE
        } else {
            Color32::from_rgb(26, 26, 26)
        }
    }

    pub fn modal_backdrop(&self) -> Color32 {
        if self.is_dark() {
            Color32::from_black_alpha(160)
        } else {
            Color32::from_black_alpha(100)
        }
    }

    /// Configures egui Visuals to match the current theme palette.
    pub fn apply_visuals(&self, ctx: &egui::Context) {
        let mut visuals = if self.is_dark() {
            Visuals::dark()
        } else {
            Visuals::light()
        };

        let bg = self.window_background();
        visuals.panel_fill = bg;
        visuals.window_fill = bg;
        visuals.faint_bg_color = bg;
        visuals.extreme_bg_color = bg;
        visuals.window_corner_radius = CornerRadius::same(8);
        visuals.widgets.noninteractive.corner_radius = CornerRadius::same(6);
        visuals.widgets.inactive.corner_radius = CornerRadius::same(6);
        visuals.widgets.hovered.corner_radius = CornerRadius::same(6);
        visuals.widgets.active.corner_radius = CornerRadius::same(6);

        ctx.set_visuals(visuals);
    }
}
