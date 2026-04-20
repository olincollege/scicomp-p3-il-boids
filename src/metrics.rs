//! Four metrics defined in Olfati-Saber's paper for evaluating flock quality

use crate::Boid;
use crate::constants::*;
use crate::math;

use macroquad::prelude::Vec2;

/// Computes connectivity with the equation `(n - c) / (n - 1)`.
/// `n` is the total number of boids,
/// `c` is the number of flocks (or components from the Laplacian).
///
/// Bypasses calculating the full Laplacian and instead uses union-find to count number of flocks.
/// Returns value between `1.0` and `0.0`.`1.0` when all boids are in a single flock, and `0.0`
/// when all boids are isolated.
pub fn relative_connectivity(boids: &[Boid], interaction_range: f32) -> f32 {
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
            let (norm_dist, _) = math::sigma_calc(boids[i].position, boids[j].position);
            if norm_dist < interaction_range {
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

/// Computes cohesion radius, the maximum distance from the flock center to any boid in the system.
///
/// Return value in macroquad screen coord units (pixels).
pub fn cohesion_radius(boids: &[Boid]) -> f32 {
    // Average position of all boids
    let center = boids.iter().fold(Vec2::ZERO, |acc, b| acc + b.position) / boids.len() as f32;

    // Take max distance a boid is from center
    boids
        .iter()
        .map(|b| (b.position - center).length())
        .fold(0.0f32, f32::max)
}

/// Computes deviation energy based on position of all boids in system.
///
/// For each boid, calculates squared deviation from desired distance to all other boids within
/// attraction range. Averages over all pairs and normalizes by `DESIRED_DISTANCE^2`.
///
/// Returns positive value roughly below `1.0` (not mathematically bounded) where `0.0` indicates
/// all boids are exactly at desired distance from each other.
pub fn normalized_deviation_energy(boids: &[Boid], interaction_range: f32) -> f32 {
    let mut total = 0.0;
    let mut edge_count = 0;

    for i in 0..boids.len() {
        for j in (i + 1)..boids.len() {
            let dist: f32 = (boids[i].position - boids[j].position).length();
            if dist < interaction_range {
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

/// Computes average velocity mismatch energy for the system average squared difference of each
/// boid's velocity from the average velocity of the system.
///
/// Return value is in units of pixels^2 / seconds^2.
pub fn normalized_velocity_mismatch(boids: &[Boid]) -> f32 {
    let avg_velocity =
        boids.iter().fold(Vec2::ZERO, |acc, b| acc + b.velocity) / (boids.len() as f32);

    let mismatch_energy: f32 = boids
        .iter()
        .map(|boid| (boid.velocity - avg_velocity).length_squared())
        .sum::<f32>()
        / 2.0;

    mismatch_energy / boids.len() as f32
}
