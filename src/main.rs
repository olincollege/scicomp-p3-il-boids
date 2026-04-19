mod boid;
mod constants;
mod control_panel;
mod math;
mod metric_graph;
mod metrics;
mod sidebar;

use boid::Boid;
use constants::*;
use macroquad::prelude::*;
use sidebar::Sidebar;

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
    let mut sidebar = Sidebar::default();

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
            sidebar.reset();
            sim_time = 0.0;
        }

        let interaction_range = DESIRED_DISTANCE * kappa;

        // Update and draw boids
        boids_prior.clone_from(&boids);
        for boid in &mut boids {
            boid.update(&boids_prior, dt, interaction_range, constant_acceleration);
            boid.draw();
        }

        // Update metric graphs, draw sidebar
        sidebar.update_metric_graphs(sim_time, &boids, interaction_range);
        sidebar.draw(kappa, constant_acceleration);

        next_frame().await
    }
}
