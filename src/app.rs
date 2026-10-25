use std::time::Duration;

use egui::{
    vec2, Align2, Color32, FontId, Pos2, Rect, RichText, Stroke, Ui,
};

use crate::i18n::{detect_system_language, Language, ALL_LANGUAGES};
use crate::platform::{cancel_shutdown, set_keepawake, trigger_shutdown, unset_keepawake};
use crate::spinbox::Spinbox;
use crate::theme::{Theme, ThemeMode, BUTTON_PRIMARY};
use crate::timer::{Timer, TimerEvent};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Modal {
    Settings,
    ConfirmQuit,
    ConfirmReset,
    InvalidValue,
}

pub struct ShutdownTimerApp {
    pub language: Language,
    pub current_title: &'static str,
    pub theme_mode: ThemeMode,
    pub timer: Timer,
    pub hour_sb: Spinbox,
    pub min_sb: Spinbox,
    pub sec_sb: Spinbox,
    pub active_modal: Option<Modal>,
    pub temp_selected_lang: Language,
    pub temp_selected_theme: ThemeMode,
    pub ctx: Option<egui::Context>,
}

impl ShutdownTimerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let loaded_config = crate::config::load_config();
        let initial_lang = loaded_config
            .as_ref()
            .and_then(|cfg| cfg.language)
            .unwrap_or_else(detect_system_language);
        let initial_theme = loaded_config
            .as_ref()
            .and_then(|cfg| cfg.theme)
            .unwrap_or_default();

        Self {
            language: initial_lang,
            current_title: initial_lang.texts().title,
            theme_mode: initial_theme,
            timer: Timer::new(),
            hour_sb: Spinbox::new(0, 99),
            min_sb: Spinbox::new(0, 59),
            sec_sb: Spinbox::new(0, 59),
            active_modal: None,
            temp_selected_lang: initial_lang,
            temp_selected_theme: initial_theme,
            ctx: Some(cc.egui_ctx.clone()),
        }
    }

    /// Spawns a background thread that continuously keeps the PC awake and advances the timer,
    /// ensuring sleep prevention and shutdown trigger reliably even when minimized or in background.
    fn spawn_background_worker(&self) {
        let timer = self.timer.clone();
        let ctx = self.ctx.clone();

        std::thread::spawn(move || {
            while timer.is_running() {
                std::thread::sleep(Duration::from_millis(500));

                // Continuously keep Windows awake by resetting the system idle timer
                set_keepawake(true);

                if let Some(event) = timer.tick() {
                    if event == TimerEvent::Triggered {
                        unset_keepawake();
                        trigger_shutdown(20);
                        if let Some(ref c) = ctx {
                            c.request_repaint();
                        }
                        break;
                    }
                }

                if let Some(ref c) = ctx {
                    c.request_repaint();
                }
            }
        });
    }

    pub fn start_timer(&mut self) {
        self.hour_sb.sync_text();
        self.min_sb.sync_text();
        self.sec_sb.sync_text();

        let total_seconds = (self.hour_sb.value() as u64) * 3600
            + (self.min_sb.value() as u64) * 60
            + (self.sec_sb.value() as u64);

        if !self.timer.start(Duration::from_secs(total_seconds)) {
            self.active_modal = Some(Modal::InvalidValue);
            return;
        }

        // Prevent Windows sleep and spawn the background keepalive worker
        set_keepawake(true);
        self.spawn_background_worker();
    }

    pub fn pause_timer(&mut self) {
        self.timer.pause();
        unset_keepawake();
    }

    pub fn resume_timer(&mut self) {
        self.timer.resume();
        set_keepawake(true);
        self.spawn_background_worker();
    }

    pub fn reset_timer(&mut self) {
        if self.timer.is_triggered() {
            cancel_shutdown();
        }
        self.timer.reset();
        self.hour_sb.set_value(0);
        self.min_sb.set_value(0);
        self.sec_sb.set_value(0);
        unset_keepawake();
    }

    pub fn update_timer(&mut self) {
        if let Some(event) = self.timer.tick() {
            if event == TimerEvent::Triggered {
                unset_keepawake();
                trigger_shutdown(20);
            }
        }
    }

    fn render_modal(&mut self, ctx: &egui::Context, style: &egui::Style, theme: &Theme) {
        let modal = match self.active_modal {
            Some(m) => m,
            None => return,
        };

        let texts = self.language.texts();

        // Dark dim backdrop overlay
        #[allow(deprecated)]
        egui::Area::new(egui::Id::new("modal_backdrop"))
            .order(egui::Order::Middle)
            .fixed_pos(Pos2::ZERO)
            .show(ctx, |ui| {
                let screen_rect = ctx
                    .input(|i| i.viewport().inner_rect.unwrap_or(egui::Rect::EVERYTHING));
                ui.painter()
                    .rect_filled(screen_rect, 0.0, theme.modal_backdrop());
            });

        match modal {
            Modal::Settings => {
                egui::Window::new("Settings")
                    .title_bar(false)
                    .collapsible(false)
                    .resizable(false)
                    .movable(false)
                    .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
                    .fixed_size(vec2(280.0, 255.0))
                    .frame(
                        egui::Frame::window(style)
                            .fill(theme.modal_background())
                            .corner_radius(8.0)
                            .stroke(Stroke::new(1.0, theme.modal_border())),
                    )
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(4.0);
                            ui.label(
                                RichText::new(texts.settings)
                                    .size(22.0)
                                    .strong()
                                    .color(theme.modal_text()),
                            );
                        });

                        ui.add_space(8.0);
                        // Theme selection
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(format!("{}:", texts.theme))
                                    .size(16.0)
                                    .strong()
                                    .color(theme.modal_text()),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = vec2(8.0, 0.0);
                            ui.radio_value(
                                &mut self.temp_selected_theme,
                                ThemeMode::Dark,
                                RichText::new(texts.theme_dark)
                                    .size(14.0)
                                    .color(theme.modal_text()),
                            );
                            ui.radio_value(
                                &mut self.temp_selected_theme,
                                ThemeMode::Light,
                                RichText::new(texts.theme_light)
                                    .size(14.0)
                                    .color(theme.modal_text()),
                            );
                            ui.radio_value(
                                &mut self.temp_selected_theme,
                                ThemeMode::Auto,
                                RichText::new(texts.theme_auto)
                                    .size(14.0)
                                    .color(theme.modal_text()),
                            );
                        });

                        ui.add_space(8.0);
                        // Language selection
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(format!("{}:", texts.language_label))
                                    .size(16.0)
                                    .strong()
                                    .color(theme.modal_text()),
                            );
                        });

                        ui.vertical(|ui| {
                            ui.spacing_mut().item_spacing = vec2(0.0, 3.0);
                            for lang in ALL_LANGUAGES {
                                let is_selected = self.temp_selected_lang == lang;
                                if ui
                                    .radio(
                                        is_selected,
                                        RichText::new(lang.display_name())
                                            .size(15.0)
                                            .color(theme.modal_text()),
                                    )
                                    .clicked()
                                {
                                    self.temp_selected_lang = lang;
                                }
                            }
                        });

                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            let save_btn = egui::Button::new(
                                RichText::new(texts.save)
                                    .size(17.0)
                                    .color(Color32::WHITE),
                            )
                            .fill(BUTTON_PRIMARY)
                            .min_size(vec2(125.0, 32.0))
                            .corner_radius(6.0);

                            if ui.add(save_btn).clicked() {
                                self.language = self.temp_selected_lang;
                                self.theme_mode = self.temp_selected_theme;
                                crate::config::save_preferences(self.language, self.theme_mode);
                                let new_theme = Theme::resolve(self.theme_mode, ctx);
                                new_theme.apply_visuals(ctx);
                                self.active_modal = None;
                            }

                            let cancel_btn = egui::Button::new(
                                RichText::new(texts.cancel)
                                    .size(17.0)
                                    .color(theme.modal_text()),
                            )
                            .fill(theme.modal_border())
                            .min_size(vec2(125.0, 32.0))
                            .corner_radius(6.0);

                            if ui.add(cancel_btn).clicked() {
                                self.active_modal = None;
                            }
                        });
                    });
            }

            Modal::ConfirmQuit => {
                let action = Self::render_dialog(
                    ctx,
                    style,
                    theme,
                    texts.quit_title,
                    texts.quit_msg,
                    texts.okay,
                    Some(texts.cancel),
                );
                match action {
                    Some(DialogAction::Okay) => {
                        self.reset_timer();
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    Some(DialogAction::Cancel) => {
                        self.active_modal = None;
                    }
                    None => {}
                }
            }

            Modal::ConfirmReset => {
                let title = format!("{}?", texts.reset);
                let action = Self::render_dialog(
                    ctx,
                    style,
                    theme,
                    &title,
                    texts.cancel_timer_msg,
                    texts.okay,
                    Some(texts.cancel),
                );
                match action {
                    Some(DialogAction::Okay) => {
                        self.reset_timer();
                        self.active_modal = None;
                    }
                    Some(DialogAction::Cancel) => {
                        self.active_modal = None;
                    }
                    None => {}
                }
            }

            Modal::InvalidValue => {
                let action = Self::render_dialog(
                    ctx,
                    style,
                    theme,
                    texts.error_title,
                    texts.error_msg,
                    texts.okay,
                    None,
                );
                if action.is_some() {
                    self.active_modal = None;
                }
            }
        }
    }

    fn render_dialog(
        ctx: &egui::Context,
        style: &egui::Style,
        theme: &Theme,
        title: &str,
        message: &str,
        ok_label: &str,
        cancel_label: Option<&str>,
    ) -> Option<DialogAction> {
        let mut result = None;

        egui::Window::new(title)
            .collapsible(false)
            .resizable(false)
            .movable(false)
            .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
            .fixed_size(vec2(220.0, 130.0))
            .frame(
                egui::Frame::window(style)
                    .fill(theme.modal_background())
                    .corner_radius(8.0)
                    .stroke(Stroke::new(1.0, theme.modal_border())),
            )
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(message)
                            .size(17.0)
                            .color(theme.modal_text()),
                    );
                    ui.add_space(14.0);

                    if let Some(cancel_text) = cancel_label {
                        ui.horizontal(|ui| {
                            ui.add_space(12.0);
                            let okay_btn = egui::Button::new(
                                RichText::new(ok_label).size(16.0).color(Color32::WHITE),
                            )
                            .fill(BUTTON_PRIMARY)
                            .min_size(vec2(80.0, 32.0))
                            .corner_radius(5.0);

                            if ui.add(okay_btn).clicked() {
                                result = Some(DialogAction::Okay);
                            }

                            ui.add_space(12.0);
                            let cancel_btn = egui::Button::new(
                                RichText::new(cancel_text).size(16.0).color(Color32::WHITE),
                            )
                            .fill(BUTTON_PRIMARY)
                            .min_size(vec2(80.0, 32.0))
                            .corner_radius(5.0);

                            if ui.add(cancel_btn).clicked() {
                                result = Some(DialogAction::Cancel);
                            }
                        });
                    } else {
                        let okay_btn = egui::Button::new(
                            RichText::new(ok_label).size(16.0).color(Color32::WHITE),
                        )
                        .fill(BUTTON_PRIMARY)
                        .min_size(vec2(160.0, 32.0))
                        .corner_radius(5.0);

                        if ui.add(okay_btn).clicked() {
                            result = Some(DialogAction::Okay);
                        }
                    }
                });
            });

        result
    }
}

