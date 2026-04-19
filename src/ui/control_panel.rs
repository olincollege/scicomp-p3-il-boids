use macroquad::prelude::*;

use crate::constants::{HIGHLIGHT_COLOR, SIDEBAR_WIDTH};

pub const CONTROL_PANEL_HEIGHT: f32 = 140.0;

pub struct ControlPanel {
    pub x: f32,
    pub y: f32,
    pub w: f32,
}

impl Default for ControlPanel {
    fn default() -> Self {
        Self {
            x: screen_width() - SIDEBAR_WIDTH + 12.0,
            y: 16.0,
            w: SIDEBAR_WIDTH - 24.0,
        }
    }
}

impl ControlPanel {
    pub fn draw(&self, kappa: f32, constant_acceleration: bool) {
        // Panel border
        draw_rectangle_lines(self.x, self.y, self.w, CONTROL_PANEL_HEIGHT, 2.0, LIGHTGRAY);

        let text_x = self.x + 12.0;
        let mut cursor_y = self.y + 24.0;
        let label_x = text_x;

        let draw_key_hint = |label: &str, y: f32| {
            let x = self.x + self.w - measure_text(label, None, 18, 1.0).width - 12.0;
            draw_text(label, x, y, 18.0, DARKGRAY);
        };

        draw_text("Controls", text_x, cursor_y, 22.0, DARKGRAY);
        cursor_y += 32.0;

        // Interaction range
        let text_size = draw_text(
            &format!("Interaction Range: "),
            label_x,
            cursor_y,
            18.0,
            DARKGRAY,
        );
        draw_text(
            &format!("{:.1}", kappa),
            label_x + text_size.width,
            cursor_y,
            20.0,
            HIGHLIGHT_COLOR,
        );
        draw_key_hint("[a/d]", cursor_y);
        cursor_y += 18.0;

        draw_text(
            "(multiplier on desired distance)",
            label_x,
            cursor_y,
            14.0,
            GRAY,
        );
        cursor_y += 22.0;

        // Constant acceleration + checkbox
        let text_size = draw_text("Constant Acceleration:", label_x, cursor_y, 18.0, DARKGRAY);

        let checkbox_size = 14.0;
        Self::draw_checkbox(
            checkbox_size,
            self.x + text_size.width + 16.0,
            cursor_y - checkbox_size + 2.0,
            constant_acceleration,
        );

        draw_key_hint("[w]", cursor_y);
        cursor_y += 24.0;

        // Restart
        draw_text(
            "Restart with new parameters",
            label_x,
            cursor_y,
            18.0,
            DARKGRAY,
        );
        draw_key_hint("[r]", cursor_y);
    }

    fn draw_checkbox(checkbox_size: f32, checkbox_x: f32, checkbox_y: f32, checked: bool) {
        draw_rectangle_lines(
            checkbox_x,
            checkbox_y,
            checkbox_size,
            checkbox_size,
            1.0,
            HIGHLIGHT_COLOR,
        );
        if checked {
            draw_line(
                checkbox_x + 3.0,
                checkbox_y + checkbox_size * 0.6,
                checkbox_x + checkbox_size * 0.45,
                checkbox_y + checkbox_size - 3.0,
                2.0,
                HIGHLIGHT_COLOR,
            );
            draw_line(
                checkbox_x + checkbox_size * 0.45,
                checkbox_y + checkbox_size - 3.0,
                checkbox_x + checkbox_size - 3.0,
                checkbox_y + 3.0,
                2.0,
                HIGHLIGHT_COLOR,
            );
        }
    }
}
