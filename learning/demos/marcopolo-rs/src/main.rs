// Global imports (needed for the simulation to run)
use crate::model::arena::Arena;
use std::f32::consts::PI;

mod model;

use krabmaga::simulate;

// knobs to play with
pub static NUMBER_OF_AGENTS: u32 = 5;
pub static TIMESTEPS_TO_SIMULATE: u64 = 200;
pub static ITERATIONS: u64 = 3;
pub static SAVE_OUTPUT: bool = true;
pub static SCALE: f32 = 1.0;

// global constants
pub static TOROIDAL: bool = false;
pub static TWOPI: f32 = 2.0 * PI;
pub static DISCRETIZATION: f32 = SCALE;

fn main() {
    let step = TIMESTEPS_TO_SIMULATE;

    let num_agents = NUMBER_OF_AGENTS;
    let dim: (f32, f32) = (50. * SCALE, 50. * SCALE);

    let state = Arena::new(dim, num_agents);

    simulate!(state, step, ITERATIONS);
}
