use egui::{vec2, Align2, Color32, CornerRadius, FontId, Pos2, Rect, RichText, Stroke, Ui};

use crate::theme::{Theme, BUTTON_DISABLED, BUTTON_PRIMARY};

#[derive(Debug, Clone)]
pub struct Spinbox {
    pub value: u32,
    pub limit: u32,
    pub text_buffer: String,
}

impl Spinbox {
    pub fn new(initial: u32, limit: u32) -> Self {
        let val = initial.min(limit);
        Self {
            value: val,
            limit,
            text_buffer: val.to_string(),
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn set_value(&mut self, val: u32) {
        self.value = val.min(self.limit);
        self.text_buffer = self.value.to_string();
    }

    pub fn increment(&mut self) {
        if self.value >= self.limit {
            self.value = 0;
        } else {
            self.value += 1;
        }
        self.text_buffer = self.value.to_string();
    }

    pub fn decrement(&mut self) {
        if self.value == 0 {
            self.value = self.limit;
        } else {
            self.value -= 1;
        }
        self.text_buffer = self.value.to_string();
    }

    pub fn sync_text(&mut self) {
        let digits: String = self
            .text_buffer
            .chars()
            .filter(|c| c.is_ascii_digit())
            .take(2)
            .collect();

        if let Ok(num) = digits.parse::<u32>() {
            self.value = num.min(self.limit);
        } else {
            self.value = 0;
        }
        self.text_buffer = self.value.to_string();
    }

    pub fn render_at(&mut self, ui: &mut Ui, center: Pos2, disabled: bool, theme: &Theme) {
        let width = 50.0;
        let btn_height = 29.0;
        let entry_height = 50.0;
        let gap = 3.0;
        let total_height = btn_height * 2.0 + entry_height + gap * 2.0; // 114.0

        let top_left = Pos2::new(center.x - width / 2.0, center.y - total_height / 2.0);

        // 1. [+] Button
        let add_rect = Rect::from_min_size(top_left, vec2(width, btn_height));
        let add_btn = egui::Button::new(
            RichText::new("+")
                .size(20.0)
                .color(Color32::WHITE),
        )
        .fill(if disabled {
            BUTTON_DISABLED
        } else {
            BUTTON_PRIMARY
        })
        .corner_radius(6.0);

        if ui.put(add_rect, add_btn).clicked() && !disabled {
            self.increment();
        }

        // 2. Middle Entry Box with adaptive theme styling
        let entry_y = top_left.y + btn_height + gap;
        let entry_rect = Rect::from_min_size(Pos2::new(top_left.x, entry_y), vec2(width, entry_height));

        ui.painter().rect(
            entry_rect,
            CornerRadius::same(6),
            theme.entry_background(),
            Stroke::new(1.5, theme.entry_border()),
            egui::StrokeKind::Inside,
        );

        // Singleline TextEdit inside the entry box
        let text_rect = entry_rect.shrink2(vec2(2.0, 4.0));
        let text_edit = egui::TextEdit::singleline(&mut self.text_buffer)
            .font(FontId::proportional(27.0))
            .text_color(theme.entry_text())
            .char_limit(2)
            .desired_width(width - 4.0)
            .align(Align2::CENTER_CENTER)
            .frame(egui::Frame::NONE);

        let resp = ui.put(text_rect, text_edit);
        if resp.changed() {
            self.text_buffer.retain(|c| c.is_ascii_digit());
            if self.text_buffer.len() > 2 {
                self.text_buffer.truncate(2);
            }
            if let Ok(n) = self.text_buffer.parse::<u32>() {
                if n > self.limit {
                    self.value = self.limit;
                    self.text_buffer = self.limit.to_string();
                } else {
                    self.value = n;
                }
            }
        }
        if resp.lost_focus() {
            self.sync_text();
        }

        // 3. [-] Button
        let sub_y = entry_y + entry_height + gap;
        let sub_rect = Rect::from_min_size(Pos2::new(top_left.x, sub_y), vec2(width, btn_height));
        let sub_btn = egui::Button::new(
            RichText::new("-")
                .size(20.0)
                .color(Color32::WHITE),
        )
        .fill(if disabled {
            BUTTON_DISABLED
        } else {
            BUTTON_PRIMARY
        })
        .corner_radius(6.0);

        if ui.put(sub_rect, sub_btn).clicked() && !disabled {
            self.decrement();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinbox_increment_decrement_wraparound() {
        let mut sb = Spinbox::new(0, 59);
        assert_eq!(sb.value(), 0);

        sb.decrement();
        assert_eq!(sb.value(), 59);

        sb.increment();
        assert_eq!(sb.value(), 0);

        sb.increment();
        assert_eq!(sb.value(), 1);

        sb.decrement();
        assert_eq!(sb.value(), 0);
    }

    #[test]
    fn test_spinbox_sync_text() {
        let mut sb = Spinbox::new(0, 59);
        sb.text_buffer = "45".to_string();
        sb.sync_text();
        assert_eq!(sb.value(), 45);

        // Clamping to limit
        sb.text_buffer = "99".to_string();
        sb.sync_text();
        assert_eq!(sb.value(), 59);

        // Stripping non-numeric
        sb.text_buffer = "ab7".to_string();
        sb.sync_text();
        assert_eq!(sb.value(), 7);
        assert_eq!(sb.text_buffer, "7");
    }
}
