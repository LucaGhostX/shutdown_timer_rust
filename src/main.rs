#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod config;
mod i18n;
mod platform;
mod spinbox;
mod theme;
mod timer;

use std::sync::Arc;
use app::ShutdownTimerApp;
use eframe::egui::{self, vec2};
use image::imageops;

fn main() -> eframe::Result<()> {
    // Load window icon from power_icon.png (extracted from power.ico)
    let icon_data = match image::load_from_memory_with_format(
        include_bytes!("../assets/power_icon.png"),
        image::ImageFormat::Png,
    ) {
        Ok(img) => {
            let resized = imageops::resize(&img, 64, 64, imageops::FilterType::Lanczos3);
            let (width, height) = resized.dimensions();
            Some(Arc::new(egui::IconData {
                rgba: resized.into_raw(),
                width,
                height,
            }))
        }
        Err(_) => None,
    };

    let initial_language = config::load_config()
        .and_then(|cfg| cfg.language)
        .unwrap_or_else(i18n::detect_system_language);
    let initial_title = initial_language.texts().title;

    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size(vec2(400.0, 300.0))
        .with_min_inner_size(vec2(400.0, 300.0))
        .with_max_inner_size(vec2(400.0, 300.0))
        .with_resizable(false)
        .with_maximize_button(false)
        .with_title(initial_title);

    if let Some(icon) = icon_data {
        viewport = viewport.with_icon(icon);
    }

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Shutdown Timer",
        native_options,
        Box::new(|cc| {
            // Install image loaders for egui::include_image!
            egui_extras::install_image_loaders(&cc.egui_ctx);

            // Configure GFSNeohellenic-Bold font as the primary proportional font
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "GFSNeohellenic-Bold".to_owned(),
                Arc::new(egui::FontData::from_static(include_bytes!(
                    "../assets/GFSNeohellenic-Bold.ttf"
                ))),
            );
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "GFSNeohellenic-Bold".to_owned());
            cc.egui_ctx.set_fonts(fonts);

            // Apply system or user-preferred visuals on initialization
            let initial_theme_mode = config::load_config()
                .and_then(|cfg| cfg.theme)
                .unwrap_or_default();
            let theme = theme::Theme::resolve(initial_theme_mode, &cc.egui_ctx);
            theme.apply_visuals(&cc.egui_ctx);

            Ok(Box::new(ShutdownTimerApp::new(cc)))
        }),
    )
}
