// Global imports (needed for the simulation to run)
use crate::model::arena::Arena;
use std::f32::consts::PI;

mod model;

use krabmaga::simulate;

pub static TOROIDAL: bool = false;
pub static TWOPI: f32 = 2.0 * PI;
pub static SCALE: f32 = 1.0;
pub static DISCRETIZATION: f32 = SCALE;
pub static SAVE_OUTPUT: bool = true;

fn main() {
    let step = 200;

    let num_agents = 5;
    let dim: (f32, f32) = (50. * SCALE, 50. * SCALE);

    let state = Arena::new(dim, num_agents);

    simulate!(state, step, 1);
}
