mod boid;

use boid::Boid;

use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    let mut boids = Vec::new();

    // Initialize boids
    for _ in 0..100 {
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