enum DialogAction {
    Okay,
    Cancel,
}

impl eframe::App for ShutdownTimerApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        let theme = match self.theme_mode {
            ThemeMode::Dark => Theme::Dark,
            ThemeMode::Light => Theme::Light,
            ThemeMode::Auto => {
                if crate::platform::is_system_dark_mode() {
                    Theme::Dark
                } else {
                    Theme::Light
                }
            }
        };
        theme.clear_color()
    }

    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        if self.ctx.is_none() {
            self.ctx = Some(ctx.clone());
        }

        // Resolve theme based on user preference (Dark, Light, or Auto)
        let theme = Theme::resolve(self.theme_mode, &ctx);

        // Explicitly paint the exact CustomTkinter background: #242424 (dark) or #EBEBEB (light)
        let bg_color = theme.window_background();
        let bg_rect = ui
            .clip_rect()
            .union(Rect::from_min_size(Pos2::ZERO, vec2(400.0, 300.0)));
        ui.painter().rect_filled(bg_rect, 0.0, bg_color);

        // Intercept window close request when timer is active
        if ctx.input(|i| i.viewport().close_requested()) {
            if self.timer.is_active() {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                self.active_modal = Some(Modal::ConfirmQuit);
            } else {
                self.reset_timer();
            }
        }

        // Advance timer
        self.update_timer();
        if self.timer.is_running() {
            ctx.request_repaint_after(Duration::from_millis(200));
        }

        let texts = self.language.texts();
        if self.current_title != texts.title {
            self.current_title = texts.title;
            ctx.send_viewport_cmd(egui::ViewportCommand::Title(self.current_title.to_owned()));
        }

        let is_active = self.timer.is_active();

        // 1. Logo on the left: center (112.0, 81.0), size (200.0, 153.0)
        let logo_rect = Rect::from_center_size(Pos2::new(112.0, 81.0), vec2(200.0, 153.0));
        if theme.is_dark() {
            let logo_image = egui::Image::new(egui::include_image!(
                "../assets/shutdown_timer_logo.png"
            ))
            .fit_to_exact_size(vec2(200.0, 153.0));
            ui.put(logo_rect, logo_image);
        } else {
            let logo_image = egui::Image::new(egui::include_image!(
                "../assets/shutdown_timer_logo_dark.png"
            ))
            .fit_to_exact_size(vec2(200.0, 153.0));
            ui.put(logo_rect, logo_image);
        }

        // 2. Column Labels: Ora (256, 27), Min. (312, 27), Sec. (368, 27)
        let label_color = theme.label_text();
        let label_size = vec2(50.0, 24.0);
        let labels = [
            (texts.hour, Pos2::new(256.0, 27.0)),
            (texts.min, Pos2::new(312.0, 27.0)),
            (texts.sec, Pos2::new(368.0, 27.0)),
        ];

        for (lbl_text, pos) in labels {
            let rect = Rect::from_center_size(pos, label_size);
            ui.put(
                rect,
                egui::Label::new(
                    RichText::new(lbl_text)
                        .size(21.0)
                        .color(label_color),
                ),
            );
        }

        // 3. Spinboxes: (256, 102), (312, 102), (368, 102)
        self.hour_sb
            .render_at(ui, Pos2::new(256.0, 102.0), is_active, &theme);
        self.min_sb
            .render_at(ui, Pos2::new(312.0, 102.0), is_active, &theme);
        self.sec_sb
            .render_at(ui, Pos2::new(368.0, 102.0), is_active, &theme);

        // 4. Action Button(s) centered at y = 186.0
        if !is_active {
            // Full width Start button: center (200.0, 186.0), size (388.0, 40.0)
            let start_rect = Rect::from_center_size(Pos2::new(200.0, 186.0), vec2(388.0, 40.0));
            let start_btn = egui::Button::new(
                RichText::new(texts.start)
                    .size(27.0)
                    .color(Color32::WHITE),
            )
            .fill(BUTTON_PRIMARY)
            .corner_radius(6.0);

            if ui.put(start_rect, start_btn).clicked() {
                self.start_timer();
            }
        } else {
            // Side-by-side Pause and Reset buttons
            let pause_text = if self.timer.is_paused() {
                texts.resume
            } else {
                texts.pause
            };

            let pause_rect = Rect::from_center_size(Pos2::new(102.0, 186.0), vec2(192.0, 40.0));
            let pause_btn = egui::Button::new(
                RichText::new(pause_text)
                    .size(24.0)
                    .color(Color32::WHITE),
            )
            .fill(BUTTON_PRIMARY)
            .corner_radius(6.0);

            if ui.put(pause_rect, pause_btn).clicked() {
                if self.timer.is_paused() {
                    self.resume_timer();
                } else {
                    self.pause_timer();
                }
            }

            let reset_rect = Rect::from_center_size(Pos2::new(298.0, 186.0), vec2(192.0, 40.0));
            let reset_btn = egui::Button::new(
                RichText::new(texts.reset)
                    .size(24.0)
                    .color(Color32::WHITE),
            )
            .fill(BUTTON_PRIMARY)
            .corner_radius(6.0);

            if ui.put(reset_rect, reset_btn).clicked() {
                self.active_modal = Some(Modal::ConfirmReset);
            }
        }

        // 5. Large Countdown Display centered at (200.0, 251.0)
        let countdown_str = self.timer.formatted_time();
        let countdown_rect = Rect::from_center_size(Pos2::new(200.0, 251.0), vec2(380.0, 68.0));
        let countdown_label = egui::Label::new(
            RichText::new(countdown_str)
                .font(FontId::proportional(67.0))
                .color(theme.countdown_text()),
        );
        ui.put(countdown_rect, countdown_label);

        // 6. Settings Gear Icon Button at bottom right: center (370.0, 270.0), size (30.0, 30.0)
        let settings_rect = Rect::from_center_size(Pos2::new(370.0, 270.0), vec2(30.0, 30.0));
        let settings_btn = if theme.is_dark() {
            let img = egui::Image::new(egui::include_image!("../assets/settings_icon.png"))
                .fit_to_exact_size(vec2(22.0, 22.0));
            egui::Button::image(img).fill(bg_color).corner_radius(4.0)
        } else {
            let img =
                egui::Image::new(egui::include_image!("../assets/settings_icon_dark.png"))
                    .fit_to_exact_size(vec2(22.0, 22.0));
            egui::Button::image(img).fill(bg_color).corner_radius(4.0)
        };

        if ui
            .put(settings_rect, settings_btn)
            .on_hover_text(texts.settings)
            .clicked()
            && !is_active
        {
            self.temp_selected_lang = self.language;
            self.temp_selected_theme = self.theme_mode;
            self.active_modal = Some(Modal::Settings);
        }

        // 7. Modals
        let style = ui.style().clone();
        self.render_modal(&ctx, &style, &theme);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_timer_zero_shows_error_modal() {
        let mut app = ShutdownTimerApp {
            language: Language::English,
            current_title: Language::English.texts().title,
            theme_mode: ThemeMode::Auto,
            timer: Timer::new(),
            hour_sb: Spinbox::new(0, 99),
            min_sb: Spinbox::new(0, 59),
            sec_sb: Spinbox::new(0, 59),
            active_modal: None,
            temp_selected_lang: Language::English,
            temp_selected_theme: ThemeMode::Auto,
            ctx: None,
        };

        app.start_timer();
        assert!(app.timer.is_idle());
        assert_eq!(app.active_modal, Some(Modal::InvalidValue));
    }

    #[test]
    fn test_start_timer_valid_duration() {
        let mut app = ShutdownTimerApp {
            language: Language::Italian,
            current_title: Language::Italian.texts().title,
            theme_mode: ThemeMode::Dark,
            timer: Timer::new(),
            hour_sb: Spinbox::new(1, 99),
            min_sb: Spinbox::new(30, 59),
            sec_sb: Spinbox::new(15, 59),
            active_modal: None,
            temp_selected_lang: Language::Italian,
            temp_selected_theme: ThemeMode::Dark,
            ctx: None,
        };

        app.start_timer();
        assert!(app.timer.is_running());
        assert_eq!(app.timer.remaining_seconds(), 3600 + 30 * 60 + 15);
        assert_eq!(app.timer.formatted_time(), "01:30:15");
        app.reset_timer();
    }

    #[test]
    fn test_pause_resume_reset_lifecycle() {
        let mut app = ShutdownTimerApp {
            language: Language::English,
            current_title: Language::English.texts().title,
            theme_mode: ThemeMode::Light,
            timer: Timer::new(),
            hour_sb: Spinbox::new(2, 99),
            min_sb: Spinbox::new(0, 59),
            sec_sb: Spinbox::new(0, 59),
            active_modal: None,
            temp_selected_lang: Language::English,
            temp_selected_theme: ThemeMode::Light,
            ctx: None,
        };

        app.start_timer();
        assert!(app.timer.is_running());

        app.pause_timer();
        assert!(app.timer.is_paused());

        app.resume_timer();
        assert!(app.timer.is_running());

        app.reset_timer();
        assert!(app.timer.is_idle());
        assert_eq!(app.timer.formatted_time(), "00:00:00");
        assert_eq!(app.hour_sb.value(), 0);
        assert_eq!(app.min_sb.value(), 0);
        assert_eq!(app.sec_sb.value(), 0);
    }
}
