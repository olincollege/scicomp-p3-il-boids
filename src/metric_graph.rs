use macroquad::prelude::*;

pub const SIDEBAR_WIDTH: f32 = 320.0;

pub struct MetricGraph {
    label: &'static str,
    color: Color,
    samples: Vec<(f32, f32)>,
    fixed_max: Option<f32>,
}

impl MetricGraph {
    pub fn new(label: &'static str, color: Color, fixed_max: Option<f32>) -> Self {
        Self {
            label,
            color,
            samples: Vec::new(),
            fixed_max,
        }
    }

    pub fn push(&mut self, time: f32, value: f32) {
        self.samples.push((time, value));
    }

    pub fn draw(&self, x: f32, y: f32, w: f32, h: f32) {
        // draw_rectangle_lines(x, y, w, h, 1.0, LIGHTGRAY);

        let current = self.samples.last().map(|(_, value)| *value).unwrap_or(0.0);

        // Header
        draw_text(
            &format!("{}: {:.3}", self.label, current),
            x + 6.0,
            y + 14.0,
            18.0,
            DARKGRAY,
        );

        if self.samples.len() < 2 {
            return;
        }

        let y_max = self.fixed_max.unwrap_or_else(|| {
            nice_ceil(
                self.samples
                    .iter()
                    .map(|(_, value)| *value)
                    .fold(0.0_f32, f32::max),
            )
        });

        let top_label_h = 22.0;

        // Actual line plot area, not including header
        let plot_x = x;
        let plot_y = y + top_label_h;
        let plot_w = w;
        let plot_h = h - top_label_h;

        // Graph box
        draw_rectangle_lines(plot_x, plot_y, plot_w, plot_h, 2.0, LIGHTGRAY);

        let label_color = DARKGRAY;
        let label_size = 16.0;

        // Y axis label
        let y_axis_label = format!("{:.3}", y_max);
        draw_text(
            &y_axis_label,
            plot_x + plot_w - measure_text(&y_axis_label, None, 16, 1.0).width - 4.0,
            plot_y + 16.0,
            label_size,
            label_color,
        );

        // Graph line
        let sample_count = self.samples.len();
        let t_start = self.samples.first().map(|(time, _)| *time).unwrap_or(0.0);
        let t_end = self.samples.last().map(|(time, _)| *time).unwrap_or(0.0);
        let t_span = (t_end - t_start).max(1e-6);

        for idx in 1..sample_count {
            let (t1, prev) = self.samples[idx - 1];
            let (t2, curr) = self.samples[idx];
            let x1 = plot_x + ((t1 - t_start) / t_span).clamp(0.0, 1.0) * plot_w;
            let x2 = plot_x + ((t2 - t_start) / t_span).clamp(0.0, 1.0) * plot_w;
            let y1 = plot_y + plot_h - (prev / y_max).clamp(0.0, 1.0) * plot_h;
            let y2 = plot_y + plot_h - (curr / y_max).clamp(0.0, 1.0) * plot_h;
            draw_line(x1, y1, x2, y2, 2.0, self.color);
        }

        // Time axis labels
        draw_text(
            "time (s)",
            plot_x + plot_w - measure_text("time (s)", None, 16, 1.0).width - 4.0,
            plot_y + plot_h - 6.0,
            16.0,
            DARKGRAY,
        );

        let time_axis_labels_y = plot_y + plot_h + 14.0;

        draw_text("0", plot_x, time_axis_labels_y, 16.0, DARKGRAY);

        let right_time_label = format!("{:.1}", t_end);
        draw_text(
            &right_time_label,
            plot_x + plot_w - measure_text(&right_time_label, None, 16, 1.0).width - 4.0,
            time_axis_labels_y,
            16.0,
            DARKGRAY,
        );
    }
}

fn nice_ceil(value: f32) -> f32 {
    let exponent = value.log10().floor();
    let base = 10_f32.powf(exponent);
    let normalized = value / base;

    // Pick value ~1.2x greater than normalized value
    let rounded = (normalized * 1.2).ceil();

    rounded * base
}

pub fn draw_sidebar(graphs: &[MetricGraph]) {
    let panel_x = screen_width() - SIDEBAR_WIDTH;
    draw_rectangle(
        panel_x,
        0.0,
        SIDEBAR_WIDTH,
        screen_height(),
        Color::new(0.95, 0.96, 0.98, 0.92),
    );

    let top = 16.0;
    let bottom = 30.0;
    let gap = 40.0;
    let graph_count = graphs.len().max(1) as f32;
    let graph_height = (screen_height() - top - bottom - gap * (graph_count - 1.0)) / graph_count;

    for (i, graph) in graphs.iter().enumerate() {
        let y = top + i as f32 * (graph_height + gap);
        let x = panel_x + 12.0;
        let w = SIDEBAR_WIDTH - 24.0;
        let h = graph_height;
        graph.draw(x, y, w, h);
    }
}
