use macroquad::prelude::*;

pub const DESIRED_DISTANCE: f32 = 50.0;
pub const ATTRACTION_RANGE: f32 = 100.0;
pub const ATTRACTION_GAIN: f32 = 1.0;
pub const REPULSION_GAIN: f32 = 1.0;
pub const BUMP_FLATNESS: f32 = 0.5;

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
        const BASE: f32 = 8.0;
        const HEIGHT: f32 = 18.0;

        let forward = self.velocity.normalize_or_zero();
        let right = vec2(-forward.y, forward.x);
        let tip = self.position + forward * HEIGHT / 2.0;
        let left_base = self.position - forward * HEIGHT / 2.0 - right * BASE / 2.0;
        let right_base = self.position - forward * HEIGHT / 2.0 + right * BASE / 2.0;

        draw_triangle(tip, left_base, right_base, DARKGRAY);
    }

    pub fn update(&mut self, boids: &[Boid], dt: f32) {
        self.velocity += self.alg_1(boids) * dt;
        self.position += self.velocity * dt;

        if self.position.x < 0.0 {
            self.position.x += screen_width();
        } else if self.position.x > screen_width() {
            self.position.x -= screen_width();
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
            let (norm_dist, grad) = Self::sigma_calc(self.position, boid.position);
            let action = self.action_function(norm_dist);
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
    fn action_function(&self, norm_dist: f32) -> f32 {
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
