/*!
 * The main module instantiates the world and starts the simulation.
 * 
 * A different main function is used if the `visualization` feature is enabled.
 * By default, the simulation is run without any visualization.
 * 
 * This module's layout is borrowed from the krABMaga template.
 */

// Global imports (needed for the simulation to run)
use crate::model::world::World;
mod model;

use krabmaga::simulate;
use nyx_space::time::Epoch;

/// Length of a single simulation step, in seconds.
pub static DT: f32 = 1.0 / 60.0;
/// number of satellites to spawn
pub static NUM_AGENTS: u32 = 1;

fn main() {
    // init for cosmic params
    let dt = Epoch::from_gregorian_tai_at_midnight(2021, 3, 4);
    // number of steps to run
    let step = 1000;
    // number of repetitions
    let rep = 1;
    // spawn agents in the world
    let state = World::new(NUM_AGENTS, dt);

    // start the sim
    simulate!(state, step, rep);
}
