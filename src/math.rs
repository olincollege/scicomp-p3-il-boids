use macroquad::prelude::Vec2;

use crate::constants::*;

/// Calculates sigma normlized distance and gradient vector between two points.
pub fn sigma_calc(p1: Vec2, p2: Vec2) -> (f32, Vec2) {
    let diff = p2 - p1;
    let scaling_factor = (1.0 + SIGMA_EPSILON * diff.length_squared()).sqrt();

    // Sigma normalizaed distance
    let norm = (1.0 / SIGMA_EPSILON) * (scaling_factor - 1.0);

    // Sigma gradient vector
    let grad = diff / scaling_factor;

    (norm, grad)
}

/// Performs attraction and repulstion based on the normalized distance to another boid.
pub fn action_function(norm_dist: f32) -> f32 {
    let z = norm_dist - DESIRED_DISTANCE;
    let c =
        (ATTRACTION_GAIN - REPULSION_GAIN).abs() / (4.0 * ATTRACTION_GAIN * REPULSION_GAIN).sqrt();
    let phi = 0.5
        * ((ATTRACTION_GAIN + REPULSION_GAIN) * sigmoid(z + c)
            + (ATTRACTION_GAIN - REPULSION_GAIN));
    let out = bump(norm_dist / ATTRACTION_RANGE) * phi;
    return out;
}

pub fn adjacency_weight(p1: Vec2, p2: Vec2) -> f32 {
    let (norm_dist, _) = sigma_calc(p1, p2);
    return bump(norm_dist / ATTRACTION_RANGE);
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
