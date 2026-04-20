//! Main entry point and simulation loop

mod boid;
mod constants;
mod math;
mod metrics;
mod ui;

use macroquad::prelude::*;

use boid::Boid;
use constants::*;
use ui::sidebar::Sidebar;

/// Window configuration for macroquad
fn window_conf() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_string(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        fullscreen: false,
        ..Default::default()
    }
}

/// Initializes boids with random positions and velocities
fn init_boids() -> Vec<Boid> {
    let mut boids = Vec::with_capacity(NUM_BOIDS);

    for _ in 0..NUM_BOIDS {
        let position = vec2(
            rand::gen_range(0.0, screen_width() - SIDEBAR_WIDTH - SIDEBAR_MARGIN),
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
    // Sim state variables
    rand::srand(42);
    let mut sim_time = 0.0_f32;
    let mut active_kappa = DEFAULT_KAPPA;
    let mut active_constant_acceleration = true;
    let mut pending_kappa = DEFAULT_KAPPA;
    let mut pending_constant_acceleration = true;

    // Initialize boids
    let mut boids = init_boids();
    let mut boids_prior: Vec<Boid> = vec![];

    // Initialize metric graphs
    let mut sidebar = Sidebar::default();

    // Main Loop
    loop {
        let dt = get_frame_time();
        sim_time += dt;
        clear_background(WHITE);

        // Handle input
        if is_key_pressed(KeyCode::A) {
            pending_kappa = (pending_kappa - KAPPA_STEP).clamp(MIN_KAPPA, MAX_KAPPA);
        }
        if is_key_pressed(KeyCode::D) {
            pending_kappa = (pending_kappa + KAPPA_STEP).clamp(MIN_KAPPA, MAX_KAPPA);
        }
        if is_key_pressed(KeyCode::W) {
            pending_constant_acceleration = !pending_constant_acceleration;
        }
        if is_key_pressed(KeyCode::R) {
            boids = init_boids();
            boids_prior.clear();
            sidebar.reset();
            sim_time = 0.0;
            active_kappa = pending_kappa;
            active_constant_acceleration = pending_constant_acceleration;
        }

        let interaction_range = DESIRED_DISTANCE * active_kappa;

        // Update and draw boids
        boids_prior.clone_from(&boids);
        for boid in &mut boids {
            boid.update(
                &boids_prior,
                dt,
                interaction_range,
                active_constant_acceleration,
            );
            boid.draw();
        }

        // Update metric graphs, draw sidebar
        sidebar.update_metric_graphs(sim_time, &boids, interaction_range);
        sidebar.draw(pending_kappa, pending_constant_acceleration);

        next_frame().await
    }
}
