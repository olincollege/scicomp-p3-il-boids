mod boid;
mod constants;
mod math;
mod metric_graph;
mod metrics;

use boid::Boid;
use constants::*;
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

#[macroquad::main(window_conf)]
async fn main() {
    // Top level sim setup
    rand::srand(42);
    let mut sim_time = 0.0_f32;
    let simulation_width = (screen_width() - SIDEBAR_WIDTH - SIDEBAR_MARGIN).max(1.0);

    // Initialize boids
    let mut boids = Vec::with_capacity(NUM_BOIDS);
    let mut boids_prior: Vec<Boid> = vec![];

    for _ in 0..NUM_BOIDS {
        let position = vec2(
            rand::gen_range(0.0, simulation_width),
            rand::gen_range(0.0, screen_height()),
        );

        let velocity =
            Vec2::from_angle(rand::gen_range(0.0, 2.0 * std::f32::consts::PI)) * INITIAL_SPEED;

        boids.push(Boid::new(position, velocity));
    }

    // Initialize metric graphs
    let mut graphs = vec![
        MetricGraph::new("Connectivity", BLACK, Some(1.0)),
        MetricGraph::new("Cohesion Radius", BLACK, None),
        MetricGraph::new("Deviation Energy", BLACK, None),
        MetricGraph::new("Velocity Mismatch", BLACK, None),
    ];

    // Main Loop
    loop {
        let dt = get_frame_time();
        sim_time += dt;
        clear_background(WHITE);

        // Update and draw boids
        boids_prior.clone_from(&boids);
        for boid in &mut boids {
            boid.update(&boids_prior, dt);
            boid.draw();
        }

        // Update and draw metric graphs
        graphs[0].push(sim_time, metrics::relative_connectivity(&boids));
        graphs[1].push(sim_time, metrics::cohesion_radius(&boids));
        graphs[2].push(sim_time, metrics::normalized_deviation_energy(&boids));
        graphs[3].push(sim_time, metrics::normalized_velocity_mismatch(&boids));
        draw_sidebar(&graphs);

        next_frame().await
    }
}
