use crate::Boid;
use crate::constants::*;
use crate::math;

use macroquad::prelude::Vec2;

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
            let (norm_dist, _) = math::sigma_calc(boids[i].position, boids[j].position);
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
