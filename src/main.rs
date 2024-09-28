use krabmaga::simulate;
mod model;

use crate::model::state::WorldSpace;

fn main() {
    // Initialize the simulation and its visualization here.
    let steps = 100;
    // Times to repeat the simulation with different initial conditions
    let iterations = 10;

    let num_agents = 20;
    let dim: (f32, f32) = (400., 400.);
    let state = WorldSpace::new(
        dim,
        num_agents,
        hifitime::Duration::from_seconds(1.0),
        hifitime::Epoch::from_unix_seconds(0.0),
        hifitime::TimeScale::UTC,
    );

    // Run the simulation with the built-in visualizer
    simulate!(state, steps, iterations);
}
