use macroquad::prelude::*;

pub const DESIRED_DISTANCE: f32 = 100.0;
pub const ATTRACTION_RANGE: f32 = 200.0;
pub const ATTRACTION_GAIN: f32 = 5.0;
pub const REPULSION_GAIN: f32 = 5.0;
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
        let c = (ATTRACTION_GAIN - REPULSION_GAIN).abs()
            / (4.0 * ATTRACTION_GAIN * REPULSION_GAIN).sqrt();
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

    pub fn normalized_deviation_energy(boids: &[Boid]) -> f32 {
        let mut total = 0.0;
        let mut edge_count = 0;

        for i in 0..boids.len() {
            for j in (i + 1)..boids.len() {
                let dist: f32 = (boids[i].position - boids[j].position).length();
                if dist < ATTRACTION_RANGE {
                    total += (dist - DESIRED_DISTANCE).powi(2);
                    edge_count += 1;
                }
            }
        }

        if edge_count == 0 {
            return 0.0;
        }

        total / (edge_count as f32 * DESIRED_DISTANCE.powi(2))
    }

    pub fn normalized_velocity_mismatch(boids: &[Boid]) -> f32 {
        let mut total = 0.0;

        let avg_velocity =
            boids.iter().fold(Vec2::ZERO, |acc, b| acc + b.velocity) / (boids.len() as f32);

        for boid in boids {
            total += (boid.velocity - avg_velocity).length_squared();
        }

        return total / (boids.len() as f32 * 2.0);
    }

    pub fn relative_connectivity(boids: &[Boid]) -> f32 {
        let n = boids.len();
        if n <= 1 {
            return 1.0;
        }

        // Define union-find
        let mut parent: Vec<usize> = (0..n).collect();

        fn find(parent: &mut Vec<usize>, x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }

        // Link boids within attraction range
        for i in 0..n {
            for j in (i + 1)..n {
                let (norm_dist, _) = Boid::sigma_calc(boids[i].position, boids[j].position);
                if norm_dist < ATTRACTION_RANGE {
                    let ri = find(&mut parent, i);
                    let rj = find(&mut parent, j);
                    if ri != rj {
                        parent[ri] = rj;
                    }
                }
            }
        }

        // Filter uniques
        let flocks = (0..n).filter(|&i| find(&mut parent, i) == i).count();
        let rank = n - flocks;
        rank as f32 / (n - 1) as f32
    }

    pub fn cohesion_radius(boids: &[Boid]) -> f32 {
        let center = boids.iter().fold(Vec2::ZERO, |acc, b| acc + b.position) / boids.len() as f32;

        boids
            .iter()
            .map(|b| (b.position - center).length())
            .fold(0.0f32, f32::max)
    }
}
