# Flocking Simulation

Boid flocking simulation, replicating results from the paper
[R. Olfati-Saber, "Flocking for multi-agent dynamic systems: algorithms and theory", 2006](https://ieeexplore.ieee.org/abstract/document/1605401).

## Usage

The simulation can be run online
[here!](https://olincollege.github.io/scicomp-p3-il-boids/)

Alternatively, you can clone the repository and run the code locally:

### Rust installation

**Linux/macOS:** (command from [rustup.rs](https://rustup.rs/))

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:** Download and run `rustup-init.exe` from
[rustup.rs](https://rustup.rs/).

### Clone and Run

```bash
# Clone the repository
git clone https://github.com/olincollege/scicomp-p3-il-boids.git
cd scicomp-p3-il-boids

# Run simulation
cargo run
```

## Simulation Controls

- [a/d]: Increases and decreases the interaction range of the boids (at what
  distance boids will attract to each other). This is a multiplier on the
  desired distance, so when this value is set to `2.0` boids will be attracted
  to each other when they are within 2x the desired distance. The value is
  clamped between `1.0` and `3.0`.
- [w]: Toggles a small constant acceleration that keeps the simulation moving
  when boids are arranged in a perfect lattice. See
  [Differences from Olfati-Saber, Constant Acceleration](#constant-acceleration)
  for more details.
- [r]: Resets the simulation, applying the current interaction range and
  constant acceleration settings.

## Simulation Overview

### Background

In 1986, [Craig Reynolds](https://www.red3d.com/cwr/papers/1987/SIGGRAPH87.pdf)
introduced three rules to simulate the flocking behavior of bird-oid objects:

1. Flock Centering: attempt to stay close to nearby flockmates.
2. Collision Avoidance: avoid collisions with nearby flockmates.
3. Velocity Matching: attempt to match velocity with nearby flockmates.

These three rules became the core of "Boids" flocking simulations.

### R. Olfati-Saber, 2006

In 2006, Reza Olfati-Saber published
[this paper](https://ieeexplore.ieee.org/abstract/document/1605401), creating
three algorithms for flocking that built on Reynolds' core three rules. The
three algorithms are:

1. Basic flocking
2. Flocking with leader following
3. Flocking with leader following and obstacle avoidance

In this project, I implement and expand upon the first algorithm, basic
flocking. There is no top down force in the simulation. Instead, the flocks are
an emergent behavior of each boid follows the flocking algorithm. The basic
flocking which consists of two main parts:

1. Gradient-based term: This term pushes the boid to a set desired distance from
   its neighbors, attracting to boids that are too far away and repelling from
   boids that are too close. Formally, this is the negative gradient of the
   collective potential energy function, which is minimized when all agents are
   at the desired distance..
2. Consensus term: This is the velocity matching term, pushing the boid's
   velocity towards a weighted average of its neighbors' velocities, based on
   distance.

### Differences from Olfati-Saber

#### Constant Acceleration

Under Olfati-Saber's basic flocking algorithm, once boids arranged into a
perfect lattice (all spaced at the desired distance) they have have no incentive
to continue moving and end up nearly stationary. To keep the simulation moving,
I added a small constant acceleration to each boid. This does change which
interaction ranges lead to stable flocking, and thus this constant acceleration
is togglable.

#### Border Avoidance

Olfati-Saber's simulation has no borders, and boids are allowed to fly out of
frame. I wanted to keep all boids in frame, so I added a force that pushes boids
back towards the center of the screen when they get close to the border.

## Results

### Metrics
