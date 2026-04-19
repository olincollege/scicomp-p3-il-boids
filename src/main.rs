mod boid;
mod constants;
mod math;
mod metric_graph;
mod metrics;

use boid::Boid;
use constants::*;
use macroquad::prelude::*;
use metric_graph::{MetricGraph, draw_sidebar};

fn window_conf() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_string(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        fullscreen: false,
        ..Default::default()
    }
}

fn init_boids(simulation_width: f32) -> Vec<Boid> {
    let mut boids = Vec::with_capacity(NUM_BOIDS);

    for _ in 0..NUM_BOIDS {
        let position = vec2(
            rand::gen_range(0.0, simulation_width),
            rand::gen_range(0.0, screen_height()),
        );

        let velocity =
            Vec2::from_angle(rand::gen_range(0.0, 2.0 * std::f32::consts::PI)) * INITIAL_SPEED;

        boids.push(Boid::new(position, velocity));
    }

    boids
}

fn init_graphs() -> Vec<MetricGraph> {
    vec![
        MetricGraph::new("Connectivity", BLACK, Some(1.0)),
        MetricGraph::new("Cohesion Radius", BLACK, None),
        MetricGraph::new("Deviation Energy", BLACK, None),
        MetricGraph::new("Velocity Mismatch", BLACK, None),
    ]
}

fn draw_control_panel(
    kappa: f32,
    constant_acceleration: bool,
    panel_x: f32,
    panel_y: f32,
    panel_w: f32,
) {
    let panel_h = 140.0;

    draw_rectangle_lines(panel_x, panel_y, panel_w, panel_h, 2.0, LIGHTGRAY);

    let text_x = panel_x + 12.0;
    let mut cursor_y = panel_y + 22.0;

    draw_text("Controls", text_x, cursor_y, 20.0, DARKGRAY);
    cursor_y += 22.0;

    draw_text("[a]", text_x, cursor_y, 18.0, DARKGRAY);
    draw_text(
        &format!("Interaction Range: {:.1}", kappa),
        text_x + 40.0,
        cursor_y,
        18.0,
        DARKGRAY,
    );
    draw_text("[d]", panel_x + panel_w - 30.0, cursor_y, 18.0, DARKGRAY);
    cursor_y += 18.0;

    draw_text(
        "(multiplier on desired distance)",
        text_x + 40.0,
        cursor_y,
        14.0,
        GRAY,
    );
    cursor_y += 22.0;

    let checkbox_size = 14.0;
    let checkbox_x = text_x;
    let checkbox_y = cursor_y - checkbox_size + 2.0;
    draw_rectangle_lines(
        checkbox_x,
        checkbox_y,
        checkbox_size,
        checkbox_size,
        1.0,
        DARKGRAY,
    );
    if constant_acceleration {
        draw_line(
            checkbox_x + 3.0,
            checkbox_y + checkbox_size * 0.6,
            checkbox_x + checkbox_size * 0.45,
            checkbox_y + checkbox_size - 3.0,
            2.0,
            DARKGRAY,
        );
        draw_line(
            checkbox_x + checkbox_size * 0.45,
            checkbox_y + checkbox_size - 3.0,
            checkbox_x + checkbox_size - 3.0,
            checkbox_y + 3.0,
            2.0,
            DARKGRAY,
        );
    }

    draw_text(
        "Constant Acceleration",
        checkbox_x + 24.0,
        cursor_y,
        18.0,
        DARKGRAY,
    );
    draw_text("[w]", panel_x + panel_w - 30.0, cursor_y, 18.0, DARKGRAY);
    cursor_y += 24.0;

    draw_text(
        "Restart with new parameters",
        text_x,
        cursor_y,
        18.0,
        DARKGRAY,
    );
    draw_text("[r]", panel_x + panel_w - 30.0, cursor_y, 18.0, DARKGRAY);
}

#[macroquad::main(window_conf)]
async fn main() {
    // Top level sim setup
    rand::srand(42);
    let mut sim_time = 0.0_f32;
    let simulation_width = (screen_width() - SIDEBAR_WIDTH - SIDEBAR_MARGIN).max(1.0);
    let mut kappa = DEFAULT_KAPPA;
    let mut constant_acceleration = true;

    // Initialize boids
    let mut boids = init_boids(simulation_width);
    let mut boids_prior: Vec<Boid> = vec![];

    // Initialize metric graphs
    let mut graphs = init_graphs();

    // Main Loop
    loop {
        let dt = get_frame_time();
        sim_time += dt;
        clear_background(WHITE);

        if is_key_pressed(KeyCode::A) {
            kappa = (kappa - KAPPA_STEP).max(MIN_KAPPA);
        }
        if is_key_pressed(KeyCode::D) {
            kappa += KAPPA_STEP;
        }
        if is_key_pressed(KeyCode::W) {
            constant_acceleration = !constant_acceleration;
        }
        if is_key_pressed(KeyCode::R) {
            boids = init_boids(simulation_width);
            boids_prior.clear();
            graphs = init_graphs();
            sim_time = 0.0;
        }

        let interaction_range = DESIRED_DISTANCE * kappa;

        let panel_x = screen_width() - SIDEBAR_WIDTH + 12.0;
        let panel_y = 16.0;
        let panel_w = SIDEBAR_WIDTH - 24.0;
        draw_control_panel(kappa, constant_acceleration, panel_x, panel_y, panel_w);

        // Update and draw boids
        boids_prior.clone_from(&boids);
        for boid in &mut boids {
            boid.update(&boids_prior, dt, interaction_range, constant_acceleration);
            boid.draw();
        }

        // Update and draw metric graphs
        graphs[0].push(
            sim_time,
            metrics::relative_connectivity(&boids, interaction_range),
        );
        graphs[1].push(sim_time, metrics::cohesion_radius(&boids));
        graphs[2].push(
            sim_time,
            metrics::normalized_deviation_energy(&boids, interaction_range),
        );
        graphs[3].push(sim_time, metrics::normalized_velocity_mismatch(&boids));
        let control_panel_h = 140.0;
        draw_sidebar(&graphs, panel_y + control_panel_h + 16.0);

        next_frame().await
    }
}
