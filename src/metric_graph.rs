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

    draw_text("Realtime Metrics", panel_x + 16.0, 28.0, 26.0, DARKGRAY);

    let top = 44.0;
    let bottom = 16.0;
    let gap = 12.0;
    let graph_count = graphs.len().max(1) as f32;
    let graph_height = (screen_height() - top - bottom - gap * (graph_count - 1.0)) / graph_count;

    for (i, graph) in graphs.iter().enumerate() {
        let y = top + i as f32 * (graph_height + gap);
        let x = panel_x + 12.0;
        let w = SIDEBAR_WIDTH - 24.0;
        let h = graph_height;

        draw_rectangle_lines(x, y, w, h, 1.0, LIGHTGRAY);

        let current = graph.samples.last().map(|(_, value)| *value).unwrap_or(0.0);
        draw_text(
            &format!("{}: {:.3}", graph.label, current),
            x + 8.0,
            y + 20.0,
            20.0,
            DARKGRAY,
        );

        if graph.samples.len() < 2 {
            continue;
        }

        let mut y_max = graph.fixed_max.unwrap_or_else(|| {
            graph
                .samples
                .iter()
                .map(|(_, value)| *value)
                .fold(0.0_f32, f32::max)
                .max(1.0)
        });
        y_max *= 1.05;

        let plot_x = x + 8.0;
        let plot_y = y + 28.0;
        let plot_w = w - 16.0;
        let plot_h = h - 36.0;

        draw_line(
            plot_x,
            plot_y + plot_h,
            plot_x + plot_w,
            plot_y + plot_h,
            1.0,
            GRAY,
        );

        let sample_count = graph.samples.len();
        let t_start = graph.samples.first().map(|(time, _)| *time).unwrap_or(0.0);
        let t_end = graph.samples.last().map(|(time, _)| *time).unwrap_or(0.0);
        let t_span = (t_end - t_start).max(1e-6);

        for idx in 1..sample_count {
            let (t1, prev) = graph.samples[idx - 1];
            let (t2, curr) = graph.samples[idx];
            let x1 = plot_x + ((t1 - t_start) / t_span).clamp(0.0, 1.0) * plot_w;
            let x2 = plot_x + ((t2 - t_start) / t_span).clamp(0.0, 1.0) * plot_w;
            let y1 = plot_y + plot_h - (prev / y_max).clamp(0.0, 1.0) * plot_h;
            let y2 = plot_y + plot_h - (curr / y_max).clamp(0.0, 1.0) * plot_h;
            draw_line(x1, y1, x2, y2, 2.0, graph.color);
        }

        draw_text(
            &format!("t = {:.1}s", t_end),
            plot_x,
            plot_y + plot_h - 6.0,
            16.0,
            DARKGRAY,
        );
    }
}
