mod boid;

use boid::Boid;
use macroquad::prelude::*;

const NUM_BOIDS: usize = 250;

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

    // Initialize boids
    for _ in 0..NUM_BOIDS {
        let position = vec2(
            rand::gen_range(0.0, screen_width()),
            rand::gen_range(0.0, screen_height()),
        );
        let velocity = vec2(
            rand::gen_range(-100.0, 100.0),
            rand::gen_range(-100.0, 100.0),
        );
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
        next_frame().await
    }
}
