mod boid;
mod constants;
mod control_panel;
mod math;
mod metric_graph;
mod metrics;

use boid::Boid;
use constants::*;
use control_panel::{CONTROL_PANEL_HEIGHT, ControlPanel};
use macroquad::prelude::*;
use metric_graph::{MetricGraph, draw_sidebar};

fn window_conf() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_string(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        fullscreen: false,
        ..Default::default()
    }
}

fn init_boids(simulation_width: f32) -> Vec<Boid> {
    let mut boids = Vec::with_capacity(NUM_BOIDS);

    for _ in 0..NUM_BOIDS {
        let position = vec2(
            rand::gen_range(0.0, simulation_width),
            rand::gen_range(0.0, screen_height()),
        );

        let velocity =
            Vec2::from_angle(rand::gen_range(0.0, 2.0 * std::f32::consts::PI)) * INITIAL_SPEED;

        boids.push(Boid::new(position, velocity));
    }

    boids
}

fn init_graphs() -> Vec<MetricGraph> {
    vec![
        MetricGraph::new("Connectivity", HIGHLIGHT_COLOR, Some(1.0)),
        MetricGraph::new("Cohesion Radius", HIGHLIGHT_COLOR, None),
        MetricGraph::new("Deviation Energy", HIGHLIGHT_COLOR, None),
        MetricGraph::new("Velocity Mismatch", HIGHLIGHT_COLOR, None),
    ]
}

#[macroquad::main(window_conf)]
async fn main() {
    // Top level sim setup
    rand::srand(42);
    let mut sim_time = 0.0_f32;
    let simulation_width = (screen_width() - SIDEBAR_WIDTH - SIDEBAR_MARGIN).max(1.0);
    let mut kappa = DEFAULT_KAPPA;
    let mut constant_acceleration = true;

    // Initialize boids
    let mut boids = init_boids(simulation_width);
    let mut boids_prior: Vec<Boid> = vec![];

    // Initialize metric graphs
    let mut graphs = init_graphs();
    let control_panel = ControlPanel::default();

    // Main Loop
    loop {
        let dt = get_frame_time();
        sim_time += dt;
        clear_background(WHITE);

        if is_key_pressed(KeyCode::A) {
            kappa = (kappa - KAPPA_STEP).max(MIN_KAPPA);
        }
        if is_key_pressed(KeyCode::D) {
            kappa += KAPPA_STEP;
        }
        if is_key_pressed(KeyCode::W) {
            constant_acceleration = !constant_acceleration;
        }
        if is_key_pressed(KeyCode::R) {
            boids = init_boids(simulation_width);
            boids_prior.clear();
            graphs = init_graphs();
            sim_time = 0.0;
        }

        let interaction_range = DESIRED_DISTANCE * kappa;

        control_panel.draw(kappa, constant_acceleration);

        // Update and draw boids
        boids_prior.clone_from(&boids);
        for boid in &mut boids {
            boid.update(&boids_prior, dt, interaction_range, constant_acceleration);
            boid.draw();
        }

        // Update and draw metric graphs
        graphs[0].push(
            sim_time,
            metrics::relative_connectivity(&boids, interaction_range),
        );
        graphs[1].push(sim_time, metrics::cohesion_radius(&boids));
        graphs[2].push(
            sim_time,
            metrics::normalized_deviation_energy(&boids, interaction_range),
        );
        graphs[3].push(sim_time, metrics::normalized_velocity_mismatch(&boids));
        draw_sidebar(&graphs, control_panel.y + CONTROL_PANEL_HEIGHT + 16.0);

        next_frame().await
    }
}
