/*!
 * The world where agents interact.
 *
 * The world is a space where agents act, react, and interact wth each other.
 * Agents may move across the world space. The world has properties that
 * constrain agent motion, like gravity, friction, drag, and so on. Similarly,
 * the world state is the "true" state of the environment in the simulation.
 */
use std::any::Any;

use crate::model::body::Body;
use krabmaga::engine::{schedule::Schedule, state::State};
use nyx_space::cosmic::{Cosm, Orbit};
use nyx_space::time::Epoch;

/// Expand the state definition according to your model, for example by having a grid struct field to
/// store the agents' locations.
pub struct World {
    pub step: u64,
    pub num_agents: u32,
    pub dt: Epoch,
}

impl World {
    pub fn new(num_agents: u32, dt: Epoch) -> World {
        World {
            step: 0,
            num_agents,
            dt,
        }
    }
}

impl State for World {
    /// Put the code that should be executed for each state update here.
    /// The state is updated once for each schedule step.
    fn update(&mut self, _step: u64) {
        // self.field.lazy_update();
    }

    /// Operations
    fn reset(&mut self) {
        self.step = 0;
    }

    /// Put the code that should be executed to initialize simulation:
    fn init(&mut self, schedule: &mut Schedule) {
        self.step = 0;
        for i in 0..self.num_agents {
            let agent = Body::new(i, 100.0);
            // Put the agent in your state
            schedule.schedule_repeating(Box::new(agent), 0., 0);
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    fn as_state(&self) -> &dyn State {
        self
    }
    fn after_step(&mut self, _schedule: &mut Schedule) {
        self.step += 1
    }
}
