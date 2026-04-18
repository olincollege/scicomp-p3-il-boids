mod boid;

use boid::Boid;
use macroquad::prelude::*;

const NUM_BOIDS: usize = 100;

fn window_conf() -> Conf {
    Conf {
        window_title: "Boid Simulation".to_string(),
        window_width: 1200,
        window_height: 800,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut boids = Vec::with_capacity(NUM_BOIDS);

    rand::srand(42);

    // Initialize boids
    for _ in 0..NUM_BOIDS {
        let position = vec2(
            rand::gen_range(0.0, screen_width()),
            rand::gen_range(0.0, screen_height()),
        );

        const INITIAL_SPEED: f32 = 100.0;
        let velocity =
            Vec2::from_angle(rand::gen_range(0.0, 2.0 * std::f32::consts::PI)) * INITIAL_SPEED;

        boids.push(Boid::new(position, velocity));
    }

    let mut boids_prior: Vec<Boid> = vec![];

    loop {
        let dt = get_frame_time();
        clear_background(WHITE);

        // Snapshot of current boid state for use in updates
        boids_prior.clone_from(&boids);

        for boid in &mut boids {
            boid.update(&boids_prior, dt);
        }

        for boid in &boids {
            boid.draw();
        }

        println!(
            "Deviation energy: {:<10.3}, Velocity mismatch: {:<10.3}, Connectivity: {:<10.3}, Cohesion radius: {:<10.3}",
            Boid::normalized_deviation_energy(&boids_prior),
            Boid::normalized_velocity_mismatch(&boids_prior),
            Boid::relative_connectivity(&boids_prior),
            Boid::cohesion_radius(&boids_prior)
        );

        next_frame().await
    }
}
