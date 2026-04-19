use crate::constants::*;
use crate::math;

use macroquad::prelude::*;

#[derive(Clone)]
pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
}

impl Boid {
    pub fn new(position: Vec2, velocity: Vec2) -> Self {
        Self { position, velocity }
    }

    pub fn draw(&self) {
        let forward = self.velocity.normalize_or_zero();
        let right = vec2(-forward.y, forward.x);
        let tip = self.position + forward * BOID_HEIGHT / 2.0;
        let left_base = self.position - forward * BOID_HEIGHT / 2.0 - right * BOID_BASE / 2.0;
        let right_base = self.position - forward * BOID_HEIGHT / 2.0 + right * BOID_BASE / 2.0;

        draw_triangle(tip, left_base, right_base, BLACK);
    }

    pub fn update(&mut self, boids: &[Boid], dt: f32) {
        self.velocity += self.alg_1(boids) * dt;
        self.velocity += self.avoid_borders() * dt;
        self.velocity += self.accelerate_to_target_speed() * dt;
        self.position += self.velocity * dt;
    }

    fn accelerate_to_target_speed(&self) -> Vec2 {
        let speed = self.velocity.length();
        if speed < TARGET_SPEED {
            return self.velocity.normalize_or_zero() * TARGET_SPEED_GAIN;
        }
        Vec2::ZERO
    }

    fn avoid_borders(&self) -> Vec2 {
        let mut steer = Vec2::ZERO;
        if self.position.x < BORDER_THRESHOLD {
            steer.x += BORDER_AVOIDANCE_GAIN;
        } else if self.position.x
            > screen_width() - BORDER_THRESHOLD - SIDEBAR_WIDTH - SIDEBAR_MARGIN
        {
            steer.x -= BORDER_AVOIDANCE_GAIN;
        }

        if self.position.y < BORDER_THRESHOLD {
            steer.y += BORDER_AVOIDANCE_GAIN;
        } else if self.position.y > screen_height() - BORDER_THRESHOLD {
            steer.y -= BORDER_AVOIDANCE_GAIN;
        }
        return steer;
    }

    fn wrap_on_edges(&mut self) {
        let sim_width = screen_width() - BORDER_THRESHOLD - SIDEBAR_WIDTH - SIDEBAR_MARGIN;
        if self.position.x < 0.0 {
            self.position.x += sim_width;
        } else if self.position.x > sim_width {
            self.position.x -= sim_width;
        }

        if self.position.y < 0.0 {
            self.position.y += screen_height();
        } else if self.position.y > screen_height() {
            self.position.y -= screen_height();
        }
    }

    fn alg_1(&self, boids: &[Boid]) -> Vec2 {
        return self.gradient_term(boids) + self.consensus_term(boids);
    }

    /// Calculate full gradient term for a boid based on all other boids in the system.
    fn gradient_term(&self, boids: &[Boid]) -> Vec2 {
        let mut total_gradient = Vec2::ZERO;
        for boid in boids {
            if std::ptr::eq(boid, self) {
                continue;
            }
            let (norm_dist, grad) = math::sigma_calc(self.position, boid.position);
            let action = math::action_function(norm_dist);
            total_gradient += action * grad;
        }
        return total_gradient;
    }

    fn consensus_term(&self, boids: &[Boid]) -> Vec2 {
        let mut total_consensus = Vec2::ZERO;
        for boid in boids {
            if std::ptr::eq(boid, self) {
                continue;
            }
            let adjacency = math::adjacency_weight(self.position, boid.position);
            total_consensus += adjacency * (boid.velocity - self.velocity);
        }
        return total_consensus;
    }
}
