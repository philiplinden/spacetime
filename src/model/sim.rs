/*!
 * This module handles the state of the simulation environment and its schedule.
 */

use krabmaga::engine::{
    fields::field_2d::Field2D,
    schedule::Schedule,
    state::State,
};
use std::any::Any;

use super::clock::Clock;

/// Realms are the connective tissue that binds the state of the simulation and
/// the "world" or field where agents exist. Realm implements the krABMaaga
/// State trait to trigger updates to the Schedule and mutate the world State.
pub struct Realm {
    pub step: u64,
    pub field: Field2D<Clock>,
    pub dim: (f32, f32),
    pub num_agents: u32,
}

impl Default for Realm {
    fn default() -> Self {
        let dim = (100.0, 100.0);
        Realm {
            step: 0,
            field: Field2D::new(dim.0, dim.1, 0.1, true),
            dim,
            num_agents: 10,
        }
    }
}

impl Realm {
    pub fn new(dim: (f32, f32), num_agents: u32, grid_resolution: f32, wrap_edges: bool) -> Realm {
        Realm {
            step: 0,
            field: Field2D::new(dim.0, dim.1, grid_resolution, wrap_edges),
            dim,
            num_agents,
        }
    }
}

impl State for Realm {
    /// Put the code that should be executed for each state update here. The state is updated once for each
    /// schedule step.
    fn update(&mut self, _step: u64) {
        // self.field.lazy_update();
    }

    /// Put the code that should be executed to reset simulation state
    fn reset(&mut self) {
        self.step = 0;
    }

    /// Put the code that should be executed to initialize simulation:
    /// Agent creation and schedule set-up
    fn init(&mut self, schedule: &mut Schedule) {
        self.step = 0;
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
