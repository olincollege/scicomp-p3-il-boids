mod boid;

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
        boids.push(boid::Boid::new(position, velocity));
    }

    loop {
        let dt = get_frame_time();
        clear_background(WHITE);
        let prev_boids = boids.clone();

        for boid in &mut boids {
            boid.update(&prev_boids, dt);
        }

        for boid in &boids {
            boid.draw();
        }
        next_frame().await
    }
}
