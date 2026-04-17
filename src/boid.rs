use macroquad::prelude::*;

pub const DESIRED_DISTANCE: f32 = 100.0;
pub const ATTRACTION_RANGE: f32 = 200.0;
pub const ATTRACTION_GAIN: f32 = 1.0;
pub const REPULSION_GAIN: f32 = 1.0;
pub const BUMP_FLATNESS: f32 = 0.2;

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
        // Isosceles triangle
        const BASE: f32 = 10.0;
        const HEIGHT: f32 = 18.0;

        let forward = self.velocity.normalize_or_zero();
        let right = vec2(-forward.y, forward.x);
        let tip = self.position + forward * HEIGHT / 2.0;
        let left_base = self.position - forward * HEIGHT / 2.0 - right * BASE / 2.0;
        let right_base = self.position - forward * HEIGHT / 2.0 + right * BASE / 2.0;

        draw_triangle(tip, left_base, right_base, BLACK);
    }

    pub fn update(&mut self, boids: &[Boid], dt: f32) {
        let gradient = self.gradient_term(boids) * 10.0;
        let consensus = self.consensus_term(boids) * 0.2;
        let fly_to_center = self.fly_to_center(boids);
        let boundary_force = self.keep_within_bounds();

        self.velocity += gradient * dt;
        self.velocity += consensus * dt;
        self.velocity += fly_to_center * dt;
        self.velocity += boundary_force * dt;

        println!(
            "Gradient: {:<10.2}, Consensus: {:<10.2}, Fly to Center: {:<10.2}, Boundary Force: {:<10.2}",
            gradient.length(),
            consensus.length(),
            fly_to_center.length(),
            boundary_force.length()
        );

        // Limit speed
        self.velocity = self.limit_speed();

        self.position += self.velocity * dt;
    }

    fn limit_speed(&self) -> Vec2 {
        const MAX_SPEED: f32 = 600.0;
        if self.velocity.length() > MAX_SPEED {
            self.velocity.normalize() * MAX_SPEED
        } else {
            self.velocity
        }
    }

    /// Calculate full gradient term for a boid based on all other boids in the system.
    fn gradient_term(&self, boids: &[Boid]) -> Vec2 {
        let mut total_gradient = Vec2::ZERO;
        for boid in boids {
            let (norm_dist, grad) = Self::sigma_calc(self.position, boid.position);
            let action = Self::action_function(norm_dist);
            total_gradient += action * grad;
        }
        return total_gradient;
    }

    fn consensus_term(&self, boids: &[Boid]) -> Vec2 {
        let mut total_consensus = Vec2::ZERO;
        for boid in boids {
            let adjacency = self.spatial_adjacency_matrix(boid);
            total_consensus += adjacency * (boid.velocity - self.velocity);
        }
        return total_consensus;
    }

    fn fly_to_center(&self, boids: &[Boid]) -> Vec2 {
        const CENTERING_FACTOR: f32 = 8.0;

        // Fly the to center which is a weighted average of other birds in the range, based on bump(sigma dist)
        let mut center = Vec2::ZERO;
        let mut total_weight = 0.0;
        for boid in boids {
            if std::ptr::eq(boid, self) {
                continue;
            }
            let (norm_dist, _) = Self::sigma_calc(self.position, boid.position);
            let weight = Self::bump(norm_dist / ATTRACTION_RANGE);
            center += weight * boid.position;
            total_weight += weight;
        }

        if total_weight == 0.0 {
            return Vec2::ZERO;
        }

        center /= total_weight;

        return (center - self.position) * CENTERING_FACTOR;
    }

    fn keep_within_bounds(&self) -> Vec2 {
        const MARGIN: f32 = 200.0;
        const BOUNDARY_FORCE: f32 = 100.0;

        let mut force = Vec2::ZERO;

        if self.position.x < MARGIN {
            force.x += BOUNDARY_FORCE;
        } else if self.position.x > screen_width() - MARGIN {
            force.x -= BOUNDARY_FORCE;
        }

        if self.position.y < MARGIN {
            force.y += BOUNDARY_FORCE;
        } else if self.position.y > screen_height() - MARGIN {
            force.y -= BOUNDARY_FORCE;
        }

        return force;
    }

    /// Calculates sigma normlized distance and gradient vector between two points.
    fn sigma_calc(p1: Vec2, p2: Vec2) -> (f32, Vec2) {
        const EPSILON: f32 = 0.1;
        let diff = p2 - p1;
        let scaling_factor = (1.0 + EPSILON * diff.length_squared()).sqrt();

        // Sigma normalizaed distance
        let norm = (1.0 / EPSILON) * (scaling_factor - 1.0);

        // Sigma gradient vector
        let grad = diff / scaling_factor;

        (norm, grad)
    }

    /// Performs attraction and repulstion based on the normalized distance to another boid.
    fn action_function(norm_dist: f32) -> f32 {
        let z = norm_dist - DESIRED_DISTANCE;
        let c = (ATTRACTION_GAIN - REPULSION_GAIN).abs() / (4.0 * ATTRACTION_GAIN * REPULSION_GAIN);
        let phi = 0.5
            * ((ATTRACTION_GAIN + REPULSION_GAIN) * Self::sigmoid(z + c)
                + (ATTRACTION_GAIN - REPULSION_GAIN));
        let out = Self::bump(norm_dist / ATTRACTION_RANGE) * phi;
        return out;
    }

    fn spatial_adjacency_matrix(&self, other: &Boid) -> f32 {
        let (norm_dist, _) = Self::sigma_calc(self.position, other.position);
        return Self::bump(norm_dist / ATTRACTION_RANGE);
    }

    /// Sigmoid step function
    fn sigmoid(x: f32) -> f32 {
        x / (1.0 + x.powi(2)).sqrt()
    }

    /// Smooths interaction between boids at the edge of the attraction range, preventing "jitter".
    fn bump(norm_dist: f32) -> f32 {
        if norm_dist < BUMP_FLATNESS {
            1.0
        } else if norm_dist <= 1.0 {
            0.5 * (1.0
                + (std::f32::consts::PI * (norm_dist - BUMP_FLATNESS) / (1.0 - BUMP_FLATNESS))
                    .cos())
        } else {
            0.0
        }
    }
}
