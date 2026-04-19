use super::control_panel::{CONTROL_PANEL_HEIGHT, ControlPanel};
use super::metric_graph::MetricGraph;
use crate::constants::{HIGHLIGHT_COLOR, SIDEBAR_WIDTH};
use crate::{Boid, metrics};
use macroquad::prelude::*;

pub struct Sidebar {
    control_panel: ControlPanel,
    graphs: Vec<MetricGraph>,
    gap_below_panel: f32,
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            control_panel: ControlPanel::default(),
            graphs: vec![
                MetricGraph::new("Connectivity", HIGHLIGHT_COLOR, Some(1.0)),
                MetricGraph::new("Cohesion Radius", HIGHLIGHT_COLOR, None),
                MetricGraph::new("Deviation Energy", HIGHLIGHT_COLOR, None),
                MetricGraph::new("Velocity Mismatch", HIGHLIGHT_COLOR, None),
            ],
            gap_below_panel: 16.0,
        }
    }
}

impl Sidebar {
    pub fn reset(&mut self) {
        self.graphs.clear();
        self.graphs.extend([
            MetricGraph::new("Connectivity", HIGHLIGHT_COLOR, Some(1.0)),
            MetricGraph::new("Cohesion Radius", HIGHLIGHT_COLOR, None),
            MetricGraph::new("Deviation Energy", HIGHLIGHT_COLOR, None),
            MetricGraph::new("Velocity Mismatch", HIGHLIGHT_COLOR, None),
        ]);
    }

    pub fn update_metric_graphs(&mut self, sim_time: f32, boids: &[Boid], interaction_range: f32) {
        self.graphs[0].push(
            sim_time,
            metrics::relative_connectivity(boids, interaction_range),
        );
        self.graphs[1].push(sim_time, metrics::cohesion_radius(boids));
        self.graphs[2].push(
            sim_time,
            metrics::normalized_deviation_energy(boids, interaction_range),
        );
        self.graphs[3].push(sim_time, metrics::normalized_velocity_mismatch(boids));
    }

    pub fn draw(&self, kappa: f32, constant_acceleration: bool) {
        self.control_panel.draw(kappa, constant_acceleration);

        let panel_x = screen_width() - SIDEBAR_WIDTH;

        // Divider line
        draw_line(panel_x, 0.0, panel_x, screen_height(), 1.0, DARKGRAY);

        // Layout graph area
        let top = self.control_panel.y + CONTROL_PANEL_HEIGHT + self.gap_below_panel;
        let bottom = 30.0;
        let gap = 40.0;
        let graph_count = self.graphs.len().max(1) as f32;
        let graph_height =
            (screen_height() - top - bottom - gap * (graph_count - 1.0)) / graph_count;

        // Draw all graphs
        for (i, graph) in self.graphs.iter().enumerate() {
            let y = top + i as f32 * (graph_height + gap);
            let x = panel_x + 12.0;
            let w = SIDEBAR_WIDTH - 24.0;
            let h = graph_height;
            graph.draw(x, y, w, h);
        }
    }
}
