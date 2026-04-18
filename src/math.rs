use macroquad::prelude::Vec2;

pub const DESIRED_DISTANCE: f32 = 50.0;
pub const ATTRACTION_RANGE: f32 = 100.0;
pub const ATTRACTION_GAIN: f32 = 5.0;
pub const REPULSION_GAIN: f32 = 5.0;
pub const BUMP_FLATNESS: f32 = 0.2;

/// Calculates sigma normalized distance and gradient vector between two points.
pub fn sigma_calc(p1: Vec2, p2: Vec2) -> (f32, Vec2) {
    const EPSILON: f32 = 0.1;
    let diff = p2 - p1;
    let scaling_factor = (1.0 + EPSILON * diff.length_squared()).sqrt();

    // Sigma normalized distance
    let norm = (1.0 / EPSILON) * (scaling_factor - 1.0);

    // Sigma gradient vector
    let grad = diff / scaling_factor;

    (norm, grad)
}

/// Performs attraction and repulsion based on the normalized distance to another boid.
pub fn action_function(norm_dist: f32) -> f32 {
    let z = norm_dist - DESIRED_DISTANCE;
    let c = (ATTRACTION_GAIN - REPULSION_GAIN).abs() / (4.0 * ATTRACTION_GAIN * REPULSION_GAIN);
    let phi = 0.5
        * ((ATTRACTION_GAIN + REPULSION_GAIN) * sigmoid(z + c)
            + (ATTRACTION_GAIN - REPULSION_GAIN));
    let out = bump(norm_dist / ATTRACTION_RANGE) * phi;
    out
}

/// Sigmoid step function
pub fn sigmoid(x: f32) -> f32 {
    x / (1.0 + x.powi(2)).sqrt()
}

/// Smooths interaction between boids at the edge of the attraction range, preventing "jitter".
pub fn bump(norm_dist: f32) -> f32 {
    if norm_dist < BUMP_FLATNESS {
        1.0
    } else if norm_dist <= 1.0 {
        0.5 * (1.0
            + (std::f32::consts::PI * (norm_dist - BUMP_FLATNESS) / (1.0 - BUMP_FLATNESS)).cos())
    } else {
        0.0
    }
}
