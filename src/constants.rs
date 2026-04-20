//! Simulation constants for algorithm and appearance

use macroquad::prelude::Color;

// ==== SIMULATION CONTROL CONSTANTS ====
pub const DESIRED_DISTANCE: f32 = 70.0;

/// Interaction range multiplier (ATTRACTION_RANGE = DESIRED_DISTANCE * KAPPA)
pub const DEFAULT_KAPPA: f32 = 1.5;
pub const KAPPA_STEP: f32 = 0.1;
pub const MIN_KAPPA: f32 = 1.0;
pub const MAX_KAPPA: f32 = 4.0;

pub const ATTRACTION_GAIN: f32 = 5.0;
pub const REPULSION_GAIN: f32 = 8.0;
/// Set to 0.2 in paper
pub const BUMP_FLATNESS: f32 = 0.2;
/// Arbitrary small value, set to 0.1 in paper
pub const SIGMA_EPSILON: f32 = 0.1;

// ==== SIMULATION INITIALIZATION ====
pub const NUM_BOIDS: usize = 100;
pub const INITIAL_SPEED: f32 = 50.0;

// ==== BOID TARGET SPEED ====
pub const TARGET_SPEED: f32 = 50.0;
pub const TARGET_SPEED_GAIN: f32 = 5.0;

// ==== BORDER AVOIDANCE ====
pub const BORDER_THRESHOLD: f32 = 100.0;
pub const BORDER_AVOIDANCE_GAIN: f32 = 30.0;

// ==== BOID TRIANGLE SHAPE ====
pub const BOID_BASE: f32 = 10.0;
pub const BOID_HEIGHT: f32 = 18.0;

// ==== WINDOW ====
pub const WINDOW_WIDTH: i32 = 1400;
pub const WINDOW_HEIGHT: i32 = 800;
pub const WINDOW_TITLE: &str = "Boid Simulation";
pub const SIDEBAR_WIDTH: f32 = 320.0;
pub const SIDEBAR_MARGIN: f32 = 20.0;
pub const HIGHLIGHT_COLOR: Color = Color::from_rgba(240, 158, 22, 255);
pub const CONTROL_PANEL_HEIGHT: f32 = 140.0;
