//! Boid object, implementing Olfati-Saber's flocking algorithm #1

use crate::constants::*;
use crate::math;

use macroquad::prelude::*;

#[derive(Clone)]
/// A single boid agent, following Olfati-Saber's flocking algorithm #1
pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
}

impl Boid {
    pub fn new(position: Vec2, velocity: Vec2) -> Self {
        Self { position, velocity }
    }

    /// Draw the boid as a triangle pointing in the direction of its velocity
    pub fn draw(&self) {
        let forward = self.velocity.normalize_or_zero();
        let right = vec2(-forward.y, forward.x);
        let tip = self.position + forward * BOID_HEIGHT / 2.0;
        let left_base = self.position - forward * BOID_HEIGHT / 2.0 - right * BOID_BASE / 2.0;
        let right_base = self.position - forward * BOID_HEIGHT / 2.0 + right * BOID_BASE / 2.0;

        draw_triangle(tip, left_base, right_base, BLACK);
    }

    /// Per frame update of the boid's position and velocity.
    /// Applies algorithm #1, as well as border avoidance and target speed acceleration.
    pub fn update(
        &mut self,
        boids: &[Boid],
        dt: f32,
        interaction_range: f32,
        constant_acceleration: bool,
    ) {
        // Algorithm 1
        self.velocity += self.gradient_term(boids, interaction_range) * dt;
        self.velocity += self.consensus_term(boids, interaction_range) * dt;

        // Personal additions: border avoidance and target speed
        self.velocity += self.avoid_borders() * dt;
        if constant_acceleration {
            self.velocity += self.accelerate_to_target_speed() * dt;
        }

        self.position += self.velocity * dt;
    }

    /// Accelerate in the same direction towards a target speed, if below it
    fn accelerate_to_target_speed(&self) -> Vec2 {
        let speed = self.velocity.length();
        if speed < TARGET_SPEED {
            return self.velocity.normalize_or_zero() * TARGET_SPEED_GAIN;
        }
        Vec2::ZERO
    }

    /// Apply constant force pushing away from borders when within a threshold
    /// Used instead of wrapping around the screen edges.
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

    /// Wrap the boid's position around the screen edges, creating a toroidal space.
    /// Unused, but an alternative to border avoidance.
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

    /// Apply a force moving boid towards a desired distance away from other boids, attracting
    /// to boids that are too far away and repelling from boids that are too close.
    ///
    /// Formally, this is the negative gradient of the collective potential energy function,
    /// which is minimized when all agents are at the desired distance.
    fn gradient_term(&self, boids: &[Boid], interaction_range: f32) -> Vec2 {
        let mut total_gradient = Vec2::ZERO;
        for boid in boids {
            if std::ptr::eq(boid, self) {
                continue;
            }
            let (norm_dist, grad) = math::sigma_calc(self.position, boid.position);
            let action = math::action_function(norm_dist, interaction_range);
            total_gradient += action * grad;
        }
        return total_gradient;
    }

    /// Velocity matching term, pushing the boid's velocity towards a weighted average of its
    /// neighbors' velocities, based on distance
    fn consensus_term(&self, boids: &[Boid], interaction_range: f32) -> Vec2 {
        let mut total_consensus = Vec2::ZERO;
        for boid in boids {
            if std::ptr::eq(boid, self) {
                continue;
            }
            let adjacency = math::adjacency_weight(self.position, boid.position, interaction_range);
            total_consensus += adjacency * (boid.velocity - self.velocity);
        }
        return total_consensus;
    }
}
