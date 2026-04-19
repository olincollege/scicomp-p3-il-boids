use macroquad::prelude::*;

use crate::constants::SIDEBAR_WIDTH;

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
        let panel_h = CONTROL_PANEL_HEIGHT;

        draw_rectangle_lines(self.x, self.y, self.w, panel_h, 2.0, LIGHTGRAY);

        let text_x = self.x + 12.0;
        let mut cursor_y = self.y + 22.0;

        draw_text("Controls", text_x, cursor_y, 20.0, DARKGRAY);
        cursor_y += 22.0;

        draw_text("[a]", text_x, cursor_y, 18.0, DARKGRAY);
        draw_text(
            &format!("Interaction Range: {:.1}", kappa),
            text_x + 40.0,
            cursor_y,
            18.0,
            DARKGRAY,
        );
        draw_text("[d]", self.x + self.w - 30.0, cursor_y, 18.0, DARKGRAY);
        cursor_y += 18.0;

        draw_text(
            "(multiplier on desired distance)",
            text_x + 40.0,
            cursor_y,
            14.0,
            GRAY,
        );
        cursor_y += 22.0;

        let checkbox_size = 14.0;
        let checkbox_x = text_x;
        let checkbox_y = cursor_y - checkbox_size + 2.0;
        draw_rectangle_lines(
            checkbox_x,
            checkbox_y,
            checkbox_size,
            checkbox_size,
            1.0,
            DARKGRAY,
        );
        if constant_acceleration {
            draw_line(
                checkbox_x + 3.0,
                checkbox_y + checkbox_size * 0.6,
                checkbox_x + checkbox_size * 0.45,
                checkbox_y + checkbox_size - 3.0,
                2.0,
                DARKGRAY,
            );
            draw_line(
                checkbox_x + checkbox_size * 0.45,
                checkbox_y + checkbox_size - 3.0,
                checkbox_x + checkbox_size - 3.0,
                checkbox_y + 3.0,
                2.0,
                DARKGRAY,
            );
        }

        draw_text(
            "Constant Acceleration",
            checkbox_x + 24.0,
            cursor_y,
            18.0,
            DARKGRAY,
        );
        draw_text("[w]", self.x + self.w - 30.0, cursor_y, 18.0, DARKGRAY);
        cursor_y += 24.0;

        draw_text(
            "Restart with new parameters",
            text_x,
            cursor_y,
            18.0,
            DARKGRAY,
        );
        draw_text("[r]", self.x + self.w - 30.0, cursor_y, 18.0, DARKGRAY);
    }
}
